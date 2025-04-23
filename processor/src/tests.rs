/// Tests in this file make sure that diagnostics presented to the user are as expected.
use alloc::string::ToString;

use assembly::{assert_diagnostic_lines, regex, source_file, testing::TestContext};
use test_utils::{build_test_by_mode, crypto::init_merkle_leaves};
use vm_core::{
    AdviceMap,
    crypto::merkle::{MerkleStore, MerkleTree},
};

use super::*;

// AdviceMapKeyAlreadyPresent
// ------------------------------------------------------------------------------------------------

/// In this test, we load 2 libraries which have a MAST forest with an advice map that contains
/// different values at the same key (which triggers the `AdviceMapKeyAlreadyPresent` error).
#[test]
fn test_diagnostic_advice_map_key_already_present() {
    let test_context = TestContext::new();

    let (lib_1, lib_2) = {
        let dummy_library_source = source_file!(&test_context, "export.foo add end");
        let module = test_context
            .parse_module_with_path("foo::bar".parse().unwrap(), dummy_library_source)
            .unwrap();
        let lib = test_context.assemble_library(std::iter::once(module)).unwrap();
        let lib_1 = lib
            .clone()
            .with_advice_map(AdviceMap::from_iter([(Digest::default(), vec![ZERO])]));
        let lib_2 = lib.with_advice_map(AdviceMap::from_iter([(Digest::default(), vec![ONE])]));

        (lib_1, lib_2)
    };

    let mut host = DefaultHost::default();
    host.load_mast_forest(lib_1.mast_forest().clone()).unwrap();
    let err = host.load_mast_forest(lib_2.mast_forest().clone()).unwrap_err();

    assert_diagnostic_lines!(
        err,
        "x value for key 0000000000000000000000000000000000000000000000000000000000000000 already present in the advice map when loading MAST forest",
        "help: previous values at key were '[0]'. Operation would have replaced them with '[1]'"
    );
}

// AdviceMapKeyNotFound
// ------------------------------------------------------------------------------------------------

#[test]
fn test_diagnostic_advice_map_key_not_found_1() {
    let source = "
        begin
            swap swap trace.2 adv.push_mapval
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "value for key 00000000000000000000000000000000ffffffff00000000feffffff01000000 not present in the advice map",
        regex!(r#",-\[test[\d]+:3:31\]"#),
        " 2 |         begin",
        " 3 |             swap swap trace.2 adv.push_mapval",
        "   :                               ^^^^^^^^^^^^^^^",
        "4 |         end",
        "   `----"
    );
}

#[test]
fn test_diagnostic_advice_map_key_not_found_2() {
    let source = "
        begin
            swap swap trace.2 adv.push_mapvaln
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "value for key 00000000000000000000000000000000ffffffff00000000feffffff01000000 not present in the advice map",
        regex!(r#",-\[test[\d]+:3:31\]"#),
        " 2 |         begin",
        " 3 |             swap swap trace.2 adv.push_mapvaln",
        "   :                               ^^^^^^^^^^^^^^^^",
        "4 |         end",
        "   `----"
    );
}

// AdviceStackReadFailed
// ------------------------------------------------------------------------------------------------

#[test]
fn test_diagnostic_advice_stack_read_failed() {
    let source = "
        begin
            swap adv_push.1 trace.2
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "advice stack read failed at clock cycle 2",
        regex!(r#",-\[test[\d]+:3:18\]"#),
        " 2 |         begin",
        " 3 |             swap adv_push.1 trace.2",
        "   :                  ^^^^^^^^^^",
        " 4 |         end",
        "   `----"
    );
}

// DivideByZero
// ------------------------------------------------------------------------------------------------

