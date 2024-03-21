mod analysis;
mod callgraph;
mod debug;
mod name_resolver;
mod phantom;
mod procedure_cache;
mod rewrites;

pub use self::callgraph::{CallGraph, CycleError};
pub use self::name_resolver::{CallerInfo, ResolvedTarget};
pub use self::procedure_cache::ProcedureCache;

use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    vec::Vec,
};
use core::ops::Index;

use smallvec::{smallvec, SmallVec};
use vm_core::Kernel;

use self::{
    analysis::MaybeRewriteCheck, name_resolver::NameResolver, phantom::PhantomCall,
    rewrites::ModuleRewriter,
};
use super::{GlobalProcedureIndex, ModuleIndex};
use crate::{
    ast::{
        Export, FullyQualifiedProcedureName, InvocationTarget, Module, Procedure, ProcedureIndex,
        ProcedureName, ResolvedProcedure,
    },
    diagnostics::{RelatedLabel, SourceFile},
    AssemblyError, LibraryPath, RpoDigest, Spanned,
};

// MODULE GRAPH
// ================================================================================================

#[derive(Default, Clone)]
pub struct ModuleGraph {
    modules: Vec<Arc<Module>>,
    /// The set of modules pending additional processing before adding them to the graph.
    ///
    /// When adding a set of inter-dependent modules to the graph, we process them as a group, so
    /// that any references between them can be resolved, and the contents of the module
    /// rewritten to reflect the changes.
    ///
    /// Once added to the graph, modules become immutable, and any additional modules added after
    /// that must by definition only depend on modules in the graph, and not be depended upon.
    #[allow(clippy::vec_box)]
    pub(crate) pending: Vec<Box<Module>>,
    /// The global call graph of calls, not counting those that are performed directly via MAST
    /// root.
    callgraph: CallGraph,
    /// The computed topological ordering of the call graph
    topo: Vec<GlobalProcedureIndex>,
    /// The set of MAST roots which have procedure definitions in this graph. There can be
    /// multiple procedures bound to the same root due to having identical code.
    roots: BTreeMap<RpoDigest, SmallVec<[GlobalProcedureIndex; 1]>>,
    /// The set of procedures in this graph which have known MAST roots
    digests: BTreeMap<GlobalProcedureIndex, RpoDigest>,
    /// The set of procedures which have no known definition in the graph, aka "phantom calls".
    /// Since we know the hash of these functions, we can proceed with compilation, but in some
    /// contexts we wish to disallow them and raise an error if any such calls are present.
    ///
    /// When we merge graphs, we attempt to resolve phantoms by attempting to find definitions in
    /// the opposite graph.
    phantoms: BTreeSet<PhantomCall>,
    kernel_index: Option<ModuleIndex>,
    kernel: Kernel,
}

/// Construction
impl ModuleGraph {
    /// Add `module` to the graph.
    ///
    /// This operation can fail for the following reasons:
    ///
    /// * Module with same [LibraryPath] is in the graph already
    /// * Too many modules in the graph
    ///
    /// NOTE: This operation only adds a module to the graph, but does not perform the
    /// important analysis needed for compilation, you must call [recompute] once all modules
    /// are added to ensure the analysis results reflect the current version of the graph.
    pub fn add_module(&mut self, module: Box<Module>) -> Result<ModuleIndex, AssemblyError> {
        let is_duplicate =
            self.is_pending(module.path()) || self.find_module_index(module.path()).is_some();
        if is_duplicate {
            return Err(AssemblyError::DuplicateModule {
                path: module.path().clone(),
            });
        }

        let module_id = self.next_module_id();
        self.pending.push(module);
        Ok(module_id)
    }

