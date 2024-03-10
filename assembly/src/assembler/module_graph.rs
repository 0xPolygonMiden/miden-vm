mod debug;

use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    vec::Vec,
};
use core::ops::{ControlFlow, Index};

use super::{callgraph::CycleError, CallGraph, GlobalProcedureIndex, ModuleIndex};
use crate::{
    ast::{
        visit, Export, FullyQualifiedProcedureName, Ident, InvocationTarget, Invoke, InvokeKind,
        Module, Procedure, ProcedureIndex, ProcedureName, ResolvedProcedure, Visit, VisitMut,
    },
    diagnostics::{RelatedLabel, SourceFile},
    AssemblyError, LibraryNamespace, LibraryPath, RpoDigest, SourceSpan, Span, Spanned,
};
use smallvec::{smallvec, SmallVec};
use vm_core::Kernel;

#[derive(Clone)]
#[allow(dead_code)]
pub struct PhantomCall {
    span: SourceSpan,
    source_file: Option<Arc<SourceFile>>,
    callee: RpoDigest,
}

impl Eq for PhantomCall {}

impl PartialEq for PhantomCall {
    fn eq(&self, other: &Self) -> bool {
        self.callee.eq(&other.callee)
    }
}

impl Ord for PhantomCall {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.callee.cmp(&other.callee)
    }
}

impl PartialOrd for PhantomCall {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, Clone)]
pub struct ModuleGraph {
    modules: Vec<Arc<Module>>,
    /// The set of modules pending additional processing
    /// before adding them to the graph.
    ///
    /// When adding a set of inter-dependent modules to the
    /// graph, we process them as a group, so that any references
    /// between them can be resolved, and the contents of the module
    /// rewritten to reflect the changes.
    ///
    /// Once added to the graph, modules become immutable, and any
    /// additional modules added after that must by definition only
    /// depend on modules in the graph, and not be depended upon.
    #[allow(clippy::vec_box)]
    pub(crate) pending: Vec<Box<Module>>,
    /// The global call graph of calls, not counting those
    /// that are performed directly via MAST root
    callgraph: CallGraph,
    /// The computed topological ordering of the call graph
    topo: Vec<GlobalProcedureIndex>,
    /// The set of MAST roots which have procedure definitions
    /// in this graph. There can be multiple procedures bound to
    /// the same root due to having identical code.
    roots: BTreeMap<RpoDigest, SmallVec<[GlobalProcedureIndex; 1]>>,
    /// The set of procedures in this graph which have known
    /// MAST roots
    digests: BTreeMap<GlobalProcedureIndex, RpoDigest>,
    /// The set of procedures which have no known definition in
    /// the graph, aka "phantom calls". Since we know the hash
    /// of these functions, we can proceed with compilation, but
    /// in some contexts we wish to disallow them and raise an
    /// error if any such calls are present.
    ///
    /// When we merge graphs, we attempt to resolve phantoms by
    /// attempting to find definitions in the opposite graph.
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
    /// NOTE: This operation only adds a module to the graph, but
    /// does not perform the important analysis needed for compilation,
    /// you must call [recompute] once all modules are added to ensure
    /// the analysis results reflect the current version of the graph.
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

