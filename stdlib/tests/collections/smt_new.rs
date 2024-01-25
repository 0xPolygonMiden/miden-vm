use super::*;
use miden_core::crypto::{hash::RpoDigest, merkle::Smt};

// Case
// + set an empty leaf
// + set a single leaf (same key)

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

        let store = MerkleStore::from(smt);
        build_test!(source, &initial_stack, &[], store, vec![]).expect_stack(&expected_output);
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

// HELPER FUNCTIONS
// ================================================================================================

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
