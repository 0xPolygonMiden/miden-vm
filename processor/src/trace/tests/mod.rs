use super::{ExecutionTrace, Felt, FieldElement, LookupTableRow, Process, Trace, NUM_RAND_ROWS};
use rand_utils::rand_array;
use vm_core::{program::blocks::CodeBlock, Operation, ProgramInputs, ONE, ZERO};

mod stack;

// TEST HELPERS
// ================================================================================================

/// Builds a sample trace by executing the provided code block against the provided stack inputs.
pub fn build_trace_from_block(program: &CodeBlock, stack: &[u64]) -> ExecutionTrace {
    let inputs = ProgramInputs::new(stack, &[], vec![]).unwrap();
    let mut process = Process::new(inputs);
    process.execute_code_block(program).unwrap();
    ExecutionTrace::new(process)
}

/// Builds a sample trace by executing a span block containing the specified operations. This
/// results in 1 additional hash cycle at the beginning of the hasher coprocessor.
pub fn build_trace_from_ops(operations: Vec<Operation>, stack: &[u64]) -> ExecutionTrace {
    let program = CodeBlock::new_span(operations);
    build_trace_from_block(&program, stack)
}
