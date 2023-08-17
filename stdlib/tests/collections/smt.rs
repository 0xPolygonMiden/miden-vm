use crate::build_test;
use test_utils::{
    crypto::{MerkleStore, Rpo256, RpoDigest, TieredSmt},
    stack_to_ints, stack_top_to_ints, Felt, StarkField, Word, ONE, ZERO,
};

type AdvMapEntry = ([u8; 32], Vec<Felt>);

// CONSTANTS
// ================================================================================================

const EMPTY_VALUE: Word = TieredSmt::EMPTY_VALUE;

// RETRIEVAL TESTS
// ================================================================================================

#[test]
fn tsmt_get_16() {
    let mut smt = TieredSmt::default();

    // create a key
    let raw_a = 0b_01010101_01101100_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);

    // make sure we get an empty value for this key
    assert_get(&smt, key_a, EMPTY_VALUE);

    // insert a value under this key and make sure we get it back when queried
    let val_a = [ONE, ONE, ONE, ONE];
    smt.insert(key_a, val_a);
    assert_get(&smt, key_a, val_a);

    // make sure that another key still returns empty value
    let raw_b = 0b_01111101_01101100_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    assert_get(&smt, key_b, EMPTY_VALUE);

    // make sure that another key with the same 16-bit prefix returns an empty value
    let raw_c = 0b_01010101_01101100_11111111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    assert_get(&smt, key_c, EMPTY_VALUE);
}

#[test]
fn tsmt_get_32() {
    let mut smt = TieredSmt::default();

    // populate the tree with two key-value pairs sharing the same 16-bit prefix for the keys
    let raw_a = 0b_01010101_01010101_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a = [ONE, ONE, ONE, ONE];
    smt.insert(key_a, val_a);

    let raw_b = 0b_01010101_01010101_11100000_11111111_10010110_10010011_11100000_00000000_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    let val_b = [ZERO, ONE, ONE, ONE];
    smt.insert(key_b, val_b);

    // make sure the values for these keys are retrieved correctly
    assert_get(&smt, key_a, val_a);
    assert_get(&smt, key_b, val_b);

    // make sure another key with the same 16-bit prefix returns an empty value
    let raw_c = 0b_01010101_01010101_11100111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    assert_get(&smt, key_c, EMPTY_VALUE);

    // make sure keys with the same 32-bit prefixes return empty value
    let raw_d = 0b_01010101_01010101_00011111_11111111_11111110_10010011_11100000_00000000_u64;
    let key_d = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_d)]);
    assert_get(&smt, key_d, EMPTY_VALUE);

    // make sure keys with the same 32-bit prefixes return empty value
    let raw_e = 0b_01010101_01010101_11100000_11111111_10011111_10010011_11100000_00000000_u64;
    let key_e = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_e)]);
    assert_get(&smt, key_e, EMPTY_VALUE);
}

#[test]
fn tsmt_get_48() {
    let mut smt = TieredSmt::default();

    // populate the tree with two key-value pairs sharing the same 32-bit prefix for the keys
    let raw_a = 0b_01010101_01010101_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a = [ONE, ONE, ONE, ONE];
    smt.insert(key_a, val_a);

    let raw_b = 0b_01010101_01010101_00011111_11111111_11111111_10010011_11100000_00000000_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    let val_b = [ZERO, ONE, ONE, ONE];
    smt.insert(key_b, val_b);

    // make sure the values for these keys are retrieved correctly
    assert_get(&smt, key_a, val_a);
    assert_get(&smt, key_b, val_b);

    // make sure another key with the same 32-bit prefix returns an empty value
    let raw_c = 0b_01010101_01010101_00011111_11111111_00000000_10010011_11100000_00000000_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    assert_get(&smt, key_c, EMPTY_VALUE);

    // make sure keys with the same 48-bit prefixes return empty value
    let raw_d = 0b_01010101_01010101_00011111_11111111_10010110_10010011_00000111_00000000_u64;
    let key_d = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_d)]);
    assert_get(&smt, key_d, EMPTY_VALUE);

    // make sure keys with the same 48-bit prefixes return empty value
    let raw_e = 0b_01010101_01010101_00011111_11111111_11111111_10010011_000001011_00000000_u64;
    let key_e = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_e)]);
    assert_get(&smt, key_e, EMPTY_VALUE);
}

