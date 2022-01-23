use super::{build_inputs, build_stack_state, compile, execute, Felt, FieldElement, Word};
use rand_utils::rand_vector;
use vm_core::{hasher::hash_elements, AdviceSet, ProgramInputs, StarkField};

// TESTS
// ================================================================================================

#[test]
fn rpperm() {
    // --- test hashing [ONE, ONE] ----------------------------------------
    let expected = hash_elements(&[Felt::ONE, Felt::ONE]);
    let inputs = build_inputs(&[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    let script = compile("begin rpperm end");

    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected.as_elements(), &last_state[..4]);

    // --- test hashing 8 random values -----------------------------------
    let mut values = rand_vector::<u64>(8);
    let expected = hash_elements(&values.iter().map(|&v| Felt::new(v)).collect::<Vec<_>>());
    values.extend_from_slice(&[0, 0, 0, 8]);
    let inputs = build_inputs(&values);
    let script = compile("begin rpperm end");

    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    assert_eq!(expected.as_elements(), &last_state[..4]);
}

#[test]
fn mtree_get() {
    let script = compile("begin mtree.get end");

    let index = 3usize;
    let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let tree = AdviceSet::new_merkle_tree(leaves.clone()).unwrap();

    let init_stack = [
        tree.depth() as u64,
        index as u64,
        tree.root()[3].as_int(),
        tree.root()[2].as_int(),
        tree.root()[1].as_int(),
        tree.root()[0].as_int(),
    ];

    let inputs = ProgramInputs::new(&init_stack, &[], vec![tree.clone()]).unwrap();

    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();
    let expected_state = build_stack_state(&[
        leaves[index][3].as_int(),
        leaves[index][2].as_int(),
        leaves[index][1].as_int(),
        leaves[index][0].as_int(),
        tree.root()[3].as_int(),
        tree.root()[2].as_int(),
        tree.root()[1].as_int(),
        tree.root()[0].as_int(),
    ]);
    assert_eq!(expected_state, last_state);
}

#[test]
fn mtree_update() {
    // --- mtree.set ----------------------------------------------------------------------
    // update a node value and replace the old root
    let script = compile("begin mtree.set end");

    let index = 5usize;
    let leaves = init_leaves(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let tree = AdviceSet::new_merkle_tree(leaves.clone()).unwrap();

    let new_node = init_leaf(9);
    let mut new_leaves = leaves;
    new_leaves[index] = new_node;
    let new_tree = AdviceSet::new_merkle_tree(new_leaves).unwrap();

    let init_stack = [
        tree.depth() as u64,
        index as u64,
        new_node[3].as_int(),
        new_node[2].as_int(),
        new_node[1].as_int(),
        new_node[0].as_int(),
        tree.root()[3].as_int(),
        tree.root()[2].as_int(),
        tree.root()[1].as_int(),
        tree.root()[0].as_int(),
    ];

    let inputs = ProgramInputs::new(&init_stack, &[], vec![tree.clone()]).unwrap();
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();

    // expected state has the new leaf and the new root of the tree
    let expected_state = build_stack_state(&[
        new_node[3].as_int(),
        new_node[2].as_int(),
        new_node[1].as_int(),
        new_node[0].as_int(),
        new_tree.root()[3].as_int(),
        new_tree.root()[2].as_int(),
        new_tree.root()[1].as_int(),
        new_tree.root()[0].as_int(),
    ]);

    assert_eq!(expected_state, last_state);

    // --- mtree.cwm ----------------------------------------------------------------------
    // update a node value and replace the old root
    let script = compile("begin mtree.cwm end");
    let inputs = ProgramInputs::new(&init_stack, &[], vec![tree.clone()]).unwrap();
    let trace = execute(&script, &inputs).unwrap();
    let last_state = trace.last_stack_state();

    // expected state has the new leaf, the new root of the tree, and the root of the old tree
    let expected_state = build_stack_state(&[
        new_node[3].as_int(),
        new_node[2].as_int(),
        new_node[1].as_int(),
        new_node[0].as_int(),
        new_tree.root()[3].as_int(),
        new_tree.root()[2].as_int(),
        new_tree.root()[1].as_int(),
        new_tree.root()[0].as_int(),
        tree.root()[3].as_int(),
        tree.root()[2].as_int(),
        tree.root()[1].as_int(),
        tree.root()[0].as_int(),
    ]);

    assert_eq!(expected_state, last_state);
}

// HELPER FUNCTIONS
// ================================================================================================

fn init_leaves(values: &[u64]) -> Vec<Word> {
    values.iter().map(|&v| init_leaf(v)).collect()
}

fn init_leaf(value: u64) -> Word {
    [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
}
