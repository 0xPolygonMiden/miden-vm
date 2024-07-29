use crate::{
    ast::{self, FullyQualifiedProcedureName, InvocationTarget, InvokeKind, ModuleKind},
    diagnostics::Report,
    library::{CompiledLibrary, KernelLibrary},
    sema::SemanticAnalysisError,
    AssemblyError, Compile, CompileOptions, Library, LibraryNamespace, LibraryPath, RpoDigest,
    Spanned,
};
use alloc::{sync::Arc, vec::Vec};
use mast_forest_builder::MastForestBuilder;
use module_graph::{ProcedureWrapper, WrappedModule};
use vm_core::{mast::MastNodeId, Decorator, DecoratorList, Felt, Kernel, Operation, Program};

mod basic_block_builder;
mod id;
mod instruction;
mod mast_forest_builder;
mod module_graph;
mod procedure;
#[cfg(test)]
mod tests;

pub use self::id::{GlobalProcedureIndex, ModuleIndex};
pub use self::procedure::{Procedure, ProcedureContext};

use self::basic_block_builder::BasicBlockBuilder;
use self::module_graph::{CallerInfo, ModuleGraph, ResolvedTarget};

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
#[derive(Clone, Default)]
pub struct Assembler {
    /// The global [ModuleGraph] for this assembler.
    module_graph: ModuleGraph,
    /// Whether to treat warning diagnostics as errors
    warnings_as_errors: bool,
    /// Whether the assembler enables extra debugging information.
    in_debug_mode: bool,
}

