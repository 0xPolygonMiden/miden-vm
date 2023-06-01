use super::{
    super::{AdviceInputs, ExecutionOptions, Felt, FieldElement, Kernel, Operation, StarkField},
    Process,
};
use crate::{MemAdviceProvider, StackInputs, Word};
use test_utils::{crypto::get_smt_remaining_key, rand::seeded_word};
use vm_core::{
    crypto::{
        hash::{Rpo256, RpoDigest},
        merkle::{EmptySubtreeRoots, MerkleStore, MerkleTree, NodeIndex, TieredSmt},
    },
    utils::IntoBytes,
    AdviceInjector, Decorator, ONE, ZERO,
};

#[test]
fn push_merkle_node() {
    let leaves = [init_leaf(1), init_leaf(2), init_leaf(3), init_leaf(4)];
    let tree = MerkleTree::new(leaves.to_vec()).unwrap();
    let store = MerkleStore::from(&tree);
    let stack_inputs = [
        tree.root()[0].as_int(),
        tree.root()[1].as_int(),
        tree.root()[2].as_int(),
        tree.root()[3].as_int(),
        1,
        tree.depth() as u64,
    ];

    let stack_inputs = StackInputs::try_from_values(stack_inputs).unwrap();
    let advice_inputs = AdviceInputs::default().with_merkle_store(store);
    let advice_provider = MemAdviceProvider::from(advice_inputs);
    let mut process =
        Process::new(Kernel::default(), stack_inputs, advice_provider, ExecutionOptions::default());
    process.execute_op(Operation::Noop).unwrap();

    // push the node onto the advice stack
    process
        .execute_decorator(&Decorator::Advice(AdviceInjector::MerkleNodeToStack))
        .unwrap();

    // pop the node from the advice stack and push it onto the operand stack
    process.execute_op(Operation::AdvPop).unwrap();
    process.execute_op(Operation::AdvPop).unwrap();
    process.execute_op(Operation::AdvPop).unwrap();
    process.execute_op(Operation::AdvPop).unwrap();

    let expected_stack = build_expected(&[
        leaves[1][3],
        leaves[1][2],
        leaves[1][1],
        leaves[1][0],
        Felt::new(2),
        Felt::new(1),
        tree.root()[3],
        tree.root()[2],
        tree.root()[1],
        tree.root()[0],
    ]);
    assert_eq!(expected_stack, process.stack.trace_state());
}

// SMTGET TESTS
// ================================================================================================

#[test]
fn push_smtget() {
    // setup the test
    let empty = EmptySubtreeRoots::empty_hashes(64);
    let initial_root = RpoDigest::from(empty[0]);
    let mut seed = 0xfb;
    let key = seeded_word(&mut seed);
    let value = seeded_word(&mut seed);

    // check leaves on empty trees
    for depth in [16, 32, 48] {
        // compute the remaining key
        let remaining = get_smt_remaining_key(key, depth);

        // compute node value
        let depth_element = Felt::from(depth);
        let store = MerkleStore::new();
        let node = Rpo256::merge_in_domain(&[remaining.into(), value.into()], depth_element);

        // expect absent value with constant depth 16
        let expected = [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ONE];
        assert_case_smtget(depth, key, value, node, initial_root, store, &expected);
    }

    // check leaves inserted on all tiers
    for depth in [16, 32, 48] {
        // compute the remaining key
        let remaining = get_smt_remaining_key(key, depth);

        // set depth flags
        let is_16_or_32 = (depth == 16 || depth == 32).then_some(ONE).unwrap_or(ZERO);
        let is_16_or_48 = (depth == 16 || depth == 48).then_some(ONE).unwrap_or(ZERO);

        // compute node value
        let index = key[3].as_int() >> 64 - depth;
        let index = NodeIndex::new(depth, index).unwrap();
        let depth_element = Felt::from(depth);
        let node = Rpo256::merge_in_domain(&[remaining.into(), value.into()], depth_element);

        // set tier node value and expect the value from the injector
        let mut store = MerkleStore::new();
        let root = store.set_node(initial_root, index, node).unwrap().root;
        let expected = [
            ONE,
            value[3],
            value[2],
            value[1],
            value[0],
            remaining[3],
            remaining[2],
            remaining[1],
            remaining[0],
            is_16_or_32,
            is_16_or_48,
        ];
        assert_case_smtget(depth, key, value, node, root, store, &expected);
    }

    // check absent siblings of non-empty trees
    for depth in [16, 32, 48] {
        // set depth flags
        let is_16_or_32 = (depth == 16 || depth == 32).then_some(ONE).unwrap_or(ZERO);
        let is_16_or_48 = (depth == 16 || depth == 48).then_some(ONE).unwrap_or(ZERO);

        // compute the index of the absent node
        let index = key[3].as_int() >> 64 - depth;
        let index = NodeIndex::new(depth, index).unwrap();

        // compute the sibling index of the target with its remaining key and node
        let sibling = index.sibling();
        let mut sibling_key = key;
        sibling_key[3] = Felt::new(sibling.value() >> depth.min(63));
        let sibling_node =
            Rpo256::merge_in_domain(&[sibling_key.into(), value.into()], depth.into());

        // run the text, expecting absent target node
        let mut store = MerkleStore::new();
        let root = store.set_node(initial_root, sibling, sibling_node).unwrap().root;
        let expected =
            [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, is_16_or_32, is_16_or_48];
        assert_case_smtget(depth, key, value, sibling_node, root, store, &expected);
    }
}

// SMTINSERT TESTS
// ================================================================================================

