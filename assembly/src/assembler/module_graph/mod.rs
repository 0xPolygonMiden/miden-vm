mod analysis;
mod callgraph;
mod debug;
mod name_resolver;
mod rewrites;

pub use self::callgraph::{CallGraph, CycleError};
pub use self::name_resolver::{CallerInfo, ResolvedTarget};

use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec::Vec};
use core::ops::Index;
use vm_core::Kernel;

use smallvec::{smallvec, SmallVec};

use self::{analysis::MaybeRewriteCheck, name_resolver::NameResolver, rewrites::ModuleRewriter};
use super::{GlobalProcedureIndex, ModuleIndex};
use crate::{
    ast::{
        Export, FullyQualifiedProcedureName, InvocationTarget, Module, ProcedureIndex,
        ProcedureName, ResolvedProcedure,
    },
    library::{ModuleInfo, ProcedureInfo},
    AssemblyError, LibraryPath, RpoDigest, Spanned,
};

/// Wraps all supported representations of a procedure in the module graph.
///
/// Currently, there are two supported representations:
/// - `Ast`: wraps a procedure for which we have access to the entire AST,
/// - `Info`: stores the procedure's name and digest (resulting from previously compiled
///   procedures).
pub enum ProcedureWrapper<'a> {
    Ast(&'a Export),
    Info(&'a ProcedureInfo),
}

impl<'a> ProcedureWrapper<'a> {
    /// Returns the name of the procedure.
    pub fn name(&self) -> &ProcedureName {
        match self {
            Self::Ast(p) => p.name(),
            Self::Info(p) => &p.name,
        }
    }

    /// Returns the wrapped procedure if in the `Ast` representation, or panics otherwise.
    ///
    /// # Panics
    /// - Panics if the wrapped procedure is not in the `Ast` representation.
    pub fn unwrap_ast(&self) -> &Export {
        match self {
            Self::Ast(proc) => proc,
            Self::Info(_) => panic!("expected AST procedure, but was compiled"),
        }
    }

    /// Returns true if the wrapped procedure is in the `Ast` representation.
    pub fn is_ast(&self) -> bool {
        matches!(self, Self::Ast(_))
    }
}

/// Wraps all supported representations of a module in the module graph.
///
/// Currently, there are two supported representations:
/// - `Ast`: wraps a module for which we have access to the entire AST,
/// - `Info`: stores only the necessary information about a module (resulting from previously
///   compiled modules).
#[derive(Clone)]
pub enum WrappedModule {
    Ast(Arc<Module>),
    Info(ModuleInfo),
}

impl WrappedModule {
    /// Returns the library path of the wrapped module.
    pub fn path(&self) -> &LibraryPath {
        match self {
            Self::Ast(m) => m.path(),
            Self::Info(m) => m.path(),
        }
    }

    /// Returns the wrapped module if in the `Ast` representation, or panics otherwise.
    ///
    /// # Panics
    /// - Panics if the wrapped module is not in the `Ast` representation.
    pub fn unwrap_ast(&self) -> &Arc<Module> {
        match self {
            Self::Ast(module) => module,
            Self::Info(_) => {
                panic!("expected module to be in AST representation, but was compiled")
            }
        }
    }

    /// Returns the wrapped module if in the `Info` representation, or panics otherwise.
    ///
    /// # Panics
    /// - Panics if the wrapped module is not in the `Info` representation.
    pub fn unwrap_info(&self) -> &ModuleInfo {
        match self {
            Self::Ast(_) => {
                panic!("expected module to be compiled, but was in AST representation")
            }
            Self::Info(module) => module,
        }
    }

    /// Resolves `name` to a procedure within the local scope of this module.
    pub fn resolve(&self, name: &ProcedureName) -> Option<ResolvedProcedure> {
        match self {
            WrappedModule::Ast(module) => module.resolve(name),
            WrappedModule::Info(module) => {
                module.get_proc_digest_by_name(name).map(ResolvedProcedure::MastRoot)
            }
        }
    }
}

/// Wraps modules that are pending in the [`ModuleGraph`].
#[derive(Clone)]
pub enum PendingModuleWrapper {
    Ast(Box<Module>),
    Info(ModuleInfo),
}

impl PendingModuleWrapper {
    /// Returns the library path of the wrapped module.
    pub fn path(&self) -> &LibraryPath {
        match self {
            Self::Ast(m) => m.path(),
            Self::Info(m) => m.path(),
        }
    }
}

// MODULE GRAPH
// ================================================================================================

#[derive(Default, Clone)]
pub struct ModuleGraph {
    modules: Vec<WrappedModule>,
    /// The set of modules pending additional processing before adding them to the graph.
    ///
    /// When adding a set of inter-dependent modules to the graph, we process them as a group, so
    /// that any references between them can be resolved, and the contents of the module
    /// rewritten to reflect the changes.
    ///
    /// Once added to the graph, modules become immutable, and any additional modules added after
    /// that must by definition only depend on modules in the graph, and not be depended upon.
    pending: Vec<PendingModuleWrapper>,
    /// The global call graph of calls, not counting those that are performed directly via MAST
    /// root.
    callgraph: CallGraph,
    /// The set of MAST roots which have procedure definitions in this graph. There can be
    /// multiple procedures bound to the same root due to having identical code.
    roots: BTreeMap<RpoDigest, SmallVec<[GlobalProcedureIndex; 1]>>,
    kernel_index: Option<ModuleIndex>,
    kernel: Kernel,
}

// ------------------------------------------------------------------------------------------------
/// Constructors
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
        self.add_module(PendingModuleWrapper::Ast(module))
    }

    /// Add the [`ModuleInfo`] to the graph.
    ///
    /// NOTE: This operation only adds a module to the graph, but does not perform the important
    /// analysis needed for compilation, you must call [`Self::recompute`] once all modules are
    /// added to ensure the analysis results reflect the current version of the graph.
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
    pub fn add_module_info(
        &mut self,
        module_info: ModuleInfo,
    ) -> Result<ModuleIndex, AssemblyError> {
        self.add_module(PendingModuleWrapper::Info(module_info))
    }

    fn add_module(&mut self, module: PendingModuleWrapper) -> Result<ModuleIndex, AssemblyError> {
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

    fn is_pending(&self, path: &LibraryPath) -> bool {
        self.pending.iter().any(|m| m.path() == path)
    }

    #[inline]
    fn next_module_id(&self) -> ModuleIndex {
        ModuleIndex::new(self.modules.len() + self.pending.len())
    }
}

