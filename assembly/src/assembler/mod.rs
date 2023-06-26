use super::{
    ast::{Instruction, ModuleAst, Node, ProcedureAst, ProgramAst},
    btree_map,
    crypto::hash::RpoDigest,
    AssemblyError, BTreeMap, CallSet, CodeBlock, CodeBlockTable, Felt, Kernel, Library,
    LibraryError, LibraryPath, Module, Operation, Procedure, ProcedureId, ProcedureName, Program,
    ToString, Vec, ONE, ZERO,
};
use core::{borrow::Borrow, cell::RefCell};
use vm_core::{utils::group_vector_elements, Decorator, DecoratorList};

mod instruction;

mod module_provider;
use module_provider::ModuleProvider;

mod span_builder;
use span_builder::SpanBuilder;

mod context;
pub use context::{AssemblyContext, AssemblyContextType};

mod procedure_cache;
use procedure_cache::ProcedureCache;

#[cfg(test)]
mod tests;

// ASSEMBLER
// ================================================================================================
/// Miden Assembler which can be used to convert Miden assembly source code into program MAST.
///
/// The assembler can be instantiated in several ways using a "builder" pattern. Specifically:
/// - If `with_kernel()` or `with_kernel_module()` methods are not used, the assembler will be
///   instantiated with a default empty kernel. Programs compiled using such assembler
///   cannot make calls to kernel procedures via `syscall` instruction.
#[derive(Default)]
pub struct Assembler {
    kernel: Kernel,
    module_provider: ModuleProvider,
    proc_cache: RefCell<ProcedureCache>,
    in_debug_mode: bool,
}

impl Assembler {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Puts the assembler into the debug mode.
    pub fn with_debug_mode(mut self, in_debug_mode: bool) -> Self {
        self.in_debug_mode = in_debug_mode;
        self
    }

    /// Adds the library to provide modules for the compilation.
    pub fn with_library<L>(mut self, library: &L) -> Result<Self, AssemblyError>
    where
        L: Library,
    {
        self.module_provider.add_library(library)?;
        Ok(self)
    }

    /// Adds a library bundle to provide modules for the compilation.
    pub fn with_libraries<I, L>(self, mut libraries: I) -> Result<Self, AssemblyError>
    where
        L: Library,
        I: Iterator<Item = L>,
    {
        libraries.try_fold(self, |slf, library| slf.with_library(&library))
    }

    /// Sets the kernel for the assembler to the kernel defined by the provided source.
    ///
    /// # Errors
    /// Returns an error if compiling kernel source results in an error.
    ///
    /// # Panics
    /// Panics if the assembler has already been used to compile programs.
    pub fn with_kernel(self, kernel_source: &str) -> Result<Self, AssemblyError> {
        let kernel_ast = ModuleAst::parse(kernel_source)?;
        self.with_kernel_module(kernel_ast)
    }

