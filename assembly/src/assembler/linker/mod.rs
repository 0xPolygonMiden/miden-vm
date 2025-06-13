mod analysis;
mod callgraph;
mod debug;
mod name_resolver;
mod rewrites;

use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec::Vec};
use core::ops::Index;

use smallvec::{SmallVec, smallvec};
use vm_core::{Kernel, crypto::hash::RpoDigest};

use self::{analysis::MaybeRewriteCheck, name_resolver::NameResolver, rewrites::ModuleRewriter};
pub use self::{
    callgraph::{CallGraph, CycleError},
    name_resolver::{CallerInfo, ResolvedTarget},
};
use super::{GlobalProcedureIndex, ModuleIndex};
use crate::{
    AssemblyError, LibraryNamespace, LibraryPath, SourceManager, Spanned,
    ast::{
        Export, InvocationTarget, InvokeKind, Module, ProcedureIndex, ProcedureName,
        ResolvedProcedure,
    },
    library::{ModuleInfo, ProcedureInfo},
};

// WRAPPER STRUCTS
// ================================================================================================

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

impl ProcedureWrapper<'_> {
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
            },
        }
    }

    /// Resolves `name` to a procedure within the local scope of this module.
    pub fn resolve(&self, name: &ProcedureName) -> Option<ResolvedProcedure> {
        match self {
            WrappedModule::Ast(module) => module.resolve(name),
            WrappedModule::Info(module) => {
                module.get_procedure_digest_by_name(name).map(ResolvedProcedure::MastRoot)
            },
        }
    }
}

#[derive(Clone)]
pub struct PendingModule {
    pub module: Box<Module>,
    pub module_index: ModuleIndex,
}

// LINKER
// ================================================================================================

#[derive(Clone)]
pub struct Linker {
    modules: Vec<Option<WrappedModule>>,
    /// The set of modules pending additional processing before adding them to the graph.
    ///
    /// When adding a set of inter-dependent modules to the graph, we process them as a group, so
    /// that any references between them can be resolved, and the contents of the module
    /// rewritten to reflect the changes.
    ///
    /// Once added to the graph, modules become immutable, and any additional modules added after
    /// that must by definition only depend on modules in the graph, and not be depended upon.
    pending: Vec<PendingModule>,
    /// The global call graph of calls, not counting those that are performed directly via MAST
    /// root.
    callgraph: CallGraph,
    /// The set of MAST roots which have procedure definitions in this graph. There can be
    /// multiple procedures bound to the same root due to having identical code.
    procedures_by_mast_root: BTreeMap<RpoDigest, SmallVec<[GlobalProcedureIndex; 1]>>,
    kernel_index: Option<ModuleIndex>,
    kernel: Kernel,
    source_manager: Arc<dyn SourceManager>,
}

// ------------------------------------------------------------------------------------------------
/// Constructors
impl Linker {
    /// Instantiate a new [ModuleGraph], using the provided [SourceManager] to resolve source info.
    pub fn new(source_manager: Arc<dyn SourceManager>) -> Self {
        Self {
            modules: Default::default(),
            pending: Default::default(),
            callgraph: Default::default(),
            procedures_by_mast_root: Default::default(),
            kernel_index: None,
            kernel: Default::default(),
            source_manager,
        }
    }

    /// Adds all module infos to the graph.
    pub fn add_compiled_modules(
        &mut self,
        module_infos: impl IntoIterator<Item = ModuleInfo>,
    ) -> Result<(), AssemblyError> {
        for module in module_infos {
            self.add_compiled_module(module)?;
        }

        Ok(())
    }

    pub fn add_compiled_module(
        &mut self,
        module: ModuleInfo,
    ) -> Result<ModuleIndex, AssemblyError> {
        log::debug!(target: "module-graph", "adding pre-assembled module {} to module graph", module.path());

        let module_path = module.path();
        let is_duplicate =
            self.is_pending(module_path) || self.find_module_index(module_path).is_some();
        if is_duplicate {
            return Err(AssemblyError::DuplicateModule { path: module_path.clone() });
        }

        let module_index = self.next_module_id();
        for (proc_index, proc) in module.procedures() {
            let gid = module_index + proc_index;
            self.register_procedure_root(gid, proc.digest)?;
            self.callgraph.get_or_insert_node(gid);
        }
        self.modules.push(Some(WrappedModule::Info(module)));
        Ok(module_index)
    }

