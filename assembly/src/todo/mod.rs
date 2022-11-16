use crate::{
    parsers::{self, Node, ProcedureAst, ProgramAst},
    AssemblerError, BTreeMap, Box, CallSet, Kernel, ModuleProvider, NamedModuleAst, Procedure,
    ProcedureId, String, ToString, Vec,
};
use core::borrow::Borrow;
use vm_core::{code_blocks::CodeBlock, CodeBlockTable, DecoratorList, Operation, Program};

mod instruction;

mod span_builder;
use span_builder::SpanBuilder;

mod context;
use context::AssemblerContext;

// ASSEMBLER
// ================================================================================================

/// TODO: add comments
pub struct Assembler {
    kernel: Kernel,
    module_provider: Box<dyn ModuleProvider>,
    proc_cache: BTreeMap<ProcedureId, Procedure>,
    in_debug_mode: bool,
}

impl Assembler {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Returns a new instance of [Assembler] instantiated with empty module map.
    pub fn new() -> Self {
        Self {
            kernel: Kernel::default(),
            module_provider: Box::new(()),
            proc_cache: BTreeMap::default(),
            in_debug_mode: false,
        }
    }

    /// Puts the assembler into the debug mode.
    pub fn with_debug_mode(mut self) -> Self {
        self.in_debug_mode = true;
        self
    }

