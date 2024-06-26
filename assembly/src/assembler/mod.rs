use crate::{
    ast::{
        self, Export, FullyQualifiedProcedureName, Instruction, InvocationTarget, InvokeKind,
        ModuleKind, ProcedureIndex,
    },
    diagnostics::{tracing::instrument, Report},
    sema::SemanticAnalysisError,
    AssemblyError, Compile, CompileOptions, Felt, Library, LibraryNamespace, LibraryPath,
    RpoDigest, Spanned, ONE, ZERO,
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use vm_core::{
    code_blocks::CodeBlock, CodeBlockTable, Decorator, DecoratorList, Kernel, Operation, Program,
};

mod context;
mod id;
mod instruction;
mod module_graph;
mod procedure;
mod span_builder;
#[cfg(test)]
mod tests;

pub use self::context::AssemblyContext;
pub use self::id::{GlobalProcedureIndex, ModuleIndex};
pub(crate) use self::module_graph::ProcedureCache;
pub use self::procedure::Procedure;

use self::context::ProcedureContext;
use self::module_graph::{CallerInfo, ModuleGraph, ResolvedTarget};
use self::span_builder::SpanBuilder;

// ARTIFACT KIND
// ================================================================================================

/// Represents the type of artifact produced by an [Assembler].
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ArtifactKind {
    /// Produce an executable program.
    ///
    /// This is the default artifact produced by the assembler, and is the only artifact that is
    /// useful on its own.
    #[default]
    Executable,
    /// Produce a MAST library
    ///
    /// The assembler will produce MAST in binary form which can be packaged and distributed.
    /// These artifacts can then be loaded by the VM with an executable program that references
    /// the contents of the library, without having to compile them together.
    Library,
    /// Produce a MAST kernel module
    ///
    /// The assembler will produce MAST for a kernel module, which is essentially the same as
    /// [crate::Library], however additional constraints are imposed on compilation to ensure that
    /// the produced kernel is valid.
    Kernel,
}

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
/// By default, an empty kernel is provided. However, you may provide your own using
/// [Assembler::with_kernel] or [Assembler::with_kernel_from_source].
///
/// <div class="warning">
/// Programs compiled with an empty kernel cannot use the `syscall` instruction.
/// </div>
///
/// * If you have a single executable module you want to compile, just call [Assembler::compile] or
///   [Assembler::compile_ast], depending on whether you have source code in raw or parsed form.
///
/// * If you want to link your executable to a few other modules that implement supporting
///   procedures, build the assembler with them first, using the various builder methods on
///   [Assembler], e.g. [Assembler::with_module], [Assembler::with_library], etc. Then, call
///   [Assembler::compile] or [Assembler::compile_ast] to get your compiled program.
pub struct Assembler {
    /// The global [ModuleGraph] for this assembler. All new [AssemblyContext]s inherit this graph
    /// as a baseline.
    module_graph: Box<ModuleGraph>,
    /// The global procedure cache for this assembler.
    procedure_cache: ProcedureCache,
    /// Whether to treat warning diagnostics as errors
    warnings_as_errors: bool,
    /// Whether the assembler enables extra debugging information.
    in_debug_mode: bool,
    /// Whether the assembler allows unknown invocation targets in compiled code.
    allow_phantom_calls: bool,
}

impl Default for Assembler {
    fn default() -> Self {
        Self {
            module_graph: Default::default(),
            procedure_cache: Default::default(),
            warnings_as_errors: false,
            in_debug_mode: false,
            allow_phantom_calls: true,
        }
    }
}

/// Builder
impl Assembler {
    /// Start building an [Assembler]
    pub fn new() -> Self {
        Self::default()
    }

    /// Start building an [Assembler] with the given [Kernel].
    pub fn with_kernel(kernel: Kernel) -> Self {
        let mut assembler = Self::new();
        assembler.module_graph.set_kernel(None, kernel);
        assembler
    }

    /// Start building an [Assembler], with a kernel given by compiling the given source module.
    ///
    /// # Errors
    /// Returns an error if compiling kernel source results in an error.
    pub fn with_kernel_from_module(module: impl Compile) -> Result<Self, Report> {
        let mut assembler = Self::new();
        let opts = CompileOptions::for_kernel();
        let module = module.compile_with_options(opts)?;
        let (kernel_index, kernel) = assembler.assemble_kernel_module(module)?;
        assembler.module_graph.set_kernel(Some(kernel_index), kernel);

        Ok(assembler)
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

    /// Sets whether to allow phantom calls.
    pub fn with_phantom_calls(mut self, yes: bool) -> Self {
        self.allow_phantom_calls = yes;
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

        self.module_graph.add_module(module)?;

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

/// Queries
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

    /// Returns true if this assembler was instantiated with phantom calls enabled.
    pub fn allow_phantom_calls(&self) -> bool {
        self.allow_phantom_calls
    }

    #[cfg(any(test, feature = "testing"))]
    #[doc(hidden)]
    pub fn procedure_cache(&self) -> &ProcedureCache {
        &self.procedure_cache
    }

    #[cfg(any(test, feature = "testing"))]
    #[doc(hidden)]
    pub fn module_graph(&self) -> &ModuleGraph {
        &self.module_graph
    }
}

/// Compilation/Assembly
impl Assembler {
    /// Compiles the provided module into a [Program]. The resulting program can be executed
    /// on Miden VM.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or compilation of the specified program fails.
    pub fn assemble(&mut self, source: impl Compile) -> Result<Program, Report> {
        let mut context = AssemblyContext::default();
        context.set_warnings_as_errors(self.warnings_as_errors);

        self.assemble_in_context(source, &mut context)
    }

    /// Like [Assembler::compile], but also takes an [AssemblyContext] to configure the assembler.
    pub fn assemble_in_context(
        &mut self,
        source: impl Compile,
        context: &mut AssemblyContext,
    ) -> Result<Program, Report> {
        let opts = CompileOptions {
            warnings_as_errors: context.warnings_as_errors(),
            ..CompileOptions::default()
        };
        self.assemble_with_options_in_context(source, opts, context)
    }

    /// Compiles the provided module into a [Program] using the provided options.
    ///
    /// The resulting program can be executed on Miden VM.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or compilation of the specified program fails, or the options
    /// are invalid.
    pub fn assemble_with_options(
        &mut self,
        source: impl Compile,
        options: CompileOptions,
    ) -> Result<Program, Report> {
        let mut context = AssemblyContext::default();
        context.set_warnings_as_errors(options.warnings_as_errors);

        self.assemble_with_options_in_context(source, options, &mut context)
    }

    /// Like [Assembler::compile_with_opts], but additionally uses the provided [AssemblyContext]
    /// to configure the assembler.
    #[instrument("assemble_with_opts_in_context", skip_all)]
    pub fn assemble_with_options_in_context(
        &mut self,
        source: impl Compile,
        options: CompileOptions,
        context: &mut AssemblyContext,
    ) -> Result<Program, Report> {
        if options.kind != ModuleKind::Executable {
            return Err(Report::msg(
                "invalid compile options: assemble_with_opts_in_context requires that the kind be 'executable'",
            ));
        }

        let program = source.compile_with_options(CompileOptions {
            // Override the module name so that we always compile the executable
            // module as #exec
            path: Some(LibraryPath::from(LibraryNamespace::Exec)),
            ..options
        })?;
        assert!(program.is_executable());

        // Remove any previously compiled executable module and clean up graph
        let prev_program = self.module_graph.find_module_index(program.path());
        if let Some(module_index) = prev_program {
            self.module_graph.remove_module(module_index);
            self.procedure_cache.remove_module(module_index);
        }

        // Recompute graph with executable module, and start compiling
        let module_index = self.module_graph.add_module(program)?;
        self.module_graph.recompute()?;

        // Find the executable entrypoint
        let entrypoint = self.module_graph[module_index]
            .index_of(|p| p.is_main())
            .map(|index| GlobalProcedureIndex {
                module: module_index,
                index,
            })
            .ok_or(SemanticAnalysisError::MissingEntrypoint)?;

        self.compile_program(entrypoint, context)
    }

    /// Compile and assembles all procedures in the specified module, adding them to the procedure
    /// cache.
    ///
    /// Returns a vector of procedure digests for all exported procedures in the module.
    ///
    /// The provided context is used to determine what type of module to assemble, i.e. either
    /// a kernel or library module.
    pub fn assemble_module(
        &mut self,
        module: impl Compile,
        options: CompileOptions,
        context: &mut AssemblyContext,
    ) -> Result<Vec<RpoDigest>, Report> {
        match context.kind() {
            _ if options.kind.is_executable() => {
                return Err(Report::msg(
                    "invalid compile options: expected configuration for library or kernel module ",
                ))
            }
            ArtifactKind::Executable => {
                return Err(Report::msg(
                    "invalid context: expected context configured for library or kernel modules",
                ))
            }
            ArtifactKind::Kernel if !options.kind.is_kernel() => {
                return Err(Report::msg(
                    "invalid context: cannot assemble a kernel from a module compiled as a library",
                ))
            }
            ArtifactKind::Library if !options.kind.is_library() => {
                return Err(Report::msg(
                    "invalid context: cannot assemble a library from a module compiled as a kernel",
                ))
            }
            ArtifactKind::Kernel | ArtifactKind::Library => (),
        }

        // Compile module
        let module = module.compile_with_options(options)?;

        // Recompute graph with the provided module, and start assembly
        let module_id = self.module_graph.add_module(module)?;
        self.module_graph.recompute()?;
        self.assemble_graph(context)?;

        self.get_module_exports(module_id)
    }

    /// Compiles the given kernel module, returning both the compiled kernel and its index in the
    /// graph.
    fn assemble_kernel_module(
        &mut self,
        module: Box<ast::Module>,
    ) -> Result<(ModuleIndex, Kernel), Report> {
        if !module.is_kernel() {
            return Err(Report::msg(format!("expected kernel module, got {}", module.kind())));
        }

        let mut context = AssemblyContext::for_kernel(module.path());
        context.set_warnings_as_errors(self.warnings_as_errors);

        let kernel_index = self.module_graph.add_module(module)?;
        self.module_graph.recompute()?;
        let kernel_module = self.module_graph[kernel_index].clone();
        let mut kernel = Vec::new();
        for (index, _syscall) in kernel_module
            .procedures()
            .enumerate()
            .filter(|(_, p)| p.visibility().is_syscall())
        {
            let gid = GlobalProcedureIndex {
                module: kernel_index,
                index: ProcedureIndex::new(index),
            };
            let compiled = self.compile_subgraph(gid, false, &mut context)?;
            kernel.push(compiled.code().hash());
        }

        Kernel::new(&kernel)
            .map(|kernel| (kernel_index, kernel))
            .map_err(|err| Report::new(AssemblyError::Kernel(err)))
    }

    /// Get the set of procedure roots for all exports of the given module
    ///
    /// Returns an error if the provided Miden Assembly is invalid.
    fn get_module_exports(&mut self, module: ModuleIndex) -> Result<Vec<RpoDigest>, Report> {
        assert!(self.module_graph.contains_module(module), "invalid module index");

        let mut exports = Vec::new();
        for (index, procedure) in self.module_graph[module].procedures().enumerate() {
            // Only add exports to the code block table, locals will
            // be added if they are in the call graph rooted at those
            // procedures
            if !procedure.visibility().is_exported() {
                continue;
            }
            let gid = match procedure {
                Export::Procedure(_) => GlobalProcedureIndex {
                    module,
                    index: ProcedureIndex::new(index),
                },
                Export::Alias(ref alias) => {
                    self.module_graph.find(alias.source_file(), alias.target())?
                }
            };
            let proc = self.procedure_cache.get(gid).unwrap_or_else(|| match procedure {
                Export::Procedure(ref proc) => {
                    panic!(
                        "compilation apparently succeeded, but did not find a \
                                entry in the procedure cache for '{}'",
                        proc.name()
                    )
                }
                Export::Alias(ref alias) => {
                    panic!(
                        "compilation apparently succeeded, but did not find a \
                                entry in the procedure cache for alias '{}', i.e. '{}'",
                        alias.name(),
                        alias.target()
                    );
                }
            });

            exports.push(proc.code().hash());
        }

        Ok(exports)
    }

    /// Compile the provided [Module] into a [Program].
    ///
    /// Returns an error if the provided Miden Assembly is invalid.
    fn compile_program(
        &mut self,
        entrypoint: GlobalProcedureIndex,
        context: &mut AssemblyContext,
    ) -> Result<Program, Report> {
        // Raise an error if we are called with an invalid entrypoint
        assert!(self.module_graph[entrypoint].name().is_main());

        // Compile the module graph rooted at the entrypoint
        let entry = self.compile_subgraph(entrypoint, true, context)?;

        // Construct the code block table by taking the call set of the
        // executable entrypoint and adding the code blocks of all those
        // procedures to the table.
        let mut code_blocks = CodeBlockTable::default();
        for callee in entry.callset().iter() {
            let code_block = self
                .procedure_cache
                .get_by_mast_root(callee)
                .map(|p| p.code().clone())
                .ok_or(AssemblyError::UndefinedCallSetProcedure { digest: *callee })?;
            code_blocks.insert(code_block);
        }

        let body = entry.code().clone();
        Ok(Program::with_kernel(body, self.module_graph.kernel().clone(), code_blocks))
    }

    /// Compile all of the uncompiled procedures in the module graph, placing them
    /// in the procedure cache once compiled.
    ///
    /// Returns an error if any of the provided Miden Assembly is invalid.
    fn assemble_graph(&mut self, context: &mut AssemblyContext) -> Result<(), Report> {
        let mut worklist = self.module_graph.topological_sort().to_vec();
        assert!(!worklist.is_empty());
        self.process_graph_worklist(&mut worklist, context, None).map(|_| ())
    }

    /// Compile the uncompiled procedure in the module graph which are members of the subgraph
    /// rooted at `root`, placing them in the procedure cache once compiled.
    ///
    /// Returns an error if any of the provided Miden Assembly is invalid.
    fn compile_subgraph(
        &mut self,
        root: GlobalProcedureIndex,
        is_entrypoint: bool,
        context: &mut AssemblyContext,
    ) -> Result<Arc<Procedure>, Report> {
        let mut worklist = self.module_graph.topological_sort_from_root(root).map_err(|cycle| {
            let iter = cycle.into_node_ids();
            let mut nodes = Vec::with_capacity(iter.len());
            for node in iter {
                let module = self.module_graph[node.module].path();
                let proc = self.module_graph[node].name();
                nodes.push(format!("{}::{}", module, proc));
            }
            AssemblyError::Cycle { nodes }
        })?;

        assert!(!worklist.is_empty());

        let compiled = if is_entrypoint {
            self.process_graph_worklist(&mut worklist, context, Some(root))?
        } else {
            let _ = self.process_graph_worklist(&mut worklist, context, None)?;
            self.procedure_cache.get(root)
        };

        Ok(compiled.expect("compilation succeeded but root not found in cache"))
    }

    fn process_graph_worklist(
        &mut self,
        worklist: &mut Vec<GlobalProcedureIndex>,
        context: &mut AssemblyContext,
        entrypoint: Option<GlobalProcedureIndex>,
    ) -> Result<Option<Arc<Procedure>>, Report> {
        // Process the topological ordering in reverse order (bottom-up), so that
        // each procedure is compiled with all of its dependencies fully compiled
        let mut compiled_entrypoint = None;
        while let Some(procedure_gid) = worklist.pop() {
            // If we have already compiled this procedure, do not recompile
            if let Some(proc) = self.procedure_cache.get(procedure_gid) {
                self.module_graph.register_mast_root(procedure_gid, proc.mast_root())?;
                continue;
            }
            let is_entry = entrypoint == Some(procedure_gid);

            // Fetch procedure metadata from the graph
            let module = &self.module_graph[procedure_gid.module];
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
            let procedure = self.compile_procedure(pctx, context)?;

            // Cache the compiled procedure, unless it's the program entrypoint
            if is_entry {
                compiled_entrypoint = Some(Arc::from(procedure));
            } else {
                // Make the MAST root available to all dependents
                let digest = procedure.mast_root();
                self.module_graph.register_mast_root(procedure_gid, digest)?;

                self.procedure_cache.insert(procedure_gid, Arc::from(procedure))?;
            }
        }

        Ok(compiled_entrypoint)
    }

    /// Compiles a single Miden Assembly procedure to its MAST representation.
    fn compile_procedure(
        &self,
        procedure: ProcedureContext,
        context: &mut AssemblyContext,
    ) -> Result<Box<Procedure>, Report> {
        // Make sure the current procedure context is available during codegen
        let gid = procedure.id();
        let num_locals = procedure.num_locals();
        context.set_current_procedure(procedure);

        let proc = self.module_graph[gid].unwrap_procedure();
        let code = if num_locals > 0 {
            // for procedures with locals, we need to update fmp register before and after the
            // procedure body is executed. specifically:
            // - to allocate procedure locals we need to increment fmp by the number of locals
            // - to deallocate procedure locals we need to decrement it by the same amount
            let num_locals = Felt::from(num_locals);
            let wrapper = BodyWrapper {
                prologue: vec![Operation::Push(num_locals), Operation::FmpUpdate],
                epilogue: vec![Operation::Push(-num_locals), Operation::FmpUpdate],
            };
            self.compile_body(proc.iter(), context, Some(wrapper))?
        } else {
            self.compile_body(proc.iter(), context, None)?
        };

        let pctx = context.take_current_procedure().unwrap();
        Ok(pctx.into_procedure(code))
    }

    fn compile_body<'a, I>(
        &self,
        body: I,
        context: &mut AssemblyContext,
        wrapper: Option<BodyWrapper>,
    ) -> Result<CodeBlock, Report>
    where
        I: Iterator<Item = &'a ast::Op>,
    {
        use ast::Op;

        let mut blocks: Vec<CodeBlock> = Vec::new();
        let mut span = SpanBuilder::new(wrapper);

        for op in body {
            match op {
                Op::Inst(inst) => {
                    if let Some(block) = self.compile_instruction(inst, &mut span, context)? {
                        span.extract_span_into(&mut blocks);
                        blocks.push(block);
                    }
                }

                Op::If {
                    then_blk, else_blk, ..
                } => {
                    span.extract_span_into(&mut blocks);

                    let then_blk = self.compile_body(then_blk.iter(), context, None)?;
                    // else is an exception because it is optional; hence, will have to be replaced
                    // by noop span
                    let else_blk = if else_blk.is_empty() {
                        CodeBlock::new_span(vec![Operation::Noop])
                    } else {
                        self.compile_body(else_blk.iter(), context, None)?
                    };

                    let block = CodeBlock::new_split(then_blk, else_blk);

                    blocks.push(block);
                }

                Op::Repeat { count, body, .. } => {
                    span.extract_span_into(&mut blocks);

                    let block = self.compile_body(body.iter(), context, None)?;

                    for _ in 0..*count {
                        blocks.push(block.clone());
                    }
                }

                Op::While { body, .. } => {
                    span.extract_span_into(&mut blocks);

                    let block = self.compile_body(body.iter(), context, None)?;
                    let block = CodeBlock::new_loop(block);

                    blocks.push(block);
                }
            }
        }

        span.extract_final_span_into(&mut blocks);
        Ok(if blocks.is_empty() {
            CodeBlock::new_span(vec![Operation::Noop])
        } else {
            combine_blocks(blocks)
        })
    }

    pub(super) fn resolve_target(
        &self,
        kind: InvokeKind,
        target: &InvocationTarget,
        context: &AssemblyContext,
    ) -> Result<RpoDigest, AssemblyError> {
        let current_proc = context.unwrap_current_procedure();
        let caller = CallerInfo {
            span: target.span(),
            source_file: current_proc.source_file(),
            module: current_proc.id().module,
            kind,
        };
        let resolved = self.module_graph.resolve_target(&caller, target)?;
        match resolved {
            ResolvedTarget::Phantom(digest) | ResolvedTarget::Cached { digest, .. } => Ok(digest),
            ResolvedTarget::Exact { gid } | ResolvedTarget::Resolved { gid, .. } => Ok(self
                .procedure_cache
                .get(gid)
                .map(|p| p.mast_root())
                .expect("expected callee to have been compiled already")),
        }
    }
}

/// Contains a set of operations which need to be executed before and after a sequence of AST
/// nodes (i.e., code body).
struct BodyWrapper {
    prologue: Vec<Operation>,
    epilogue: Vec<Operation>,
}

fn combine_blocks(mut blocks: Vec<CodeBlock>) -> CodeBlock {
    debug_assert!(!blocks.is_empty(), "cannot combine empty block list");
    // merge consecutive Span blocks.
    let mut merged_blocks: Vec<CodeBlock> = Vec::with_capacity(blocks.len());
    // Keep track of all the consecutive Span blocks and are merged together when
    // there is a discontinuity.
    let mut contiguous_spans: Vec<CodeBlock> = Vec::new();

    blocks.drain(0..).for_each(|block| {
        if block.is_span() {
            contiguous_spans.push(block);
        } else {
            if !contiguous_spans.is_empty() {
                merged_blocks.push(combine_spans(&mut contiguous_spans));
            }
            merged_blocks.push(block);
        }
    });
    if !contiguous_spans.is_empty() {
        merged_blocks.push(combine_spans(&mut contiguous_spans));
    }

    // build a binary tree of blocks joining them using JOIN blocks
    let mut blocks = merged_blocks;
    while blocks.len() > 1 {
        let last_block = if blocks.len() % 2 == 0 { None } else { blocks.pop() };

        let mut source_blocks = Vec::new();
        core::mem::swap(&mut blocks, &mut source_blocks);

        let mut source_block_iter = source_blocks.drain(0..);
        while let (Some(left), Some(right)) = (source_block_iter.next(), source_block_iter.next()) {
            blocks.push(CodeBlock::new_join([left, right]));
        }
        if let Some(block) = last_block {
            blocks.push(block);
        }
    }

    debug_assert!(!blocks.is_empty(), "no blocks");
    blocks.remove(0)
}

/// Combines a vector of SPAN blocks into a single SPAN block.
///
/// # Panics
/// Panics if any of the provided blocks is not a SPAN block.
fn combine_spans(spans: &mut Vec<CodeBlock>) -> CodeBlock {
    if spans.len() == 1 {
        return spans.remove(0);
    }

    let mut ops = Vec::<Operation>::new();
    let mut decorators = DecoratorList::new();
    spans.drain(0..).for_each(|block| {
        if let CodeBlock::Span(span) = block {
            for decorator in span.decorators() {
                decorators.push((decorator.0 + ops.len(), decorator.1.clone()));
            }
            for batch in span.op_batches() {
                ops.extend_from_slice(batch.ops());
            }
        } else {
            panic!("CodeBlock was expected to be a Span Block, got {block:?}.");
        }
    });
    CodeBlock::new_span_with_decorators(ops, decorators)
}
