use alloc::{boxed::Box, vec::Vec};
use pretty_assertions::assert_eq;
use vm_core::{
    assert_matches,
    mast::{MastForest, MastNode},
    Program,
};

use super::{Assembler, Library, Operation};
use crate::{
    assembler::{combine_mast_node_ids, mast_forest_builder::MastForestBuilder},
    ast::{Module, ModuleKind},
    LibraryNamespace, Version,
};

// TESTS
// ================================================================================================

// TODO: Fix test after we implement the new `Assembler::add_library()`
#[ignore]
#[allow(unused)]
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

    let assembler = Assembler::new().with_library(&DummyLibrary::default()).unwrap();

    // The expected `MastForest` for the program (that we will build by hand)
    let mut expected_mast_forest_builder = MastForestBuilder::default();

    // fetch the kernel digest and store into a syscall block
    //
    // Note: this assumes the current internal implementation detail that `assembler.mast_forest`
    // contains the MAST nodes for the kernel after a call to
    // `Assembler::with_kernel_from_module()`.
    let syscall_foo_node_id = {
        let kernel_foo_node_id =
            expected_mast_forest_builder.ensure_block(vec![Operation::Add], None).unwrap();

        expected_mast_forest_builder.ensure_syscall(kernel_foo_node_id).unwrap()
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

    let program = assembler.assemble_program(program).unwrap();

    let exec_bar_node_id = {
        // bar procedure
        let basic_block_1_id = expected_mast_forest_builder
            .ensure_block(vec![Operation::Push(17_u32.into())], None)
            .unwrap();

        // Basic block representing the `foo` procedure
        let basic_block_2_id = expected_mast_forest_builder
            .ensure_block(vec![Operation::Push(19_u32.into())], None)
            .unwrap();

        expected_mast_forest_builder
            .ensure_join(basic_block_1_id, basic_block_2_id)
            .unwrap()
    };

    // basic block representing foo::bar.baz procedure
    let exec_foo_bar_baz_node_id = expected_mast_forest_builder
        .ensure_block(vec![Operation::Push(29_u32.into())], None)
        .unwrap();

    let before = expected_mast_forest_builder
        .ensure_block(vec![Operation::Push(2u32.into())], None)
        .unwrap();

    let r#true1 = expected_mast_forest_builder
        .ensure_block(vec![Operation::Push(3u32.into())], None)
        .unwrap();
    let r#false1 = expected_mast_forest_builder
        .ensure_block(vec![Operation::Push(5u32.into())], None)
        .unwrap();
    let r#if1 = expected_mast_forest_builder.ensure_split(r#true1, r#false1).unwrap();

    let r#true3 = expected_mast_forest_builder
        .ensure_block(vec![Operation::Push(7u32.into())], None)
        .unwrap();
    let r#false3 = expected_mast_forest_builder
        .ensure_block(vec![Operation::Push(11u32.into())], None)
        .unwrap();
    let r#true2 = expected_mast_forest_builder.ensure_split(r#true3, r#false3).unwrap();

    let r#while = {
        let push_basic_block_id = {
            expected_mast_forest_builder
                .ensure_block(vec![Operation::Push(23u32.into())], None)
                .unwrap()
        };
        let body_node_id = expected_mast_forest_builder
            .ensure_join(exec_bar_node_id, push_basic_block_id)
            .unwrap();

        expected_mast_forest_builder.ensure_loop(body_node_id).unwrap()
    };
    let push_13_basic_block_id = expected_mast_forest_builder
        .ensure_block(vec![Operation::Push(13u32.into())], None)
        .unwrap();

    let r#false2 = expected_mast_forest_builder
        .ensure_join(push_13_basic_block_id, r#while)
        .unwrap();
    let nested = expected_mast_forest_builder.ensure_split(r#true2, r#false2).unwrap();

    let combined_node_id = combine_mast_node_ids(
        vec![before, r#if1, nested, exec_foo_bar_baz_node_id, syscall_foo_node_id],
        &mut expected_mast_forest_builder,
    )
    .unwrap();

    let expected_program = Program::new(expected_mast_forest_builder.build(), combined_node_id);
    assert_eq!(expected_program.hash(), program.hash());

    // also check that the program has the right number of procedures
    assert_eq!(program.num_procedures(), 5);
}

/// Ensures that a single copy of procedures with the same MAST root are added only once to the MAST
/// forest.
#[test]
fn duplicate_procedure() {
    let assembler = Assembler::new();

    let program_source = r#"
        proc.foo
            add
            mul
        end

        proc.bar
            add
            mul
        end

        begin
            # specific impl irrelevant
            exec.foo
            exec.bar
        end
    "#;

    let program = assembler.assemble_program(program_source).unwrap();
    assert_eq!(program.num_procedures(), 2);
}

/// Ensures that equal MAST nodes don't get added twice to a MAST forest
#[test]
fn duplicate_nodes() {
    let assembler = Assembler::new();

    let program_source = r#"
    begin
        if.true
            mul
        else
            if.true add else mul end
        end
    end
    "#;

    let program = assembler.assemble_program(program_source).unwrap();

    let mut expected_mast_forest = MastForest::new();

    // basic block: mul
    let mul_basic_block_id = {
        let node = MastNode::new_basic_block(vec![Operation::Mul]);
        expected_mast_forest.add_node(node).unwrap()
    };

    // basic block: add
    let add_basic_block_id = {
        let node = MastNode::new_basic_block(vec![Operation::Add]);
        expected_mast_forest.add_node(node).unwrap()
    };

    // inner split: `if.true add else mul end`
    let inner_split_id = {
        let node =
            MastNode::new_split(add_basic_block_id, mul_basic_block_id, &expected_mast_forest);
        expected_mast_forest.add_node(node).unwrap()
    };

    // root: outer split
    let root_id = {
        let node = MastNode::new_split(mul_basic_block_id, inner_split_id, &expected_mast_forest);
        expected_mast_forest.add_node(node).unwrap()
    };
    expected_mast_forest.make_root(root_id);

    let expected_program = Program::new(expected_mast_forest, root_id);

    assert_eq!(program, expected_program);
}

#[test]
fn explicit_fully_qualified_procedure_references() {
    const BAR_NAME: &str = "foo::bar";
    const BAR: &str = r#"
        export.bar
            add
        end"#;
    const BAZ_NAME: &str = "foo::baz";
    const BAZ: &str = r#"
        export.baz
            exec.::foo::bar::bar
        end"#;

    pub struct DummyLibrary {
        namespace: LibraryNamespace,
        #[allow(clippy::vec_box)]
        modules: Vec<Box<Module>>,
        dependencies: Vec<LibraryNamespace>,
    }

    impl Default for DummyLibrary {
        fn default() -> Self {
            let bar =
                Module::parse_str(BAR_NAME.parse().unwrap(), ModuleKind::Library, BAR).unwrap();
            let baz =
                Module::parse_str(BAZ_NAME.parse().unwrap(), ModuleKind::Library, BAZ).unwrap();
            let namespace = LibraryNamespace::new("foo").unwrap();
            Self {
                namespace,
                modules: vec![bar, baz],
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

    let assembler = Assembler::new().with_library(&DummyLibrary::default()).unwrap();

    let program = r#"
    begin
        exec.::foo::baz::baz
    end"#;

    assert_matches!(assembler.assemble_program(program), Ok(_));
}