/// Asserts key/value opens to root for the provided Tiered Sparse Merkle tree.
fn assert_get(smt: &TieredSmt, key: RpoDigest, value: Word) {
    let root = smt.root();
    let source = r#"
        use.std::collections::smt

        begin
            exec.smt::get
        end
    "#;
    let initial_stack = [
        root[0].as_int(),
        root[1].as_int(),
        root[2].as_int(),
        root[3].as_int(),
        key[0].as_int(),
        key[1].as_int(),
        key[2].as_int(),
        key[3].as_int(),
    ];
    let expected_output = [
        value[3].as_int(),
        value[2].as_int(),
        value[1].as_int(),
        value[0].as_int(),
        root[3].as_int(),
        root[2].as_int(),
        root[1].as_int(),
        root[0].as_int(),
    ];

    let (store, advice_map) = build_advice_inputs(smt);
    let advice_stack = [];
    build_test!(source, &initial_stack, &advice_stack, store, advice_map.into_iter())
        .expect_stack(&expected_output);
}

fn build_advice_inputs(smt: &TieredSmt) -> (MerkleStore, Vec<([u8; 32], Vec<Felt>)>) {
    let store = MerkleStore::from(smt);
    let advice_map = smt
        .upper_leaves()
        .map(|(node, key, value)| {
            let mut elements = key.as_elements().to_vec();
            elements.extend(&value);
            (node.as_bytes(), elements)
        })
        .collect::<Vec<_>>();

    (store, advice_map)
}

// INSERTION TESTS
// ================================================================================================

#[test]
fn tsmt_insert_16() {
    let mut smt = TieredSmt::default();

    let raw_a = 0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a1 = [ONE, ZERO, ZERO, ZERO];
    let val_a2 = [ONE, ONE, ZERO, ZERO];

    // insert a value under key_a into an empty tree; this inserts one entry into the advice map
    let init_smt = smt.clone();
    smt.insert(key_a.into(), val_a1);
    let new_map_entries = [build_node_entry(key_a, val_a1, 16)];
    assert_insert(&init_smt, key_a, EMPTY_VALUE, val_a1, smt.root().into(), &new_map_entries);

    // update a value under key_a; this inserts one entry into the advice map
    let init_smt = smt.clone();
    smt.insert(key_a.into(), val_a2);
    let new_map_entries = [build_node_entry(key_a, val_a2, 16)];
    assert_insert(&init_smt, key_a, val_a1, val_a2, smt.root().into(), &new_map_entries);
}

#[test]
fn tsmt_insert_32() {
    let mut smt = TieredSmt::default();

    // insert a value under key_a into an empty tree
    let raw_a = 0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a = [ONE, ZERO, ZERO, ZERO];
    smt.insert(key_a.into(), val_a);

    // insert a value under key_b which has the same 16-bit prefix as A
    let raw_b = 0b00000000_00000000_01111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    let val_b = [ONE, ONE, ZERO, ZERO];

    // this tests a complex insertion when a leaf node moves from depth 16 to depth 32; this
    // moves the original node to depth 32, and thus two new entries are added to the advice map
    let init_smt = smt.clone();
    smt.insert(key_b.into(), val_b);
    let new_map_entries = [build_node_entry(key_a, val_a, 32), build_node_entry(key_b, val_b, 32)];
    assert_insert(&init_smt, key_b, EMPTY_VALUE, val_b, smt.root().into(), &new_map_entries);

    // update a value under key_a; this adds one new entry to the advice map
    let init_smt = smt.clone();
    let val_a2 = [ONE, ZERO, ZERO, ONE];
    smt.insert(key_a.into(), val_a2);
    let new_map_entries = [build_node_entry(key_a, val_a2, 32)];
    assert_insert(&init_smt, key_a, val_a, val_a2, smt.root().into(), &new_map_entries);

    // insert a value under key_c which has the same 16-bit prefix as A and B; this inserts a new
    // node at depth 32, and thus adds one entry to the advice map
    let raw_c = 0b00000000_00000000_00111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    let val_c = [ONE, ONE, ONE, ZERO];

    let init_smt = smt.clone();
    smt.insert(key_c.into(), val_c);
    let new_map_entries = [build_node_entry(key_c, val_c, 32)];
    assert_insert(&init_smt, key_c, EMPTY_VALUE, val_c, smt.root().into(), &new_map_entries);
}