    /// Adds the specified [ModuleProvider] to the assembler.
    pub fn with_module_provider<P>(mut self, provider: P) -> Self
    where
        P: ModuleProvider + 'static,
    {
        self.module_provider = Box::new(provider);
        self
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
    pub fn compile<S>(&self, source: S) -> Result<Program, AssemblerError>
    where
        S: AsRef<str>,
    {
        // parse the program into an AST
        let source = source.as_ref();
        let ProgramAst { local_procs, body } = parsers::parse_program(source)?;

        // compile all local procedures
        let mut context = AssemblerContext::for_program();
        let mut callset = CallSet::default();
        for (proc_idx, proc_ast) in local_procs.iter().enumerate() {
            if proc_ast.is_export {
                // TODO: return an error
                panic!("exported procedure in program");
            }
            let proc = self.compile_procedure(proc_ast, proc_idx as u16, &context)?;
            callset.append(proc.callset());
            context.add_local_proc(proc);
        }

        // compile the program body
        let program_root = self.compile_body(body.iter(), &context, &mut callset)?;

        // build the code block table based on the callset constructed during program compilation;
        // called procedures can be either in the global procedure cache (for procedures imported
        // from other modules) or in the context (for procedures defined locally).
        let mut cb_table = CodeBlockTable::default();
        for proc_id in callset.iter() {
            let proc = self
                .proc_cache
                .get(proc_id)
                .or_else(|| context.find_local_proc(proc_id))
                .expect("callset procedure not found");

            cb_table.insert(proc.code_root().clone());
        }

        // build and return the program
        Ok(Program::with_kernel(
            program_root,
            self.kernel.clone(),
            cb_table,
        ))
    }

    // MODULE COMPILER
    // --------------------------------------------------------------------------------------------

    /// Compiles all procedures in the specified module and adds them to the procedure cache.
    #[allow(clippy::cast_ref_to_mut)]
    fn compile_module(&self, module: &NamedModuleAst) -> Result<(), AssemblerError> {
        // compile all procedures in the module and also build a combined callset for all
        // procedures
        let mut ctx = AssemblerContext::for_module(module.path().to_string());
        let mut callset = CallSet::default();
        for (proc_idx, proc_ast) in module.local_procs.iter().enumerate() {
            let proc = self.compile_procedure(proc_ast, proc_idx as u16, &ctx)?;
            callset.append(proc.callset());
            ctx.add_local_proc(proc);
        }

        // add the compiled procedures to the assembler's cache. the procedures are added to the
        // cache only if:
        // - a procedure is exported from the module, or
        // - a procedure is present in the combined callset - i.e., it is an internal procedure
        //   which has been invoked via a local call instruction.
        for proc in ctx.into_local_procs() {
            if proc.is_export() || callset.contains(proc.id()) {
                // TODO: figure out how to do this using interior mutability
                unsafe {
                    let mutable_self = &mut *(self as *const _ as *mut Assembler);
                    mutable_self.proc_cache.insert(*proc.id(), proc);
                }
            }
        }

        Ok(())
    }

    // PROCEDURE COMPILER
    // --------------------------------------------------------------------------------------------

    /// Compiles procedure AST into MAST and returns the compiled procedure.
    ///
    /// In addition to building the MAST, this also does the following:
    /// - Computes and assigns an ID to the procedure. This is done for both exported and internal
    ///   procedures.
    /// - Determines the procedure's callset - i.e., a set of procedures which may be invoked
    ///   during execution of this procedure.
    fn compile_procedure(
        &self,
        proc: &ProcedureAst,
        proc_idx: u16,
        context: &AssemblerContext,
    ) -> Result<Procedure, AssemblerError> {
        // compile the procedure body
        let mut callset = CallSet::default();
        let code_root = self.compile_body(proc.body.iter(), context, &mut callset)?;

        // build an ID for the procedure as follows:
        // - for exported procedures: hash("module_path::proc_name")
        // - for internal procedures: hash("module_path::proc_index")
        //
        // this ID can then be used to retrieve procedures from the assembler's procedure cache.
        let proc_id = if proc.is_export {
            ProcedureId::from_name(&proc.name, context.module_path())
        } else {
            ProcedureId::from_index(proc_idx, context.module_path())
        };

        Ok(Procedure {
            id: proc_id,
            label: proc.name.to_string(),
            is_export: proc.is_export,
            num_locals: proc.num_locals,
            code_root,
            callset,
        })
    }

    // CODE BODY COMPILER
    // --------------------------------------------------------------------------------------------

    /// TODO: add comments
    fn compile_body<A, N>(
        &self,
        body: A,
        context: &AssemblerContext,
        callset: &mut CallSet,
    ) -> Result<CodeBlock, AssemblerError>
    where
        A: Iterator<Item = N>,
        N: Borrow<Node>,
    {
        let (size, hint) = body.size_hint();
        let size = hint.unwrap_or(size);

        let mut blocks: Vec<CodeBlock> = Vec::with_capacity(size);
        let mut span = SpanBuilder::default();

        for node in body {
            match node.borrow() {
                Node::Instruction(instruction) => {
                    if let Some(block) =
                        self.compile_instruction(instruction, &mut span, context, callset)?
                    {
                        span.extract_span_into(&mut blocks);
                        blocks.push(block);
                    }
                }

                Node::IfElse(t, f) => {
                    span.extract_span_into(&mut blocks);

                    let t = self.compile_body(t.iter(), context, callset)?;
                    let f = self.compile_body(f.iter(), context, callset)?;
                    let block = CodeBlock::new_split(t, f);

                    blocks.push(block);
                }

                Node::Repeat(n, nodes) => {
                    span.extract_span_into(&mut blocks);

                    let block = self.compile_body(nodes.iter(), context, callset)?;

                    for _ in 0..*n {
                        blocks.push(block.clone());
                    }
                }

                Node::While(nodes) => {
                    span.extract_span_into(&mut blocks);

                    let block = self.compile_body(nodes.iter(), context, callset)?;
                    let block = CodeBlock::new_loop(block);

                    blocks.push(block);
                }
            }
        }

        span.extract_span_into(&mut blocks);

        Ok(parsers::combine_blocks(blocks))
    }

    // PROCEDURE GETTER
    // --------------------------------------------------------------------------------------------
    /// Returns procedure MAST for a procedure with the specified ID.
    ///
    /// This will first check if procedure is in the assembler's cache, and if not, will attempt
    /// to find the module in which the procedure is located, compile the module, and return the
    /// compiled procedure MAST.
    fn get_imported_proc(&self, proc_id: &ProcedureId) -> Result<&Procedure, AssemblerError> {
        // if the procedure is already in the procedure cache, return it
        if let Some(p) = self.proc_cache.get(proc_id) {
            return Ok(p);
        }

        // otherwise, get the module to which the procedure belongs and compile the entire module;
        // this will add all procedures exported from the module to the procedure cache
        let module = self
            .module_provider
            .get_module(proc_id)
            .ok_or_else(|| AssemblerError::undefined_imported_proc(proc_id))?;
        self.compile_module(&module)?;

        // then, get the procedure out of the procedure cache and return
        let proc = self
            .proc_cache
            .get(proc_id)
            .expect("compiled imported procedure not in procedure cache");
        Ok(proc)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

// TESTS
// ================================================================================================

#[test]
fn nested_block_works() {
    use crate::{ModuleAst, NamedModuleAst};

    struct DummyModuleProvider {
        module: ModuleAst,
    }

    impl ModuleProvider for DummyModuleProvider {
        fn get_source(&self, _path: &str) -> Option<&str> {
            None
        }

        fn get_module(&self, _id: &ProcedureId) -> Option<NamedModuleAst<'_>> {
            Some(NamedModuleAst::new("foo::bar", &self.module))
        }
    }

    let module_provider = DummyModuleProvider {
        module: parsers::parse_module(
            r#"
            export.baz
                push.29
            end"#,
        )
        .unwrap(),
    };

    let program = r#"
    use.foo::bar

    proc.foo
        push.19
    end

    proc.bar
        push.17
        exec.foo
    end

    begin
        push.2
        if.true
            push.3
        else
            push.5
        end
        if.true
            if.true
                push.7
            else
                push.11
            end
        else
            push.13
            while.true
                exec.bar
                push.23
            end
        end
        exec.bar::baz
    end"#;

    let before = CodeBlock::new_span(vec![Operation::Push(2u64.into())]);

    let r#true = CodeBlock::new_span(vec![Operation::Push(3u64.into())]);
    let r#false = CodeBlock::new_span(vec![Operation::Push(5u64.into())]);
    let r#if = CodeBlock::new_split(r#true, r#false);

    let r#true = CodeBlock::new_span(vec![Operation::Push(7u64.into())]);
    let r#false = CodeBlock::new_span(vec![Operation::Push(11u64.into())]);
    let r#true = CodeBlock::new_split(r#true, r#false);
    let r#while = CodeBlock::new_span(vec![
        Operation::Push(17u64.into()),
        Operation::Push(19u64.into()),
        Operation::Push(23u64.into()),
    ]);
    let r#while = CodeBlock::new_loop(r#while);
    let span = CodeBlock::new_span(vec![Operation::Push(13u64.into())]);
    let r#false = CodeBlock::new_join([span, r#while]);
    let nested = CodeBlock::new_split(r#true, r#false);

    let exec = CodeBlock::new_span(vec![Operation::Push(29u64.into())]);

    let combined = parsers::combine_blocks(vec![before, r#if, nested, exec]);
    let program = Assembler::new()
        .with_module_provider(module_provider)
        .compile(program)
        .unwrap();

    assert_eq!(combined.hash(), program.hash());
}