// ------------------------------------------------------------------------------------------------
/// Constructors
impl Assembler {
    /// Start building an [`Assembler`] with a kernel defined by the provided [KernelLibrary].
    pub fn with_kernel(kernel_lib: KernelLibrary) -> Self {
        let (kernel, kernel_module, _) = kernel_lib.into_parts();
        Self {
            module_graph: ModuleGraph::with_kernel(kernel, kernel_module),
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

        let module = module.compile_with_options(options)?;
        assert_eq!(module.kind(), kind, "expected module kind to match compilation options");

        self.module_graph.add_ast_module(module)?;

        Ok(())
    }

    /// Adds the compiled library to provide modules for the compilation.
    pub fn add_compiled_library(&mut self, library: CompiledLibrary) -> Result<(), Report> {
        self.module_graph
            .add_compiled_modules(library.into_module_infos())
            .map_err(Report::from)?;
        Ok(())
    }

    /// Adds the library to provide modules for the compilation.
    pub fn with_library<L>(mut self, library: &L) -> Result<Self, Report>
    where
        L: ?Sized + Library + 'static,
    {
        self.add_library(library)?;

        Ok(self)
    }

    /// Adds the library to provide modules for the compilation.
    pub fn add_library<L>(&mut self, library: &L) -> Result<(), Report>
    where
        L: ?Sized + Library + 'static,
    {
        let namespace = library.root_ns();
        library.modules().try_for_each(|module| {
            if !module.is_in_namespace(namespace) {
                return Err(Report::new(AssemblyError::InconsistentNamespace {
                    expected: namespace.clone(),
                    actual: module.namespace().clone(),
                }));
            }

            self.add_module(module)?;

            Ok(())
        })
    }

    /// Adds a library bundle to provide modules for the compilation.
    pub fn with_libraries<'a, I, L>(mut self, libraries: I) -> Result<Self, Report>
    where
        L: ?Sized + Library + 'static,
        I: IntoIterator<Item = &'a L>,
    {
        self.add_libraries(libraries)?;
        Ok(self)
    }

    /// Adds a library bundle to provide modules for the compilation.
    pub fn add_libraries<'a, I, L>(&mut self, libraries: I) -> Result<(), Report>
    where
        L: ?Sized + Library + 'static,
        I: IntoIterator<Item = &'a L>,
    {
        for library in libraries {
            self.add_library(library)?;
        }
        Ok(())
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

    #[cfg(any(test, feature = "testing"))]
    #[doc(hidden)]
    pub fn module_graph(&self) -> &ModuleGraph {
        &self.module_graph
    }
}

// ------------------------------------------------------------------------------------------------
/// Compilation/Assembly
impl Assembler {
    /// Assembles a set of modules into a [CompiledLibrary].
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or compilation of the specified modules fails.
    pub fn assemble_library(
        mut self,
        modules: impl Iterator<Item = impl Compile>,
    ) -> Result<CompiledLibrary, Report> {
        let ast_module_indices: Vec<ModuleIndex> = modules
            .map(|module| {
                let module = module.compile_with_options(CompileOptions::for_library())?;

                Ok(self.module_graph.add_ast_module(module)?)
            })
            .collect::<Result<_, Report>>()?;
        self.module_graph.recompute()?;

        let mut mast_forest_builder = MastForestBuilder::default();

        let exports = {
            let mut exports = Vec::new();

            for module_idx in ast_module_indices {
                // Note: it is safe to use `unwrap_ast()` here, since all modules looped over are
                // AST (we just added them to the module graph)
                let ast_module = self.module_graph[module_idx].unwrap_ast().clone();

                for (gid, fqn) in ast_module.exported_procedures(module_idx) {
                    self.compile_subgraph(gid, false, &mut mast_forest_builder)?;
                    exports.push(fqn);
                }
            }

            exports
        };

        Ok(CompiledLibrary::new(mast_forest_builder.build(), exports)?)
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

        let module = module.compile_with_options(options)?;
        let module_idx = self.module_graph.add_ast_module(module)?;

        self.module_graph.recompute()?;

        let mut mast_forest_builder = MastForestBuilder::default();

        // Note: it is safe to use `unwrap_ast()` here, since all modules looped over are
        // AST (we just added them to the module graph)
        let ast_module = self.module_graph[module_idx].unwrap_ast().clone();

        let exports = ast_module
            .exported_procedures(module_idx)
            .map(|(gid, fqn)| {
                self.compile_subgraph(gid, false, &mut mast_forest_builder)?;
                Ok(fqn)
            })
            .collect::<Result<Vec<FullyQualifiedProcedureName>, Report>>()?;

        let library = CompiledLibrary::new(mast_forest_builder.build(), exports)?;
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

        let program = source.compile_with_options(options)?;
        assert!(program.is_executable());

        // Recompute graph with executable module, and start compiling
        let ast_module_index = self.module_graph.add_ast_module(program)?;
        self.module_graph.recompute()?;

        // Find the executable entrypoint Note: it is safe to use `unwrap_ast()` here, since this is
        // the module we just added, which is in AST representation.
        let entrypoint = self.module_graph[ast_module_index]
            .unwrap_ast()
            .index_of(|p| p.is_main())
            .map(|index| GlobalProcedureIndex {
                module: ast_module_index,
                index,
            })
            .ok_or(SemanticAnalysisError::MissingEntrypoint)?;

        // Compile the module graph rooted at the entrypoint
        let mut mast_forest_builder = MastForestBuilder::default();
        let entry_procedure = self.compile_subgraph(entrypoint, true, &mut mast_forest_builder)?;

        Ok(Program::with_kernel(
            mast_forest_builder.build(),
            entry_procedure.body_node_id(),
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
        is_entrypoint: bool,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Arc<Procedure>, Report> {
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

        let compiled = if is_entrypoint {
            self.process_graph_worklist(&mut worklist, Some(root), mast_forest_builder)?
        } else {
            let _ = self.process_graph_worklist(&mut worklist, None, mast_forest_builder)?;
            mast_forest_builder.get_procedure(root)
        };

        Ok(compiled.expect("compilation succeeded but root not found in cache"))
    }

    /// Compiles all procedures in the `worklist`.
    fn process_graph_worklist(
        &mut self,
        worklist: &mut Vec<GlobalProcedureIndex>,
        entrypoint: Option<GlobalProcedureIndex>,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Option<Arc<Procedure>>, Report> {
        // Process the topological ordering in reverse order (bottom-up), so that
        // each procedure is compiled with all of its dependencies fully compiled
        let mut compiled_entrypoint = None;
        while let Some(procedure_gid) = worklist.pop() {
            // If we have already compiled this procedure, do not recompile
            if let Some(proc) = mast_forest_builder.get_procedure(procedure_gid) {
                self.module_graph.register_mast_root(procedure_gid, proc.mast_root())?;
                continue;
            }
            let is_entry = entrypoint == Some(procedure_gid);

            // Fetch procedure metadata from the graph
            let module = match &self.module_graph[procedure_gid.module] {
                WrappedModule::Ast(ast_module) => ast_module,
                // Note: if the containing module is in `Info` representation, there is nothing to
                // compile.
                WrappedModule::Info(_) => continue,
            };
            let ast = &module[procedure_gid.index];
            let num_locals = ast.num_locals();
            let name = FullyQualifiedProcedureName {
                span: ast.span(),
                module: module.path().clone(),
                name: ast.name().clone(),
            };
            let pctx = ProcedureContext::new(procedure_gid, name, ast.visibility())
                .with_num_locals(num_locals as u16)
                .with_span(ast.span())
                .with_source_file(ast.source_file());

            // Compile this procedure
            let procedure = self.compile_procedure(pctx, mast_forest_builder)?;

            // Cache the compiled procedure, unless it's the program entrypoint
            if is_entry {
                mast_forest_builder.make_root(procedure.body_node_id());
                compiled_entrypoint = Some(Arc::from(procedure));
            } else {
                // Make the MAST root available to all dependents
                self.module_graph.register_mast_root(procedure_gid, procedure.mast_root())?;
                mast_forest_builder.insert_procedure(procedure_gid, procedure)?;
            }
        }

        Ok(compiled_entrypoint)
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

        let mut mast_node_ids: Vec<MastNodeId> = Vec::new();
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
                            mast_node_ids.push(basic_block_id);
                        }

                        mast_node_ids.push(mast_node_id);
                    }
                }

                Op::If {
                    then_blk, else_blk, ..
                } => {
                    if let Some(basic_block_id) =
                        basic_block_builder.make_basic_block(mast_forest_builder)?
                    {
                        mast_node_ids.push(basic_block_id);
                    }

                    let then_blk =
                        self.compile_body(then_blk.iter(), proc_ctx, None, mast_forest_builder)?;
                    let else_blk =
                        self.compile_body(else_blk.iter(), proc_ctx, None, mast_forest_builder)?;

                    let split_node_id = mast_forest_builder.ensure_split(then_blk, else_blk)?;
                    mast_node_ids.push(split_node_id);
                }

                Op::Repeat { count, body, .. } => {
                    if let Some(basic_block_id) =
                        basic_block_builder.make_basic_block(mast_forest_builder)?
                    {
                        mast_node_ids.push(basic_block_id);
                    }

                    let repeat_node_id =
                        self.compile_body(body.iter(), proc_ctx, None, mast_forest_builder)?;

                    for _ in 0..*count {
                        mast_node_ids.push(repeat_node_id);
                    }
                }

                Op::While { body, .. } => {
                    if let Some(basic_block_id) =
                        basic_block_builder.make_basic_block(mast_forest_builder)?
                    {
                        mast_node_ids.push(basic_block_id);
                    }

                    let loop_body_node_id =
                        self.compile_body(body.iter(), proc_ctx, None, mast_forest_builder)?;

                    let loop_node_id = mast_forest_builder.ensure_loop(loop_body_node_id)?;
                    mast_node_ids.push(loop_node_id);
                }
            }
        }

        if let Some(basic_block_id) =
            basic_block_builder.try_into_basic_block(mast_forest_builder)?
        {
            mast_node_ids.push(basic_block_id);
        }

        Ok(if mast_node_ids.is_empty() {
            mast_forest_builder.ensure_block(vec![Operation::Noop], None)?
        } else {
            combine_mast_node_ids(mast_node_ids, mast_forest_builder)?
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
            source_file: proc_ctx.source_file(),
            module: proc_ctx.id().module,
            kind,
        };
        let resolved = self.module_graph.resolve_target(&caller, target)?;
        match resolved {
            ResolvedTarget::Phantom(digest) => Ok(digest),
            ResolvedTarget::Exact { gid } | ResolvedTarget::Resolved { gid, .. } => {
                match mast_forest_builder.get_procedure(gid) {
                    Some(p) => Ok(p.mast_root()),
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

fn combine_mast_node_ids(
    mut mast_node_ids: Vec<MastNodeId>,
    mast_forest_builder: &mut MastForestBuilder,
) -> Result<MastNodeId, AssemblyError> {
    debug_assert!(!mast_node_ids.is_empty(), "cannot combine empty MAST node id list");

    // build a binary tree of blocks joining them using JOIN blocks
    while mast_node_ids.len() > 1 {
        let last_mast_node_id = if mast_node_ids.len() % 2 == 0 {
            None
        } else {
            mast_node_ids.pop()
        };

        let mut source_mast_node_ids = Vec::new();
        core::mem::swap(&mut mast_node_ids, &mut source_mast_node_ids);

        let mut source_mast_node_iter = source_mast_node_ids.drain(0..);
        while let (Some(left), Some(right)) =
            (source_mast_node_iter.next(), source_mast_node_iter.next())
        {
            let join_mast_node_id = mast_forest_builder.ensure_join(left, right)?;

            mast_node_ids.push(join_mast_node_id);
        }
        if let Some(mast_node_id) = last_mast_node_id {
            mast_node_ids.push(mast_node_id);
        }
    }

    Ok(mast_node_ids.remove(0))
}