// ------------------------------------------------------------------------------------------------
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

// ------------------------------------------------------------------------------------------------
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

        // Visit all of the pending modules, assigning them ids, and adding them to the module
        // graph after rewriting any calls to use absolute paths
        let high_water_mark = self.modules.len();
        let pending = core::mem::take(&mut self.pending);
        for (pending_index, pending_module) in pending.iter().enumerate() {
            let module_id = ModuleIndex::new(high_water_mark + pending_index);

            // Apply module to call graph
            match pending_module {
                PendingModuleWrapper::Ast(pending_module) => {
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
                PendingModuleWrapper::Info(pending_module) => {
                    for (proc_index, _procedure) in pending_module.procedure_infos() {
                        let global_id = GlobalProcedureIndex {
                            module: module_id,
                            index: proc_index,
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
            if let PendingModuleWrapper::Ast(module) = module {
                resolver.push_pending(module);
            }
        }
        let mut edges = Vec::new();
        let mut finished: Vec<WrappedModule> = Vec::new();

        // Visit all of the newly-added modules and perform any rewrites to AST modules.
        for (pending_index, module) in pending.into_iter().enumerate() {
            match module {
                PendingModuleWrapper::Ast(mut ast_module) => {
                    let module_id = ModuleIndex::new(high_water_mark + pending_index);

                    let mut rewriter = ModuleRewriter::new(&resolver);
                    rewriter.apply(module_id, &mut ast_module)?;

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

                    finished.push(WrappedModule::Ast(Arc::new(*ast_module)))
                }
                PendingModuleWrapper::Info(module) => {
                    finished.push(WrappedModule::Info(module));
                }
            }
        }

        // Release the graph again
        drop(resolver);

        // Extend the graph with all of the new additions
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

            if let WrappedModule::Ast(module) = module {
                // Re-analyze the module, and if we needed to clone-on-write, the new module will be
                // returned. Otherwise, `Ok(None)` indicates that the module is unchanged, and `Err`
                // indicates that re-analysis has found an issue with this module.
                if let Some(new_module) = self.reanalyze_module(module_id, module)? {
                    self.modules[module_id.as_usize()] = WrappedModule::Ast(new_module);
                }
            }
        }

        // Make sure the graph is free of cycles
        self.callgraph.toposort().map_err(|cycle| {
            let iter = cycle.into_node_ids();
            let mut nodes = Vec::with_capacity(iter.len());
            for node in iter {
                let module = self[node.module].path();
                let proc = self.get_procedure_unsafe(node);
                nodes.push(format!("{}::{}", module, proc.name()));
            }
            AssemblyError::Cycle { nodes }
        })?;

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

            Ok(Some(Arc::from(module)))
        } else {
            Ok(None)
        }
    }
}

// ------------------------------------------------------------------------------------------------
/// Accessors/Queries
impl ModuleGraph {
    /// Compute the topological sort of the callgraph rooted at `caller`
    pub fn topological_sort_from_root(
        &self,
        caller: GlobalProcedureIndex,
    ) -> Result<Vec<GlobalProcedureIndex>, CycleError> {
        Ok(self.callgraph.toposort_caller(caller)?)
    }

    /// Fetch a [Module] by [ModuleIndex]
    #[allow(unused)]
    pub fn get_module(&self, id: ModuleIndex) -> Option<WrappedModule> {
        self.modules.get(id.as_usize()).cloned()
    }

    /// Fetch a [WrapperProcedure] by [GlobalProcedureIndex], or `None` if index is invalid.
    #[allow(unused)]
    pub fn get_procedure(&self, id: GlobalProcedureIndex) -> Option<ProcedureWrapper> {
        match &self.modules[id.module.as_usize()] {
            WrappedModule::Ast(m) => m.get(id.index).map(ProcedureWrapper::Ast),
            WrappedModule::Info(m) => {
                m.get_proc_info_by_index(id.index).map(ProcedureWrapper::Info)
            }
        }
    }

    /// Fetch a [WrapperProcedure] by [GlobalProcedureIndex].
    ///
    /// # Panics
    /// - Panics if index is invalid.
    pub fn get_procedure_unsafe(&self, id: GlobalProcedureIndex) -> ProcedureWrapper {
        match &self.modules[id.module.as_usize()] {
            WrappedModule::Ast(m) => ProcedureWrapper::Ast(&m[id.index]),
            WrappedModule::Info(m) => {
                ProcedureWrapper::Info(m.get_proc_info_by_index(id.index).unwrap())
            }
        }
    }

    pub fn get_procedure_index_by_digest(
        &self,
        digest: &RpoDigest,
    ) -> Option<GlobalProcedureIndex> {
        self.roots.get(digest).map(|indices| indices[0])
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
                            WrappedModule::Ast(module) => Some(&module[prev_id.index]),
                            WrappedModule::Info(_) => None,
                        }
                    };
                    let current_proc = {
                        match &self.modules[id.module.as_usize()] {
                            WrappedModule::Ast(module) => Some(&module[id.index]),
                            WrappedModule::Info(_) => None,
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

        Ok(())
    }

    /// Resolve a [LibraryPath] to a [ModuleIndex] in this graph
    pub fn find_module_index(&self, name: &LibraryPath) -> Option<ModuleIndex> {
        self.modules.iter().position(|m| m.path() == name).map(ModuleIndex::new)
    }

    /// Resolve a [LibraryPath] to a [Module] in this graph
    pub fn find_module(&self, name: &LibraryPath) -> Option<WrappedModule> {
        self.modules.iter().find(|m| m.path() == name).cloned()
    }

    /// Returns an iterator over the set of [Module]s in this graph, and their indices
    #[allow(unused)]
    pub fn modules(&self) -> impl Iterator<Item = (ModuleIndex, WrappedModule)> + '_ {
        self.modules
            .iter()
            .enumerate()
            .map(|(idx, m)| (ModuleIndex::new(idx), m.clone()))
    }

    /// Like [modules], but returns a reference to the module, rather than an owned pointer
    #[allow(unused)]
    pub fn modules_by_ref(&self) -> impl Iterator<Item = (ModuleIndex, &WrappedModule)> + '_ {
        self.modules.iter().enumerate().map(|(idx, m)| (ModuleIndex::new(idx), m))
    }
}

impl Index<ModuleIndex> for ModuleGraph {
    type Output = WrappedModule;

    fn index(&self, index: ModuleIndex) -> &Self::Output {
        self.modules.index(index.as_usize())
    }
}