#[test]
fn tsmt_insert_48() {
    let mut smt = TieredSmt::default();

    // insert a value under key_a into an empty tree
    let raw_a = 0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a = [ONE, ZERO, ZERO, ZERO];
    smt.insert(key_a.into(), val_a);

    // insert a value under key_b which has the same 32-bit prefix as A
    let raw_b = 0b00000000_00000000_11111111_11111111_01111111_11111111_11111111_11111111_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    let val_b = [ONE, ONE, ZERO, ZERO];

    // this tests a complex insertion when a leaf moves from depth 16 to depth 48; this moves
    // node at depth 16 to depth 48 and inserts a new node at depth 48
    let init_smt = smt.clone();
    smt.insert(key_b.into(), val_b);
    let new_map_entries = [build_node_entry(key_a, val_a, 48), build_node_entry(key_b, val_b, 48)];
    assert_insert(&init_smt, key_b, EMPTY_VALUE, val_b, smt.root().into(), &new_map_entries);

    // update a value under key_a; this inserts one entry into the advice map
    let init_smt = smt.clone();
    let val_a2 = [ONE, ZERO, ZERO, ONE];
    smt.insert(key_a.into(), val_a2);
    let new_map_entries = [build_node_entry(key_a, val_a2, 48)];
    assert_insert(&init_smt, key_a, val_a, val_a2, smt.root().into(), &new_map_entries);

    // insert a value under key_c which has the same 32-bit prefix as A and B; this inserts
    // one entry into the advice map
    let raw_c = 0b00000000_00000000_11111111_11111111_00111111_11111111_11111111_11111111_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    let val_c = [ONE, ONE, ONE, ZERO];

    let init_smt = smt.clone();
    smt.insert(key_c.into(), val_c);
    let new_map_entries = [build_node_entry(key_c, val_c, 48)];
    assert_insert(&init_smt, key_c, EMPTY_VALUE, val_c, smt.root().into(), &new_map_entries);
}

#[test]
fn tsmt_insert_48_from_32() {
    let mut smt = TieredSmt::default();

    // insert a value under key_a into an empty tree
    let raw_a = 0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a = [ONE, ZERO, ZERO, ZERO];
    smt.insert(key_a.into(), val_a);

    // insert a value under key_b which has the same 16-bit prefix as A
    let raw_b = 0b00000000_00000000_01111111_11111111_01111111_11111111_11111111_11111111_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    let val_b = [ONE, ONE, ZERO, ZERO];
    smt.insert(key_b.into(), val_b);

    // insert a value under key_c which has the same 32-bit prefix as A
    let raw_c = 0b00000000_00000000_11111111_11111111_00111111_11111111_11111111_11111111_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    let val_c = [ONE, ONE, ONE, ZERO];

    // this tests a complex insertion when a leaf moves from depth 32 to depth 48; two new
    // entries are added to the advice map
    let init_smt = smt.clone();
    smt.insert(key_c.into(), val_c);
    let new_map_entries = [build_node_entry(key_a, val_a, 48), build_node_entry(key_c, val_c, 48)];
    assert_insert(&init_smt, key_c, EMPTY_VALUE, val_c, smt.root().into(), &new_map_entries);
}

fn assert_insert(
    init_smt: &TieredSmt,
    key: RpoDigest,
    old_value: Word,
    new_value: Word,
    new_root: RpoDigest,
    new_map_entries: &[AdvMapEntry],
) {
    let old_root = init_smt.root();
    let source = r#"
        use.std::collections::smt

        begin
            exec.smt::insert
        end
    "#;
    let initial_stack = [
        old_root[0].as_int(),
        old_root[1].as_int(),
        old_root[2].as_int(),
        old_root[3].as_int(),
        key[0].as_int(),
        key[1].as_int(),
        key[2].as_int(),
        key[3].as_int(),
        new_value[0].as_int(),
        new_value[1].as_int(),
        new_value[2].as_int(),
        new_value[3].as_int(),
    ];
    let expected_output = stack_top_to_ints(&[
        old_value[3].as_int(),
        old_value[2].as_int(),
        old_value[1].as_int(),
        old_value[0].as_int(),
        new_root[3].as_int(),
        new_root[2].as_int(),
        new_root[1].as_int(),
        new_root[0].as_int(),
    ]);
    let (store, adv_map) = build_advice_inputs(init_smt);
    let process = build_test!(source, &initial_stack, &[], store, adv_map.clone())
        .execute_process()
        .unwrap();

    // check the returned values
    let stack = stack_to_ints(&process.stack.trace_state());
    assert_eq!(stack, expected_output);

    // remove the initial key-value pairs from the advice map
    let mut new_adv_map = process.advice_provider.map().clone();
    for (key, value) in adv_map.iter() {
        let init_value = new_adv_map.remove(key).unwrap();
        assert_eq!(value, &init_value);
    }

    // make sure the remaining values in the advice map are the same as expected new entries
    assert_eq!(new_adv_map.len(), new_map_entries.len());
    for (key, val) in new_map_entries {
        let old_val = new_adv_map.get(key).unwrap();
        assert_eq!(old_val, val);
    }
}