    /// Remove a module from the graph by discarding any edges involving that module. We do not
    /// remove the module from the node set by default, so as to preserve the stability of indices
    /// in the graph. However, we do remove the module from the set if it is the most recently
    /// added module, as that matches the most common case of compiling multiple programs in a row,
    /// where we discard the executable module each time.
    pub fn remove_module(&mut self, index: ModuleIndex) {
        use alloc::collections::btree_map::Entry;

        // If the given index is a pending module, we just remove it from the pending set and call
        // it a day
        let pending_offset = self.modules.len();
        if index.as_usize() >= pending_offset {
            self.pending.remove(index.as_usize() - pending_offset);
            return;
        }

        self.callgraph.remove_edges_for_module(index);

        // We remove all nodes from the topological sort that belong to the given module. The
        // resulting sort is still valid, but may change the next time it is computed
        self.topo.retain(|gid| gid.module != index);

        // Remove any cached procedure roots for the given module
        for (gid, digest) in self.digests.iter() {
            if gid.module != index {
                continue;
            }
            if let Entry::Occupied(mut entry) = self.roots.entry(*digest) {
                if entry.get().iter().all(|gid| gid.module == index) {
                    entry.remove();
                } else {
                    entry.get_mut().retain(|gid| gid.module != index);
                }
            }
        }
        self.digests.retain(|gid, _| gid.module != index);
        self.roots.retain(|_, gids| !gids.is_empty());

        // Handle removing the kernel module
        if self.kernel_index == Some(index) {
            self.kernel_index = None;
            self.kernel = Default::default();
        }

        // If the module being removed comes last in the node set, remove it from the set to avoid
        // growing the set unnecessarily over time.
        if index.as_usize() == self.modules.len().saturating_sub(1) {
            self.modules.pop();
        }
    }

    fn is_pending(&self, path: &LibraryPath) -> bool {
        self.pending.iter().any(|m| m.path() == path)
    }

    #[inline]
    fn next_module_id(&self) -> ModuleIndex {
        ModuleIndex::new(self.modules.len() + self.pending.len())
    }
}

