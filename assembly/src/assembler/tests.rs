use super::{combine_blocks, Assembler, CodeBlock, Library, Module, Operation};
use crate::{parse_module, LibraryNamespace, ModulePath, Version};
use core::slice::Iter;

// TESTS
// ================================================================================================

#[test]
fn nested_blocks() {
    const NAMESPACE: &str = "foo";
    const MODULE: &str = "bar";
    const KERNEL: &str = r#"
        export.foo
            add
        end"#;
    const PROCEDURE: &str = r#"
        export.baz
            push.29
        end"#;

    pub struct DummyLibrary {
        namespace: LibraryNamespace,
        modules: Vec<Module>,
    }

    impl Default for DummyLibrary {
        fn default() -> Self {
            let namespace = LibraryNamespace::try_from(NAMESPACE.to_string()).unwrap();
            let path = ModulePath::try_from(MODULE.to_string()).unwrap().to_absolute(&namespace);
            let ast = parse_module(PROCEDURE).unwrap();
            Self {
                namespace,
                modules: vec![Module { path, ast }],
            }
        }
    }

    impl Library for DummyLibrary {
        type ModuleIterator<'a> = Iter<'a, Module>;

        fn root_ns(&self) -> &LibraryNamespace {
            &self.namespace
        }

        fn version(&self) -> &Version {
            &Version::MIN
        }

        fn modules(&self) -> Self::ModuleIterator<'_> {
            self.modules.iter()
        }
    }

    let assembler = Assembler::default()
        .with_kernel(KERNEL)
        .unwrap()
        .with_library(&DummyLibrary::default())
        .unwrap();

    // the assembler should have a single kernel proc in its cache before the compilation of the
    // source
    assert_eq!(assembler.proc_cache.borrow().len(), 1);

    // fetch the kernel digest and store into a syscall block
    let syscall = assembler
        .proc_cache
        .borrow()
        .values()
        .next()
        .map(|p| CodeBlock::new_syscall(p.code_root().hash()))
        .unwrap();

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
    let program = assembler.compile(program).unwrap();

    assert_eq!(combined.hash(), program.hash());
}