    /// Remove a module from the graph by discarding any edges
    /// involving that module. We do not remove the module from
    /// the node set by default, so as to preserve the stability
    /// of indices in the graph. However, we do remove the module
    /// from the set if it is the most recently added module, as
    /// that matches the most common case of compiling multiple
    /// programs in a row, where we discard the executable module
    /// each time.
    pub fn remove_module(&mut self, index: ModuleIndex) {
        use alloc::collections::btree_map::Entry;

        // If the given index is a pending module, we
        // just remove it from the pending set and call it a day
        let pending_offset = self.modules.len();
        if index.as_usize() >= pending_offset {
            self.pending.remove(index.as_usize() - pending_offset);
            return;
        }

        self.callgraph.remove_edges_for_module(index);

        // We remove all nodes from the topological sort
        // that belong to the given module. The resulting
        // sort is still valid, but may change the next
        // time it is computed
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

        // If the module being removed comes last in the node set,
        // remove it from the set to avoid growing the set unnecessarily
        // over time.
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
    /// This should be called any time `add_module`, `add_library`, etc., are
    /// called, when all such modifications to the graph are complete. For example,
    /// if you have a pair of libraries, a kernel module, and a program module that
    /// you want to compile together, you can call the various graph builder methods
    /// to add those modules to the pending set. Doing so does some initial sanity
    /// checking, but the bulk of the analysis work is done once the set of modules is
    /// final, and we can reason globally about a program or library.
    ///
    /// When this function is called, some initial information is calculated about the
    /// modules which are to be added to the graph, and then each module is visited to
    /// perform a deeper analysis than can be done by the `sema` module, as we now have
    /// the full set of modules available to do import resolution, and to rewrite invoke
    /// targets with their absolute paths and/or mast roots. A variety of issues are
    /// caught at this stage.
    ///
    /// Once each module is validated, the various analysis results stored as part of the
    /// graph structure are updated to reflect that module being added to the graph. Once
    /// part of the graph, the module becomes immutable/clone-on-write, so as to allow
    /// the graph to be cheaply cloned.
    ///
    /// The final, and most important, analysis done by this function is the topological
    /// sort of the global call graph, which contains the inter-procedural dependencies
    /// of every procedure in the module graph. We use this sort order to do two things:
    ///
    /// 1. Verify that there are no static cycles in the graph that would prevent us from
    /// being able to hash the generated MAST of the program. NOTE: dynamic cycles,
    /// e.g. those induced by `dynexec`, are perfectly fine, we are only interested in
    /// preventing cycles that interfere with the ability to generate MAST roots.
    ///
    /// 2. Visit the call graph bottom-up, so that we can fully compile a procedure before
    /// any of its callers, and thus rewrite those callers to reference that procedure by
    /// MAST root, rather than by name. As a result, a compiled MAST program is like an
    /// immutable snapshot of the entire call graph at the time of compilation. Later, if
    /// we choose to recompile a subset of modules (currently we do not have support for
    /// this in the assembler API), we can re-analyze/re-compile only those parts of the
    /// graph which have actually changed.
    ///
    /// NOTE: This will return `Err` if we detect a validation error, a cycle in the graph,
    /// or an operation not supported by the current configuration. Basically, for any reason
    ///  that would cause the resulting graph to represent an invalid program.
    pub fn recompute(&mut self) -> Result<(), AssemblyError> {
        // It is acceptable for there to be no changes, but if
        // the graph is empty and no changes are being made, we
        // treat that as an error
        if self.modules.is_empty() && self.pending.is_empty() {
            return Err(AssemblyError::Empty);
        }

        // If no changes are being made, we're done
        if self.pending.is_empty() {
            return Ok(());
        }

        // Remove previous topological sort, since it is no longer valid
        self.topo.clear();

        // Visit all of the pending modules, assigning them ids, and
        // adding them to the module graph after rewriting any calls
        // to use absolute paths
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

                // Ensure all entrypoints and exported symbols are represented
                // in the call graph, even if they have no edges, we need them
                // in the graph for the topological sort
                if matches!(procedure, Export::Procedure(_)) {
                    self.callgraph.get_or_insert_node(global_id);
                }
            }
        }

        // Obtain a set of resolvers for the pending modules so that we
        // can do name resolution before they are added to the graph
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

            let source_file = module.source_file();
            let mut visitor = ModuleRewriteVisitor {
                resolver: &resolver,
                module_id,
                invoked: Default::default(),
                phantoms: Default::default(),
                source_file,
            };
            if let ControlFlow::Break(err) = visitor.visit_mut_module(&mut module) {
                return Err(err);
            }

            phantoms.extend(visitor.phantoms.into_iter());

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

