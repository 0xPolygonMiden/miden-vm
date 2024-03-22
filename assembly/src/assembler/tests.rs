use alloc::{boxed::Box, vec::Vec};

use super::{combine_blocks, Assembler, CodeBlock, Library, Operation};
use crate::{
    ast::{Module, ModuleKind},
    LibraryNamespace, Version,
};

// TESTS
// ================================================================================================

#[test]
fn nested_blocks() {
    const MODULE: &str = "foo::bar";
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
        #[allow(clippy::vec_box)]
        modules: Vec<Box<Module>>,
        dependencies: Vec<LibraryNamespace>,
    }

    impl Default for DummyLibrary {
        fn default() -> Self {
            let ast =
                Module::parse_str(MODULE.parse().unwrap(), ModuleKind::Library, PROCEDURE).unwrap();
            let namespace = ast.namespace().clone();
            Self {
                namespace,
                modules: vec![ast],
                dependencies: Vec::new(),
            }
        }
    }

    impl Library for DummyLibrary {
        fn root_ns(&self) -> &LibraryNamespace {
            &self.namespace
        }

        fn version(&self) -> &Version {
            const MIN: Version = Version::min();
            &MIN
        }

        fn modules(&self) -> impl ExactSizeIterator<Item = &Module> + '_ {
            self.modules.iter().map(|m| m.as_ref())
        }

        fn dependencies(&self) -> &[LibraryNamespace] {
            &self.dependencies
        }
    }

    let mut assembler = Assembler::new()
        .with_kernel_from_module(KERNEL)
        .unwrap()
        .with_library(&DummyLibrary::default())
        .unwrap();

    // the assembler should have a single kernel proc in its cache before the compilation of the
    // source
    assert_eq!(assembler.procedure_cache().len(), 1);

    // fetch the kernel digest and store into a syscall block
    let syscall = assembler
        .procedure_cache()
        .entries()
        .next()
        .map(|p| CodeBlock::new_syscall(p.mast_root()))
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

    let program = assembler.assemble(program).unwrap();

    let exec_bar = assembler
        .procedure_cache()
        .get_by_name(&"#exec::bar".parse().unwrap())
        .map(|p| CodeBlock::new_proxy(p.code().hash()))
        .unwrap();

    let exec_foo_bar_baz = assembler
        .procedure_cache()
        .get_by_name(&"foo::bar::baz".parse().unwrap())
        .map(|p| CodeBlock::new_proxy(p.code().hash()))
        .unwrap();

    let before = CodeBlock::new_span(vec![Operation::Push(2u32.into())]);

    let r#true = CodeBlock::new_span(vec![Operation::Push(3u32.into())]);
    let r#false = CodeBlock::new_span(vec![Operation::Push(5u32.into())]);
    let r#if = CodeBlock::new_split(r#true, r#false);

    let r#true = CodeBlock::new_span(vec![Operation::Push(7u32.into())]);
    let r#false = CodeBlock::new_span(vec![Operation::Push(11u32.into())]);
    let r#true = CodeBlock::new_split(r#true, r#false);

    let r#while =
        CodeBlock::new_join([exec_bar, CodeBlock::new_span(vec![Operation::Push(23u32.into())])]);
    let r#while = CodeBlock::new_loop(r#while);
    let span = CodeBlock::new_span(vec![Operation::Push(13u32.into())]);
    let r#false = CodeBlock::new_join([span, r#while]);
    let nested = CodeBlock::new_split(r#true, r#false);

    //let exec = CodeBlock::new_span(vec![Operation::Push(29u32.into())]);

    let combined = combine_blocks(vec![before, r#if, nested, exec_foo_bar_baz, syscall]);

    assert_eq!(combined.hash(), program.hash());
}