#[test]
fn test_diagnostic_divide_by_zero_1() {
    let source = "
        begin
            trace.2 div
        end";

    let build_test = build_test_by_mode!(true, source, &[]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "division by zero at clock cycle 1",
        regex!(r#",-\[test[\d]+:3:21\]"#),
        " 2 |         begin",
        " 3 |             trace.2 div",
        "   :                     ^^^",
        " 4 |         end",
        "   `----"
    );
}

#[test]
fn test_diagnostic_divide_by_zero_2() {
    let source = "
        begin
            trace.2 u32div
        end";

    let build_test = build_test_by_mode!(true, source, &[]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "division by zero at clock cycle 1",
        regex!(r#",-\[test[\d]+:3:21\]"#),
        " 2 |         begin",
        " 3 |             trace.2 u32div",
        "   :                     ^^^^^^",
        " 4 |         end",
        "   `----"
    );
}

// DynamicNodeNotFound
// ------------------------------------------------------------------------------------------------

#[test]
fn test_diagnostic_dynamic_node_not_found_1() {
    let source = "
        begin
            trace.2 dynexec
        end";

    let build_test = build_test_by_mode!(true, source, &[]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "failed to execute the dynamic code block provided by the stack with root 0x0000000000000000000000000000000000000000000000000000000000000000; the block could not be found",
        regex!(r#",-\[test[\d]+:3:21\]"#),
        " 2 |         begin",
        " 3 |             trace.2 dynexec",
        "   :                     ^^^^^^^",
        " 4 |         end",
        "   `----"
    );
}

#[test]
fn test_diagnostic_dynamic_node_not_found_2() {
    let source = "
        begin
            trace.2 dyncall
        end";

    let build_test = build_test_by_mode!(true, source, &[]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "failed to execute the dynamic code block provided by the stack with root 0x0000000000000000000000000000000000000000000000000000000000000000; the block could not be found",
        regex!(r#",-\[test[\d]+:3:21\]"#),
        " 2 |         begin",
        " 3 |             trace.2 dyncall",
        "   :                     ^^^^^^^",
        " 4 |         end",
        "   `----"
    );
}

// FailedAssertion
// ------------------------------------------------------------------------------------------------

// TODO(plafer): re-enable this and fix after `assert*` lexing is fixed
#[test]
#[ignore]
fn test_diagnostic_failed_assertion() {
    let source = "
        begin
            push.1.2
            assertz
            push.3.4
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "when returning from a call or dyncall, stack depth must be 16, but was 17",
        regex!(r#",-\[test[\d]+:7:21\]"#),
        " 6 |         begin",
        " 7 |             trace.2 call.foo",
        "   :                     ^^^^|^^^",
        "   :                         `-- when returning from this call site",
        " 8 |         end",
        "   `----"
    );
}

// InvalidMerkleTreeNodeIndex
// ------------------------------------------------------------------------------------------------

#[test]
fn test_diagnostic_invalid_merkle_tree_node_index() {
    let source = "
        begin
            mtree_get
        end";

    let depth = 4;
    let index = 16;

    let build_test = build_test_by_mode!(true, source, &[index, depth]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "provided node index 16 is out of bounds for a merkle tree node at depth 4",
        regex!(r#",-\[test[\d]+:3:13\]"#),
        " 2 |         begin",
        " 3 |             mtree_get",
        "   :             ^^^^^^^^^",
        " 4 |         end",
        "   `----"
    );
}

// InvalidStackDepthOnReturn
// ------------------------------------------------------------------------------------------------

/// Ensures that the proper `ExecutionError::InvalidStackDepthOnReturn` diagnostic is generated when
/// the stack depth is invalid on return from a call.
#[test]
fn test_diagnostic_invalid_stack_depth_on_return_call() {
    // returning from a function with non-empty overflow table should result in an error
    // Note: we add the `trace.2` to ensure that asm ops co-exist well with other decorators.
    let source = "
        proc.foo
            push.1
        end

        begin
            trace.2 call.foo
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "when returning from a call or dyncall, stack depth must be 16, but was 17",
        regex!(r#",-\[test[\d]+:7:21\]"#),
        " 6 |         begin",
        " 7 |             trace.2 call.foo",
        "   :                     ^^^^|^^^",
        "   :                         `-- when returning from this call site",
        " 8 |         end",
        "   `----"
    );
}

/// Ensures that the proper `ExecutionError::InvalidStackDepthOnReturn` diagnostic is generated when
/// the stack depth is invalid on return from a dyncall.
#[test]
fn test_diagnostic_invalid_stack_depth_on_return_dyncall() {
    // returning from a function with non-empty overflow table should result in an error
    let source = "
        proc.foo
            push.1
        end

        begin
            procref.foo mem_storew.100 dropw push.100 
            dyncall
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "when returning from a call or dyncall, stack depth must be 16, but was 17",
        regex!(r#",-\[test[\d]+:8:13\]"#),
        " 7 |             procref.foo mem_storew.100 dropw push.100",
        " 8 |             dyncall",
        "   :             ^^^|^^^",
        "   :                `-- when returning from this call site",
        " 9 |         end",
        "   `----"
    );
}

// LogArgumentZero
// ------------------------------------------------------------------------------------------------

#[test]
fn test_diagnostic_log_argument_zero() {
    // returning from a function with non-empty overflow table should result in an error
    let source = "
        begin
            trace.2 ilog2    
        end";

    let build_test = build_test_by_mode!(true, source, &[]);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "attempted to calculate integer logarithm with zero argument at clock cycle 1",
        regex!(r#",-\[test[\d]+:3:21\]"#),
        " 2 |         begin",
        " 3 |             trace.2 ilog2",
        "   :                     ^^^^^",
        " 4 |         end",
        "   `----"
    );
}

// MemoryError
// ------------------------------------------------------------------------------------------------

#[test]
fn test_diagnostic_unaligned_word_access() {
    // mem_storew
    let source = "
        proc.foo add end
        begin
            exec.foo mem_storew.3
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2, 3, 4]);
    let err = build_test.execute().expect_err("expected error");

    assert_diagnostic_lines!(
        err,
        "word memory access at address 3 in context 0 is unaligned at clock cycle 3",
        regex!(r#",-\[test[\d]+:4:22\]"#),
        " 3 |         begin",
        " 4 |             exec.foo mem_storew.3",
        "   :                      ^^^^^^|^^^^^",
        "   :                            `-- tried to access memory address 3",
        " 5 |         end",
        "   `----",
        "  help: ensure that the memory address accessed is aligned to a word boundary (it is a multiple of 4)"
    );

    // mem_loadw
    let source = "
        begin
            mem_loadw.3
        end";

    let build_test = build_test_by_mode!(true, source, &[1, 2, 3, 4]);
    let err = build_test.execute().expect_err("expected error");

    assert_diagnostic_lines!(
        err,
        "word memory access at address 3 in context 0 is unaligned at clock cycle 2",
        regex!(r#",-\[test[\d]+:3:13\]"#),
        " 2 |         begin",
        " 3 |             mem_loadw.3",
        "   :             ^^^^^|^^^^^",
        "   :                  `-- tried to access memory address 3",
        " 4 |         end",
        "   `----",
        "  help: ensure that the memory address accessed is aligned to a word boundary (it is a multiple of 4)"
    );
}

#[test]
fn test_diagnostic_address_out_of_bounds() {
    // mem_store
    let source = "
        begin
            mem_store
        end";

    let build_test = build_test_by_mode!(true, source, &[u32::MAX as u64 + 1_u64]);
    let err = build_test.execute().expect_err("expected error");

    assert_diagnostic_lines!(
        err,
        "memory address cannot exceed 2^32 but was 4294967296",
        regex!(r#",-\[test[\d]+:3:13\]"#),
        " 2 |         begin",
        " 3 |             mem_store",
        "   :             ^^^^^^^^^",
        " 4 |         end",
        "   `----"
    );

    // mem_storew
    let source = "
        begin
            mem_storew
        end";

    let build_test = build_test_by_mode!(true, source, &[u32::MAX as u64 + 1_u64]);
    let err = build_test.execute().expect_err("expected error");

    assert_diagnostic_lines!(
        err,
        "memory address cannot exceed 2^32 but was 4294967296",
        regex!(r#",-\[test[\d]+:3:13\]"#),
        " 2 |         begin",
        " 3 |             mem_storew",
        "   :             ^^^^^^^^^^",
        " 4 |         end",
        "   `----"
    );

    // mem_load
    let source = "
        begin
            swap swap mem_load push.1 drop
        end";

    let build_test = build_test_by_mode!(true, source, &[u32::MAX as u64 + 1_u64]);
    let err = build_test.execute().expect_err("expected error");

    assert_diagnostic_lines!(
        err,
        "memory address cannot exceed 2^32 but was 4294967296",
        regex!(r#",-\[test[\d]+:3:23\]"#),
        " 2 |         begin",
        " 3 |             swap swap mem_load push.1 drop",
        "   :                       ^^^^^^^^",
        " 4 |         end",
        "   `----"
    );

    // mem_loadw
    let source = "
        begin
            swap swap mem_loadw push.1 drop
        end";

    let build_test = build_test_by_mode!(true, source, &[u32::MAX as u64 + 1_u64]);
    let err = build_test.execute().expect_err("expected error");

    assert_diagnostic_lines!(
        err,
        "memory address cannot exceed 2^32 but was 4294967296",
        regex!(r#",-\[test[\d]+:3:23\]"#),
        " 2 |         begin",
        " 3 |             swap swap mem_loadw push.1 drop",
        "   :                       ^^^^^^^^^",
        " 4 |         end",
        "   `----"
    );
}

// MerkleStoreLookupFailed
// -------------------------------------------------------------------------------------------------

#[test]
fn test_diagnostic_merkle_store_lookup_failed() {
    let source = "
        begin
            mtree_set
        end";

    let leaves = &[1, 2, 3, 4];
    let merkle_tree = MerkleTree::new(init_merkle_leaves(leaves)).unwrap();
    let merkle_root = merkle_tree.root();
    let merkle_store = MerkleStore::from(&merkle_tree);
    let advice_stack = Vec::new();

    let stack = {
        let log_depth = 10;
        let index = 0;

        &[
            1,
            merkle_root[0].as_int(),
            merkle_root[1].as_int(),
            merkle_root[2].as_int(),
            merkle_root[3].as_int(),
            index,
            log_depth,
        ]
    };

    let build_test = build_test_by_mode!(true, source, stack, &advice_stack, merkle_store);
    let err = build_test.execute().expect_err("expected error");
    assert_diagnostic_lines!(
        err,
        "failed to lookup value in Merkle store",
        "  `-> node RpoDigest([1, 0, 0, 0]) with index `depth=10, value=0` not found in the store",
        regex!(r#",-\[test[\d]+:3:13\]"#),
        " 2 |         begin",
        " 3 |             mtree_set",
        "   :             ^^^^^^^^^",
        " 4 |         end",
        "   `----"
    );
}