/// Kernels
impl ModuleGraph {
    pub(super) fn set_kernel(&mut self, kernel_index: Option<ModuleIndex>, kernel: Kernel) {
        self.kernel_index = kernel_index;
        self.kernel = kernel;
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    pub fn kernel_index(&self) -> Option<ModuleIndex> {
        self.kernel_index
    }

    pub fn has_nonempty_kernel(&self) -> bool {
        self.kernel_index.is_some() || !self.kernel.is_empty()
    }

    #[allow(unused)]
    pub fn is_kernel_procedure_root(&self, digest: &RpoDigest) -> bool {
        self.kernel.contains_proc(*digest)
    }

    #[allow(unused)]
    pub fn is_kernel_procedure(&self, name: &ProcedureName) -> bool {
        self.kernel_index
            .map(|index| self[index].resolve(name).is_some())
            .unwrap_or(false)
    }

    #[allow(unused)]
    pub fn is_kernel_procedure_fully_qualified(&self, name: &FullyQualifiedProcedureName) -> bool {
        self.find_module_index(&name.module)
            .filter(|module_index| self.kernel_index == Some(*module_index))
            .map(|module_index| self[module_index].resolve(&name.name).is_some())
            .unwrap_or(false)
    }
}

/// Analysis
impl ModuleGraph {
    /// Recompute the module graph.
    ///
    /// This should be called any time `add_module`, `add_library`, etc., are called, when all such
    /// modifications to the graph are complete. For example, if you have a pair of libraries, a
    /// kernel module, and a program module that you want to compile together, you can call the
    /// various graph builder methods to add those modules to the pending set. Doing so does some
    /// initial sanity checking, but the bulk of the analysis work is done once the set of modules
    /// is final, and we can reason globally about a program or library.
    ///
    /// When this function is called, some initial information is calculated about the modules
    /// which are to be added to the graph, and then each module is visited to perform a deeper
    /// analysis than can be done by the `sema` module, as we now have the full set of modules
    /// available to do import resolution, and to rewrite invoke targets with their absolute paths
    /// and/or MAST roots. A variety of issues are caught at this stage.
    ///
    /// Once each module is validated, the various analysis results stored as part of the graph
    /// structure are updated to reflect that module being added to the graph. Once part of the
    /// graph, the module becomes immutable/clone-on-write, so as to allow the graph to be
    /// cheaply cloned.
    ///
    /// The final, and most important, analysis done by this function is the topological sort of
    /// the global call graph, which contains the inter-procedural dependencies of every procedure
    /// in the module graph. We use this sort order to do two things:
    ///
    /// 1. Verify that there are no static cycles in the graph that would prevent us from being able
    ///    to hash the generated MAST of the program. NOTE: dynamic cycles, e.g. those induced by
    ///    `dynexec`, are perfectly fine, we are only interested in preventing cycles that interfere
    ///    with the ability to generate MAST roots.
    ///
    /// 2. Visit the call graph bottom-up, so that we can fully compile a procedure before any of
    ///    its callers, and thus rewrite those callers to reference that procedure by MAST root,
    ///    rather than by name. As a result, a compiled MAST program is like an immutable snapshot
    ///    of the entire call graph at the time of compilation. Later, if we choose to recompile a
    ///    subset of modules (currently we do not have support for this in the assembler API), we
    ///    can re-analyze/re-compile only those parts of the graph which have actually changed.
    ///
    /// NOTE: This will return `Err` if we detect a validation error, a cycle in the graph, or an
    /// operation not supported by the current configuration. Basically, for any reason that would
    /// cause the resulting graph to represent an invalid program.
    pub fn recompute(&mut self) -> Result<(), AssemblyError> {
        // It is acceptable for there to be no changes, but if the graph is empty and no changes
        // are being made, we treat that as an error
        if self.modules.is_empty() && self.pending.is_empty() {
            return Err(AssemblyError::Empty);
        }

        // If no changes are being made, we're done
        if self.pending.is_empty() {
            return Ok(());
        }

        // Remove previous topological sort, since it is no longer valid
        self.topo.clear();

        // Visit all of the pending modules, assigning them ids, and adding them to the module
        // graph after rewriting any calls to use absolute paths
        let high_water_mark = self.modules.len();
        let pending = core::mem::take(&mut self.pending);
        for (pending_index, pending_module) in pending.iter().enumerate() {
            let module_id = ModuleIndex::new(high_water_mark + pending_index);

            // Apply module to call graph
            for (index, procedure) in pending_module.procedures().enumerate() {
                let procedure_id = ProcedureIndex::new(index);
                let global_id = GlobalProcedureIndex {
                    module: module_id,
                    index: procedure_id,
                };

                // Ensure all entrypoints and exported symbols are represented in the call graph,
                // even if they have no edges, we need them in the graph for the topological sort
                if matches!(procedure, Export::Procedure(_)) {
                    self.callgraph.get_or_insert_node(global_id);
                }
            }
        }

        // Obtain a set of resolvers for the pending modules so that we can do name resolution
        // before they are added to the graph
        let mut resolver = NameResolver::new(self);
        for module in pending.iter() {
            resolver.push_pending(module);
        }
        let mut phantoms = BTreeSet::default();
        let mut edges = Vec::new();
        let mut finished = Vec::<Arc<Module>>::new();

        // Visit all of the newly-added modules and perform any rewrites
        for (pending_index, mut module) in pending.into_iter().enumerate() {
            let module_id = ModuleIndex::new(high_water_mark + pending_index);

            let mut rewriter = ModuleRewriter::new(&resolver);
            rewriter.apply(module_id, &mut module)?;

            // Gather the phantom calls found while rewriting the module
            phantoms.extend(rewriter.phantoms());

            for (index, procedure) in module.procedures().enumerate() {
                let procedure_id = ProcedureIndex::new(index);
                let gid = GlobalProcedureIndex {
                    module: module_id,
                    index: procedure_id,
                };

                for invoke in procedure.invoked() {
                    let caller = CallerInfo {
                        span: invoke.span(),
                        source_file: module.source_file(),
                        module: module_id,
                        kind: invoke.kind,
                    };
                    if let Some(callee) =
                        resolver.resolve_target(&caller, &invoke.target)?.into_global_id()
                    {
                        edges.push((gid, callee));
                    }
                }
            }

            finished.push(Arc::from(module));
        }

        // Release the graph again
        drop(resolver);

        // Extend the graph with all of the new additions
        self.phantoms.extend(phantoms);
        self.modules.append(&mut finished);
        edges
            .into_iter()
            .for_each(|(callee, caller)| self.callgraph.add_edge_unchecked(callee, caller));

        // Visit all of the modules in the base module graph, and modify them if any of the
        // pending modules allow additional information to be inferred (such as the absolute path
        // of imports, etc)
        for module_index in 0..high_water_mark {
            let module_id = ModuleIndex::new(module_index);
            let module = self.modules[module_id.as_usize()].clone();

            // Re-analyze the module, and if we needed to clone-on-write, the new module will be
            // returned. Otherwise, `Ok(None)` indicates that the module is unchanged, and `Err`
            // indicates that re-analysis has found an issue with this module.
            if let Some(new_module) = self.reanalyze_module(module_id, module)? {
                self.modules[module_id.as_usize()] = new_module;
            }
        }

        // Make sure the graph is free of cycles
        let topo = self.callgraph.toposort().map_err(|cycle| {
            let iter = cycle.into_node_ids();
            let mut nodes = Vec::with_capacity(iter.len());
            for node in iter {
                let module = self[node.module].path();
                let proc = self[node].name();
                nodes.push(format!("{}::{}", module, proc));
            }
            AssemblyError::Cycle { nodes }
        })?;
        self.topo = topo;

        Ok(())
    }

