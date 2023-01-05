use super::{ExecutionTrace, Felt, FieldElement, LookupTableRow, Process, Trace, NUM_RAND_ROWS};
use crate::{AdviceProvider, BaseAdviceProvider};
use rand_utils::rand_array;
use vm_core::{
    code_blocks::CodeBlock, CodeBlockTable, Kernel, Operation, ProgramOutputs, Word, ONE, ZERO, StackInputs,
};

mod chiplets;
mod hasher;
mod range;
mod stack;

// TEST HELPERS
// ================================================================================================

/// Builds a sample trace by executing the provided code block against the provided stack inputs.
pub fn build_trace_from_block(program: &CodeBlock, stack: &[u64]) -> ExecutionTrace {
    let mut process =
        Process::new(&Kernel::default(), BaseAdviceProvider::default(), StackInputs::from_vec(stack));
    process.execute_code_block(program, &CodeBlockTable::default()).unwrap();
    ExecutionTrace::new(process, ProgramOutputs::default())
}

/// Builds a sample trace by executing a span block containing the specified operations. This
/// results in 1 additional hash cycle (8 rows) at the beginning of the hash chiplet.
pub fn build_trace_from_ops(operations: Vec<Operation>, stack: &[u64]) -> ExecutionTrace {
    let program = CodeBlock::new_span(operations);
    build_trace_from_block(&program, stack)
}

/// Builds a sample trace by executing a span block containing the specified operations. Unlike the
/// function above, this function accepts the full [ProgramInputs] object, which means it can run
/// the programs with initialized advice provider.
pub fn build_trace_from_ops_with_inputs<ADV>(
    operations: Vec<Operation>,
    advice: ADV,
    stack: &[u64],
) -> ExecutionTrace
where
    ADV: AdviceProvider,
{
    let mut process = Process::new(&Kernel::default(), advice, StackInputs::from_vec(stack));
    let program = CodeBlock::new_span(operations);
    process.execute_code_block(&program, &CodeBlockTable::default()).unwrap();
    ExecutionTrace::new(process, ProgramOutputs::default())
}
