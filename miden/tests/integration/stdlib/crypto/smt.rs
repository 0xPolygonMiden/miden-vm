use super::{build_test, Felt};
use crate::helpers::crypto::init_merkle_leaves;
use crypto::{hashers::Rp64_256, ElementHasher};
use rand_utils::rand_array;
use vm_core::{AdviceSet, FieldElement, SmtStore, SparseMerkleTree, StarkField, Word};

#[test]
fn smt_get_key() {
    let source = "
    use.std::crypto::smt
    begin
        exec.smt::get 
    end";

    // Initialize a compact SMT storing two random Word-sized keys sharing the bit prefix "00".
    // An array of compact keys [0, 1] are provided pointing to the compacted values
    // (see std::crypto::smt for more information).
    let depth = 2;
    let key_prefix = 0;
    let (keys, keys_compact, values, tree) = init_merkle_data(key_prefix, depth);

    // Retrieve the full key and value located at index i (i.e. the first stored key/val pair)
    let i = 0;
    let key = keys[i];
    let value = values[i];

    let mut stack_inputs = vec![];
    stack_inputs.extend(to_stack(key));
    stack_inputs.extend(to_stack(tree.root()));

    let advice_tape = vec![
        // Word 1 : [0, 0, d, i]
        keys_compact[i] as u64,
        depth as u64,
        0,
        0,
        // Word 2 : [V]
        values[i][0].as_int(),
        values[i][1].as_int(),
        values[i][2].as_int(),
        values[i][3].as_int(),
    ];

    let mut final_stack = vec![];
    final_stack.extend(to_stack_reversed(value));
    final_stack.extend(to_stack_reversed(tree.root()));
    final_stack.extend(to_stack_reversed(key));

    let test = build_test!(source, &stack_inputs, &advice_tape, vec![tree]);
    test.expect_stack(&final_stack);
}

#[test]
fn smt_update_key() {
    let source = "
    use.std::crypto::smt
    begin
        exec.smt::set
    end";

    let depth = 2;
    let key_prefix = 0;
    let (keys, keys_compact, values, tree) = init_merkle_data(key_prefix, depth);

    // Retrieve the full key and value to be updated
    let i = 0;
    let key = keys[i];
    let old_value = values[i];

    let new_value = int_to_word(5);

    let stack_inputs = [
        new_value[0].as_int(),
        new_value[1].as_int(),
        new_value[2].as_int(),
        new_value[3].as_int(),
        key[0].as_int(),
        key[1].as_int(),
        key[2].as_int(),
        key[3].as_int(),
        tree.root()[0].as_int(),
        tree.root()[1].as_int(),
        tree.root()[2].as_int(),
        tree.root()[3].as_int(),
    ];

    let advice_tape = [
        // Word 0 : [0, 0, 0, 1]
        1, // conditional flag specifying an update (instead of an insertion)
        0,
        0,
        0,
        // Word 1 : [0, 0, d, i]
        keys_compact[i] as u64,
        depth as u64,
        0,
        0,
        // Word 2 : [V]
        old_value[0].as_int(),
        old_value[1].as_int(),
        old_value[2].as_int(),
        old_value[3].as_int(),
    ];

    let mut new_tree = tree.clone();
    let new_hash_value = Rp64_256::hash_elements(&[key, new_value].concat()).into();
    new_tree
        .update_leaf(keys_compact[i], new_hash_value)
        .expect("failed to update leaf");

    let final_stack = [
        new_tree.root()[3].as_int(),
        new_tree.root()[2].as_int(),
        new_tree.root()[1].as_int(),
        new_tree.root()[0].as_int(),
        new_value[3].as_int(),
        new_value[2].as_int(),
        new_value[1].as_int(),
        new_value[0].as_int(),
        key[3].as_int(),
        key[2].as_int(),
        key[1].as_int(),
        key[0].as_int(),
    ];

    let test = build_test!(source, &stack_inputs, &advice_tape, vec![tree]);
    test.expect_stack(&final_stack);
}

