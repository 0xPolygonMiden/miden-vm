use crate::build_op_test;
use crate::helpers::crypto::{init_merkle_leaf, init_merkle_store};
use rand_utils::rand_vector;
use vm_core::{
    crypto::merkle::{MerkleStore, MerkleTree},
    StarkField, Word,
};

#[test]
fn hperm() {
    let asm_op = "hperm";
    let pub_inputs = rand_vector::<u64>(8);

    build_op_test!(asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

#[test]
fn hmerge() {
    let asm_op = "hmerge";
    let pub_inputs = rand_vector::<u64>(8);

    build_op_test!(asm_op, &pub_inputs).prove_and_verify(pub_inputs, false);
}

#[test]
fn mtree_get() {
    let asm_op = "mtree_get";

    let index = 3usize;
    let (leaves, store) = init_merkle_store(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let tree = MerkleTree::new(leaves.clone()).unwrap();

    let stack_inputs = [
        tree.root()[0].as_int(),
        tree.root()[1].as_int(),
        tree.root()[2].as_int(),
        tree.root()[3].as_int(),
        index as u64,
        tree.depth() as u64,
    ];

    build_op_test!(asm_op, &stack_inputs, &[], store)
        .prove_and_verify(stack_inputs.to_vec(), false);
}

#[test]
fn mtree_set() {
    let asm_op = "mtree_set";
    let (stack_inputs, store, _leaves) = build_mtree_update_test_inputs();

    build_op_test!(asm_op, &stack_inputs, &[], store)
        .prove_and_verify(stack_inputs.to_vec(), false);
}

#[test]
fn mtree_cwm() {
    let asm_op = "mtree_cwm";
    let (stack_inputs, store, _leaves) = build_mtree_update_test_inputs();

    build_op_test!(asm_op, &stack_inputs, &[], store).prove_and_verify(stack_inputs, false);
}

/// Helper function that builds a test stack and Merkle tree for testing mtree updates.
fn build_mtree_update_test_inputs() -> (Vec<u64>, MerkleStore, Vec<Word>) {
    let index = 5_usize;
    let (leaves, store) = init_merkle_store(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let tree = MerkleTree::new(leaves.clone()).unwrap();

    let new_node = init_merkle_leaf(9);
    let mut new_leaves = leaves.clone();
    new_leaves[index] = new_node;

    let stack_inputs = vec![
        new_node[0].as_int(),
        new_node[1].as_int(),
        new_node[2].as_int(),
        new_node[3].as_int(),
        tree.root()[0].as_int(),
        tree.root()[1].as_int(),
        tree.root()[2].as_int(),
        tree.root()[3].as_int(),
        index as u64,
        tree.depth() as u64,
    ];

    (stack_inputs, store, leaves)
}