    /// Add `module` to the graph.
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
    #[allow(unused)]
    pub fn add_included_ast_module(&mut self, module: Box<Module>) -> Result<(), AssemblyError> {
        self.include_module(module).map(|_| ())
    }

    pub fn add_included_ast_modules(
        &mut self,
        modules: impl IntoIterator<Item = Box<Module>>,
    ) -> Result<Vec<ModuleIndex>, AssemblyError> {
        modules.into_iter().map(|m| self.include_module(m)).collect()
    }

    fn include_module(&mut self, module: Box<Module>) -> Result<ModuleIndex, AssemblyError> {
        log::debug!(target: "module-graph", "adding unprocessed module {}", module.path());
        let module_path = module.path();

        let is_duplicate =
            self.is_pending(module_path) || self.find_module_index(module_path).is_some();
        if is_duplicate {
            return Err(AssemblyError::DuplicateModule { path: module_path.clone() });
        }

        let module_index = self.next_module_id();
        self.modules.push(None);
        self.pending.push(PendingModule { module, module_index });
        Ok(module_index)
    }

    fn is_pending(&self, path: &LibraryPath) -> bool {
        self.pending.iter().any(|m| m.module.path() == path)
    }

    #[inline]
    fn next_module_id(&self) -> ModuleIndex {
        ModuleIndex::new(self.modules.len())
    }
}

