use crate::{
    ast::{
        self, instrument, Export, FullyQualifiedProcedureName, Instruction, InvocationTarget,
        InvokeKind, Module, ModuleKind, ProcedureIndex,
    },
    diagnostics::{Report, SourceFile},
    sema::SemanticAnalysisError,
    AssemblyError, Felt, Library, LibraryNamespace, LibraryPath, RpoDigest, Spanned, ONE, ZERO,
};
use alloc::{boxed::Box, string::ToString, sync::Arc, vec::Vec};
use vm_core::{
    code_blocks::CodeBlock, utils::group_vector_elements, CodeBlockTable, Decorator, DecoratorList,
    Kernel, Operation, Program,
};

mod callgraph;
mod context;
mod id;
mod instruction;
mod module_graph;
mod procedure;
mod procedure_cache;
mod span_builder;
#[cfg(test)]
mod tests;

pub use self::context::AssemblyContext;
pub use self::id::{GlobalProcedureIndex, ModuleIndex};
pub use self::procedure::Procedure;

use self::module_graph::{CallerInfo, ModuleGraph, ResolvedTarget};
pub(crate) use self::procedure_cache::ProcedureCache;
use self::span_builder::SpanBuilder;
use self::{callgraph::CallGraph, context::ProcedureContext};

/// Represents the type of artifact produced by an [Assembler]
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ArtifactKind {
    /// Produce an executable program.
    ///
    /// This is the default artifact produced by the assembler,
    /// and is the only artifact that is useful on its own.
    #[default]
    Executable,
    /// Produce a MAST library
    ///
    /// The assembler will produce MAST in binary form which can
    /// be packaged and distributed. These artifacts can then be
    /// loaded by the VM with an executable program that references
    /// the contents of the library, without having to compile them
    /// together.
    Library,
    /// Produce a MAST kernel module
    ///
    /// The assembler will produce MAST for a kernel module, which is
    /// essentially the same as `Library`, however additional constraints
    /// are imposed on compilation to ensure that the produced kernel is
    /// valid.
    Kernel,
}