    fn reanalyze_module(
        &mut self,
        module_id: ModuleIndex,
        module: Arc<Module>,
    ) -> Result<Option<Arc<Module>>, AssemblyError> {
        let resolver = NameResolver::new(self);
        let maybe_rewrite = MaybeRewriteCheck::new(&resolver);
        if maybe_rewrite.check(module_id, &module)? {
            // We need to rewrite this module again, so get an owned copy of the original
            // and use that
            let mut module = Box::new(Arc::unwrap_or_clone(module));
            let mut rewriter = ModuleRewriter::new(&resolver);
            rewriter.apply(module_id, &mut module)?;

            self.phantoms.extend(rewriter.phantoms());

            Ok(Some(Arc::from(module)))
        } else {
            Ok(None)
        }
    }
}

/// Accessors/Queries
impl ModuleGraph {
    /// Get a slice representing the topological ordering of this graph.
    ///
    /// The slice is ordered such that when a node is encountered, all of its dependencies come
    /// after it in the slice. Thus, by walking the slice in reverse, we visit the leaves of the
    /// graph before any of the dependents of those leaves. We use this property to resolve MAST
    /// roots for the entire program, bottom-up.
    pub fn topological_sort(&self) -> &[GlobalProcedureIndex] {
        self.topo.as_slice()
    }

    /// Compute the topological sort of the callgraph rooted at `caller`
    pub fn topological_sort_from_root(
        &self,
        caller: GlobalProcedureIndex,
    ) -> Result<Vec<GlobalProcedureIndex>, CycleError> {
        self.callgraph.toposort_caller(caller)
    }

    /// Fetch a [Module] by [ModuleIndex]
    #[allow(unused)]
    pub fn get_module(&self, id: ModuleIndex) -> Option<Arc<Module>> {
        self.modules.get(id.as_usize()).cloned()
    }

    /// Fetch a [Module] by [ModuleIndex]
    pub fn contains_module(&self, id: ModuleIndex) -> bool {
        self.modules.get(id.as_usize()).is_some()
    }