// ------------------------------------------------------------------------------------------------
/// Kernels
impl Linker {
    /// Returns a new [ModuleGraph] instantiated from the provided kernel and kernel info module.
    ///
    /// Note: it is assumed that kernel and kernel_module are consistent, but this is not checked.
    ///
    /// TODO: consider passing `KerneLibrary` into this constructor as a parameter instead.
    pub(super) fn with_kernel(
        source_manager: Arc<dyn SourceManager>,
        kernel: Kernel,
        kernel_module: ModuleInfo,
    ) -> Self {
        assert!(!kernel.is_empty());
        assert_eq!(kernel_module.path(), &LibraryPath::from(LibraryNamespace::Kernel));
        log::debug!(target: "module-graph", "instantiating module graph with kernel {}", kernel_module.path());

        let mut graph = Self::new(source_manager);
        let kernel_index = graph
            .add_compiled_module(kernel_module)
            .expect("failed to add kernel module to the module graph");

        graph.kernel_index = Some(kernel_index);
        graph.kernel = kernel;
        graph
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    pub fn has_nonempty_kernel(&self) -> bool {
        self.kernel_index.is_some() || !self.kernel.is_empty()
    }
}

// ------------------------------------------------------------------------------------------------
/// Analysis
impl Linker {
    pub fn process(
        &mut self,
        modules: impl IntoIterator<Item = Box<Module>>,
    ) -> Result<Vec<ModuleIndex>, AssemblyError> {
        let module_indices = self.add_included_ast_modules(modules)?;

        self.recompute()?;

        Ok(module_indices)
    }

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
    fn recompute(&mut self) -> Result<(), AssemblyError> {
        log::debug!(target: "module-graph", "processing {} new modules, and recomputing module graph", self.pending.len());

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
        for PendingModule { module: pending_module, module_index } in pending.iter() {
            log::debug!(
                target: "module-graph",
                "adding procedures from pending module {} (index {}) to call graph",
                pending_module.path(),
                module_index.as_usize()
            );

            // Apply module to call graph
            for (index, _) in pending_module.procedures().enumerate() {
                let procedure_id = ProcedureIndex::new(index);
                let global_id = GlobalProcedureIndex {
                    module: *module_index,
                    index: procedure_id,
                };

                // Ensure all entrypoints and exported symbols are represented in the call
                // graph, even if they have no edges, we need them
                // in the graph for the topological sort
                self.callgraph.get_or_insert_node(global_id);
            }
        }

        // Obtain a set of resolvers for the pending modules so that we can do name resolution
        // before they are added to the graph
        let mut resolver = NameResolver::new(self);
        for module in pending.iter() {
            resolver.push_pending(module);
        }
        let mut edges = Vec::new();
        let mut finished: Vec<PendingModule> = Vec::with_capacity(pending.len());

        // Visit all of the newly-added modules and perform any rewrites to AST modules.
        for PendingModule { mut module, module_index } in pending.into_iter() {
            log::debug!(target: "module-graph", "rewriting pending module {} (index {})", module.path(), module_index.as_usize());

            let mut rewriter = ModuleRewriter::new(&resolver);
            rewriter.apply(module_index, &mut module)?;

            log::debug!(
                target: "module-graph",
                "processing procedures of pending module {} (index {})",
                module.path(),
                module_index.as_usize()
            );
            for (index, procedure) in module.procedures().enumerate() {
                log::debug!(target: "module-graph", "  * processing {} at index {index}", procedure.name());

                let procedure_id = ProcedureIndex::new(index);
                let gid = GlobalProcedureIndex {
                    module: module_index,
                    index: procedure_id,
                };

                // Add edge to the call graph to represent dependency on aliased procedures
                if let Export::Alias(alias) = procedure {
                    log::debug!(target: "module-graph", "  | resolving alias {}..", alias.target());

                    let caller = CallerInfo {
                        span: alias.span(),
                        module: module_index,
                        kind: InvokeKind::ProcRef,
                    };
                    let target = alias.target().into();
                    if let Some(callee) =
                        resolver.resolve_target(&caller, &target)?.into_global_id()
                    {
                        log::debug!(
                            target: "module-graph",
                            "  | resolved alias to gid {:?}:{:?}",
                            callee.module,
                            callee.index
                        );
                        edges.push((gid, callee));
                    }
                }

                // Add edges to all transitive dependencies of this procedure due to calls
                for invoke in procedure.invoked() {
                    log::debug!(target: "module-graph", "  | recording {} dependency on {}", invoke.kind, &invoke.target);

                    let caller = CallerInfo {
                        span: invoke.span(),
                        module: module_index,
                        kind: invoke.kind,
                    };
                    if let Some(callee) =
                        resolver.resolve_target(&caller, &invoke.target)?.into_global_id()
                    {
                        log::debug!(
                            target: "module-graph",
                            "  | resolved dependency to gid {}:{}",
                            callee.module.as_usize(),
                            callee.index.as_usize()
                        );
                        edges.push((gid, callee));
                    }
                }
            }

            finished.push(PendingModule { module, module_index });
        }

        // Release the graph again
        drop(resolver);

        // Update the graph with the processed modules
        for PendingModule { module, module_index } in finished {
            self.modules[module_index.as_usize()] = Some(WrappedModule::Ast(Arc::from(module)));
        }

        edges
            .into_iter()
            .for_each(|(caller, callee)| self.callgraph.add_edge(caller, callee));

        // Visit all of the (AST) modules in the base module graph, and modify them if any of the
        // pending modules allow additional information to be inferred (such as the absolute path of
        // imports, etc)
        for module_index in 0..high_water_mark {
            let module_index = ModuleIndex::new(module_index);
            let module = self.modules[module_index.as_usize()].clone().unwrap_or_else(|| {
                panic!(
                    "expected module at index {} to have been processed, but it is None",
                    module_index.as_usize()
                )
            });

            match module {
                WrappedModule::Ast(module) => {
                    log::debug!(target: "module-graph", "re-analyzing module {} (index {})", module.path(), module_index.as_usize());
                    // Re-analyze the module, and if we needed to clone-on-write, the new module will be
                    // returned. Otherwise, `Ok(None)` indicates that the module is unchanged, and `Err`
                    // indicates that re-analysis has found an issue with this module.
                    let new_module =
                        self.reanalyze_module(module_index, module).map(WrappedModule::Ast)?;
                    self.modules[module_index.as_usize()] = Some(new_module);
                },
                module => {
                    self.modules[module_index.as_usize()] = Some(module);
                },
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
            AssemblyError::Cycle { nodes: nodes.into() }
        })?;

        Ok(())
    }

