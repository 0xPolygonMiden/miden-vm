use crate::{
    ast::{
        self, AliasTarget, Export, FullyQualifiedProcedureName, Instruction, InvocationTarget,
        InvokeKind, Module, ModuleKind, ProcedureIndex,
    },
    compiled_library::{
        CompiledFullyQualifiedProcedureName, CompiledLibrary, CompiledLibraryMetadata,
        ProcedureInfo,
    },
    diagnostics::{tracing::instrument, Report},
    sema::SemanticAnalysisError,
    AssemblyError, Compile, CompileOptions, Felt, Library, LibraryNamespace, LibraryPath,
    RpoDigest, Spanned, ONE, ZERO,
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use mast_forest_builder::MastForestBuilder;
use miette::miette;
use vm_core::{
    mast::{MastForest, MastNode, MastNodeId, MerkleTreeNode},
    Decorator, DecoratorList, Kernel, Operation, Program,
};

mod basic_block_builder;
mod context;
mod id;
mod instruction;
mod mast_forest_builder;
mod module_graph;
mod procedure;
#[cfg(test)]
mod tests;

pub use self::context::AssemblyContext;
pub use self::id::{GlobalProcedureIndex, ModuleIndex};
pub(crate) use self::module_graph::ProcedureCache;
pub use self::procedure::Procedure;

use self::basic_block_builder::BasicBlockBuilder;
use self::context::ProcedureContext;
use self::module_graph::{CallerInfo, ModuleGraph, ResolvedTarget};

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
/// [Assembler::with_kernel] or [Assembler::with_kernel_from_module].
///
/// <div class="warning">
/// Programs compiled with an empty kernel cannot use the `syscall` instruction.
/// </div>
///
/// * If you have a single executable module you want to compile, just call [Assembler::assemble].
/// * If you want to link your executable to a few other modules that implement supporting
///   procedures, build the assembler with them first, using the various builder methods on
///   [Assembler], e.g. [Assembler::with_module], [Assembler::with_library], etc. Then, call
///   [Assembler::assemble] to get your compiled program.
#[derive(Clone)]
pub struct Assembler {
    mast_forest_builder: MastForestBuilder,
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
            mast_forest_builder: Default::default(),
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

    /// Start building an [`Assembler`] with the given [`Kernel`].
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

        let mut mast_forest_builder = MastForestBuilder::new();

        let (kernel_index, kernel) =
            assembler.assemble_kernel_module(module, &mut mast_forest_builder)?;
        assembler.module_graph.set_kernel(Some(kernel_index), kernel);

        assembler.mast_forest_builder = mast_forest_builder;

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

        self.module_graph.add_ast_module(module)?;

        Ok(())
    }

    /// Adds the compiled library to provide modules for the compilation.
    pub fn add_compiled_library(&mut self, library: CompiledLibrary) -> Result<(), Report> {
        let module_indexes: Vec<ModuleIndex> = library
            .into_module_infos()
            .map(|module| self.module_graph.add_module_info(module))
            .collect::<Result<_, _>>()?;

        self.module_graph.recompute()?;

        // Register all procedures as roots
        for module_index in module_indexes {
            for (proc_index, proc) in
                self.module_graph[module_index].unwrap_info().clone().procedure_infos()
            {
                let gid = GlobalProcedureIndex {
                    module: module_index,
                    index: proc_index,
                };

                self.module_graph.register_mast_root(gid, proc.digest)?;
            }
        }

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
    pub fn module_graph(&self) -> &ModuleGraph {
        &self.module_graph
    }
}

/// Compilation/Assembly
impl Assembler {
    /// Assembles a set of modules into a library.
    ///
    /// The returned library can be added to the assembler assembling a program that depends on the
    /// library using [`Self::add_compiled_library`].
    pub fn assemble_library(
        mut self,
        modules: impl Iterator<Item = impl Compile>,
        metadata: CompiledLibraryMetadata, // name, version etc.
    ) -> Result<CompiledLibrary, Report> {
        let module_ids: Vec<ModuleIndex> = modules
            .map(|module| {
                let module = module.compile_with_options(CompileOptions::for_library())?;

                if module.path().namespace() != &metadata.name {
                    return Err(miette!(
                        "library namespace is {}, but module {} has namespace {}",
                        metadata.name,
                        module.name(),
                        module.path().namespace()
                    ));
                }

                Ok(self.module_graph.add_ast_module(module)?)
            })
            .collect::<Result<_, Report>>()?;
        self.module_graph.recompute()?;

        let mut mast_forest_builder = core::mem::take(&mut self.mast_forest_builder);
        let mut context = AssemblyContext::default();

        self.assemble_graph(&mut context, &mut mast_forest_builder)?;

        let exports = {
            let mut exports = Vec::new();
            for module_id in module_ids {
                let module = self.module_graph.get_module(module_id).unwrap();
                let module_path = module.path();

                let exports_in_module: Vec<CompiledFullyQualifiedProcedureName> = self
                    .get_module_exports(module_id, mast_forest_builder.forest())
                    .map(|procedures| {
                        procedures
                            .into_iter()
                            .map(|proc| {
                                CompiledFullyQualifiedProcedureName::new(
                                    module_path.clone(),
                                    proc.name,
                                )
                            })
                            .collect()
                    })?;

                exports.extend(exports_in_module);
            }

            exports
        };

        Ok(CompiledLibrary::new(mast_forest_builder.build(), exports, metadata)?)
    }

    /// Compiles the provided module into a [`Program`]. The resulting program can be executed on
    /// Miden VM.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or compilation of the specified program fails, or if the source
    /// doesn't have an entrypoint.
    pub fn assemble(self, source: impl Compile) -> Result<Program, Report> {
        let mut context = AssemblyContext::default();
        context.set_warnings_as_errors(self.warnings_as_errors);

        self.assemble_in_context(source, &mut context)
    }

    /// Like [Assembler::assemble], but also takes an [AssemblyContext] to configure the assembler.
    pub fn assemble_in_context(
        self,
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
        self,
        source: impl Compile,
        options: CompileOptions,
    ) -> Result<Program, Report> {
        let mut context = AssemblyContext::default();
        context.set_warnings_as_errors(options.warnings_as_errors);

        self.assemble_with_options_in_context(source, options, &mut context)
    }

    /// Like [Assembler::assemble_with_options], but additionally uses the provided
    /// [AssemblyContext] to configure the assembler.
    #[instrument("assemble_with_opts_in_context", skip_all)]
    pub fn assemble_with_options_in_context(
        self,
        source: impl Compile,
        options: CompileOptions,
        context: &mut AssemblyContext,
    ) -> Result<Program, Report> {
        self.assemble_with_options_in_context_impl(source, options, context)
    }

    /// Implementation of [`Self::assemble_with_options_in_context`] which doesn't consume `self`.
    ///
    /// The main purpose of this separation is to enable some tests to access the assembler state
    /// after assembly.
    fn assemble_with_options_in_context_impl(
        mut self,
        source: impl Compile,
        options: CompileOptions,
        context: &mut AssemblyContext,
    ) -> Result<Program, Report> {
        if options.kind != ModuleKind::Executable {
            return Err(Report::msg(
                "invalid compile options: assemble_with_opts_in_context requires that the kind be 'executable'",
            ));
        }

        let mast_forest_builder = core::mem::take(&mut self.mast_forest_builder);

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
        let module_index = self.module_graph.add_ast_module(program)?;
        self.module_graph.recompute()?;

        // Find the executable entrypoint
        let entrypoint = self.module_graph[module_index]
            .unwrap_ast()
            .index_of(|p| p.is_main())
            .map(|index| GlobalProcedureIndex {
                module: module_index,
                index,
            })
            .ok_or(SemanticAnalysisError::MissingEntrypoint)?;

        self.compile_program(entrypoint, context, mast_forest_builder)
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
        let module_id = self.module_graph.add_ast_module(module)?;
        self.module_graph.recompute()?;

        let mut mast_forest_builder = core::mem::take(&mut self.mast_forest_builder);

        self.assemble_graph(context, &mut mast_forest_builder)?;
        let exported_procedure_digests = self
            .get_module_exports(module_id, mast_forest_builder.forest())
            .map(|procedures| procedures.into_iter().map(|proc| proc.digest).collect());

        // Reassign the mast_forest to the assembler for use is a future program assembly
        self.mast_forest_builder = mast_forest_builder;

        exported_procedure_digests
    }

    /// Compiles the given kernel module, returning both the compiled kernel and its index in the
    /// graph.
    fn assemble_kernel_module(
        &mut self,
        module: Box<ast::Module>,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<(ModuleIndex, Kernel), Report> {
        if !module.is_kernel() {
            return Err(Report::msg(format!("expected kernel module, got {}", module.kind())));
        }

        let mut context = AssemblyContext::for_kernel(module.path());
        context.set_warnings_as_errors(self.warnings_as_errors);

        let kernel_index = self.module_graph.add_ast_module(module)?;
        self.module_graph.recompute()?;
        let kernel_module = self.module_graph[kernel_index].clone();
        let mut kernel = Vec::new();
        for (index, _syscall) in kernel_module
            .unwrap_ast()
            .procedures()
            .enumerate()
            .filter(|(_, p)| p.visibility().is_syscall())
        {
            let gid = GlobalProcedureIndex {
                module: kernel_index,
                index: ProcedureIndex::new(index),
            };
            let compiled = self.compile_subgraph(gid, false, &mut context, mast_forest_builder)?;
            kernel.push(compiled.mast_root(mast_forest_builder.forest()));
        }

        Kernel::new(&kernel)
            .map(|kernel| (kernel_index, kernel))
            .map_err(|err| Report::new(AssemblyError::Kernel(err)))
    }

    /// Get the set of exported procedure infos of the given module.
    ///
    /// Returns an error if the provided Miden Assembly is invalid.
    fn get_module_exports(
        &mut self,
        module_index: ModuleIndex,
        mast_forest: &MastForest,
    ) -> Result<Vec<ProcedureInfo>, Report> {
        assert!(self.module_graph.contains_module(module_index), "invalid module index");

        let exports: Vec<ProcedureInfo> = match &self.module_graph[module_index] {
            module_graph::WrappedModule::Ast(module) => {
                self.get_module_exports_ast(module_index, module, mast_forest)?
            }
            module_graph::WrappedModule::Info(module) => {
                module.procedure_infos().map(|(_idx, proc)| proc).cloned().collect()
            }
        };

        Ok(exports)
    }

    /// Helper function for [`Self::get_module_exports`], specifically for when the inner
    /// [`module_graph::WrappedModule`] is in `Ast` representation.
    fn get_module_exports_ast(
        &self,
        module_index: ModuleIndex,
        module: &Arc<Module>,
        mast_forest: &MastForest,
    ) -> Result<Vec<ProcedureInfo>, Report> {
        let mut exports = Vec::new();
        for (index, procedure) in module.procedures().enumerate() {
            // Only add exports; locals will be added if they are in the call graph rooted
            // at those procedures
            if !procedure.visibility().is_exported() {
                continue;
            }
            let gid = match procedure {
                        Export::Procedure(_) => GlobalProcedureIndex {
                            module: module_index,
                            index: ProcedureIndex::new(index),
                        },
                        Export::Alias(ref alias) => {
                            match alias.target() {
                                AliasTarget::MastRoot(digest) => {
                                    self.procedure_cache.contains_mast_root(digest)
                                        .unwrap_or_else(|| {
                                            panic!(
                                                "compilation apparently succeeded, but did not find a \
                                                        entry in the procedure cache for alias '{}', i.e. '{}'",
                                                alias.name(),
                                                digest
                                            );
                                        })
                                }
                                AliasTarget::Path(ref name)=> {
                                    self.module_graph.find(alias.source_file(), name)?
                                }
                            }
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

            let compiled_proc = ProcedureInfo {
                name: proc.name().clone(),
                digest: mast_forest[proc.body_node_id()].digest(),
            };

            exports.push(compiled_proc);
        }

        Ok(exports)
    }

    /// Compile the provided [Module] into a [Program].
    ///
    /// Ensures that the [`MastForest`] entrypoint is set to the entrypoint of the program.
    ///
    /// Returns an error if the provided Miden Assembly is invalid.
    fn compile_program(
        &mut self,
        entrypoint: GlobalProcedureIndex,
        context: &mut AssemblyContext,
        mut mast_forest_builder: MastForestBuilder,
    ) -> Result<Program, Report> {
        // Raise an error if we are called with an invalid entrypoint
        assert!(self.module_graph.get_procedure_unsafe(entrypoint).name().is_main());

        // Compile the module graph rooted at the entrypoint
        let entry_procedure =
            self.compile_subgraph(entrypoint, true, context, &mut mast_forest_builder)?;

        Ok(Program::with_kernel(
            mast_forest_builder.build(),
            entry_procedure.body_node_id(),
            self.module_graph.kernel().clone(),
        ))
    }

    /// Compile all of the uncompiled procedures in the module graph, placing them
    /// in the procedure cache once compiled.
    ///
    /// Returns an error if any of the provided Miden Assembly is invalid.
    fn assemble_graph(
        &mut self,
        context: &mut AssemblyContext,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<(), Report> {
        let mut worklist = self.module_graph.topological_sort().to_vec();
        assert!(!worklist.is_empty());
        self.process_graph_worklist(&mut worklist, context, None, mast_forest_builder)
            .map(|_| ())
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
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Arc<Procedure>, Report> {
        let mut worklist =
            self.module_graph.topological_sort_ast_procs_from_root(root).map_err(|cycle| {
                let iter = cycle.into_node_ids();
                let mut nodes = Vec::with_capacity(iter.len());
                for node in iter {
                    let module = self.module_graph[node.module].path();
                    let proc = self.module_graph.get_procedure_unsafe(node);
                    nodes.push(format!("{}::{}", module, proc.name()));
                }
                AssemblyError::Cycle { nodes }
            })?;

        assert!(!worklist.is_empty());

        let compiled = if is_entrypoint {
            self.process_graph_worklist(&mut worklist, context, Some(root), mast_forest_builder)?
        } else {
            let _ =
                self.process_graph_worklist(&mut worklist, context, None, mast_forest_builder)?;
            self.procedure_cache.get(root)
        };

        Ok(compiled.expect("compilation succeeded but root not found in cache"))
    }

    fn process_graph_worklist(
        &mut self,
        worklist: &mut Vec<GlobalProcedureIndex>,
        context: &mut AssemblyContext,
        entrypoint: Option<GlobalProcedureIndex>,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Option<Arc<Procedure>>, Report> {
        // Process the topological ordering in reverse order (bottom-up), so that
        // each procedure is compiled with all of its dependencies fully compiled
        let mut compiled_entrypoint = None;
        while let Some(procedure_gid) = worklist.pop() {
            // If we have already compiled this procedure, do not recompile
            if let Some(proc) = self.procedure_cache.get(procedure_gid) {
                self.module_graph.register_mast_root(
                    procedure_gid,
                    proc.mast_root(mast_forest_builder.forest()),
                )?;
                continue;
            }
            let is_entry = entrypoint == Some(procedure_gid);

            // Fetch procedure metadata from the graph
            let module = &self.module_graph[procedure_gid.module].unwrap_ast();
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
            let procedure = self.compile_procedure(pctx, context, mast_forest_builder)?;

            // Cache the compiled procedure, unless it's the program entrypoint
            if is_entry {
                compiled_entrypoint = Some(Arc::from(procedure));
            } else {
                // Make the MAST root available to all dependents
                let digest = procedure.mast_root(mast_forest_builder.forest());
                self.module_graph.register_mast_root(procedure_gid, digest)?;

                self.procedure_cache.insert(
                    procedure_gid,
                    Arc::from(procedure),
                    mast_forest_builder.forest(),
                )?;
            }
        }

        Ok(compiled_entrypoint)
    }

    /// Compiles a single Miden Assembly procedure to its MAST representation.
    fn compile_procedure(
        &self,
        procedure: ProcedureContext,
        context: &mut AssemblyContext,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Box<Procedure>, Report> {
        // Make sure the current procedure context is available during codegen
        let gid = procedure.id();
        let num_locals = procedure.num_locals();
        context.set_current_procedure(procedure);

        let wrapper_proc = self.module_graph.get_procedure_unsafe(gid);
        let proc = wrapper_proc.unwrap_ast().unwrap_procedure();
        let proc_body_root = if num_locals > 0 {
            // for procedures with locals, we need to update fmp register before and after the
            // procedure body is executed. specifically:
            // - to allocate procedure locals we need to increment fmp by the number of locals
            // - to deallocate procedure locals we need to decrement it by the same amount
            let num_locals = Felt::from(num_locals);
            let wrapper = BodyWrapper {
                prologue: vec![Operation::Push(num_locals), Operation::FmpUpdate],
                epilogue: vec![Operation::Push(-num_locals), Operation::FmpUpdate],
            };
            self.compile_body(proc.iter(), context, Some(wrapper), mast_forest_builder)?
        } else {
            self.compile_body(proc.iter(), context, None, mast_forest_builder)?
        };

        mast_forest_builder.make_root(proc_body_root);

        let pctx = context.take_current_procedure().unwrap();
        Ok(pctx.into_procedure(proc_body_root))
    }

    fn compile_body<'a, I>(
        &self,
        body: I,
        context: &mut AssemblyContext,
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
                        context,
                        mast_forest_builder,
                    )? {
                        if let Some(basic_block_id) =
                            basic_block_builder.make_basic_block(mast_forest_builder)
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
                        basic_block_builder.make_basic_block(mast_forest_builder)
                    {
                        mast_node_ids.push(basic_block_id);
                    }

                    let then_blk =
                        self.compile_body(then_blk.iter(), context, None, mast_forest_builder)?;
                    let else_blk =
                        self.compile_body(else_blk.iter(), context, None, mast_forest_builder)?;

                    let split_node_id = {
                        let split_node =
                            MastNode::new_split(then_blk, else_blk, mast_forest_builder.forest());

                        mast_forest_builder.ensure_node(split_node)
                    };
                    mast_node_ids.push(split_node_id);
                }

                Op::Repeat { count, body, .. } => {
                    if let Some(basic_block_id) =
                        basic_block_builder.make_basic_block(mast_forest_builder)
                    {
                        mast_node_ids.push(basic_block_id);
                    }

                    let repeat_node_id =
                        self.compile_body(body.iter(), context, None, mast_forest_builder)?;

                    for _ in 0..*count {
                        mast_node_ids.push(repeat_node_id);
                    }
                }

                Op::While { body, .. } => {
                    if let Some(basic_block_id) =
                        basic_block_builder.make_basic_block(mast_forest_builder)
                    {
                        mast_node_ids.push(basic_block_id);
                    }

                    let loop_body_node_id =
                        self.compile_body(body.iter(), context, None, mast_forest_builder)?;

                    let loop_node_id = {
                        let loop_node =
                            MastNode::new_loop(loop_body_node_id, mast_forest_builder.forest());
                        mast_forest_builder.ensure_node(loop_node)
                    };
                    mast_node_ids.push(loop_node_id);
                }
            }
        }

        if let Some(basic_block_id) = basic_block_builder.into_basic_block(mast_forest_builder) {
            mast_node_ids.push(basic_block_id);
        }

        Ok(if mast_node_ids.is_empty() {
            let basic_block_node = MastNode::new_basic_block(vec![Operation::Noop]);
            mast_forest_builder.ensure_node(basic_block_node)
        } else {
            combine_mast_node_ids(mast_node_ids, mast_forest_builder)
        })
    }

    pub(super) fn resolve_target(
        &self,
        kind: InvokeKind,
        target: &InvocationTarget,
        context: &AssemblyContext,
        mast_forest: &MastForest,
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
            ResolvedTarget::Exact { gid } | ResolvedTarget::Resolved { gid, .. } => Ok(
                // first look in the module graph, and fallback to the procedure cache
                self.module_graph.get_mast_root(gid).copied().unwrap_or_else(|| {
                    self.procedure_cache
                        .get(gid)
                        .map(|p| p.mast_root(mast_forest))
                        .expect("expected callee to have been compiled already")
                }),
            ),
        }
    }
}

/// Contains a set of operations which need to be executed before and after a sequence of AST
/// nodes (i.e., code body).
struct BodyWrapper {
    prologue: Vec<Operation>,
    epilogue: Vec<Operation>,
}

fn combine_mast_node_ids(
    mut mast_node_ids: Vec<MastNodeId>,
    mast_forest_builder: &mut MastForestBuilder,
) -> MastNodeId {
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
            let join_mast_node = MastNode::new_join(left, right, mast_forest_builder.forest());
            let join_mast_node_id = mast_forest_builder.ensure_node(join_mast_node);

            mast_node_ids.push(join_mast_node_id);
        }
        if let Some(mast_node_id) = last_mast_node_id {
            mast_node_ids.push(mast_node_id);
        }
    }

    mast_node_ids.remove(0)
}
