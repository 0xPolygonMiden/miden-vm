use crate::build_test;
use test_utils::{
    crypto::{get_smt_remaining_key, EmptySubtreeRoots, MerkleStore, NodeIndex, Rpo256},
    rand::{seeded_element, seeded_word},
    Felt, IntoBytes, StarkField, Word,
};

#[test]
fn smtget_single_leaf_depth_16() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // append a leaf
    let path = seeded_element(&mut seed).as_int();
    let leaf = SmtLeaf::new(&mut seed, path, 16);
    let root = leaf.insert(&mut store, root);

    // run the test
    let advice_map = build_advice_map([leaf]);
    assert_smt_get_opens_correctly(leaf.key, leaf.value, root, store, &advice_map);
}

#[test]
fn smtget_absent_leaf_depth_16() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // generate two paths that diverges at depth 16
    let a = 0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let b = 0b10000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;

    // generate two leaves from the paths
    let a = SmtLeaf::new(&mut seed, a, 16);
    let b = SmtLeaf::new(&mut seed, b, 16);

    // append only `b` to the store
    let root = b.insert(&mut store, root);

    // sanity check if `b` is properly returned
    let advice_map = build_advice_map([b]);
    assert_smt_get_opens_correctly(b.key, b.value, root, store.clone(), &advice_map);

    // `a` should return zeroes as it was not inserted
    assert_smt_get_opens_correctly(a.key, Word::default(), root, store.clone(), &advice_map);
}

#[test]
fn smtget_absent_leaf_with_conflicting_path_at_depth_16() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // generate two paths that diverges after depth 16
    let a = 0b00000000_00000000_01111111_11111111_11111111_11111111_11111111_11111111_u64;
    let b = 0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;

    // generate two leaves from the paths
    let a = SmtLeaf::new(&mut seed, a, 16);
    let b = SmtLeaf::new(&mut seed, b, 16);

    // append only `b` to the store
    let root = b.insert(&mut store, root);

    // sanity check if `b` is properly returned
    let advice_map = build_advice_map([b]);
    assert_smt_get_opens_correctly(b.key, b.value, root, store.clone(), &advice_map);

    // `a` should return zeroes as it was not inserted
    assert_smt_get_opens_correctly(a.key, Word::default(), root, store.clone(), &advice_map);
}

#[test]
fn smtget_single_leaf_depth_32() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // append a leaf
    let path = seeded_element(&mut seed).as_int();
    let leaf = SmtLeaf::new(&mut seed, path, 32);
    let root = leaf.insert(&mut store, root);

    // run the test
    let advice_map = build_advice_map([leaf]);
    assert_smt_get_opens_correctly(leaf.key, leaf.value, root, store, &advice_map);
}

#[test]
fn smtget_absent_leaf_depth_32() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // generate two paths that diverges at depth 32
    let a = 0b00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_u64;
    let b = 0b00000000_00000000_10000000_00000000_11111111_11111111_11111111_11111111_u64;

    // generate two leaves from the paths
    let a = SmtLeaf::new(&mut seed, a, 32);
    let b = SmtLeaf::new(&mut seed, b, 32);

    // append only `b` to the store
    let root = b.insert(&mut store, root);

    // sanity check if `b` is properly returned
    let advice_map = build_advice_map([b]);
    assert_smt_get_opens_correctly(b.key, b.value, root, store.clone(), &advice_map);

    // `a` should return zeroes as it was not inserted
    assert_smt_get_opens_correctly(a.key, Word::default(), root, store.clone(), &advice_map);
}

#[test]
fn smtget_absent_leaf_with_conflicting_path_at_depth_32() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // generate two paths that diverges after depth 32
    let a = 0b00000000_00000000_00000000_00000000_01111111_11111111_11111111_11111111_u64;
    let b = 0b00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_u64;

    // generate two leaves from the paths
    let a = SmtLeaf::new(&mut seed, a, 32);
    let b = SmtLeaf::new(&mut seed, b, 32);

    // append only `b` to the store
    let root = b.insert(&mut store, root);

    // sanity check if `b` is properly returned
    let advice_map = build_advice_map([b]);
    assert_smt_get_opens_correctly(b.key, b.value, root, store.clone(), &advice_map);

    // `a` should return zeroes as it was not inserted
    assert_smt_get_opens_correctly(a.key, Word::default(), root, store.clone(), &advice_map);
}