    /// Fetch a [Export] by [GlobalProcedureIndex]
    #[allow(unused)]
    pub fn get_procedure(&self, id: GlobalProcedureIndex) -> Option<&Export> {
        self.modules.get(id.module.as_usize()).and_then(|m| m.get(id.index))
    }

    /// Fetches a [Procedure] by [RpoDigest].
    ///
    /// NOTE: This implicitly chooses the first definition for a procedure if the same digest is
    /// shared for multiple definitions.
    #[allow(unused)]
    pub fn get_procedure_by_digest(&self, digest: &RpoDigest) -> Option<&Procedure> {
        self.roots
            .get(digest)
            .and_then(|indices| match self.get_procedure(indices[0])? {
                Export::Procedure(ref proc) => Some(proc),
                Export::Alias(_) => None,
            })
    }

    pub fn get_procedure_index_by_digest(
        &self,
        digest: &RpoDigest,
    ) -> Option<GlobalProcedureIndex> {
        self.roots.get(digest).map(|indices| indices[0])
    }

    /// Look up the [RpoDigest] associated with the given [GlobalProcedureIndex], if one is known
    /// at this point in time.
    pub fn get_mast_root(&self, id: GlobalProcedureIndex) -> Option<&RpoDigest> {
        self.digests.get(&id)
    }

    #[allow(unused)]
    pub fn callees(&self, gid: GlobalProcedureIndex) -> &[GlobalProcedureIndex] {
        self.callgraph.out_edges(gid)
    }

    /// Resolves `target` from the perspective of `caller`.
    pub fn resolve_target(
        &self,
        caller: &CallerInfo,
        target: &InvocationTarget,
    ) -> Result<ResolvedTarget, AssemblyError> {
        let resolver = NameResolver::new(self);
        resolver.resolve_target(caller, target)
    }

