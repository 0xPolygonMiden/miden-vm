use super::*;
use miden_core::crypto::{hash::RpoDigest, merkle::Smt};

// TEST DATA
// ================================================================================================

const LEAVES: [(RpoDigest, Word); 2] = [
    (RpoDigest::new([ZERO, ZERO, ZERO, ZERO]), [ONE, ZERO, ZERO, ZERO]),
    // Most significant Felt differs from previous
    (RpoDigest::new([ZERO, ZERO, ZERO, ONE]), [ONE, ONE, ZERO, ZERO]),
];

/// Tests `get` on every key present in the SMT, as well as an empty leaf
#[test]
fn test_smt_get() {
    fn expect_value_from_get(key: RpoDigest, value: Word, smt: &Smt) {
        let source = "
            use.std::collections::smt_new
            begin
            exec.smt_new::get
            end
        ";
        let mut initial_stack = Vec::new();
        append_word_to_vec(&mut initial_stack, smt.root().into());
        append_word_to_vec(&mut initial_stack, key.into());
        let expected_output = build_expected_stack(value, smt.root().into());

        let (store, advice_map) = build_advice_inputs(smt);
        build_test!(source, &initial_stack, &[], store, advice_map).expect_stack(&expected_output);
    }

    let smt = Smt::with_entries(LEAVES).unwrap();

    // Get all leaves present in tree
    for (key, value) in LEAVES {
        expect_value_from_get(key, value, &smt);
    }

    // Get an empty leaf
    expect_value_from_get(
        RpoDigest::new([42_u64.into(), 42_u64.into(), 42_u64.into(), 42_u64.into()]),
        EMPTY_WORD,
        &smt,
    );
}

/// Tests inserting and removing key-value pairs to an SMT. Also tests updating an existing key with
/// a different value.
#[test]
fn test_smt_set() {
    let mut smt = Smt::new();
    let empty_tree_root = smt.root();

    let source = "
    use.std::collections::smt_new
    begin
      exec.smt_new::set
    end
    ";

    // insert values one-by-one into the tree
    let mut old_roots = Vec::new();
    for (key, value) in LEAVES {
        old_roots.push(smt.root());
        let (init_stack, final_stack, store, advice_map) =
            prepare_insert_or_set(key, value, &mut smt);
        build_test!(source, &init_stack, &[], store, advice_map).expect_stack(&final_stack);
    }

    // update one of the previously inserted values (on a cloned tree)
    let mut smt2 = smt.clone();
    let key = LEAVES[0].0;
    let value = [42323_u64.into(); 4];
    let (init_stack, final_stack, store, advice_map) = prepare_insert_or_set(key, value, &mut smt2);
    build_test!(source, &init_stack, &[], store, advice_map).expect_stack(&final_stack);

    // setting to [ZERO; 4] should return the tree to the prior state
    for (key, old_value) in LEAVES.iter().rev() {
        let value = EMPTY_WORD;
        let (init_stack, final_stack, store, advice_map) =
            prepare_insert_or_set(*key, value, &mut smt);

        let expected_final_stack =
            build_expected_stack(*old_value, old_roots.pop().unwrap().into());
        assert_eq!(expected_final_stack, final_stack);
        build_test!(source, &init_stack, &[], store, advice_map).expect_stack(&final_stack);
    }

    assert_eq!(smt.root(), empty_tree_root);
}

// HELPER FUNCTIONS
// ================================================================================================

fn prepare_insert_or_set(
    key: RpoDigest,
    value: Word,
    smt: &mut Smt,
) -> (Vec<u64>, Vec<u64>, MerkleStore, Vec<([u8; 32], Vec<Felt>)>) {
    // set initial state of the stack to be [VALUE, KEY, ROOT, ...]
    let mut initial_stack = Vec::new();
    append_word_to_vec(&mut initial_stack, smt.root().into());
    append_word_to_vec(&mut initial_stack, key.into());
    append_word_to_vec(&mut initial_stack, value);

    // build a Merkle store for the test before the tree is updated, and then update the tree
    let (store, advice_map) = build_advice_inputs(smt);
    let old_value = smt.insert(key, value);

    // after insert or set, the stack should be [OLD_VALUE, ROOT, ...]
    let expected_output = build_expected_stack(old_value, smt.root().into());

    (initial_stack, expected_output, store, advice_map)
}

fn build_advice_inputs(smt: &Smt) -> (MerkleStore, Vec<([u8; 32], Vec<Felt>)>) {
    let store = MerkleStore::from(smt);
    let advice_map = smt
        .leaves()
        .map(|(_, leaf)| {
            let leaf_hash = leaf.hash();
            (leaf_hash.as_bytes(), leaf.to_elements())
        })
        .collect::<Vec<_>>();

    (store, advice_map)
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