// SET TESTS
// ================================================================================================

#[test]
fn tsmt_set_16() {
    let mut smt = TieredSmt::default();

    let raw_a = 0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a1 = [ONE, ZERO, ZERO, ZERO];
    let val_a2 = [ONE, ONE, ZERO, ZERO];

    // set a value under key_a into an empty tree; this inserts one entry into the advice map
    let init_smt = smt.clone();
    smt.insert(key_a.into(), val_a1);
    let new_map_entries = [build_node_entry(key_a, val_a1, 16)];
    assert_set(&init_smt, key_a, EMPTY_VALUE, val_a1, smt.root().into(), &new_map_entries);

    // update a value under key_a; this inserts one entry into the advice map
    let init_smt = smt.clone();
    smt.insert(key_a.into(), val_a2);
    let new_map_entries = [build_node_entry(key_a, val_a2, 16)];
    assert_set(&init_smt, key_a, val_a1, val_a2, smt.root().into(), &new_map_entries);

    // set an empty value for a previously un-set key; this should not change the tree
    let raw_b = 0b00000000_10000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    assert_set(&smt, key_b, EMPTY_VALUE, EMPTY_VALUE, smt.root().into(), &[]);

    // set an empty value for a previously un-set key which shares 16-bit prefix with A;
    // this should not change the tree
    let raw_c = 0b00000000_00000000_01111111_11111111_11111111_11111111_11111111_11111111_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    assert_set(&smt, key_c, EMPTY_VALUE, EMPTY_VALUE, smt.root().into(), &[]);

    // set the value at key A to an empty word
    let init_smt = smt.clone();
    smt.insert(key_a.into(), EMPTY_VALUE);
    assert_set(&init_smt, key_a, val_a2, EMPTY_VALUE, smt.root().into(), &[]);
}

fn assert_set(
    init_smt: &TieredSmt,
    key: RpoDigest,
    old_value: Word,
    new_value: Word,
    new_root: RpoDigest,
    new_map_entries: &[AdvMapEntry],
) {
    let old_root = init_smt.root();
    let source = r#"
        use.std::collections::smt

        begin
            exec.smt::set
        end
    "#;
    let initial_stack = [
        old_root[0].as_int(),
        old_root[1].as_int(),
        old_root[2].as_int(),
        old_root[3].as_int(),
        key[0].as_int(),
        key[1].as_int(),
        key[2].as_int(),
        key[3].as_int(),
        new_value[0].as_int(),
        new_value[1].as_int(),
        new_value[2].as_int(),
        new_value[3].as_int(),
    ];
    let expected_output = stack_top_to_ints(&[
        old_value[3].as_int(),
        old_value[2].as_int(),
        old_value[1].as_int(),
        old_value[0].as_int(),
        new_root[3].as_int(),
        new_root[2].as_int(),
        new_root[1].as_int(),
        new_root[0].as_int(),
    ]);
    let (store, adv_map) = build_advice_inputs(init_smt);
    let process = build_test!(source, &initial_stack, &[], store, adv_map.clone())
        .execute_process()
        .unwrap();

    // check the returned values
    let stack = stack_to_ints(&process.stack.trace_state());
    assert_eq!(stack, expected_output);

    // remove the initial key-value pairs from the advice map
    let mut new_adv_map = process.advice_provider.map().clone();
    for (key, value) in adv_map.iter() {
        let init_value = new_adv_map.remove(key).unwrap();
        assert_eq!(value, &init_value);
    }

    // make sure the remaining values in the advice map are the same as expected new entries
    assert_eq!(new_adv_map.len(), new_map_entries.len());
    for (key, val) in new_map_entries {
        let old_val = new_adv_map.get(key).unwrap();
        assert_eq!(old_val, val);
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_node_entry(key: RpoDigest, value: Word, depth: u8) -> AdvMapEntry {
    let digest = Rpo256::merge_in_domain(&[key.into(), value.into()], depth.into());
    let mut elements = key.to_vec();
    elements.extend_from_slice(&value);
    (digest.into(), elements)
}
