use super::{
    super::StarkField, build_inputs, compile, execute, rand_word, test_compilation_failure,
    test_execution_failure, test_op_execution, test_param_out_of_bounds, Felt,
};

mod arithmetic_ops;
mod bitwise_ops;
mod comparison_ops;
mod conversion_ops;

// CONSTANTS
// ================================================================================================
const U32_BOUND: u64 = u32::MAX as u64 + 1;
const WORD_LEN: usize = 4;

// HELPER FUNCTIONS
// ================================================================================================

/// This helper function tests a provided u32 assembly operation, which takes a single input, to
/// ensure that it fails when the input is >= 2^32.
fn test_input_out_of_bounds(asm_op: &str) {
    test_execution_failure(asm_op, &[U32_BOUND], "FailedAssertion");
}

/// This helper function tests a provided u32 assembly operation, which takes multiple inputs, to
/// ensure that it fails when any one of the inputs is >= 2^32. Each input is tested independently.
fn test_inputs_out_of_bounds(asm_op: &str, input_count: usize) {
    let inputs = vec![0_u64; input_count];

    for i in 0..input_count {
        let mut i_inputs = inputs.clone();
        // should fail when the value of the input at index i is out of bounds
        i_inputs[i] = U32_BOUND;
        test_execution_failure(asm_op, &i_inputs, "FailedAssertion");
    }
}

/// This helper function tests that when the given u32 assembly instruction is executed on
/// out-of-bounds inputs it does not fail. Each input is tested independently.
fn test_unsafe_execution(asm_op: &str, input_count: usize) {
    let script = compile(format!("begin {} end", asm_op).as_str());
    let values = vec![1_u64; input_count];

    for i in 0..input_count {
        let mut i_values = values.clone();
        // should execute successfully when the value of the input at index i is out of bounds
        i_values[i] = U32_BOUND;
        let inputs = build_inputs(&i_values);
        assert!(execute(&script, &inputs).is_ok());
    }
}