    fn reanalyze_module(
        &mut self,
        module_id: ModuleIndex,
        module: Arc<Module>,
    ) -> Result<Arc<Module>, AssemblyError> {
        let resolver = NameResolver::new(self);
        let maybe_rewrite = MaybeRewriteCheck::new(&resolver);
        if maybe_rewrite.check(module_id, &module)? {
            // We need to rewrite this module again, so get an owned copy of the original
            // and use that
            let mut module = Box::new(Arc::unwrap_or_clone(module));
            let mut rewriter = ModuleRewriter::new(&resolver);
            rewriter.apply(module_id, &mut module)?;

            Ok(Arc::from(module))
        } else {
            Ok(module)
        }
    }
}

// ------------------------------------------------------------------------------------------------
/// Accessors/Queries
impl Linker {
    /// Compute the topological sort of the callgraph rooted at `caller`
    pub fn topological_sort_from_root(
        &self,
        caller: GlobalProcedureIndex,
    ) -> Result<Vec<GlobalProcedureIndex>, CycleError> {
        self.callgraph.toposort_caller(caller)
    }

    /// Fetch a [WrapperProcedure] by [GlobalProcedureIndex].
    ///
    /// # Panics
    /// - Panics if index is invalid.
    pub fn get_procedure_unsafe(&self, id: GlobalProcedureIndex) -> ProcedureWrapper<'_> {
        match self.modules[id.module.as_usize()]
            .as_ref()
            .expect("invalid reference to pending module")
        {
            WrappedModule::Ast(m) => ProcedureWrapper::Ast(&m[id.index]),
            WrappedModule::Info(m) => {
                ProcedureWrapper::Info(m.get_procedure_by_index(id.index).unwrap())
            },
        }
    }

    /// Returns a procedure index which corresponds to the provided procedure digest.
    ///
    /// Note that there can be many procedures with the same digest - due to having the same code,
    /// and/or using different decorators which don't affect the MAST root. This method returns an
    /// arbitrary one.
    pub fn get_procedure_index_by_digest(
        &self,
        procedure_digest: &RpoDigest,
    ) -> Option<GlobalProcedureIndex> {
        self.procedures_by_mast_root.get(procedure_digest).map(|indices| indices[0])
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

    /// Registers a [MastNodeId] as corresponding to a given [GlobalProcedureIndex].
    ///
    /// # SAFETY
    ///
    /// It is essential that the caller _guarantee_ that the given digest belongs to the specified
    /// procedure. It is fine if there are multiple procedures with the same digest, but it _must_
    /// be the case that if a given digest is specified, it can be used as if it was the definition
    /// of the referenced procedure, i.e. they are referentially transparent.
    pub(crate) fn register_procedure_root(
        &mut self,
        id: GlobalProcedureIndex,
        procedure_mast_root: RpoDigest,
    ) -> Result<(), AssemblyError> {
        use alloc::collections::btree_map::Entry;
        match self.procedures_by_mast_root.entry(procedure_mast_root) {
            Entry::Occupied(ref mut entry) => {
                let prev_id = entry.get()[0];
                if prev_id != id {
                    // Multiple procedures with the same root, but compatible
                    entry.get_mut().push(id);
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(smallvec![id]);
            },
        }

        Ok(())
    }

    /// Resolve a [LibraryPath] to a [ModuleIndex] in this graph
    pub fn find_module_index(&self, name: &LibraryPath) -> Option<ModuleIndex> {
        self.modules
            .iter()
            .position(|m| m.as_ref().is_some_and(|m| m.path() == name))
            .map(ModuleIndex::new)
    }

    /// Resolve a [LibraryPath] to a [Module] in this graph
    pub fn find_module(&self, name: &LibraryPath) -> Option<WrappedModule> {
        self.modules
            .iter()
            .find(|m| m.as_ref().is_some_and(|m| m.path() == name))
            .cloned()
            .unwrap_or(None)
    }
}

impl Index<ModuleIndex> for Linker {
    type Output = WrappedModule;

    fn index(&self, index: ModuleIndex) -> &Self::Output {
        self.modules
            .index(index.as_usize())
            .as_ref()
            .expect("invalid reference to pending module")
    }
}
