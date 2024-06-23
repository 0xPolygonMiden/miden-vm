use alloc::{boxed::Box, vec::Vec};
use vm_core::{
    mast::{MastForest, MastNode},
    Program,
};

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

    let assembler = Assembler::with_kernel_from_module(KERNEL)
        .unwrap()
        .with_library(&DummyLibrary::default())
        .unwrap();

    // The expected `MastForest` for the program (that we will build by hand)
    let mut expected_mast_forest = MastForest::new();

    // fetch the kernel digest and store into a syscall block
    //
    // Note: this assumes the current internal implementation detail that `assembler.mast_forest`
    // contains the MAST nodes for the kernel after a call to
    // `Assembler::with_kernel_from_module()`.
    let syscall_foo_node_id = {
        let kernel_foo_node = MastNode::new_basic_block(vec![Operation::Add]);
        let kernel_foo_node_id = expected_mast_forest.add_node(kernel_foo_node);

        let syscall_node = MastNode::new_syscall(kernel_foo_node_id, &expected_mast_forest);
        expected_mast_forest.add_node(syscall_node)
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

    let program = assembler.assemble(program).unwrap();

    let exec_bar_node_id = {
        // bar procedure
        let basic_block_1 = MastNode::new_basic_block(vec![Operation::Push(17_u32.into())]);
        let basic_block_1_id = expected_mast_forest.add_node(basic_block_1);

        // Basic block representing the `foo` procedure
        let basic_block_2 = MastNode::new_basic_block(vec![Operation::Push(19_u32.into())]);
        let basic_block_2_id = expected_mast_forest.add_node(basic_block_2);

        let join_node =
            MastNode::new_join(basic_block_1_id, basic_block_2_id, &expected_mast_forest);
        expected_mast_forest.add_node(join_node)
    };

    let exec_foo_bar_baz_node_id = {
        // basic block representing foo::bar.baz procedure
        let basic_block = MastNode::new_basic_block(vec![Operation::Push(29_u32.into())]);
        expected_mast_forest.add_node(basic_block)
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

    let expected_program = Program::new(expected_mast_forest.into(), combined_node_id);
    assert_eq!(expected_program.hash(), program.hash());

    // also check that the program has the right number of procedures
    assert_eq!(program.num_procedures(), 5);
}
