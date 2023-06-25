use super::{Felt, MerkleStore, SimpleSmt, StarkField, TestError, Word, ONE, ZERO};
use crate::build_test;

// TEST DATA
// ================================================================================================

const LEAVES: [(u64, Word); 5] = [
    (
        0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64,
        [Felt::new(1), ZERO, ZERO, ZERO],
    ),
    (
        // different from the first key starting from the first bit
        0b10000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64,
        [Felt::new(2), ZERO, ZERO, ZERO],
    ),
    (
        // same 16-bit prefix as the first key
        0b00000000_00000000_01111111_11111111_11111111_11111111_11111111_11111111_u64,
        [Felt::new(2), ZERO, ZERO, ZERO],
    ),
    (
        // same 32-bit prefix as the first key
        0b00000000_00000000_11111111_11111111_01111111_11111111_11111111_11111111_u64,
        [Felt::new(3), ZERO, ZERO, ZERO],
    ),
    (
        // same 48-bit prefix as the first key
        0b00000000_00000000_11111111_11111111_11111111_11111111_01111111_11111111_u64,
        [Felt::new(4), ZERO, ZERO, ZERO],
    ),
];

// TESTS
// ================================================================================================

#[test]
fn get() {
    let smt = SimpleSmt::with_leaves(64, LEAVES).unwrap();

    let source = "
    use.std::collections::smt64
    begin
      exec.smt64::get
    end
    ";

    for (index, value) in LEAVES {
        let mut initial_stack = Vec::new();
        append_word_to_vec(&mut initial_stack, smt.root().into());
        initial_stack.push(index);
        let expected_output = build_expected_stack(value, smt.root().into());

        let store = MerkleStore::from(&smt);
        build_test!(source, &initial_stack, &[], store, vec![]).expect_stack(&expected_output);
    }
}

#[test]
fn insert() {
    let mut smt = SimpleSmt::new(64).unwrap();

    let source = "
    use.std::collections::smt64
    begin
      exec.smt64::insert
    end
    ";

    // insert values one-by-one into the tree
    for (index, value) in LEAVES {
        let (init_stack, final_stack, store) = prepare_insert_or_set(index, value, &mut smt);
        build_test!(source, &init_stack, &[], store, vec![]).expect_stack(&final_stack);
    }

    // update one of the previously inserted values
    let index = LEAVES[0].0;
    let value = [ONE; 4];
    let (init_stack, final_stack, store) = prepare_insert_or_set(index, value, &mut smt);
    build_test!(source, &init_stack, &[], store, vec![]).expect_stack(&final_stack);

    // try to insert an invalid value
    let value = [ZERO; 4];
    let (init_stack, _, store) = prepare_insert_or_set(index, value, &mut smt);
    build_test!(source, &init_stack, &[], store, vec![])
        .expect_error(TestError::ExecutionError("FailedAssertion"));
}

#[test]
fn set() {
    let mut smt = SimpleSmt::new(64).unwrap();
    let empty_tree_root = smt.root();

    let source = "
    use.std::collections::smt64
    begin
      exec.smt64::set
    end
    ";

    // insert values one-by-one into the tree
    let mut old_roots = Vec::new();
    for (index, value) in LEAVES {
        old_roots.push(smt.root());
        let (init_stack, final_stack, store) = prepare_insert_or_set(index, value, &mut smt);
        build_test!(source, &init_stack, &[], store, vec![]).expect_stack(&final_stack);
    }

    // update one of the previously inserted values
    let mut smt2 = smt.clone();
    let index = LEAVES[0].0;
    let value = [ONE; 4];
    let (init_stack, final_stack, store) = prepare_insert_or_set(index, value, &mut smt2);
    build_test!(source, &init_stack, &[], store, vec![]).expect_stack(&final_stack);

    // setting to [ZERO; 4] should return the tree to the prior state
    for (index, old_value) in LEAVES.iter().rev() {
        let value = [ZERO; 4];
        let (init_stack, final_stack, store) = prepare_insert_or_set(*index, value, &mut smt);

        let expected_final_stack =
            build_expected_stack(*old_value, old_roots.pop().unwrap().into());
        assert_eq!(expected_final_stack, final_stack);
        build_test!(source, &init_stack, &[], store, vec![]).expect_stack(&final_stack);
    }

    assert_eq!(smt.root(), empty_tree_root);
}

// HELPER FUNCTIONS
// ================================================================================================

fn prepare_insert_or_set(
    index: u64,
    value: Word,
    smt: &mut SimpleSmt,
) -> (Vec<u64>, Vec<u64>, MerkleStore) {
    // set initial state of the stack to be [VALUE, key, ROOT, ...]
    let mut initial_stack = Vec::new();
    append_word_to_vec(&mut initial_stack, smt.root().into());
    initial_stack.push(index);
    append_word_to_vec(&mut initial_stack, value);

    // build a Merkle store for the test before the tree is updated, and then update the tree
    let store: MerkleStore = (&*smt).into();
    let old_value = smt.update_leaf(index, value).unwrap();

    // after insert or set, the stack should be [OLD_VALUE, ROOT, ...]
    let expected_output = build_expected_stack(old_value, smt.root().into());

    (initial_stack, expected_output, store)
}

fn build_expected_stack(word0: Word, word1: Word) -> Vec<u64> {
    vec![
        word0[3].as_int(),
        word0[2].as_int(),
        word0[1].as_int(),
        word0[0].as_int(),
        word1[3].as_int(),
        word1[2].as_int(),
        word1[1].as_int(),
        word1[0].as_int(),
    ]
}

fn append_word_to_vec(target: &mut Vec<u64>, word: Word) {
    target.push(word[0].as_int());
    target.push(word[1].as_int());
    target.push(word[2].as_int());
    target.push(word[3].as_int());
}
