use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};

use mast_forest_builder::MastForestBuilder;
use module_graph::{ProcedureWrapper, WrappedModule};
use vm_core::{mast::MastNodeId, DecoratorList, Felt, Kernel, Operation, Program};

use crate::{
    ast::{self, Export, InvocationTarget, InvokeKind, ModuleKind, QualifiedProcedureName},
    diagnostics::Report,
    library::{KernelLibrary, Library},
    sema::SemanticAnalysisError,
    AssemblyError, Compile, CompileOptions, LibraryNamespace, LibraryPath, RpoDigest,
    SourceManager, Spanned,
};

mod basic_block_builder;
mod id;
mod instruction;
mod mast_forest_builder;
mod module_graph;
mod procedure;

#[cfg(test)]
mod tests;

use self::{
    basic_block_builder::BasicBlockBuilder,
    module_graph::{CallerInfo, ModuleGraph, ResolvedTarget},
};
pub use self::{
    id::{GlobalProcedureIndex, ModuleIndex},
    procedure::{Procedure, ProcedureContext},
};

// ASSEMBLER
// ================================================================================================

/// The [Assembler] is the primary interface for compiling Miden Assembly to the Miden Abstract
/// Syntax Tree (MAST).
///
/// # Usage
///
/// Depending on your needs, there are multiple ways of using the assembler, and whether or not you
/// want to provide a custom kernel.
///
/// <div class="warning">
/// Programs compiled with an empty kernel cannot use the `syscall` instruction.
/// </div>
///
/// * If you have a single executable module you want to compile, just call
///   [Assembler::assemble_program].
/// * If you want to link your executable to a few other modules that implement supporting
///   procedures, build the assembler with them first, using the various builder methods on
///   [Assembler], e.g. [Assembler::with_module], [Assembler::with_library], etc. Then, call
///   [Assembler::assemble_program] to get your compiled program.
#[derive(Clone)]
pub struct Assembler {
    /// The source manager to use for compilation and source location information
    source_manager: Arc<dyn SourceManager>,
    /// The global [ModuleGraph] for this assembler.
    module_graph: ModuleGraph,
    /// Whether to treat warning diagnostics as errors
    warnings_as_errors: bool,
    /// Whether the assembler enables extra debugging information.
    in_debug_mode: bool,
}