/// Type 1 key insertion
#[test]
fn smt_insert_key_type_1() {
    let source = "
    use.std::crypto::smt
    begin
        exec.smt::set
    end";

    // Generate random keys with the desired bit prefixes ("00" and "0111")
    let key_00 = rand_prefix_word(0, 2);
    let key_0111 = rand_prefix_word(7, 4);

    // Create initial compact SMT
    let smt = SparseMerkleTree::new_compact(
        vec![(0, 2), (1, 2)],
        vec![
            hash_kv(key_00, int_to_word(1)),
            hash_kv(key_0111, int_to_word(2)),
        ],
        4,
    )
    .expect("failed to create compact sparse Merkle tree");

    // Truncate tree at depth 2 and convert to an advice set
    let mut smt_truncated = smt.clone();
    smt_truncated.truncate(2);
    let tree = AdviceSet::SparseMerkleTree(smt_truncated);
    let (store, _) = SmtStore::new(4);

    // --- Reinsert the key/value currently at compact key "01" to "0111"

    let leaf_node_depth = 2;
    let leaf_node_key = 1;

    let leaf_key = key_0111;
    let leaf_value = int_to_word(2);

    let empty_hash_1: Word = store.get_empty_hash(leaf_node_depth).into();

    let reinserted_leaf_node_depth = 4;
    let reinserted_leaf_node_key = 7;

    // Compact SMT with empty hash inserted in place of compact key 1 at depth 2
    let smt_2 =
        SparseMerkleTree::new_compact(vec![(0, 2)], vec![hash_kv(key_00, int_to_word(1))], 4)
            .expect("failed to create compact sparse Merkle tree");
    let tree_2 = AdviceSet::SparseMerkleTree(smt_2);

    // Compact SMT with key_0111 reinserted at depth 4
    let smt_3 = SparseMerkleTree::new_compact(
        vec![(0, 2), (7, 4)],
        vec![
            hash_kv(key_00, int_to_word(1)),
            hash_kv(key_0111, int_to_word(2)),
        ],
        4,
    )
    .expect("failed to create compact sparse Merkle tree");
    let tree_3 = AdviceSet::SparseMerkleTree(smt_3);

    // --- Insert the new key/value pair at compact key "0110"

    // New key/value pair to be inserted
    let new_depth = 4;
    let new_compact_key = 6; // 0110
    let new_key = rand_prefix_word(new_compact_key, new_depth);
    let new_value = int_to_word(3);

    // Existing internal node (parent node of new key)
    let internal_node_depth = 3;
    let internal_node_key = 3;

    // Children of internal node
    let left_child_hash = tree_3
        .get_node(internal_node_depth + 1, internal_node_key * 2)
        .expect("failed to retrieve left child of internal node");
    let right_child_hash = tree_3
        .get_node(internal_node_depth + 1, internal_node_key * 2 + 1)
        .expect("failed to retrieve right child of internal node");

    // Empty hash at depth of child
    let empty_hash_2: Word = store
        .get_empty_hash(internal_node_depth as usize + 1)
        .into();

    let stack_inputs = [
        new_value[0].as_int(),
        new_value[1].as_int(),
        new_value[2].as_int(),
        new_value[3].as_int(),
        new_key[0].as_int(),
        new_key[1].as_int(),
        new_key[2].as_int(),
        new_key[3].as_int(),
        tree.root()[0].as_int(),
        tree.root()[1].as_int(),
        tree.root()[2].as_int(),
        tree.root()[3].as_int(),
    ];

    let new_key_limb_lo = new_key[0].as_int() as u32;
    let new_key_limb_hi = (new_key[0].as_int() >> 32) as u32;

    let advice_tape = [
        // Word 0
        0, // conditional flag specifying an insertion
        0,
        0,
        0,
        // Word 1
        1, // conditional flag specifying insertion type 1
        0,
        0,
        0,
        // Word 2
        leaf_node_key as u64,
        leaf_node_depth as u64,
        0,
        0,
        // Word 3
        leaf_key[0].as_int(), // leaf key to be reinserted (K')
        leaf_key[1].as_int(), // ''
        leaf_key[2].as_int(), // ''
        leaf_key[3].as_int(), // ''
        // Word 4
        leaf_value[0].as_int(), // leaf value to be reinserted (V')
        leaf_value[1].as_int(), // ''
        leaf_value[2].as_int(), // ''
        leaf_value[3].as_int(), // ''
        // Word 5
        empty_hash_1[0].as_int(), // empty hash at reinserted key depth
        empty_hash_1[1].as_int(), // ''
        empty_hash_1[2].as_int(), // ''
        empty_hash_1[3].as_int(), // ''
        // Word 6
        reinserted_leaf_node_key as u64,
        reinserted_leaf_node_depth as u64,
        0,
        0,
        // Word 7
        tree_2.root()[0].as_int(),
        tree_2.root()[1].as_int(),
        tree_2.root()[2].as_int(),
        tree_2.root()[3].as_int(),
        // Word 8
        4,
        0,
        0,
        0,
        // Word 9
        internal_node_key as u64,   // internal node key
        internal_node_depth as u64, // internal node depth
        new_key_limb_lo as u64,     // lower u32 of first limb of new key
        new_key_limb_hi as u64,     // upper u32 of first limb of new key
        // Word 10
        left_child_hash[0].as_int(), // left child of internal node
        left_child_hash[1].as_int(), // ''
        left_child_hash[2].as_int(), // ''
        left_child_hash[3].as_int(), // ''
        // Word 11
        right_child_hash[0].as_int(), // right child of internal node
        right_child_hash[1].as_int(), // ''
        right_child_hash[2].as_int(), // ''
        right_child_hash[3].as_int(), // ''
        // Word 12
        empty_hash_2[0].as_int(), // empty hash at depth of children
        empty_hash_2[1].as_int(), // ''
        empty_hash_2[2].as_int(), // ''
        empty_hash_2[3].as_int(), // ''
        // Word 13
        1, // is empty hash left (1) or right (0) child of internal node?
        0, //
        0, //
        0, //
        // Word 14
        60, // number of bits to rightshift key limb k
        0,  //
        0,  //
        0,  //
    ];

    let new_smt = SparseMerkleTree::new_compact(
        vec![(0, 2), (7, 4), (6, 4)],
        vec![
            hash_kv(key_00, int_to_word(1)),
            hash_kv(key_0111, int_to_word(2)),
            hash_kv(new_key, int_to_word(3)),
        ],
        4,
    )
    .expect("failed to create compact sparse Merkle tree");

    let final_stack = [
        new_smt.root()[3].as_int(),
        new_smt.root()[2].as_int(),
        new_smt.root()[1].as_int(),
        new_smt.root()[0].as_int(),
        new_value[3].as_int(),
        new_value[2].as_int(),
        new_value[1].as_int(),
        new_value[0].as_int(),
        new_key[3].as_int(),
        new_key[2].as_int(),
        new_key[1].as_int(),
        new_key[0].as_int(),
    ];

    let test = build_test!(
        source,
        &stack_inputs,
        &advice_tape,
        vec![tree] //, tree_2, tree_3]
    );
    test.expect_stack(&final_stack);
}

