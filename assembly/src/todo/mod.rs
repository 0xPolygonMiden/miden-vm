mod instruction;

use core::borrow::Borrow;

use crate::BTreeMap;
use vm_core::{code_blocks::CodeBlock, CodeBlockTable, DecoratorList, Operation, Program};

use crate::{
    parsers::{self, Node, ProcedureAst, ProgramAst},
    procedures::Procedure,
    AssemblerError, ModuleProvider, ProcedureId,
};

#[derive(Default)]
struct SpanBuilder {
    ops: Vec<Operation>,
    #[allow(dead_code)]
    decorators: DecoratorList,
}

impl SpanBuilder {
    pub fn add_op(&mut self, op: Operation) -> Result<Option<CodeBlock>, AssemblerError> {
        self.ops.push(op);
        Ok(None)
    }

    pub fn add_ops<I, O>(&mut self, ops: I) -> Result<Option<CodeBlock>, AssemblerError>
    where
        I: IntoIterator<Item = O>,
        O: Borrow<Operation>,
    {
        self.ops.extend(ops.into_iter().map(|o| *o.borrow()));
        Ok(None)
    }

    #[allow(dead_code)]
    pub fn push_op(&mut self, op: Operation) {
        self.ops.push(op);
    }

    #[allow(dead_code)]
    pub fn has_ops(&self) -> bool {
        self.ops.is_empty()
    }

    pub fn extract_span_into(&mut self, target: &mut Vec<CodeBlock>) {
        if !self.ops.is_empty() {
            let ops: Vec<_> = self.ops.drain(..).collect();
            target.push(CodeBlock::new_span(ops));
        }
    }
}

pub struct AssemblerContext {
    cb_table: CodeBlockTable,
    locals: Vec<Procedure>,
    procedures: BTreeMap<ProcedureId, CodeBlock>,
}

impl AssemblerContext {
    pub fn get_code(&self, id: &ProcedureId) -> Option<&CodeBlock> {
        self.procedures.get(id)
    }

    pub fn add_local_procedure(&mut self, procedure: Procedure) {
        if procedure.is_export() {
            /*
            // what is the local path for exported procedures?
            let id = todo!();
            self.procedures.insert(id, procedure.code_root().clone());
            */
        }
        self.locals.push(procedure);
    }
}

pub struct Assembler {
    module_provider: Box<dyn ModuleProvider>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            module_provider: Box::new(()),
        }
    }

    pub fn with_module_provider<P>(mut self, provider: P) -> Self
    where
        P: ModuleProvider + 'static,
    {
        self.module_provider = Box::new(provider);
        self
    }

    pub fn compile<S>(&self, source: S) -> Result<Program, AssemblerError>
    where
        S: AsRef<str>,
    {
        let source = source.as_ref();
        let ProgramAst { local_procs, body } = parsers::parse_program(source)?;
        let mut context = self.context(local_procs.iter())?;

        let root = self.compile_body(body.iter(), &mut context)?;

        Ok(Program::new(root))
    }

    fn context<I, P>(&self, procs: I) -> Result<AssemblerContext, AssemblerError>
    where
        I: Iterator<Item = P>,
        P: Borrow<ProcedureAst>,
    {
        let (size, hint) = procs.size_hint();
        let size = hint.unwrap_or(size);

        let mut context = AssemblerContext {
            cb_table: CodeBlockTable::default(),
            locals: Vec::with_capacity(size),
            procedures: BTreeMap::new(),
        };

        for proc in procs {
            self.compile_procedure(proc, &mut context)
                .map(|proc| context.add_local_procedure(proc))?;
        }

        Ok(context)
    }

    fn compile_procedure<P>(
        &self,
        procedure: P,
        context: &mut AssemblerContext,
    ) -> Result<Procedure, AssemblerError>
    where
        P: Borrow<ProcedureAst>,
    {
        let ProcedureAst {
            name,
            num_locals,
            body,
            is_export,
            ..
        } = procedure.borrow();

        let code_root = self.compile_body(body.iter(), context)?;

        Ok(Procedure {
            label: name.to_string(),
            is_export: *is_export,
            num_locals: *num_locals,
            code_root,
        })
    }

    fn compile_body<A, N>(
        &self,
        ast: A,
        context: &mut AssemblerContext,
    ) -> Result<CodeBlock, AssemblerError>
    where
        A: Iterator<Item = N>,
        N: Borrow<Node>,
    {
        let (size, hint) = ast.size_hint();
        let size = hint.unwrap_or(size);

        let mut blocks: Vec<CodeBlock> = Vec::with_capacity(size);
        let mut span = SpanBuilder::default();

        for node in ast {
            match node.borrow() {
                Node::Instruction(instruction) => {
                    if let Some(block) =
                        self.compile_instruction(context, &mut span, instruction)?
                    {
                        span.extract_span_into(&mut blocks);
                        blocks.push(block);
                    }
                }

                Node::IfElse(t, f) => {
                    span.extract_span_into(&mut blocks);

                    let t = self.compile_body(t.iter(), context)?;
                    let f = self.compile_body(f.iter(), context)?;
                    let block = CodeBlock::new_split(t, f);

                    blocks.push(block);
                }

                Node::Repeat(n, nodes) => {
                    span.extract_span_into(&mut blocks);

                    let block = self.compile_body(nodes.iter(), context)?;

                    for _ in 0..*n {
                        blocks.push(block.clone());
                    }
                }

                Node::While(nodes) => {
                    span.extract_span_into(&mut blocks);

                    let block = self.compile_body(nodes.iter(), context)?;
                    let block = CodeBlock::new_loop(block);

                    blocks.push(block);
                }
            }
        }

        span.extract_span_into(&mut blocks);

        Ok(parsers::combine_blocks(blocks))
    }
}

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
            proc.baz
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