        // Visit all of the modules in the base module graph, and modify
        // them if any of the pending modules allow additional information
        // to be inferred (such as the absolute path of imports, etc)
        for module_index in 0..high_water_mark {
            let module_id = ModuleIndex::new(module_index);
            let module = self.modules[module_id.as_usize()].clone();

            // Re-analyze the module, and if we needed to clone-on-write,
            // the new module will be returned. Otherwise, `Ok(None)`
            // indicates that the module is unchaged, and `Err` indicates
            // that re-analysis has found an issue with this module.
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
        let mut visitor = ReanalyzeCheck {
            resolver: &resolver,
            module_id,
            source_file: module.source_file(),
        };
        match visitor.visit_module(&module) {
            ControlFlow::Break(result) => {
                if result? {
                    // We need to rewrite this module again, so get an owned copy of the original
                    // and use that
                    let mut module = Box::new(Arc::unwrap_or_clone(module));
                    let mut visitor = ModuleRewriteVisitor {
                        resolver: &resolver,
                        module_id,
                        invoked: Default::default(),
                        phantoms: Default::default(),
                        source_file: module.source_file(),
                    };
                    if let ControlFlow::Break(err) = visitor.visit_mut_module(&mut module) {
                        return Err(err);
                    }
                    self.phantoms.extend(visitor.phantoms);
                    Ok(Some(Arc::from(module)))
                } else {
                    Ok(None)
                }
            }
            ControlFlow::Continue(_) => Ok(None),
        }
    }
}

/// Accessors/Queries
impl ModuleGraph {
    /// Get a slice representing the topological ordering of this graph.
    ///
    /// The slice is ordered such that when a node is encountered, all of
    /// its dependencies come after it in the slice. Thus, by walking the
    /// slice in reverse, we visit the leaves of the graph before any of
    /// the dependents of those leaves. We use this property to resolve
    /// MAST roots for the entire program, bottom-up.
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

    /// Fetch a [Procedure] by [RpoDigest]
    ///
    /// NOTE: This implicitly chooses the first definition for a procedure
    /// if the same digest is shared for multiple definitions.
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

    /// Look up the [RpoDigest] associated with the given [GlobalProcedureIndex],
    /// if one is known at this point in time.
    pub fn get_mast_root(&self, id: GlobalProcedureIndex) -> Option<&RpoDigest> {
        self.digests.get(&id)
    }

    #[allow(unused)]
    pub fn callees(&self, gid: GlobalProcedureIndex) -> &[GlobalProcedureIndex] {
        self.callgraph.out_edges(gid)
    }

    /// Resolve `target` from the perspective of `caller`
    pub fn resolve_target(
        &self,
        caller: &CallerInfo,
        target: &InvocationTarget,
    ) -> Result<ResolvedTarget, AssemblyError> {
        let resolver = NameResolver::new(self);
        resolver.resolve_target(caller, target)
    }

    /// Register a [RpoDigest] as corresponding to a given [GlobalProcedureIndex].
    ///
    /// # SAFETY
    ///
    /// It is essential that the caller _guarantee_ that the given digest belongs
    /// to the specified procedure. It is fine if there are multiple procedures
    /// with the same digest, but it _must_ be the case that if a given digest
    /// is specified, it can be used as if it was the definition of the referenced
    /// procedure, i.e. they are referentially transparent.
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

    /// Resolve a [FullyQualifiedProcedureName] to its defining [Procedure]
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
                    // If we see that we're about to enter an infinite
                    // resolver loop because of a recursive alias, return
                    // an error
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

struct ThinModule {
    source_file: Option<Arc<SourceFile>>,
    path: LibraryPath,
    resolver: crate::ast::LocalNameResolver,
}

#[derive(Debug, Clone)]
pub struct CallerInfo {
    pub span: SourceSpan,
    pub source_file: Option<Arc<SourceFile>>,
    pub module: ModuleIndex,
    pub kind: InvokeKind,
}

#[derive(Debug)]
pub enum ResolvedTarget {
    Cached {
        digest: RpoDigest,
        gid: Option<GlobalProcedureIndex>,
    },
    Exact {
        gid: GlobalProcedureIndex,
    },
    Resolved {
        gid: GlobalProcedureIndex,
        target: InvocationTarget,
    },
    Phantom(RpoDigest),
}

impl ResolvedTarget {
    pub fn into_global_id(self) -> Option<GlobalProcedureIndex> {
        match self {
            ResolvedTarget::Exact { gid } | ResolvedTarget::Resolved { gid, .. } => Some(gid),
            ResolvedTarget::Cached { gid, .. } => gid,
            ResolvedTarget::Phantom(_) => None,
        }
    }
}

struct NameResolver<'a> {
    graph: &'a ModuleGraph,
    pending: Vec<ThinModule>,
}

impl<'a> NameResolver<'a> {
    pub fn new(graph: &'a ModuleGraph) -> Self {
        Self {
            graph,
            pending: vec![],
        }
    }