/// The [Assembler] is the primary interface for compiling Miden Assembly
/// to the Miden Abstract Syntax Tree (MAST).
///
/// # Usage
///
/// Depending on your needs, there are multiple ways of using the assembler,
/// and whether or not you want to provide a custom kernel.
///
/// By default, an empty kernel is provided. However, you may provide your
/// own using [Assembler::with_kernel] or [Assembler::with_kernel_from_source].
///
/// <div class="warning">Programs compiled with an empty kernel cannot use the `syscall` instruction.</div>
///
/// * If you have a single executable module you want to compile, just call
/// [Assembler::compile] or [Assembler::compile_ast], depending on whether
/// you have source code in raw or parsed form.
///
/// * If you want to link your executable to a few other modules that implement
/// supporting procedures, build the assembler with them first, using the various
/// builder methods on [Assembler], e.g. [Assembler::with_module],
/// [Assembler::with_library], etc. Then, call [Assembler::compile] or
/// [Assembler::compile_ast] to get your compiled program.
///
/// # Assembly Contexts
///
/// Using the instructions above, all of the code you provide will be compiled
/// and cached using a single global context. That works fine if you are creating
/// the assembler and discarding it after you've compiled your program. However,
/// if you plan to compile multiple distinct programs, you will want to use
/// [AssemblyContext]s and [Assembler::compile_in_context].
///
/// An [AssemblyContext] is essentially a way to isolate the program-specific
/// elements of a compilation session in a separate cache, so that you avoid
/// polluting the global cache with a bunch of objects from multiple programs,
/// causing analysis to become more expensive. By isolating those in a context-
/// specific cache, you have more fine control over how things are cached.
///
/// More precisely, the [Assembler] has a global module graph and procedure
/// cache, which it uses to perform analysis during compilation, and to avoid
/// redundantly compiling the same procedures for every program. Any modules
/// you add to the global context will be inherited by _all_ contexts. This
/// is where you will cache the kernel, standard libraries, anything else
/// that is quite common.
///
/// Each [AssemblyContext] has its own module graph and procedure cache, which
/// contains only those modules which you add to it. When you call [Assembler::compile_in_context]
/// with that context, it is merged with the global context, inter-procedural
/// analysis is performed, and then the compiled objects are cached in the
/// provided [AssemblyContext], allowing it to be used multiple times if desired.
///
/// <div class="warning">The context isolation described above is not currently how
/// things are implemented, but I believe represent where we will want to ultimately
/// take the `AssemblyContext` struct. The main obstacle right now is that we don't
/// have a clear picture of how an `Assembler` will be used, so we want to make
/// sure that we design the `Assembler` and `AssemblyContext` relationship in
/// such a way that it plays well with the most common usage patterns.</div>
pub struct Assembler {
    /// The global [ModuleGraph] for this assembler. All new
    /// [AssemblyContext]s inherit this graph as a baseline.
    module_graph: Box<ModuleGraph>,
    /// The global procedure cache for this assembler
    procedure_cache: ProcedureCache,
    /// Whether the assembler enables extra debugging information
    in_debug_mode: bool,
    /// Whether the assembler allows unknown invocation targets in compiled code
    allow_phantom_calls: bool,
}
impl Default for Assembler {
    fn default() -> Self {
        Self {
            module_graph: Default::default(),
            procedure_cache: Default::default(),
            in_debug_mode: false,
            allow_phantom_calls: true,
        }
    }
}
impl Assembler {
    /// Start building an [Assembler]
    pub fn new() -> Self {
        Self::default()
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

    /// Returns the [ModuleIndex] of the kernel module, if the kernel was provided
    /// in source form to the assembler. This returns None when the kernel was
    /// provided in precompiled form.
    fn kernel_index(&self) -> Option<ModuleIndex> {
        self.module_graph.kernel_index()
    }

    /// Returns true if this assembler was instantiated with phantom calls enabled
    pub fn allow_phantom_calls(&self) -> bool {
        self.allow_phantom_calls
    }

    /// Puts the assembler into the debug mode.
    pub fn with_debug_mode(mut self, yes: bool) -> Self {
        self.in_debug_mode = yes;
        self
    }

    /// Set whether to allow phantom calls
    pub fn with_phantom_calls(mut self, yes: bool) -> Self {
        self.allow_phantom_calls = yes;
        self
    }

    /// Add `module` to the module graph of the assembler
    ///
    /// The given module must be a library module, or an error will be returned
    pub fn with_module(mut self, module: Box<ast::Module>) -> Result<Self, Report> {
        self.add_module(module)?;

        Ok(self)
    }

    /// Add `module` to the module graph of the assembler
    ///
    /// The given module must be a library module, or an error will be returned
    pub fn add_module(&mut self, module: Box<ast::Module>) -> Result<(), Report> {
        match module.kind() {
            ModuleKind::Kernel if self.kernel_index().is_some() => {
                Err(Report::new(AssemblyError::ConflictingKernels))
            }
            ModuleKind::Kernel => {
                let (kernel_index, kernel) = self.compile_kernel_module(module)?;
                self.module_graph.set_kernel(Some(kernel_index), kernel);

                Ok(())
            }
            ModuleKind::Executable => {
                Err(Report::msg("cannot call `add_module` with an executable module: you must provide it via `compile` instead"))
            }
            ModuleKind::Library => {
                self.module_graph.add_module(module)?;

                Ok(())
            }
        }
    }

    /// Parse `source` as a library module with path `path`, and add it to the module graph of the assembler
    pub fn with_module_from_source(
        mut self,
        path: LibraryPath,
        source: impl ToString,
    ) -> Result<Self, Report> {
        self.add_module_from_source(path, source)?;
        Ok(self)
    }

    /// Parse `source` as a library module with path `path`, and add it to the module graph of the assembler
    pub fn add_module_from_source(
        &mut self,
        path: LibraryPath,
        source: impl ToString,
    ) -> Result<(), Report> {
        let kind = ModuleKind::Library;
        let source = Arc::new(SourceFile::new(path.path(), source.to_string()));
        let ast = ast::Module::parse(path, kind, source)?;
        self.add_module(ast)
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

            self.add_module(Box::new(module.clone()))?;

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

    /// Sets the kernel for the assmbler to `kernel`
    pub fn with_kernel(mut self, kernel: Kernel) -> Result<Self, Report> {
        if self.module_graph.has_nonempty_kernel() {
            return Err(Report::new(AssemblyError::ConflictingKernels));
        }
        self.module_graph.set_kernel(None, kernel);
        Ok(self)
    }

    /// Sets the kernel for the assembler to the kernel defined by the provided source.
    ///
    /// # Errors
    /// Returns an error if compiling kernel source results in an error.
    ///
    /// # Panics
    /// Panics if the assembler has already been used to compile programs.
    pub fn with_kernel_from_source(self, source: impl ToString) -> Result<Self, Report> {
        let ns = LibraryNamespace::Kernel;
        let source_file = Arc::from(SourceFile::new(LibraryNamespace::Kernel, source.to_string()));
        let kernel = Module::parse(ns.into(), ModuleKind::Kernel, source_file)?;
        self.with_kernel_from_module(kernel)
    }

    /// Sets the kernel for the assembler to the kernel defined by the provided abstract syntax tree.
    ///
    /// # Errors
    ///
    /// Returns an error if the given module is not a valid kernel module, or if
    /// compiling kernel source results in an error.
    pub fn with_kernel_from_module(mut self, module: Box<Module>) -> Result<Self, Report> {
        if self.module_graph.has_nonempty_kernel() {
            return Err(Report::new(AssemblyError::ConflictingKernels));
        }
        let (kernel_index, kernel) = self.compile_kernel_module(module)?;
        self.module_graph.set_kernel(Some(kernel_index), kernel);

        Ok(self)
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

/// Compilation
impl Assembler {
    /// Compiles the provided source code into a [Program]. The resulting program can be executed
    /// on Miden VM.
    ///
    /// # Errors
    /// Returns an error if parsing or compilation of the specified program fails.
    pub fn compile<S>(&mut self, source: S) -> Result<Program, Report>
    where
        S: ToString,
    {
        let ns = LibraryNamespace::Exec;
        let source = Arc::new(SourceFile::new(ns.as_str(), source.to_string()));
        self.compile_source(source)
    }

    /// Like [Assembler::compile], but takes a [SourceFile], allowing the caller to provide their
    /// own file name to be used in diagnostics.
    pub fn compile_source(&mut self, source: Arc<SourceFile>) -> Result<Program, Report> {
        // parse the program into an AST
        let kind = ModuleKind::Executable;
        let ns = LibraryNamespace::Exec;
        let ast = ast::Module::parse(ns.into(), kind, source)?;

        // compile the program and return
        self.compile_ast(ast)
    }

    /// Compile the file at the provided path into a [Program]. The resulting program can be
    /// executed on Miden VM.
    ///
    /// # Errors
    /// Returns an error if we fail to read the file, or if parsing/compilation fails.
    #[cfg(feature = "std")]
    pub fn compile_file<P>(&mut self, path: P) -> Result<Program, Report>
    where
        P: AsRef<std::path::Path>,
    {
        // parse the program into an AST
        let kind = ModuleKind::Executable;
        let ns = LibraryNamespace::Exec;
        let ast = ast::Module::parse_file(ns.into(), kind, path)?;

        // compile the program and return
        self.compile_ast(ast)
    }

    /// Compiles the provided abstract syntax tree into a [Program]. The resulting program can be
    /// executed on Miden VM.
    ///
    /// # Errors
    ///
    /// * If the provided context is not appropriate for compiling a program.
    /// * If compilation of the program fails
    #[instrument("compile_ast", skip_all)]
    pub fn compile_ast(&mut self, program: Box<ast::Module>) -> Result<Program, Report> {
        let mut context = AssemblyContext::default();
        self.compile_in_context(program, &mut context)
    }

    /// Compile `program` using the provided [AssemblyContext] to configure compilation
    pub fn compile_in_context(
        &mut self,
        program: Box<ast::Module>,
        context: &mut AssemblyContext,
    ) -> Result<Program, Report> {
        if !program.is_executable() {
            return Err(Report::msg(format!("expected executable module, got {}", program.kind())));
        }

        // Remove any previously compiled executable module and clean up graph
        let prev_program = self.module_graph.find_module_index(&LibraryNamespace::Exec.into());
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

    /// Compile all procedures in the specified module, adding them to the procedure cache.
    ///
    /// Returns a vector of procedure digests for all exported procedures in the module.
    ///
    /// The provided context is used to determine what type of module to compile, i.e. either
    /// a kernel or library module.
    pub fn compile_module(
        &mut self,
        module: Box<ast::Module>,
        context: &mut AssemblyContext,
    ) -> Result<Vec<RpoDigest>, Report> {
        if module.is_executable() {
            return Err(Report::msg(format!(
                "expected library or kernel module, got {}",
                module.kind()
            )));
        }
        match context.kind() {
            ArtifactKind::Executable => return Err(Report::msg("invalid context: expected context configured for library or kernel modules")),
            ArtifactKind::Kernel if !module.is_kernel() => return Err(Report::msg("invalid context: cannot compile a kernel with a context configured for library compilation")),
            ArtifactKind::Library if module.is_kernel() => return Err(Report::msg("invalid context: cannot compile a library with a context configured for kernel compilation")),
            ArtifactKind::Kernel | ArtifactKind::Library => (),
        }

        // Recompute graph with the provided module, and start compiling
        let module_id = self.module_graph.add_module(module)?;
        self.module_graph.recompute()?;
        self.compile_graph(context)?;

        self.module_exports(module_id)
    }

    /// Compiles the given kernel module.
    ///
    /// This will return an error if the kernel is invalid.
    pub fn compile_kernel(&mut self, module: Box<ast::Module>) -> Result<Kernel, Report> {
        self.compile_kernel_module(module).map(|(_, kernel)| kernel)
    }

    /// Compiles the given kernel module, returning both the compiled kernel and its index in the graph.
    fn compile_kernel_module(
        &mut self,
        module: Box<ast::Module>,
    ) -> Result<(ModuleIndex, Kernel), Report> {
        if !module.is_kernel() {
            return Err(Report::msg(format!("expected kernel module, got {}", module.kind())));
        }

        let mut context = AssemblyContext::for_kernel(module.path());
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
    fn module_exports(&mut self, module: ModuleIndex) -> Result<Vec<RpoDigest>, Report> {
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
            let proc = self.procedure_cache
                .get(gid)
                .unwrap_or_else(|| match procedure {
                    Export::Procedure(ref proc) => {
                        panic!("compilation apparently succeeded, but did not find a entry in the procedure cache for '{}'", proc.name())
                    }
                    Export::Alias(ref alias) => {
                        panic!("compilation apparently succeeded, but did not find a entry in the procedure cache for alias '{}', i.e. '{}'",
                            alias.name(), alias.target());
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
    fn compile_graph(&mut self, context: &mut AssemblyContext) -> Result<(), Report> {
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

        let mut grouped_blocks = Vec::new();
        core::mem::swap(&mut blocks, &mut grouped_blocks);
        let mut grouped_blocks = group_vector_elements::<CodeBlock, 2>(grouped_blocks);
        grouped_blocks.drain(0..).for_each(|pair| {
            blocks.push(CodeBlock::new_join(pair));
        });

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
