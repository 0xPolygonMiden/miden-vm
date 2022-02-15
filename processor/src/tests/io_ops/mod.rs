use super::{
    super::ExecutionTrace, super::Process, super::ProgramInputs, super::Script, build_inputs,
    compile, convert_to_stack, execute, push_to_stack, test_execution_failure, test_op_execution,
    test_script_execution, test_script_execution_failure, Felt,
};

mod adv_ops;
mod constant_ops;
mod env_ops;
mod local_ops;
mod mem_ops;

// HELPER FUNCTIONS
// ================================================================================================

fn test_memory_write(
    script: &Script,
    stack_inputs: &[u64],
    final_stack: &[u64],
    mem_addr: u64,
    expected_mem: &[u64],
) {
    let inputs = build_inputs(stack_inputs);
    let mut process = Process::new(inputs);

    // execute the test
    process.execute_code_block(script.root()).unwrap();

    // validate the memory state
    let mem_state = process.memory.get_value(mem_addr).unwrap();
    let expected_mem: Vec<Felt> = expected_mem.iter().map(|&v| Felt::new(v)).collect();
    assert_eq!(expected_mem, mem_state);

    // validate the stack state
    let stack_state = ExecutionTrace::new(process).last_stack_state();
    let expected_stack = convert_to_stack(final_stack);
    assert_eq!(expected_stack, stack_state);
}