#[test]
fn smtget_single_leaf_depth_48() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // append a leaf
    let path = seeded_element(&mut seed).as_int();
    let leaf = SmtLeaf::new(&mut seed, path, 48);
    let root = leaf.insert(&mut store, root);

    // run the test
    let advice_map = build_advice_map([leaf]);
    assert_smt_get_opens_correctly(leaf.key, leaf.value, root, store, &advice_map);
}

#[test]
fn smtget_absent_leaf_depth_48() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // generate two paths that diverges at depth 48
    let a = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111_u64;
    let b = 0b00000000_00000000_00000000_00000000_10000000_00000000_11111111_11111111_u64;

    // generate two leaves from the paths
    let a = SmtLeaf::new(&mut seed, a, 48);
    let b = SmtLeaf::new(&mut seed, b, 48);

    // append only `b` to the store
    let root = b.insert(&mut store, root);

    // sanity check if `b` is properly returned
    let advice_map = build_advice_map([b]);
    assert_smt_get_opens_correctly(b.key, b.value, root, store.clone(), &advice_map);

    // `a` should return zeroes as it was not inserted
    assert_smt_get_opens_correctly(a.key, Word::default(), root, store.clone(), &advice_map);
}

#[test]
fn smtget_absent_leaf_with_conflicting_path_at_depth_48() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // generate two paths that diverges after depth 48
    let a = 0b00000000_00000000_00000000_00000000_00000000_00000000_01111111_11111111_u64;
    let b = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111_u64;

    // generate two leaves from the paths
    let a = SmtLeaf::new(&mut seed, a, 48);
    let b = SmtLeaf::new(&mut seed, b, 48);

    // append only `b` to the store
    let root = b.insert(&mut store, root);

    // sanity check if `b` is properly returned
    let advice_map = build_advice_map([b]);
    assert_smt_get_opens_correctly(b.key, b.value, root, store.clone(), &advice_map);

    // `a` should return zeroes as it was not inserted
    assert_smt_get_opens_correctly(a.key, Word::default(), root, store.clone(), &advice_map);
}

#[test]
fn smtget_opens_correctly_from_tree_with_multiple_leaves() {
    // setup the base values
    let mut seed = 1 << 40;
    let (root, mut store) = setup();

    // define some paths
    let a_3 = 0b00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let b_3 = 0b10000000_00000000_01111111_11111111_11111111_11111111_11111111_11111111_u64;
    let c_3 = 0b10000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_u64;
    let d_3 = 0b11000000_00000000_00000000_00000000_01111111_11111111_11111111_11111111_u64;
    let e_3 = 0b11000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_u64;

    // `a` has no conflicts on depth `16` (first tier)
    let a = SmtLeaf::new(&mut seed, a_3, 16);

    // `b` conflicts with `c` up to `16`; the target tier is `32`
    let b = SmtLeaf::new(&mut seed, b_3, 32);
    let c = SmtLeaf::new(&mut seed, c_3, 32);

    // `d` conflicts with `e` up to `32`; the target tier is `48`
    let d = SmtLeaf::new(&mut seed, d_3, 48);
    let e = SmtLeaf::new(&mut seed, e_3, 48);

    // append all leaves to the storage
    let root = a.insert(&mut store, root);
    let root = b.insert(&mut store, root);
    let root = c.insert(&mut store, root);
    let root = d.insert(&mut store, root);
    let root = e.insert(&mut store, root);

    // assert all nodes are returned
    let advice_map = build_advice_map([a, b, c, d, e]);
    assert_smt_get_opens_correctly(a.key, a.value, root, store.clone(), &advice_map);
    assert_smt_get_opens_correctly(b.key, b.value, root, store.clone(), &advice_map);
    assert_smt_get_opens_correctly(c.key, c.value, root, store.clone(), &advice_map);
    assert_smt_get_opens_correctly(d.key, d.value, root, store.clone(), &advice_map);
    assert_smt_get_opens_correctly(e.key, e.value, root, store.clone(), &advice_map);

    // assert similar siblings returns zeroes
    let x = 0b00000000_00000000_10111111_11111111_11111111_11111111_11111111_11111111_u64;
    let y = 0b10000000_00000000_10111111_11111111_11111111_11111111_11111111_11111111_u64;
    let z = 0b11000000_00000000_00000000_00000000_10111111_11111111_11111111_11111111_u64;

    // `x` is the same path of `a` for the included part of `a` (i.e. until depth `16)
    let mut key = a.key;
    key[3] = Felt::new(x);
    assert_smt_get_opens_correctly(key, Word::default(), root, store.clone(), &advice_map);

    // `y` is the same path of `c` until depth `32`
    let mut key = c.key;
    key[3] = Felt::new(y);
    assert_smt_get_opens_correctly(key, Word::default(), root, store.clone(), &advice_map);

    // `z` is the same path of `e` until depth `48`
    let mut key = e.key;
    key[3] = Felt::new(z);
    assert_smt_get_opens_correctly(key, Word::default(), root, store.clone(), &advice_map);
}