#[test]
fn inject_smtinsert() {
    let mut smt = TieredSmt::default();

    // --- insert into empty tree ---------------------------------------------

    let raw_a = 0b_01101001_01101100_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_a = build_key(raw_a);
    let val_a = [ONE, ZERO, ZERO, ZERO];

    // insertion should happen at depth 16 and thus 16_or_32 and 16_or_48 flags should be set to ONE;
    // since we are replacing a node which is an empty subtree, the is_empty flag should also be ONE
    let expected_stack = [ONE, ONE, ONE];
    let process = prepare_smt_insert(key_a, val_a, &smt, expected_stack.len(), Vec::new());
    assert_eq!(build_expected(&expected_stack), process.stack.trace_state());

    // --- update same key with different value -------------------------------

    // insert val_a into the tree so that val_b overwrites it
    smt.insert(key_a.into(), val_a);
    let val_b = [ONE, ONE, ZERO, ZERO];

    // we are updating a node at depth 16 and thus 16_or_32 and 16_or_48 flags should be set to ONE;
    // since we are updating an existing leaf, the is_empty flag should be set to ZERO
    let expected_stack = [ZERO, ONE, ONE];
    let adv_map = vec![build_adv_map_entry(key_a, val_a, 16)];
    let process = prepare_smt_insert(key_a, val_b, &smt, expected_stack.len(), adv_map);
    assert_eq!(build_expected(&expected_stack), process.stack.trace_state());
}

fn prepare_smt_insert(
    key: Word,
    value: Word,
    smt: &TieredSmt,
    adv_stack_depth: usize,
    adv_map: Vec<([u8; 32], Vec<Felt>)>,
) -> Process<MemAdviceProvider> {
    let root: Word = smt.root().into();
    let store = MerkleStore::from(smt);

    let stack_inputs = build_stack_inputs(value, key, root);
    let advice_inputs = AdviceInputs::default().with_merkle_store(store).with_map(adv_map);
    let mut process = build_process(stack_inputs, advice_inputs);

    process.execute_op(Operation::Noop).unwrap();
    process
        .execute_decorator(&Decorator::Advice(AdviceInjector::SmtInsert))
        .unwrap();

    move_adv_to_stack(&mut process, adv_stack_depth);

    process
}

// HELPER FUNCTIONS
// ================================================================================================

fn init_leaf(value: u64) -> Word {
    [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
}

fn build_expected(values: &[Felt]) -> [Felt; 16] {
    let mut expected = [Felt::ZERO; 16];
    for (&value, result) in values.iter().zip(expected.iter_mut()) {
        *result = value
    }
    expected
}

fn assert_case_smtget(
    depth: u8,
    key: Word,
    value: Word,
    node: RpoDigest,
    root: RpoDigest,
    store: MerkleStore,
    expected_stack: &[Felt],
) {
    // build the process
    let stack_inputs = build_stack_inputs(key, root, Word::default());
    let remaining = get_smt_remaining_key(key, depth);
    let mapped = remaining.into_iter().chain(value.into_iter()).collect();
    let advice_inputs = AdviceInputs::default()
        .with_merkle_store(store)
        .with_map([(node.into_bytes(), mapped)]);
    let advice_provider = MemAdviceProvider::from(advice_inputs);
    let mut process =
        Process::new(Kernel::default(), stack_inputs, advice_provider, ExecutionOptions::default());

    // call the injector and clear the stack
    process.execute_op(Operation::Noop).unwrap();
    process.execute_decorator(&Decorator::Advice(AdviceInjector::SmtGet)).unwrap();

    // replace operand stack contents with the data on the advice stack
    move_adv_to_stack(&mut process, expected_stack.len());

    assert_eq!(build_expected(expected_stack), process.stack.trace_state());
}

fn build_process(
    stack_inputs: StackInputs,
    adv_inputs: AdviceInputs,
) -> Process<MemAdviceProvider> {
    let advice_provider = MemAdviceProvider::from(adv_inputs);
    Process::new(Kernel::default(), stack_inputs, advice_provider)
}

fn build_stack_inputs(w0: Word, w1: Word, w2: Word) -> StackInputs {
    StackInputs::try_from_values([
        w2[0].as_int(),
        w2[1].as_int(),
        w2[2].as_int(),
        w2[3].as_int(),
        w1[0].as_int(),
        w1[1].as_int(),
        w1[2].as_int(),
        w1[3].as_int(),
        w0[0].as_int(),
        w0[1].as_int(),
        w0[2].as_int(),
        w0[3].as_int(),
    ])
    .unwrap()
}

fn build_key(prefix: u64) -> Word {
    [ONE, ONE, ONE, Felt::new(prefix)]
}

/// Removes all items from the operand stack and pushes the specified number of values from
/// the advice tack onto it.
fn move_adv_to_stack(process: &mut Process<MemAdviceProvider>, adv_stack_depth: usize) {
    let stack_depth = process.stack.depth();
    for _ in 0..stack_depth {
        process.execute_op(Operation::Drop).unwrap();
    }

    for _ in 0..adv_stack_depth {
        process.execute_op(Operation::AdvPop).unwrap();
    }
}

fn build_adv_map_entry(key: Word, val: Word, depth: u8) -> ([u8; 32], Vec<Felt>) {
    let remaining_key = get_smt_remaining_key(key, depth);
    let node = Rpo256::merge_in_domain(&[remaining_key.into(), val.into()], Felt::from(depth));
    println!("node: {node:?}");
    let mut elements = Vec::new();
    elements.extend_from_slice(&remaining_key);
    elements.extend_from_slice(&val);
    (node.into(), elements)
}