/// Type 2 key insertion
#[test]
fn smt_insert_key_type_2() {
    let source = "
    use.std::crypto::smt
    begin
        exec.smt::set
    end";

    // Generate random keys with the desired bit prefixes
    let key_00 = rand_prefix_word(0, 2);
    let key_0111 = rand_prefix_word(7, 4);
    let key_0110 = rand_prefix_word(14, 4);

    // Create initial compact SMT
    let smt = SparseMerkleTree::new_compact(
        vec![(0, 2), (7, 4), (6, 4)],
        vec![
            hash_kv(key_00, int_to_word(1)),
            hash_kv(key_0111, int_to_word(2)),
            hash_kv(key_0110, int_to_word(3)),
        ],
        4,
    )
    .expect("failed to create compact sparse Merkle tree");

    // Truncate tree at depth 3 and convert to advice set
    let mut smt_truncated = smt.clone();
    smt_truncated.truncate(3);
    let tree = AdviceSet::SparseMerkleTree(smt_truncated);

    // New key/value pair to be inserted
    let new_depth = 3;
    let new_compact_key = 2; // 010
    let new_key = rand_prefix_word(new_compact_key, new_depth);
    let new_value = int_to_word(4);

    // Existing internal node (parent node of new key)
    let internal_node_depth = 2;
    let internal_node_key = 1;

    // Children of internal node
    let left_child_hash = tree
        .get_node(internal_node_depth + 1, internal_node_key * 2)
        .expect("failed to retrieve left child of internal node");
    let right_child_hash = tree
        .get_node(internal_node_depth + 1, internal_node_key * 2 + 1)
        .expect("failed to retrieve right child of internal node");

    // Empty hash at depth of child
    let (store, _) = SmtStore::new(4);
    let empty_hash: Word = store
        .get_empty_hash(internal_node_depth as usize + 1)
        .into();

    let stack_inputs = [
        new_value[0].as_int(),
        new_value[1].as_int(),
        new_value[2].as_int(),
        new_value[3].as_int(),
        new_key[0].as_int(),
        new_key[1].as_int(),
        new_key[2].as_int(),
        new_key[3].as_int(),
        tree.root()[0].as_int(),
        tree.root()[1].as_int(),
        tree.root()[2].as_int(),
        tree.root()[3].as_int(),
    ];

    let new_key_limb_lo = new_key[0].as_int() as u32;
    let new_key_limb_hi = (new_key[0].as_int() >> 32) as u32;

    let advice_tape = [
        // Word 0
        0, // conditional flag specifying an insertion
        0,
        0,
        0,
        // Word 1
        0, // conditional flag specifying insertion type 2
        0,
        0,
        0,
        // Word 2
        internal_node_key as u64,   // internal node key
        internal_node_depth as u64, // internal node depth
        new_key_limb_lo as u64,     // lower u32 of first limb of new key
        new_key_limb_hi as u64,     // upper u32 of first limb of new key
        // Word 3
        left_child_hash[0].as_int(), // left child of internal node
        left_child_hash[1].as_int(), // ''
        left_child_hash[2].as_int(), // ''
        left_child_hash[3].as_int(), // ''
        // Word 4
        right_child_hash[0].as_int(), // right child of internal node
        right_child_hash[1].as_int(), // ''
        right_child_hash[2].as_int(), // ''
        right_child_hash[3].as_int(), // ''
        // Word 5
        empty_hash[0].as_int(), // empty hash at depth of children
        empty_hash[1].as_int(), // ''
        empty_hash[2].as_int(), // ''
        empty_hash[3].as_int(), // ''
        // Word 6
        1, // is empty hash left (1) or right (0) child of internal node?
        0, //
        0, //
        0, //
        // Word 7
        61, // number of bits to rightshift key limb k
        0,  //
        0,  //
        0,  //
    ];

    let new_smt = SparseMerkleTree::new_compact(
        vec![(0, 2), (7, 4), (6, 4), (2, 3)],
        vec![
            hash_kv(key_00, int_to_word(1)),
            hash_kv(key_0111, int_to_word(2)),
            hash_kv(key_0110, int_to_word(3)),
            hash_kv(new_key, int_to_word(4)), // 010
        ],
        4,
    )
    .expect("failed to create compact sparse Merkle tree");

    let final_stack = [
        new_smt.root()[3].as_int(),
        new_smt.root()[2].as_int(),
        new_smt.root()[1].as_int(),
        new_smt.root()[0].as_int(),
        new_value[3].as_int(),
        new_value[2].as_int(),
        new_value[1].as_int(),
        new_value[0].as_int(),
        new_key[3].as_int(),
        new_key[2].as_int(),
        new_key[1].as_int(),
        new_key[0].as_int(),
    ];

    let test = build_test!(source, &stack_inputs, &advice_tape, vec![tree]);
    test.expect_stack(&final_stack);
}

