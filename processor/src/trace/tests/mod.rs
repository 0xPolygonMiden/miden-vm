use super::{
    ExecutionTrace, Felt, FieldElement, LookupTableRow, Process, Trace, Vec, NUM_RAND_ROWS,
};
use crate::{AdviceInputs, MemAdviceProvider, StackInputs};
use rand_utils::rand_array;
use vm_core::{
    code_blocks::CodeBlock, CodeBlockTable, Kernel, Operation, StackOutputs, Word, ONE, ZERO,
};

mod chiplets;
mod hasher;
mod range;
mod stack;

// TEST HELPERS
// ================================================================================================

/// Builds a sample trace by executing the provided code block against the provided stack inputs.
pub fn build_trace_from_block(program: &CodeBlock, stack_inputs: &[u64]) -> ExecutionTrace {
    let stack_inputs = StackInputs::try_from_values(stack_inputs.iter().copied()).unwrap();
    let advice_provider = MemAdviceProvider::default();
    let mut process = Process::new(Kernel::default(), stack_inputs, advice_provider);
    process.execute_code_block(program, &CodeBlockTable::default()).unwrap();
    ExecutionTrace::new(process, StackOutputs::default())
}

/// Builds a sample trace by executing a span block containing the specified operations. This
/// results in 1 additional hash cycle (8 rows) at the beginning of the hash chiplet.
pub fn build_trace_from_ops(operations: Vec<Operation>, stack: &[u64]) -> ExecutionTrace {
    let program = CodeBlock::new_span(operations);
    build_trace_from_block(&program, stack)
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
    let mut process = Process::new(Kernel::default(), stack_inputs, advice_provider);
    let program = CodeBlock::new_span(operations);
    process.execute_code_block(&program, &CodeBlockTable::default()).unwrap();
    ExecutionTrace::new(process, StackOutputs::default())
}
