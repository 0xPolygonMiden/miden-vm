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
use vm_core::Kernel;

use smallvec::{smallvec, SmallVec};

use self::{
    analysis::MaybeRewriteCheck, name_resolver::NameResolver, phantom::PhantomCall,
    rewrites::ModuleRewriter,
};
use super::{GlobalProcedureIndex, ModuleIndex};
use crate::compiled_library::{CompiledModule, CompiledProcedure};
use crate::{
    ast::{
        Export, FullyQualifiedProcedureName, InvocationTarget, Module, ProcedureIndex,
        ProcedureName, ResolvedProcedure,
    },
    diagnostics::{RelatedLabel, SourceFile},
    AssemblyError, LibraryPath, RpoDigest, Spanned,
};

// TODOP: Better doc
pub enum WrapperProcedure<'a> {
    Ast(&'a Export),
    Compiled(&'a CompiledProcedure),
}

impl<'a> WrapperProcedure<'a> {
    pub fn name(&self) -> &ProcedureName {
        match self {
            WrapperProcedure::Ast(p) => p.name(),
            WrapperProcedure::Compiled(p) => p.name(),
        }
    }

    pub fn unwrap_ast(&self) -> &Export {
        match self {
            WrapperProcedure::Ast(proc) => proc,
            WrapperProcedure::Compiled(_) => panic!("expected AST procedure, but was compiled"),
        }
    }
}

// TODOP: Rename
#[derive(Clone)]
pub enum WrapperModule {
    Ast(Arc<Module>),
    Exports(CompiledModule),
}

impl WrapperModule {
    pub fn path(&self) -> &LibraryPath {
        match self {
            WrapperModule::Ast(m) => m.path(),
            WrapperModule::Exports(m) => m.path(),
        }
    }

    pub fn unwrap_ast(&self) -> &Arc<Module> {
        match self {
            WrapperModule::Ast(module) => module,
            WrapperModule::Exports(_) => {
                panic!("expected module to be in AST representation, but was compiled")
            }
        }
    }
}

// TODOP: Try to do without this `Pending*` version
#[derive(Clone)]
pub enum PendingWrapperModule {
    Ast(Box<Module>),
    Exports(CompiledModule),
}

impl PendingWrapperModule {
    pub fn path(&self) -> &LibraryPath {
        match self {
            PendingWrapperModule::Ast(m) => m.path(),
            PendingWrapperModule::Exports(m) => m.path(),
        }
    }
}

// MODULE GRAPH
// ================================================================================================

#[derive(Default, Clone)]
pub struct ModuleGraph {
    modules: Vec<WrapperModule>,
    /// The set of modules pending additional processing before adding them to the graph.
    ///
    /// When adding a set of inter-dependent modules to the graph, we process them as a group, so
    /// that any references between them can be resolved, and the contents of the module
    /// rewritten to reflect the changes.
    ///
    /// Once added to the graph, modules become immutable, and any additional modules added after
    /// that must by definition only depend on modules in the graph, and not be depended upon.
    pending: Vec<PendingWrapperModule>,
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
    /// NOTE: This operation only adds a module to the graph, but does not perform the
    /// important analysis needed for compilation, you must call [recompute] once all modules
    /// are added to ensure the analysis results reflect the current version of the graph.
    ///
    /// # Errors
    ///
    /// This operation can fail for the following reasons:
    ///
    /// * Module with same [LibraryPath] is in the graph already
    /// * Too many modules in the graph
    ///
    /// # Panics
    ///
    /// This function will panic if the number of modules exceeds the maximum representable
    /// [ModuleIndex] value, `u16::MAX`.
    pub fn add_ast_module(&mut self, module: Box<Module>) -> Result<ModuleIndex, AssemblyError> {
        self.add_module(PendingWrapperModule::Ast(module))
    }

    /// Add compiled `module` to the graph.
    ///
    /// NOTE: This operation only adds a module to the graph, but does not perform the
    /// important analysis needed for compilation, you must call [recompute] once all modules
    /// are added to ensure the analysis results reflect the current version of the graph.
    ///
    /// # Errors
    ///
    /// This operation can fail for the following reasons:
    ///
    /// * Module with same [LibraryPath] is in the graph already
    /// * Too many modules in the graph
    ///
    /// # Panics
    ///
    /// This function will panic if the number of modules exceeds the maximum representable
    /// [ModuleIndex] value, `u16::MAX`.
    pub fn add_compiled_module(
        &mut self,
        module: CompiledModule,
    ) -> Result<ModuleIndex, AssemblyError> {
        self.add_module(PendingWrapperModule::Exports(module))
    }

    fn add_module(&mut self, module: PendingWrapperModule) -> Result<ModuleIndex, AssemblyError> {
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

    #[allow(unused)]
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
            match pending_module {
                PendingWrapperModule::Ast(pending_module) => {
                    for (index, procedure) in pending_module.procedures().enumerate() {
                        let procedure_id = ProcedureIndex::new(index);
                        let global_id = GlobalProcedureIndex {
                            module: module_id,
                            index: procedure_id,
                        };

                        // Ensure all entrypoints and exported symbols are represented in the call
                        // graph, even if they have no edges, we need them
                        // in the graph for the topological sort
                        if matches!(procedure, Export::Procedure(_)) {
                            self.callgraph.get_or_insert_node(global_id);
                        }
                    }
                }
                PendingWrapperModule::Exports(pending_module) => {
                    for (procedure_id, _procedure) in pending_module.procedures().iter() {
                        let global_id = GlobalProcedureIndex {
                            module: module_id,
                            index: *procedure_id,
                        };
                        self.callgraph.get_or_insert_node(global_id);
                    }
                }
            }
        }

        // Obtain a set of resolvers for the pending modules so that we can do name resolution
        // before they are added to the graph
        let mut resolver = NameResolver::new(self);
        for module in pending.iter() {
            if let PendingWrapperModule::Ast(module) = module {
                resolver.push_pending(module);
            }
        }
        let mut phantoms = BTreeSet::default();
        let mut edges = Vec::new();
        let mut finished: Vec<WrapperModule> = Vec::new();

        // Visit all of the newly-added modules and perform any rewrites to AST modules.
        for (pending_index, module) in pending.into_iter().enumerate() {
            match module {
                PendingWrapperModule::Ast(mut ast_module) => {
                    let module_id = ModuleIndex::new(high_water_mark + pending_index);

                    let mut rewriter = ModuleRewriter::new(&resolver);
                    rewriter.apply(module_id, &mut ast_module)?;

                    // Gather the phantom calls found while rewriting the module
                    phantoms.extend(rewriter.phantoms());

                    for (index, procedure) in ast_module.procedures().enumerate() {
                        let procedure_id = ProcedureIndex::new(index);
                        let gid = GlobalProcedureIndex {
                            module: module_id,
                            index: procedure_id,
                        };

                        for invoke in procedure.invoked() {
                            let caller = CallerInfo {
                                span: invoke.span(),
                                source_file: ast_module.source_file(),
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

                    finished.push(WrapperModule::Ast(Arc::new(*ast_module)))
                }
                PendingWrapperModule::Exports(module) => {
                    finished.push(WrapperModule::Exports(module));
                }
            }
        }

        // Release the graph again
        drop(resolver);

        // Extend the graph with all of the new additions
        self.phantoms.extend(phantoms);
        self.modules.append(&mut finished);
        edges
            .into_iter()
            .for_each(|(caller, callee)| self.callgraph.add_edge(caller, callee));

        // Visit all of the (AST) modules in the base module graph, and modify them if any of the
        // pending modules allow additional information to be inferred (such as the absolute path of
        // imports, etc)
        for module_index in 0..high_water_mark {
            let module_id = ModuleIndex::new(module_index);
            let module = self.modules[module_id.as_usize()].clone();

            if let WrapperModule::Ast(module) = module {
                // Re-analyze the module, and if we needed to clone-on-write, the new module will be
                // returned. Otherwise, `Ok(None)` indicates that the module is unchanged, and `Err`
                // indicates that re-analysis has found an issue with this module.
                if let Some(new_module) = self.reanalyze_module(module_id, module)? {
                    self.modules[module_id.as_usize()] = WrapperModule::Ast(new_module);
                }
            }
        }

        // Make sure the graph is free of cycles
        let topo = self.callgraph.toposort().map_err(|cycle| {
            let iter = cycle.into_node_ids();
            let mut nodes = Vec::with_capacity(iter.len());
            for node in iter {
                let module = self[node.module].path();
                let proc = self.get_procedure_unsafe(node);
                nodes.push(format!("{}::{}", module, proc.name()));
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
    pub fn get_module(&self, id: ModuleIndex) -> Option<WrapperModule> {
        self.modules.get(id.as_usize()).cloned()
    }

    /// Fetch a [Module] by [ModuleIndex]
    pub fn contains_module(&self, id: ModuleIndex) -> bool {
        self.modules.get(id.as_usize()).is_some()
    }

    /// Fetch a [WrapperProcedure] by [GlobalProcedureIndex], or `None` if index is invalid.
    pub fn get_procedure(&self, id: GlobalProcedureIndex) -> Option<WrapperProcedure> {
        match &self.modules[id.module.as_usize()] {
            WrapperModule::Ast(m) => m.get(id.index).map(WrapperProcedure::Ast),
            WrapperModule::Exports(m) => m
                .procedures()
                .get(id.index.as_usize())
                .map(|(_idx, proc)| WrapperProcedure::Compiled(proc)),
        }
    }

    /// Fetch a [WrapperProcedure] by [GlobalProcedureIndex].
    ///
    /// # Panics
    /// - Panics if index is invalid.
    pub fn get_procedure_unsafe(&self, id: GlobalProcedureIndex) -> WrapperProcedure {
        match &self.modules[id.module.as_usize()] {
            WrapperModule::Ast(m) => WrapperProcedure::Ast(&m[id.index]),
            WrapperModule::Exports(m) => {
                WrapperProcedure::Compiled(&m.procedures()[id.index.as_usize()].1)
            }
        }
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
                    let prev_proc = {
                        match &self.modules[prev_id.module.as_usize()] {
                            WrapperModule::Ast(module) => Some(&module[prev_id.index]),
                            WrapperModule::Exports(_) => None,
                        }
                    };
                    let current_proc = {
                        match &self.modules[id.module.as_usize()] {
                            WrapperModule::Ast(module) => Some(&module[id.index]),
                            WrapperModule::Exports(_) => None,
                        }
                    };

                    // Note: For compiled procedures, we can't check further if they're compatible,
                    // so we assume they are.
                    if let (Some(prev_proc), Some(current_proc)) = (prev_proc, current_proc) {
                        if prev_proc.num_locals() != current_proc.num_locals() {
                            // Multiple procedures with the same root, but incompatible
                            let prev_module = self.modules[prev_id.module.as_usize()].path();
                            let prev_name = FullyQualifiedProcedureName {
                                span: prev_proc.span(),
                                module: prev_module.clone(),
                                name: prev_proc.name().clone(),
                            };
                            let current_module = self.modules[id.module.as_usize()].path();
                            let current_name = FullyQualifiedProcedureName {
                                span: current_proc.span(),
                                module: current_module.clone(),
                                name: current_proc.name().clone(),
                            };
                            return Err(AssemblyError::ConflictingDefinitions {
                                first: prev_name,
                                second: current_name,
                            });
                        }
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

            match module {
                WrapperModule::Ast(module) => {
                    match module.resolve(&next.name) {
                        Some(ResolvedProcedure::Local(index)) => {
                            let id = GlobalProcedureIndex {
                                module: module_index,
                                index: index.into_inner(),
                            };
                            break Ok(id);
                        }
                        Some(ResolvedProcedure::External(fqn)) => {
                            // If we see that we're about to enter an infinite resolver loop because
                            // of a recursive alias, return an error
                            if name == &fqn {
                                break Err(AssemblyError::RecursiveAlias {
                                    source_file: caller.clone(),
                                    name: name.clone(),
                                });
                            }
                            next = Cow::Owned(fqn);
                            caller = module.source_file();
                        }
                        Some(ResolvedProcedure::MastRoot(ref digest)) => {
                            if let Some(id) = self.get_procedure_index_by_digest(digest) {
                                break Ok(id);
                            }
                            break Err(AssemblyError::Failed {
                                labels: vec![RelatedLabel::error("undefined procedure")
                                    .with_source_file(source_file)
                                    .with_labeled_span(
                                        next.span(),
                                        "unable to resolve this reference",
                                    )],
                            });
                        }
                        None => {
                            // No such procedure known to `module`
                            break Err(AssemblyError::Failed {
                                labels: vec![RelatedLabel::error("undefined procedure")
                                    .with_source_file(source_file)
                                    .with_labeled_span(
                                        next.span(),
                                        "unable to resolve this reference",
                                    )],
                            });
                        }
                    }
                }
                WrapperModule::Exports(module) => {
                    break module
                        .procedures()
                        .iter()
                        .find(|(_index, procedure)| procedure.name() == &name.name)
                        .map(|(index, _)| GlobalProcedureIndex {
                            module: module_index,
                            index: *index,
                        })
                        .ok_or(AssemblyError::Failed {
                            labels: vec![RelatedLabel::error("undefined procedure")
                                .with_source_file(source_file)
                                .with_labeled_span(
                                    next.span(),
                                    "unable to resolve this reference",
                                )],
                        })
                }
            }
        }
    }

    /// Resolve a [LibraryPath] to a [ModuleIndex] in this graph
    pub fn find_module_index(&self, name: &LibraryPath) -> Option<ModuleIndex> {
        self.modules.iter().position(|m| m.path() == name).map(ModuleIndex::new)
    }

    /// Resolve a [LibraryPath] to a [Module] in this graph
    pub fn find_module(&self, name: &LibraryPath) -> Option<WrapperModule> {
        self.modules.iter().find(|m| m.path() == name).cloned()
    }

    /// Returns an iterator over the set of [Module]s in this graph, and their indices
    #[allow(unused)]
    pub fn modules(&self) -> impl Iterator<Item = (ModuleIndex, WrapperModule)> + '_ {
        self.modules
            .iter()
            .enumerate()
            .map(|(idx, m)| (ModuleIndex::new(idx), m.clone()))
    }

    /// Like [modules], but returns a reference to the module, rather than an owned pointer
    #[allow(unused)]
    pub fn modules_by_ref(&self) -> impl Iterator<Item = (ModuleIndex, &WrapperModule)> + '_ {
        self.modules.iter().enumerate().map(|(idx, m)| (ModuleIndex::new(idx), m))
    }
}

impl Index<ModuleIndex> for ModuleGraph {
    type Output = WrapperModule;

    fn index(&self, index: ModuleIndex) -> &Self::Output {
        self.modules.index(index.as_usize())
    }
}