/// Generate a random Word with the first 'depth' bits set to the binary representation of 'key'
fn rand_prefix_word(key: u64, depth: u32) -> Word {
    let mut v = rand_array::<Felt, 4>();
    let mut limb = v[0].as_int();
    limb /= 2u64.pow(depth);
    limb += key * 2u64.pow(64 - depth);
    v[0] = Felt::new(limb);
    v
}

/// Construct an SMT with two random keys sharing a common prefix of a specified depth
fn init_merkle_data(key_prefix: u64, depth: u32) -> (Vec<Word>, Vec<u64>, Vec<Word>, AdviceSet) {
    // Initialize keys
    let keys = (0..2)
        .map(|_| rand_prefix_word(key_prefix, depth))
        .collect::<Vec<[Felt; 4]>>();
    let keys_compact = vec![0, 1];

    // Initialize values
    let values = init_merkle_leaves(&[3, 4]);
    let values_compact = (0..2)
        .map(|i| Rp64_256::hash_elements(&[keys[i], values[i]].concat()).into())
        .collect();

    let tree =
        AdviceSet::new_sparse_merkle_tree(keys_compact.clone(), values_compact, depth).unwrap();
    (keys, keys_compact, values, tree)
}

fn hash_kv(key: Word, val: Word) -> Word {
    Rp64_256::hash_elements(&[key, val].concat()).into()
}

const fn int_to_word(value: u64) -> Word {
    [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
}

fn to_stack(val: Word) -> [u64; 4] {
    [
        val[0].as_int(),
        val[1].as_int(),
        val[2].as_int(),
        val[3].as_int(),
    ]
}

fn to_stack_reversed(val: Word) -> [u64; 4] {
    [
        val[3].as_int(),
        val[2].as_int(),
        val[1].as_int(),
        val[0].as_int(),
    ]
}