    pub fn push_pending(&mut self, module: &Module) {
        self.pending.push(ThinModule {
            source_file: module.source_file(),
            path: module.path().clone(),
            resolver: module.resolver(),
        });
    }

    pub fn resolve(
        &self,
        caller: &CallerInfo,
        callee: &ProcedureName,
    ) -> Result<ResolvedTarget, AssemblyError> {
        match self.resolve_local(caller, callee) {
            Some(ResolvedProcedure::Local(index)) if matches!(caller.kind, InvokeKind::SysCall) => {
                let gid = GlobalProcedureIndex {
                    module: self.graph.kernel_index.unwrap(),
                    index: index.into_inner(),
                };
                match self.graph.get_mast_root(gid) {
                    Some(digest) => Ok(ResolvedTarget::Cached {
                        digest: *digest,
                        gid: Some(gid),
                    }),
                    None => Ok(ResolvedTarget::Exact { gid }),
                }
            }
            Some(ResolvedProcedure::Local(index)) => {
                let gid = GlobalProcedureIndex {
                    module: caller.module,
                    index: index.into_inner(),
                };
                match self.graph.get_mast_root(gid) {
                    Some(digest) => Ok(ResolvedTarget::Cached {
                        digest: *digest,
                        gid: Some(gid),
                    }),
                    None => Ok(ResolvedTarget::Exact { gid }),
                }
            }
            Some(ResolvedProcedure::External(ref fqn)) => {
                let gid = self.find(caller, fqn)?;
                match self.graph.get_mast_root(gid) {
                    Some(digest) => Ok(ResolvedTarget::Cached {
                        digest: *digest,
                        gid: Some(gid),
                    }),
                    None => {
                        let path = self.module_path(gid.module);
                        let pending_offset = self.graph.modules.len();
                        let name = if gid.module.as_usize() >= pending_offset {
                            self.pending[gid.module.as_usize() - pending_offset]
                                .resolver
                                .get_name(gid.index)
                                .clone()
                        } else {
                            self.graph[gid].name().clone()
                        };
                        Ok(ResolvedTarget::Resolved {
                            gid,
                            target: InvocationTarget::AbsoluteProcedurePath { name, path },
                        })
                    }
                }
            }
            None => Err(AssemblyError::Failed {
                labels: vec![RelatedLabel::error("undefined procedure")
                    .with_source_file(caller.source_file.clone())
                    .with_labeled_span(caller.span, "unable to resolve this name locally")],
            }),
        }
    }

    pub fn resolve_import(&self, caller: &CallerInfo, name: &Ident) -> Option<Span<&LibraryPath>> {
        let pending_offset = self.graph.modules.len();
        if caller.module.as_usize() >= pending_offset {
            self.pending[caller.module.as_usize() - pending_offset]
                .resolver
                .resolve_import(name)
        } else {
            self.graph[caller.module]
                .resolve_import(name)
                .map(|import| Span::new(import.span(), import.path()))
        }
    }