// TEST HELPERS
// ================================================================================================

/// Common initial test setup
///
/// Returns the set of empty digests for SMT, the initial root, and an empty MerkleStore
fn setup() -> (Word, MerkleStore) {
    let empty = EmptySubtreeRoots::empty_hashes(64);
    let root = Word::from(empty[0]);
    let store = MerkleStore::new();
    (root, store)
}

/// Asserts key/value opens to root, provided the advice map and store
fn assert_smt_get_opens_correctly(
    key: Word,
    value: Word,
    root: Word,
    store: MerkleStore,
    advice_map: &[([u8; 32], Vec<Felt>)],
) {
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
    let advice_stack = [];
    build_test!(source, &initial_stack, &advice_stack, store, advice_map.iter().cloned())
        .expect_stack(&expected_output);
}

/// Builds the advice map from the given leaves
fn build_advice_map<I>(leaves: I) -> Vec<([u8; 32], Vec<Felt>)>
where
    I: IntoIterator<Item = SmtLeaf>,
{
    leaves
        .into_iter()
        .map(|leaf| {
            let node = leaf.node.into_bytes();
            let mapped = leaf.remaining_key.into_iter().chain(leaf.value.into_iter()).collect();
            (node, mapped)
        })
        .collect()
}

/// A representation of the post-insertion state of a leaf
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SmtLeaf {
    /// Key
    pub key: Word,
    /// Remaining key for the depth
    pub remaining_key: Word,
    /// Generated value
    pub value: Word,
    /// Index in which the node was inserted
    pub index: NodeIndex,
    /// Computed node value
    pub node: Word,
}

impl SmtLeaf {
    /// Creates a new leaf, generating a random key/value from the seed and replacing the last limb
    /// of the key by `path`.
    ///
    /// The node value and remaining key will be defined by `depth`.
    pub fn new(seed: &mut u64, path: u64, depth: u8) -> Self {
        // generate a random pair
        let mut key = seeded_word(seed);
        let value = seeded_word(seed);

        // override the limb that defines the SMT path
        key[3] = Felt::new(path);

        // compute the target index, remaining key, and node value
        let index = NodeIndex::new(depth, path >> (64 - depth)).unwrap();
        let remaining_key = get_smt_remaining_key(key, depth);
        let depth = Felt::from(depth);
        let node = Rpo256::merge_in_domain(&[remaining_key.into(), value.into()], depth).into();

        // return the values
        SmtLeaf {
            key,
            remaining_key,
            value,
            index,
            node,
        }
    }

    /// Insert the leaf onto the [MerkleStore], returning the new root value.
    pub fn insert(&self, store: &mut MerkleStore, root: Word) -> Word {
        store.set_node(root.into(), self.index, self.node.into()).unwrap().root.into()
    }
}
