use super::{
    combine_blocks, parsers, Assembler, CodeBlock, ModuleProvider, Operation, ProcedureId,
};

// TESTS
// ================================================================================================

#[test]
fn nested_blocks() {
    use crate::{ModuleAst, NamedModuleAst};

    let kernel = r#"
        export.foo
            add
        end"#;

    let assembler = Assembler::new().with_kernel(&kernel).unwrap();

    // the assembler should have a single kernel proc in its cache
    assert_eq!(assembler.proc_cache.borrow().len(), 1);

    // fetch the kernel digest and store into a syscall block
    let syscall = assembler
        .proc_cache
        .borrow()
        .values()
        .next()
        .map(|p| CodeBlock::new_syscall(p.code_root().hash()))
        .unwrap();

    struct DummyModuleProvider {
        module: ModuleAst,
    }

    impl ModuleProvider for DummyModuleProvider {
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
        syscall.foo
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

    let combined = combine_blocks(vec![before, r#if, nested, exec, syscall]);
    let program = assembler
        .with_module_provider(module_provider)
        .compile(program)
        .unwrap();

    assert_eq!(combined.hash(), program.hash());
}