    pub fn resolve_target(
        &self,
        caller: &CallerInfo,
        target: &InvocationTarget,
    ) -> Result<ResolvedTarget, AssemblyError> {
        match target {
            InvocationTarget::MastRoot(mast_root) => {
                match self.graph.get_procedure_index_by_digest(mast_root) {
                    None => Ok(ResolvedTarget::Phantom(mast_root.into_inner())),
                    Some(gid) => Ok(ResolvedTarget::Exact { gid }),
                }
            }
            InvocationTarget::ProcedureName(ref callee) => self.resolve(caller, callee),
            InvocationTarget::ProcedurePath {
                ref name,
                module: ref imported_module,
            } => match self.resolve_import(caller, imported_module) {
                Some(imported_module) => {
                    let fqn = FullyQualifiedProcedureName {
                        span: target.span(),
                        module: imported_module.into_inner().clone(),
                        name: name.clone(),
                    };
                    let gid = self.find(caller, &fqn)?;
                    match self.graph.get_mast_root(gid) {
                        Some(digest) => Ok(ResolvedTarget::Cached {
                            digest: *digest,
                            gid: Some(gid),
                        }),
                        None => {
                            let path = self.module_path(gid.module);
                            let pending_offset = self.graph.modules.len();
                            let name = if gid.module.as_usize() >= pending_offset {
                                self.pending[gid.module.as_usize() - pending_offset]
                                    .resolver
                                    .get_name(gid.index)
                                    .clone()
                            } else {
                                self.graph[gid].name().clone()
                            };
                            Ok(ResolvedTarget::Resolved {
                                gid,
                                target: InvocationTarget::AbsoluteProcedurePath { name, path },
                            })
                        }
                    }
                }
                None => Err(AssemblyError::UndefinedModule {
                    span: target.span(),
                    source_file: caller.source_file.clone(),
                    path: LibraryPath::new_from_components(
                        LibraryNamespace::User(imported_module.clone().into_inner()),
                        [],
                    ),
                }),
            },
            InvocationTarget::AbsoluteProcedurePath { ref name, ref path } => {
                let fqn = FullyQualifiedProcedureName {
                    span: target.span(),
                    module: path.clone(),
                    name: name.clone(),
                };
                let gid = self.find(caller, &fqn)?;
                match self.graph.get_mast_root(gid) {
                    Some(digest) => Ok(ResolvedTarget::Cached {
                        digest: *digest,
                        gid: Some(gid),
                    }),
                    None => Ok(ResolvedTarget::Exact { gid }),
                }
            }
        }
    }

    fn resolve_local(
        &self,
        caller: &CallerInfo,
        callee: &ProcedureName,
    ) -> Option<ResolvedProcedure> {
        let module = if matches!(caller.kind, InvokeKind::SysCall) {
            // Resolve local names relative to the kernel
            self.graph.kernel_index?
        } else {
            caller.module
        };
        self.resolve_local_with_index(module, callee)
    }

    fn resolve_local_with_index(
        &self,
        module: ModuleIndex,
        callee: &ProcedureName,
    ) -> Option<ResolvedProcedure> {
        let pending_offset = self.graph.modules.len();
        let module_index = module.as_usize();
        if module_index >= pending_offset {
            self.pending[module_index - pending_offset].resolver.resolve(callee)
        } else {
            self.graph[module].resolve(callee)
        }
    }

    pub fn module_source(&self, module: ModuleIndex) -> Option<Arc<SourceFile>> {
        let pending_offset = self.graph.modules.len();
        let module_index = module.as_usize();
        if module_index >= pending_offset {
            self.pending[module_index - pending_offset].source_file.clone()
        } else {
            self.graph[module].source_file()
        }
    }

    fn module_path(&self, module: ModuleIndex) -> LibraryPath {
        let pending_offset = self.graph.modules.len();
        let module_index = module.as_usize();
        if module_index >= pending_offset {
            self.pending[module_index - pending_offset].path.clone()
        } else {
            self.graph[module].path().clone()
        }
    }