    /// Sets the kernel for the assembler to the kernel defined by the provided module.
    ///
    /// # Errors
    /// Returns an error if compiling kernel source results in an error.
    pub fn with_kernel_module(mut self, module: ModuleAst) -> Result<Self, AssemblyError> {
        // compile the kernel; this adds all exported kernel procedures to the procedure cache
        let mut context = AssemblyContext::new(AssemblyContextType::Kernel);
        let kernel = Module::kernel(module);
        self.compile_module(&kernel, &mut context)?;

        // convert the context into Kernel; this builds the kernel from hashes of procedures
        // exported form the kernel module
        self.kernel = context.into_kernel();

        Ok(self)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns true if this assembler was instantiated in debug mode.
    pub fn in_debug_mode(&self) -> bool {
        self.in_debug_mode
    }

    /// Returns a reference to the kernel for this assembler.
    ///
    /// If the assembler was instantiated without a kernel, the internal kernel will be empty.
    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    // PROGRAM COMPILER
    // --------------------------------------------------------------------------------------------
    /// Compiles the provided source code into a [Program]. The resulting program can be executed
    /// on Miden VM.
    ///
    /// # Errors
    /// Returns an error if parsing or compilation of the specified program fails.
    pub fn compile<S>(&self, source: S) -> Result<Program, AssemblyError>
    where
        S: AsRef<str>,
    {
        // parse the program into an AST
        let source = source.as_ref();
        let program = ProgramAst::parse(source)?;

        // compile the program
        let mut context = AssemblyContext::new(AssemblyContextType::Program);
        let program_root = self.compile_in_context(&program, &mut context)?;

        // convert the context into a call block table for the program
        let cb_table = context.into_cb_table(&self.proc_cache.borrow())?;

        // build and return the program
        Ok(Program::with_kernel(program_root, self.kernel.clone(), cb_table))
    }

    /// Compiles the provided [ProgramAst] into a program and returns the program root
    /// ([CodeBlock]).  Mutates the provided context by adding all of the call targets of
    /// the program to the [CallSet].
    ///
    /// # Errors
    /// - If the provided context is not appropriate for compiling a program.
    /// - If any of the local procedures defined in the program are exported.
    /// - If compilation of any of the local procedures fails.
    /// - if compilation of the program body fails.
    pub fn compile_in_context(
        &self,
        program: &ProgramAst,
        context: &mut AssemblyContext,
    ) -> Result<CodeBlock, AssemblyError> {
        // check to ensure that the context is appropriate for compiling a program
        if context.current_context_name() != ProcedureName::main().as_str() {
            return Err(AssemblyError::InvalidProgramAssemblyContext);
        }

        // compile all local procedures; this will add the procedures to the specified context
        for proc_ast in program.procedures() {
            if proc_ast.is_export {
                return Err(AssemblyError::exported_proc_in_program(&proc_ast.name));
            }
            self.compile_procedure(proc_ast, context)?;
        }

        // compile the program body
        let program_root = self.compile_body(program.body().nodes().iter(), context, None)?;

        Ok(program_root)
    }
    // MODULE COMPILER
    // --------------------------------------------------------------------------------------------

    /// Compiles all procedures in the specified module and adds them to the procedure cache.
    /// Returns a vector of procedure digests for all exported procedures in the module.
    ///
    /// # Errors
    /// - If a module with the same path already exists in the module stack of the
    ///   [AssemblyContext].
    /// - If a lock to the [ProcedureCache] can not be attained.
    pub fn compile_module(
        &self,
        module: &Module,
        context: &mut AssemblyContext,
    ) -> Result<Vec<RpoDigest>, AssemblyError> {
        // a variable to track MAST roots of all procedures exported from this module
        let mut proc_roots = Vec::new();

        context.begin_module(&module.path)?;

        // process all re-exported procedures
        for reexporteed_proc in module.ast.reexported_procs().iter() {
            // make sure the re-exported procedure is loaded into the procedure cache
            let ref_proc_id = reexporteed_proc.proc_id();
            self.ensure_procedure_is_in_cache(&ref_proc_id, context)?;

            // build procedure ID for the alias, and add it to the procedure cache
            let proc_name = reexporteed_proc.name();
            let alias_proc_id = ProcedureId::from_name(proc_name, &module.path);
            let proc_mast_root = self
                .proc_cache
                .try_borrow_mut()
                .map_err(|_| AssemblyError::InvalidCacheLock)?
                .insert_proc_alias(alias_proc_id, ref_proc_id)?;

            // add the MAST root of the re-exported procedure to the set of procedures exported
            // from this module
            proc_roots.push(proc_mast_root);
        }

        // compile all local procedures in the module; once the compilation is complete, we get
        // all compiled procedures (and their combined callset) from the context
        for proc_ast in module.ast.procs().iter() {
            self.compile_procedure(proc_ast, context)?;
        }
        let (module_procs, module_callset) = context.complete_module();

        // add the compiled procedures to the assembler's cache. the procedures are added to the
        // cache only if:
        // - a procedure is exported from the module, or
        // - a procedure is present in the combined callset - i.e., it is an internal procedure
        //   which has been invoked via a local call instruction.
        for proc in module_procs.into_iter() {
            if proc.is_export() {
                proc_roots.push(proc.code_root().hash());
            }

            if proc.is_export() || module_callset.contains(proc.id()) {
                // this is safe because we fail if the cache is borrowed.
                self.proc_cache
                    .try_borrow_mut()
                    .map_err(|_| AssemblyError::InvalidCacheLock)?
                    .insert(proc)?;
            }
        }

        Ok(proc_roots)
    }

    // PROCEDURE COMPILER
    // --------------------------------------------------------------------------------------------

    /// Compiles procedure AST into MAST and adds the complied procedure to the provided context.
    fn compile_procedure(
        &self,
        proc: &ProcedureAst,
        context: &mut AssemblyContext,
    ) -> Result<(), AssemblyError> {
        context.begin_proc(&proc.name, proc.is_export, proc.num_locals)?;

        let code_root = if proc.num_locals > 0 {
            // for procedures with locals, we need to update fmp register before and after the
            // procedure body is executed. specifically:
            // - to allocate procedure locals we need to increment fmp by the number of locals
            // - to deallocate procedure locals we need to decrement it by the same amount
            let num_locals = Felt::from(proc.num_locals);
            let wrapper = BodyWrapper {
                prologue: vec![Operation::Push(num_locals), Operation::FmpUpdate],
                epilogue: vec![Operation::Push(-num_locals), Operation::FmpUpdate],
            };
            self.compile_body(proc.body.nodes().iter(), context, Some(wrapper))?
        } else {
            self.compile_body(proc.body.nodes().iter(), context, None)?
        };

        context.complete_proc(code_root);

        Ok(())
    }

    // CODE BODY COMPILER
    // --------------------------------------------------------------------------------------------

    /// TODO: add comments
    fn compile_body<A, N>(
        &self,
        body: A,
        context: &mut AssemblyContext,
        wrapper: Option<BodyWrapper>,
    ) -> Result<CodeBlock, AssemblyError>
    where
        A: Iterator<Item = N>,
        N: Borrow<Node>,
    {
        let mut blocks: Vec<CodeBlock> = Vec::new();
        let mut span = SpanBuilder::new(wrapper);

        for node in body {
            match node.borrow() {
                Node::Instruction(inner) => {
                    if let Some(block) = self.compile_instruction(inner, &mut span, context)? {
                        span.extract_span_into(&mut blocks);
                        blocks.push(block);
                    }
                }

                Node::IfElse {
                    true_case,
                    false_case,
                } => {
                    span.extract_span_into(&mut blocks);

                    let true_case = self.compile_body(true_case.nodes().iter(), context, None)?;

                    // else is an exception because it is optional; hence, will have to be replaced
                    // by noop span
                    let false_case = if !false_case.nodes().is_empty() {
                        self.compile_body(false_case.nodes().iter(), context, None)?
                    } else {
                        CodeBlock::new_span(vec![Operation::Noop])
                    };

                    let block = CodeBlock::new_split(true_case, false_case);

                    blocks.push(block);
                }

                Node::Repeat { times, body } => {
                    span.extract_span_into(&mut blocks);

                    let block = self.compile_body(body.nodes().iter(), context, None)?;

                    for _ in 0..*times {
                        blocks.push(block.clone());
                    }
                }

                Node::While { body } => {
                    span.extract_span_into(&mut blocks);

                    let block = self.compile_body(body.nodes().iter(), context, None)?;
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

    // PROCEDURE CACHE
    // --------------------------------------------------------------------------------------------

    /// Ensures that a procedure with the specified [ProcedureId] exists in the cache. Otherwise,
    /// attempt to fetch it from the module provider, compile, and check again.
    ///
    /// If `Ok` is returned, the procedure can be safely unwrapped from the cache.
    ///
    /// # Panics
    ///
    /// This function will panic if the internal procedure cache is mutably borrowed somewhere.
    fn ensure_procedure_is_in_cache(
        &self,
        proc_id: &ProcedureId,
        context: &mut AssemblyContext,
    ) -> Result<(), AssemblyError> {
        if !self.proc_cache.borrow().contains_id(proc_id) {
            // if procedure is not in cache, try to get its module and compile it
            let module = self
                .module_provider
                .get_module(proc_id)
                .ok_or_else(|| AssemblyError::imported_proc_module_not_found(proc_id))?;
            self.compile_module(module, context)?;
            // if the procedure is still not in cache, then there was some error
            if !self.proc_cache.borrow().contains_id(proc_id) {
                return Err(AssemblyError::imported_proc_not_found_in_module(
                    proc_id,
                    &module.path,
                ));
            }
        }

        Ok(())
    }

    // CODE BLOCK BUILDER
    // --------------------------------------------------------------------------------------------
    /// Returns the [CodeBlockTable] associated with the [AssemblyContext].
    ///
    /// # Errors
    /// Returns an error if a required procedure is not found in the [Assembler] procedure cache.
    pub fn build_cb_table(
        &self,
        context: AssemblyContext,
    ) -> Result<CodeBlockTable, AssemblyError> {
        context.into_cb_table(&self.proc_cache.borrow())
    }
}

// BODY WRAPPER
// ================================================================================================

/// Contains a set of operations which need to be executed before and after a sequence of AST
/// nodes (i.e., code body).
struct BodyWrapper {
    prologue: Vec<Operation>,
    epilogue: Vec<Operation>,
}

// UTILITY FUNCTIONS
// ================================================================================================

pub fn combine_blocks(mut blocks: Vec<CodeBlock>) -> CodeBlock {
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

    // build a binary tree of blocks joining them using Join blocks
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

/// Returns a CodeBlock [Span] from sequence of Span blocks provided as input.
pub fn combine_spans(spans: &mut Vec<CodeBlock>) -> CodeBlock {
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
            panic!("Codeblock was expected to be a Span Block, got {block:?}.");
        }
    });
    CodeBlock::new_span_with_decorators(ops, decorators)
}
