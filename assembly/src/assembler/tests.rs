use alloc::{boxed::Box, vec::Vec};
use vm_core::mast::{MastForest, MastNode, MerkleTreeNode};

use super::{Assembler, Library, Operation};
use crate::{
    assembler::combine_mast_node_ids,
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

    let mut assembler = Assembler::with_kernel_from_module(KERNEL)
        .unwrap()
        .with_library(&DummyLibrary::default())
        .unwrap();

    // The expected `MastForest` for the program (that we will build by hand)
    let mut expected_mast_forest = MastForest::new();

    // the assembler should have a single kernel proc in its cache before the compilation of the
    // source
    assert_eq!(assembler.procedure_cache().len(), 1);

    // fetch the kernel digest and store into a syscall block
    //
    // Note: this assumes the current internal implementation detail that `assembler.mast_forest`
    // contains the MAST nodes for the kernel after a call to
    // `Assembler::with_kernel_from_module()`.
    let syscall_foo_node_id = {
        let syscall_foo_node = assembler
            .procedure_cache()
            .entries()
            .next()
            .map(|p| MastNode::new_syscall(p.code(), &assembler.mast_forest))
            .unwrap();

        expected_mast_forest.add_node(syscall_foo_node)
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

    let program = assembler.assemble_test(program).unwrap();

    let exec_bar_node_id = {
        let exec_bar_node = assembler
            .procedure_cache()
            .get_by_name(&"#exec::bar".parse().unwrap())
            .map(|p| {
                let proc_node = program.get_node_by_id(p.code());
                MastNode::new_external(proc_node.digest())
            })
            .unwrap();

        expected_mast_forest.add_node(exec_bar_node)
    };

    let exec_foo_bar_baz_node_id = {
        let exec_foo_bar_baz_node = assembler
            .procedure_cache()
            .get_by_name(&"foo::bar::baz".parse().unwrap())
            .map(|p| {
                let proc_node = program.get_node_by_id(p.code());
                MastNode::new_external(proc_node.digest())
            })
            .unwrap();

        expected_mast_forest.add_node(exec_foo_bar_baz_node)
    };

    let before = {
        let before_node = MastNode::new_basic_block(vec![Operation::Push(2u32.into())]);
        expected_mast_forest.add_node(before_node)
    };

    let r#true1 = {
        let r#true_node = MastNode::new_basic_block(vec![Operation::Push(3u32.into())]);
        expected_mast_forest.add_node(r#true_node)
    };
    let r#false1 = {
        let r#false_node = MastNode::new_basic_block(vec![Operation::Push(5u32.into())]);
        expected_mast_forest.add_node(r#false_node)
    };
    let r#if1 = {
        let r#if_node = MastNode::new_split(r#true1, r#false1, &expected_mast_forest);
        expected_mast_forest.add_node(r#if_node)
    };

    let r#true3 = {
        let r#true_node = MastNode::new_basic_block(vec![Operation::Push(7u32.into())]);
        expected_mast_forest.add_node(r#true_node)
    };
    let r#false3 = {
        let r#false_node = MastNode::new_basic_block(vec![Operation::Push(11u32.into())]);
        expected_mast_forest.add_node(r#false_node)
    };
    let r#true2 = {
        let r#if_node = MastNode::new_split(r#true3, r#false3, &expected_mast_forest);
        expected_mast_forest.add_node(r#if_node)
    };

    let r#while = {
        let push_basic_block_id = {
            let push_basic_block = MastNode::new_basic_block(vec![Operation::Push(23u32.into())]);
            expected_mast_forest.add_node(push_basic_block)
        };
        let body_node_id = {
            let body_node =
                MastNode::new_join(exec_bar_node_id, push_basic_block_id, &expected_mast_forest);

            expected_mast_forest.add_node(body_node)
        };

        let loop_node = MastNode::new_loop(body_node_id, &expected_mast_forest);
        expected_mast_forest.add_node(loop_node)
    };
    let push_13_basic_block_id = {
        let node = MastNode::new_basic_block(vec![Operation::Push(13u32.into())]);
        expected_mast_forest.add_node(node)
    };

    let r#false2 = {
        let node = MastNode::new_join(push_13_basic_block_id, r#while, &expected_mast_forest);
        expected_mast_forest.add_node(node)
    };
    let nested = {
        let node = MastNode::new_split(r#true2, r#false2, &expected_mast_forest);
        expected_mast_forest.add_node(node)
    };

    let combined_node_id = combine_mast_node_ids(
        vec![before, r#if1, nested, exec_foo_bar_baz_node_id, syscall_foo_node_id],
        &mut expected_mast_forest,
    );

    let combined_node = expected_mast_forest.get_node_by_id(combined_node_id);

    assert_eq!(combined_node.digest(), program.entrypoint_digest().unwrap());
}