    /// Resolve `name` to its concrete definition, returning the corresponding [GlobalProcedureIndex]
    ///
    /// If an error occurs during resolution, or the name cannot be resolved, `Err` is returned.
    pub fn find(
        &self,
        caller: &CallerInfo,
        callee: &FullyQualifiedProcedureName,
    ) -> Result<GlobalProcedureIndex, AssemblyError> {
        // If the caller is a syscall, set the invoke kind
        // to exec until we have resolved the procedure, then
        // verify that it is in the kernel module
        let mut current_caller = if matches!(caller.kind, InvokeKind::SysCall) {
            let mut caller = caller.clone();
            caller.kind = InvokeKind::Exec;
            Cow::Owned(caller)
        } else {
            Cow::Borrowed(caller)
        };
        let mut current_callee = Cow::Borrowed(callee);
        let mut visited = BTreeSet::default();
        loop {
            let module_index = self.find_module_index(&current_callee.module).ok_or_else(|| {
                AssemblyError::UndefinedModule {
                    span: current_callee.span(),
                    source_file: current_caller.source_file.clone(),
                    path: current_callee.module.clone(),
                }
            })?;
            let resolved = self.resolve_local_with_index(module_index, &current_callee.name);
            match resolved {
                Some(ResolvedProcedure::Local(index)) => {
                    let id = GlobalProcedureIndex {
                        module: module_index,
                        index: index.into_inner(),
                    };
                    if matches!(current_caller.kind, InvokeKind::SysCall if self.graph.kernel_index != Some(module_index))
                    {
                        break Err(AssemblyError::InvalidSysCallTarget {
                            span: current_callee.span(),
                            source_file: current_caller.source_file.clone(),
                            callee: current_callee.into_owned(),
                        });
                    }
                    break Ok(id);
                }
                Some(ResolvedProcedure::External(fqn)) => {
                    // If we see that we're about to enter an infinite
                    // resolver loop because of a recursive alias, return
                    // an error
                    if !visited.insert(fqn.clone()) {
                        break Err(AssemblyError::Failed {
                            labels: vec![
                                RelatedLabel::error("recursive alias")
                                    .with_source_file(self.module_source(module_index))
                                    .with_labeled_span(fqn.span(), "occurs because this import causes import resolution to loop back on itself"),
                                RelatedLabel::advice("recursive alias")
                                    .with_source_file(caller.source_file.clone())
                                    .with_labeled_span(caller.span, "as a result of resolving this procedure reference"),
                            ],
                        });
                    }
                    let source_file = self
                        .find_module_index(&fqn.module)
                        .and_then(|index| self.module_source(index));
                    current_caller = Cow::Owned(CallerInfo {
                        span: fqn.span(),
                        source_file,
                        module: module_index,
                        kind: current_caller.kind,
                    });
                    current_callee = Cow::Owned(fqn);
                }
                None if matches!(current_caller.kind, InvokeKind::SysCall) => {
                    if self.graph.has_nonempty_kernel() {
                        // No kernel, so this invoke is invalid anyway
                        break Err(AssemblyError::Failed {
                            labels: vec![
                                RelatedLabel::error("undefined kernel procedure")
                                    .with_source_file(caller.source_file.clone())
                                    .with_labeled_span(caller.span, "unable to resolve this reference to a procedure in the current kernel"),
                                RelatedLabel::error("invalid syscall")
                                    .with_source_file(self.module_source(module_index))
                                    .with_labeled_span(
                                        current_callee.span(),
                                        "this name cannot be resolved, because the assembler has an empty kernel",
                                    ),
                            ]
                        });
                    } else {
                        // No such kernel procedure
                        break Err(AssemblyError::Failed {
                            labels: vec![
                                RelatedLabel::error("undefined kernel procedure")
                                    .with_source_file(caller.source_file.clone())
                                    .with_labeled_span(caller.span, "unable to resolve this reference to a procedure in the current kernel"),
                                RelatedLabel::error("name resolution cannot proceed")
                                    .with_source_file(self.module_source(module_index))
                                    .with_labeled_span(
                                        current_callee.span(),
                                        "this name cannot be resolved",
                                    ),
                            ]
                        });
                    }
                }
                None => {
                    // No such procedure known to `module`
                    break Err(AssemblyError::Failed {
                        labels: vec![
                            RelatedLabel::error("undefined procedure")
                                .with_source_file(caller.source_file.clone())
                                .with_labeled_span(
                                    caller.span,
                                    "unable to resolve this reference to its definition",
                                ),
                            RelatedLabel::error("name resolution cannot proceed")
                                .with_source_file(self.module_source(module_index))
                                .with_labeled_span(
                                    current_callee.span(),
                                    "this name cannot be resolved",
                                ),
                        ],
                    });
                }
            }
        }
    }

    /// Resolve a [LibraryPath] to a [ModuleIndex] in this graph
    pub fn find_module_index(&self, name: &LibraryPath) -> Option<ModuleIndex> {
        self.graph
            .modules
            .iter()
            .map(|m| m.path())
            .chain(self.pending.iter().map(|m| &m.path))
            .position(|path| path == name)
            .map(ModuleIndex::new)
    }
}

struct ReanalyzeCheck<'a, 'b: 'a> {
    resolver: &'a NameResolver<'b>,
    module_id: ModuleIndex,
    source_file: Option<Arc<SourceFile>>,
}