impl Default for Assembler {
    fn default() -> Self {
        let source_manager = Arc::new(crate::DefaultSourceManager::default());
        let module_graph = ModuleGraph::new(source_manager.clone());
        Self {
            source_manager,
            module_graph,
            warnings_as_errors: false,
            in_debug_mode: false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
/// Constructors
impl Assembler {
    /// Start building an [Assembler]
    pub fn new(source_manager: Arc<dyn SourceManager>) -> Self {
        let module_graph = ModuleGraph::new(source_manager.clone());
        Self {
            source_manager,
            module_graph,
            warnings_as_errors: false,
            in_debug_mode: false,
        }
    }

    /// Start building an [`Assembler`] with a kernel defined by the provided [KernelLibrary].
    pub fn with_kernel(source_manager: Arc<dyn SourceManager>, kernel_lib: KernelLibrary) -> Self {
        let (kernel, kernel_module, _) = kernel_lib.into_parts();
        let module_graph = ModuleGraph::with_kernel(source_manager.clone(), kernel, kernel_module);
        Self {
            source_manager,
            module_graph,
            ..Default::default()
        }
    }

    /// Sets the default behavior of this assembler with regard to warning diagnostics.
    ///
    /// When true, any warning diagnostics that are emitted will be promoted to errors.
    pub fn with_warnings_as_errors(mut self, yes: bool) -> Self {
        self.warnings_as_errors = yes;
        self
    }

    /// Puts the assembler into the debug mode.
    pub fn with_debug_mode(mut self, yes: bool) -> Self {
        self.in_debug_mode = yes;
        self
    }

    /// Sets the debug mode flag of the assembler
    pub fn set_debug_mode(&mut self, yes: bool) {
        self.in_debug_mode = yes;
    }

    /// Adds `module` to the module graph of the assembler.
    ///
    /// The given module must be a library module, or an error will be returned.
    #[inline]
    pub fn with_module(mut self, module: impl Compile) -> Result<Self, Report> {
        self.add_module(module)?;

        Ok(self)
    }

    /// Adds `module` to the module graph of the assembler with the given options.
    ///
    /// The given module must be a library module, or an error will be returned.
    #[inline]
    pub fn with_module_and_options(
        mut self,
        module: impl Compile,
        options: CompileOptions,
    ) -> Result<Self, Report> {
        self.add_module_with_options(module, options)?;

        Ok(self)
    }

    /// Adds `module` to the module graph of the assembler.
    ///
    /// The given module must be a library module, or an error will be returned.
    #[inline]
    pub fn add_module(&mut self, module: impl Compile) -> Result<(), Report> {
        self.add_module_with_options(module, CompileOptions::for_library())
    }

    /// Adds `module` to the module graph of the assembler, using the provided options.
    ///
    /// The given module must be a library or kernel module, or an error will be returned
    pub fn add_module_with_options(
        &mut self,
        module: impl Compile,
        options: CompileOptions,
    ) -> Result<(), Report> {
        let kind = options.kind;
        if kind != ModuleKind::Library {
            return Err(Report::msg(
                "only library modules are supported by `add_module_with_options`",
            ));
        }

        let module = module.compile_with_options(&self.source_manager, options)?;
        assert_eq!(module.kind(), kind, "expected module kind to match compilation options");

        self.module_graph.add_ast_module(module)?;

        Ok(())
    }

    /// Adds all modules (defined by ".masm" files) from the specified directory to the module
    /// of this assembler graph.
    ///
    /// The modules will be added under the specified namespace, but otherwise preserving the
    /// structure of the directory. Any module named `mod.masm` will be added using parent
    /// directory path For example, if `namespace` = "ns", modules from the ~/masm directory
    /// will be added as follows:
    ///
    /// - ~/masm/foo.masm        -> "ns::foo"
    /// - ~/masm/bar/mod.masm    -> "ns::bar"
    /// - ~/masm/bar/baz.masm    -> "ns::bar::baz"
    #[cfg(feature = "std")]
    pub fn add_modules_from_dir(
        &mut self,
        namespace: crate::LibraryNamespace,
        dir: &std::path::Path,
    ) -> Result<(), Report> {
        for module in crate::parser::read_modules_from_dir(namespace, dir, &self.source_manager)? {
            self.module_graph.add_ast_module(module)?;
        }

        Ok(())
    }

    /// Adds the compiled library to provide modules for the compilation.
    pub fn add_library(&mut self, library: impl AsRef<Library>) -> Result<(), Report> {
        self.module_graph
            .add_compiled_modules(library.as_ref().module_infos())
            .map_err(Report::from)?;
        Ok(())
    }

    /// Adds the compiled library to provide modules for the compilation.
    pub fn with_library(mut self, library: impl AsRef<Library>) -> Result<Self, Report> {
        self.add_library(library)?;
        Ok(self)
    }
}

// ------------------------------------------------------------------------------------------------
/// Public Accessors
impl Assembler {
    /// Returns true if this assembler promotes warning diagnostics as errors by default.
    pub fn warnings_as_errors(&self) -> bool {
        self.warnings_as_errors
    }

    /// Returns true if this assembler was instantiated in debug mode.
    pub fn in_debug_mode(&self) -> bool {
        self.in_debug_mode
    }

    /// Returns a reference to the kernel for this assembler.
    ///
    /// If the assembler was instantiated without a kernel, the internal kernel will be empty.
    pub fn kernel(&self) -> &Kernel {
        self.module_graph.kernel()
    }

    /// Returns a link to the source manager used by this assembler.
    pub fn source_manager(&self) -> Arc<dyn SourceManager> {
        self.source_manager.clone()
    }

    #[cfg(any(test, feature = "testing"))]
    #[doc(hidden)]
    pub fn module_graph(&self) -> &ModuleGraph {
        &self.module_graph
    }
}

// ------------------------------------------------------------------------------------------------
/// Compilation/Assembly
impl Assembler {
    /// Assembles a set of modules into a [Library].
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or compilation of the specified modules fails.
    pub fn assemble_library(
        mut self,
        modules: impl IntoIterator<Item = impl Compile>,
    ) -> Result<Library, Report> {
        let ast_module_indices =
            modules.into_iter().try_fold(Vec::default(), |mut acc, module| {
                module
                    .compile_with_options(&self.source_manager, CompileOptions::for_library())
                    .and_then(|module| {
                        self.module_graph.add_ast_module(module).map_err(Report::from)
                    })
                    .map(move |module_id| {
                        acc.push(module_id);
                        acc
                    })
            })?;

        self.module_graph.recompute()?;

        let mut mast_forest_builder = MastForestBuilder::default();

        let exports = {
            let mut exports = BTreeMap::new();

            for module_idx in ast_module_indices {
                // Note: it is safe to use `unwrap_ast()` here, since all of the modules contained
                // in `ast_module_indices` are in AST form by definition.
                let ast_module = self.module_graph[module_idx].unwrap_ast().clone();

                for (proc_idx, fqn) in ast_module.exported_procedures() {
                    let gid = module_idx + proc_idx;
                    self.compile_subgraph(gid, &mut mast_forest_builder)?;

                    let proc_hash = mast_forest_builder
                        .get_procedure_hash(gid)
                        .expect("compilation succeeded but root not found in cache");
                    exports.insert(fqn, proc_hash);
                }
            }

            exports
        };

        // TODO: show a warning if library exports are empty?
        let (mast_forest, _) = mast_forest_builder.build();
        Ok(Library::new(mast_forest.into(), exports)?)
    }

    /// Assembles the provided module into a [KernelLibrary] intended to be used as a Kernel.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or compilation of the specified modules fails.
    pub fn assemble_kernel(mut self, module: impl Compile) -> Result<KernelLibrary, Report> {
        let options = CompileOptions {
            kind: ModuleKind::Kernel,
            warnings_as_errors: self.warnings_as_errors,
            path: Some(LibraryPath::from(LibraryNamespace::Kernel)),
        };

        let module = module.compile_with_options(&self.source_manager, options)?;
        let module_idx = self.module_graph.add_ast_module(module)?;

        self.module_graph.recompute()?;

        let mut mast_forest_builder = MastForestBuilder::default();

        // Note: it is safe to use `unwrap_ast()` here, since all modules looped over are
        // AST (we just added them to the module graph)
        let ast_module = self.module_graph[module_idx].unwrap_ast().clone();

        let exports = ast_module
            .exported_procedures()
            .map(|(proc_idx, fqn)| {
                let gid = module_idx + proc_idx;
                self.compile_subgraph(gid, &mut mast_forest_builder)?;

                let proc_hash = mast_forest_builder
                    .get_procedure_hash(gid)
                    .expect("compilation succeeded but root not found in cache");
                Ok((fqn, proc_hash))
            })
            .collect::<Result<BTreeMap<QualifiedProcedureName, RpoDigest>, Report>>()?;

        // TODO: show a warning if library exports are empty?

        let (mast_forest, _) = mast_forest_builder.build();
        let library = Library::new(mast_forest.into(), exports)?;
        Ok(library.try_into()?)
    }

    /// Compiles the provided module into a [`Program`]. The resulting program can be executed on
    /// Miden VM.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or compilation of the specified program fails, or if the source
    /// doesn't have an entrypoint.
    pub fn assemble_program(mut self, source: impl Compile) -> Result<Program, Report> {
        let options = CompileOptions {
            kind: ModuleKind::Executable,
            warnings_as_errors: self.warnings_as_errors,
            path: Some(LibraryPath::from(LibraryNamespace::Exec)),
        };

        let program = source.compile_with_options(&self.source_manager, options)?;
        assert!(program.is_executable());

        // Recompute graph with executable module, and start compiling
        let ast_module_index = self.module_graph.add_ast_module(program)?;
        self.module_graph.recompute()?;

        // Find the executable entrypoint Note: it is safe to use `unwrap_ast()` here, since this is
        // the module we just added, which is in AST representation.
        let entrypoint = self.module_graph[ast_module_index]
            .unwrap_ast()
            .index_of(|p| p.is_main())
            .map(|index| GlobalProcedureIndex { module: ast_module_index, index })
            .ok_or(SemanticAnalysisError::MissingEntrypoint)?;

        // Compile the module graph rooted at the entrypoint
        let mut mast_forest_builder = MastForestBuilder::default();
        self.compile_subgraph(entrypoint, &mut mast_forest_builder)?;
        let entry_node_id = mast_forest_builder
            .get_procedure(entrypoint)
            .expect("compilation succeeded but root not found in cache")
            .body_node_id();

        // in case the node IDs changed, update the entrypoint ID to the new value
        let (mast_forest, id_remappings) = mast_forest_builder.build();
        let entry_node_id = id_remappings
            .map(|id_remappings| id_remappings[&entry_node_id])
            .unwrap_or(entry_node_id);

        Ok(Program::with_kernel(
            mast_forest.into(),
            entry_node_id,
            self.module_graph.kernel().clone(),
        ))
    }

    /// Compile the uncompiled procedure in the module graph which are members of the subgraph
    /// rooted at `root`, placing them in the MAST forest builder once compiled.
    ///
    /// Returns an error if any of the provided Miden Assembly is invalid.
    fn compile_subgraph(
        &mut self,
        root: GlobalProcedureIndex,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<(), Report> {
        let mut worklist: Vec<GlobalProcedureIndex> = self
            .module_graph
            .topological_sort_from_root(root)
            .map_err(|cycle| {
                let iter = cycle.into_node_ids();
                let mut nodes = Vec::with_capacity(iter.len());
                for node in iter {
                    let module = self.module_graph[node.module].path();
                    let proc = self.module_graph.get_procedure_unsafe(node);
                    nodes.push(format!("{}::{}", module, proc.name()));
                }
                AssemblyError::Cycle { nodes }
            })?
            .into_iter()
            .filter(|&gid| self.module_graph.get_procedure_unsafe(gid).is_ast())
            .collect();

        assert!(!worklist.is_empty());

        self.process_graph_worklist(&mut worklist, mast_forest_builder)
    }

    /// Compiles all procedures in the `worklist`.
    fn process_graph_worklist(
        &mut self,
        worklist: &mut Vec<GlobalProcedureIndex>,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<(), Report> {
        // Process the topological ordering in reverse order (bottom-up), so that
        // each procedure is compiled with all of its dependencies fully compiled
        while let Some(procedure_gid) = worklist.pop() {
            // If we have already compiled this procedure, do not recompile
            if let Some(proc) = mast_forest_builder.get_procedure(procedure_gid) {
                self.module_graph.register_mast_root(procedure_gid, proc.mast_root())?;
                continue;
            }
            // Fetch procedure metadata from the graph
            let module = match &self.module_graph[procedure_gid.module] {
                WrappedModule::Ast(ast_module) => ast_module,
                // Note: if the containing module is in `Info` representation, there is nothing to
                // compile.
                WrappedModule::Info(_) => continue,
            };

            let export = &module[procedure_gid.index];
            match export {
                Export::Procedure(proc) => {
                    let num_locals = proc.num_locals();
                    let name = QualifiedProcedureName {
                        span: proc.span(),
                        module: module.path().clone(),
                        name: proc.name().clone(),
                    };
                    let pctx = ProcedureContext::new(
                        procedure_gid,
                        name,
                        proc.visibility(),
                        module.is_kernel(),
                        self.source_manager.clone(),
                    )
                    .with_num_locals(num_locals)
                    .with_span(proc.span());

                    // Compile this procedure
                    let procedure = self.compile_procedure(pctx, mast_forest_builder)?;
                    // TODO: if a re-exported procedure with the same MAST root had been previously
                    // added to the builder, this will result in unreachable nodes added to the
                    // MAST forest. This is because while we won't insert a duplicate node for the
                    // procedure body node itself, all nodes that make up the procedure body would
                    // be added to the forest.

                    // Cache the compiled procedure
                    self.module_graph.register_mast_root(procedure_gid, procedure.mast_root())?;
                    mast_forest_builder.insert_procedure(procedure_gid, procedure)?;
                },
                Export::Alias(proc_alias) => {
                    let name = QualifiedProcedureName {
                        span: proc_alias.span(),
                        module: module.path().clone(),
                        name: proc_alias.name().clone(),
                    };
                    let pctx = ProcedureContext::new(
                        procedure_gid,
                        name,
                        ast::Visibility::Public,
                        module.is_kernel(),
                        self.source_manager.clone(),
                    )
                    .with_span(proc_alias.span());

                    let proc_mast_root = self.resolve_target(
                        InvokeKind::ProcRef,
                        &proc_alias.target().into(),
                        &pctx,
                        mast_forest_builder,
                    )?;

                    // insert external node into the MAST forest for this procedure; if a procedure
                    // with the same MAST rood had been previously added to the builder, this will
                    // have no effect
                    let proc_node_id = mast_forest_builder.ensure_external(proc_mast_root)?;
                    let procedure = pctx.into_procedure(proc_mast_root, proc_node_id);

                    // Make the MAST root available to all dependents
                    self.module_graph.register_mast_root(procedure_gid, proc_mast_root)?;
                    mast_forest_builder.insert_procedure(procedure_gid, procedure)?;
                },
            }
        }

        Ok(())
    }

    /// Compiles a single Miden Assembly procedure to its MAST representation.
    fn compile_procedure(
        &self,
        mut proc_ctx: ProcedureContext,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Procedure, Report> {
        // Make sure the current procedure context is available during codegen
        let gid = proc_ctx.id();
        let num_locals = proc_ctx.num_locals();

        let wrapper_proc = self.module_graph.get_procedure_unsafe(gid);
        let proc = wrapper_proc.unwrap_ast().unwrap_procedure();
        let proc_body_id = if num_locals > 0 {
            // for procedures with locals, we need to update fmp register before and after the
            // procedure body is executed. specifically:
            // - to allocate procedure locals we need to increment fmp by the number of locals
            // - to deallocate procedure locals we need to decrement it by the same amount
            let num_locals = Felt::from(num_locals);
            let wrapper = BodyWrapper {
                prologue: vec![Operation::Push(num_locals), Operation::FmpUpdate],
                epilogue: vec![Operation::Push(-num_locals), Operation::FmpUpdate],
            };
            self.compile_body(proc.iter(), &mut proc_ctx, Some(wrapper), mast_forest_builder)?
        } else {
            self.compile_body(proc.iter(), &mut proc_ctx, None, mast_forest_builder)?
        };

        let proc_body_node = mast_forest_builder
            .get_mast_node(proc_body_id)
            .expect("no MAST node for compiled procedure");
        Ok(proc_ctx.into_procedure(proc_body_node.digest(), proc_body_id))
    }

    fn compile_body<'a, I>(
        &self,
        body: I,
        proc_ctx: &mut ProcedureContext,
        wrapper: Option<BodyWrapper>,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<MastNodeId, Report>
    where
        I: Iterator<Item = &'a ast::Op>,
    {
        use ast::Op;

        let mut node_ids: Vec<MastNodeId> = Vec::new();
        let mut basic_block_builder = BasicBlockBuilder::new(wrapper);

        for op in body {
            match op {
                Op::Inst(inst) => {
                    if let Some(mast_node_id) = self.compile_instruction(
                        inst,
                        &mut basic_block_builder,
                        proc_ctx,
                        mast_forest_builder,
                    )? {
                        if let Some(basic_block_id) =
                            basic_block_builder.make_basic_block(mast_forest_builder)?
                        {
                            node_ids.push(basic_block_id);
                        }

                        node_ids.push(mast_node_id);
                    }
                },

                Op::If { then_blk, else_blk, .. } => {
                    if let Some(basic_block_id) =
                        basic_block_builder.make_basic_block(mast_forest_builder)?
                    {
                        node_ids.push(basic_block_id);
                    }

                    let then_blk =
                        self.compile_body(then_blk.iter(), proc_ctx, None, mast_forest_builder)?;
                    let else_blk =
                        self.compile_body(else_blk.iter(), proc_ctx, None, mast_forest_builder)?;

                    let split_node_id = mast_forest_builder.ensure_split(then_blk, else_blk)?;
                    node_ids.push(split_node_id);
                },

                Op::Repeat { count, body, .. } => {
                    if let Some(basic_block_id) =
                        basic_block_builder.make_basic_block(mast_forest_builder)?
                    {
                        node_ids.push(basic_block_id);
                    }

                    let repeat_node_id =
                        self.compile_body(body.iter(), proc_ctx, None, mast_forest_builder)?;

                    for _ in 0..*count {
                        node_ids.push(repeat_node_id);
                    }
                },

                Op::While { body, .. } => {
                    if let Some(basic_block_id) =
                        basic_block_builder.make_basic_block(mast_forest_builder)?
                    {
                        node_ids.push(basic_block_id);
                    }

                    let loop_body_node_id =
                        self.compile_body(body.iter(), proc_ctx, None, mast_forest_builder)?;

                    let loop_node_id = mast_forest_builder.ensure_loop(loop_body_node_id)?;
                    node_ids.push(loop_node_id);
                },
            }
        }

        if let Some(basic_block_id) =
            basic_block_builder.try_into_basic_block(mast_forest_builder)?
        {
            node_ids.push(basic_block_id);
        }

        Ok(if node_ids.is_empty() {
            mast_forest_builder.ensure_block(vec![Operation::Noop], None)?
        } else {
            mast_forest_builder.join_nodes(node_ids)?
        })
    }

    pub(super) fn resolve_target(
        &self,
        kind: InvokeKind,
        target: &InvocationTarget,
        proc_ctx: &ProcedureContext,
        mast_forest_builder: &MastForestBuilder,
    ) -> Result<RpoDigest, AssemblyError> {
        let caller = CallerInfo {
            span: target.span(),
            module: proc_ctx.id().module,
            kind,
        };
        let resolved = self.module_graph.resolve_target(&caller, target)?;
        match resolved {
            ResolvedTarget::Phantom(digest) => Ok(digest),
            ResolvedTarget::Exact { gid } | ResolvedTarget::Resolved { gid, .. } => {
                match mast_forest_builder.get_procedure_hash(gid) {
                    Some(proc_hash) => Ok(proc_hash),
                    None => match self.module_graph.get_procedure_unsafe(gid) {
                        ProcedureWrapper::Info(p) => Ok(p.digest),
                        ProcedureWrapper::Ast(_) => panic!("Did not find procedure {gid:?} neither in module graph nor procedure cache"),
                    },
                }
            }
        }
    }
}

// HELPERS
// ================================================================================================

/// Contains a set of operations which need to be executed before and after a sequence of AST
/// nodes (i.e., code body).
struct BodyWrapper {
    prologue: Vec<Operation>,
    epilogue: Vec<Operation>,
}