    /// Registers a [RpoDigest] as corresponding to a given [GlobalProcedureIndex].
    ///
    /// # SAFETY
    ///
    /// It is essential that the caller _guarantee_ that the given digest belongs to the specified
    /// procedure. It is fine if there are multiple procedures with the same digest, but it _must_
    /// be the case that if a given digest is specified, it can be used as if it was the definition
    /// of the referenced procedure, i.e. they are referentially transparent.
    pub(crate) fn register_mast_root(
        &mut self,
        id: GlobalProcedureIndex,
        digest: RpoDigest,
    ) -> Result<(), AssemblyError> {
        use alloc::collections::btree_map::Entry;
        match self.roots.entry(digest) {
            Entry::Occupied(ref mut entry) => {
                let prev_id = entry.get()[0];
                if prev_id != id {
                    // Multiple procedures with the same root, but incompatible
                    let prev = &self.modules[prev_id.module.as_usize()][prev_id.index];
                    let current = &self.modules[id.module.as_usize()][id.index];
                    if prev.num_locals() != current.num_locals() {
                        let prev_module = self.modules[prev_id.module.as_usize()].path();
                        let prev_name = FullyQualifiedProcedureName {
                            span: prev.span(),
                            module: prev_module.clone(),
                            name: prev.name().clone(),
                        };
                        let current_module = self.modules[id.module.as_usize()].path();
                        let current_name = FullyQualifiedProcedureName {
                            span: current.span(),
                            module: current_module.clone(),
                            name: current.name().clone(),
                        };
                        return Err(AssemblyError::ConflictingDefinitions {
                            first: prev_name,
                            second: current_name,
                        });
                    }

                    // Multiple procedures with the same root, but compatible
                    entry.get_mut().push(id);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(smallvec![id]);
            }
        }

        match self.digests.entry(id) {
            Entry::Occupied(ref entry) => {
                assert_eq!(
                    entry.get(),
                    &digest,
                    "attempted to register the same procedure with different digests!"
                );
            }
            Entry::Vacant(entry) => {
                entry.insert(digest);
            }
        }

        Ok(())
    }

    /// Resolves a [FullyQualifiedProcedureName] to its defining [Procedure].
    pub fn find(
        &self,
        source_file: Option<Arc<SourceFile>>,
        name: &FullyQualifiedProcedureName,
    ) -> Result<GlobalProcedureIndex, AssemblyError> {
        let mut next = Cow::Borrowed(name);
        let mut caller = source_file.clone();
        loop {
            let module_index = self.find_module_index(&next.module).ok_or_else(|| {
                AssemblyError::UndefinedModule {
                    span: next.span(),
                    source_file: caller.clone(),
                    path: name.module.clone(),
                }
            })?;
            let module = &self.modules[module_index.as_usize()];
            match module.resolve(&next.name) {
                Some(ResolvedProcedure::Local(index)) => {
                    let id = GlobalProcedureIndex {
                        module: module_index,
                        index: index.into_inner(),
                    };
                    break Ok(id);
                }
                Some(ResolvedProcedure::External(fqn)) => {
                    // If we see that we're about to enter an infinite resolver loop because of a
                    // recursive alias, return an error
                    if name == &fqn {
                        break Err(AssemblyError::RecursiveAlias {
                            source_file: caller.clone(),
                            name: name.clone(),
                        });
                    }
                    next = Cow::Owned(fqn);
                    caller = module.source_file();
                }
                None => {
                    // No such procedure known to `module`
                    break Err(AssemblyError::Failed {
                        labels: vec![RelatedLabel::error("undefined procedure")
                            .with_source_file(source_file)
                            .with_labeled_span(next.span(), "unable to resolve this reference")],
                    });
                }
            }
        }
    }

    /// Resolve a [LibraryPath] to a [ModuleIndex] in this graph
    pub fn find_module_index(&self, name: &LibraryPath) -> Option<ModuleIndex> {
        self.modules.iter().position(|m| m.path() == name).map(ModuleIndex::new)
    }

    /// Resolve a [LibraryPath] to a [Module] in this graph
    pub fn find_module(&self, name: &LibraryPath) -> Option<Arc<Module>> {
        self.modules.iter().find(|m| m.path() == name).cloned()
    }

    /// Returns an iterator over the set of [Module]s in this graph, and their indices
    #[allow(unused)]
    pub fn modules(&self) -> impl Iterator<Item = (ModuleIndex, Arc<Module>)> + '_ {
        self.modules
            .iter()
            .enumerate()
            .map(|(idx, m)| (ModuleIndex::new(idx), m.clone()))
    }

    /// Like [modules], but returns a reference to the module, rather than an owned pointer
    #[allow(unused)]
    pub fn modules_by_ref(&self) -> impl Iterator<Item = (ModuleIndex, &Module)> + '_ {
        self.modules
            .iter()
            .enumerate()
            .map(|(idx, m)| (ModuleIndex::new(idx), m.as_ref()))
    }

    /// Returns an iterator over the set of [Procedure]s in this graph, and their indices
    #[allow(unused)]
    pub fn procedures(&self) -> impl Iterator<Item = (GlobalProcedureIndex, &Procedure)> + '_ {
        self.modules_by_ref().flat_map(|(module_index, module)| {
            module.procedures().enumerate().filter_map(move |(index, p)| {
                let index = ProcedureIndex::new(index);
                let id = GlobalProcedureIndex {
                    module: module_index,
                    index,
                };
                match p {
                    Export::Procedure(ref p) => Some((id, p)),
                    Export::Alias(_) => None,
                }
            })
        })
    }
}

impl Index<ModuleIndex> for ModuleGraph {
    type Output = Arc<Module>;

    fn index(&self, index: ModuleIndex) -> &Self::Output {
        self.modules.index(index.as_usize())
    }
}

impl Index<GlobalProcedureIndex> for ModuleGraph {
    type Output = Export;

    fn index(&self, index: GlobalProcedureIndex) -> &Self::Output {
        self.modules[index.module.as_usize()].index(index.index)
    }
}