impl<'a, 'b: 'a> ReanalyzeCheck<'a, 'b> {
    fn resolve_target(
        &self,
        kind: InvokeKind,
        target: &InvocationTarget,
    ) -> ControlFlow<Result<bool, AssemblyError>> {
        let caller = CallerInfo {
            span: target.span(),
            source_file: self.source_file.clone(),
            module: self.module_id,
            kind,
        };
        match self.resolver.resolve_target(&caller, target) {
            Err(err) => ControlFlow::Break(Err(err)),
            Ok(ResolvedTarget::Resolved { .. }) => ControlFlow::Break(Ok(true)),
            Ok(ResolvedTarget::Exact { .. } | ResolvedTarget::Phantom(_)) => {
                ControlFlow::Continue(())
            }
            Ok(ResolvedTarget::Cached { .. }) => {
                if let InvocationTarget::MastRoot(_) = target {
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(Ok(true))
                }
            }
        }
    }
}

impl<'a, 'b: 'a> Visit<Result<bool, AssemblyError>> for ReanalyzeCheck<'a, 'b> {
    fn visit_syscall(
        &mut self,
        target: &InvocationTarget,
    ) -> ControlFlow<Result<bool, AssemblyError>> {
        self.resolve_target(InvokeKind::SysCall, target)
    }
    fn visit_call(
        &mut self,
        target: &InvocationTarget,
    ) -> ControlFlow<Result<bool, AssemblyError>> {
        self.resolve_target(InvokeKind::Call, target)
    }
    fn visit_invoke_target(
        &mut self,
        target: &InvocationTarget,
    ) -> ControlFlow<Result<bool, AssemblyError>> {
        self.resolve_target(InvokeKind::Exec, target)
    }
}

struct ModuleRewriteVisitor<'a, 'b: 'a> {
    resolver: &'a NameResolver<'b>,
    module_id: ModuleIndex,
    invoked: BTreeSet<Invoke>,
    phantoms: BTreeSet<PhantomCall>,
    source_file: Option<Arc<SourceFile>>,
}

impl<'a, 'b: 'a> ModuleRewriteVisitor<'a, 'b> {
    fn rewrite_target(
        &mut self,
        kind: InvokeKind,
        target: &mut InvocationTarget,
    ) -> ControlFlow<AssemblyError> {
        let caller = CallerInfo {
            span: target.span(),
            source_file: self.source_file.clone(),
            module: self.module_id,
            kind,
        };
        match self.resolver.resolve_target(&caller, target) {
            Err(err) => return ControlFlow::Break(err),
            Ok(ResolvedTarget::Cached { digest, .. }) => {
                *target = InvocationTarget::MastRoot(Span::new(target.span(), digest));
                self.invoked.insert(Invoke {
                    kind,
                    target: target.clone(),
                });
            }
            Ok(ResolvedTarget::Phantom(callee)) => {
                let call = PhantomCall {
                    span: target.span(),
                    source_file: self.source_file.clone(),
                    callee,
                };
                self.phantoms.insert(call);
            }
            Ok(ResolvedTarget::Exact { .. }) => {
                self.invoked.insert(Invoke {
                    kind,
                    target: target.clone(),
                });
            }
            Ok(ResolvedTarget::Resolved {
                target: new_target, ..
            }) => {
                *target = new_target;
                self.invoked.insert(Invoke {
                    kind,
                    target: target.clone(),
                });
            }
        }

        ControlFlow::Continue(())
    }
}

impl<'a, 'b: 'a> VisitMut<AssemblyError> for ModuleRewriteVisitor<'a, 'b> {
    fn visit_mut_procedure(&mut self, procedure: &mut Procedure) -> ControlFlow<AssemblyError> {
        self.invoked.clear();
        self.invoked.extend(procedure.invoked().cloned());
        visit::visit_mut_procedure(self, procedure)?;
        procedure.extend_invoked(core::mem::take(&mut self.invoked));
        ControlFlow::Continue(())
    }
    fn visit_mut_syscall(&mut self, target: &mut InvocationTarget) -> ControlFlow<AssemblyError> {
        self.rewrite_target(InvokeKind::SysCall, target)
    }
    fn visit_mut_call(&mut self, target: &mut InvocationTarget) -> ControlFlow<AssemblyError> {
        self.rewrite_target(InvokeKind::Call, target)
    }
    fn visit_mut_invoke_target(
        &mut self,
        target: &mut InvocationTarget,
    ) -> ControlFlow<AssemblyError> {
        self.rewrite_target(InvokeKind::Exec, target)
    }
}
