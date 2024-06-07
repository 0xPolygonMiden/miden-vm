use super::{
    super::chiplets::init_state_from_words, ExecutionTrace, Felt, FieldElement, Process, Trace,
    NUM_RAND_ROWS,
};
use crate::{AdviceInputs, DefaultHost, ExecutionOptions, MemAdviceProvider, StackInputs};
use alloc::vec::Vec;
use test_utils::rand::rand_array;
use vm_core::{
    code_blocks::CodeBlock, CodeBlockTable, Kernel, MastForest, MastNode, Operation, StackOutputs,
    Word, ONE, ZERO,
};

mod chiplets;
mod decoder;
mod hasher;
mod range;
mod stack;

// TEST HELPERS
// ================================================================================================

/// Builds a sample trace by executing the provided code block against the provided stack inputs.
pub fn build_trace_from_program(program: &MastForest, stack_inputs: &[u64]) -> ExecutionTrace {
    let stack_inputs = StackInputs::try_from_ints(stack_inputs.iter().copied()).unwrap();
    let host = DefaultHost::default();
    let mut process =
        Process::new(Kernel::default(), stack_inputs, host, ExecutionOptions::default());
    process.execute_mast_forest(program).unwrap();
    ExecutionTrace::new(process, StackOutputs::default())
}

/// Builds a sample trace by executing a span block containing the specified operations. This
/// results in 1 additional hash cycle (8 rows) at the beginning of the hash chiplet.
pub fn build_trace_from_ops(operations: Vec<Operation>, stack: &[u64]) -> ExecutionTrace {
    let mut mast_forest = MastForest::new();

    let basic_block = MastNode::new_basic_block(operations);
    let basic_block_id = mast_forest.add_node(basic_block);
    mast_forest.set_entrypoint(basic_block_id);

    build_trace_from_program(&mast_forest, stack)
}

/// Builds a sample trace by executing a span block containing the specified operations. Unlike the
/// function above, this function accepts the full [AdviceInputs] object, which means it can run
/// the programs with initialized advice provider.
pub fn build_trace_from_ops_with_inputs(
    operations: Vec<Operation>,
    stack_inputs: StackInputs,
    advice_inputs: AdviceInputs,
) -> ExecutionTrace {
    let advice_provider = MemAdviceProvider::from(advice_inputs);
    let host = DefaultHost::new(advice_provider);
    let mut process =
        Process::new(Kernel::default(), stack_inputs, host, ExecutionOptions::default());
    let program = CodeBlock::new_span(operations);
    process.execute_code_block(&program, &CodeBlockTable::default()).unwrap();
    ExecutionTrace::new(process, StackOutputs::default())
}
