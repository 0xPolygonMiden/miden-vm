use alloc::string::ToString;

use miden_air::ExecutionOptions;
use rstest::rstest;
use vm_core::{Kernel, StackInputs};

use super::*;
use crate::{DefaultHost, Process};

/// Test a number of combinations of stack inputs and operations to ensure that the fast processor
/// produces the same results as `Process`.
///
/// This creates a test for each element of the cross product of the given stack inputs and
/// operations.
#[rstest]
fn test_basic_block(
    #[values(
        vec![],
        vec![1_u32.into()],
        vec![1_u32.into(), 2_u32.into()],
        vec![1_u32.into(), 2_u32.into(), 3_u32.into()],
        vec![1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into()],
        vec![1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into()],
    )]
    stack_inputs: Vec<Felt>,
    #[values(
        vec![Operation::Add],
        vec![Operation::Swap],
        vec![Operation::Dup1],
    )]
    operations: Vec<Operation>,
) {
    let program = simple_program_with_ops(operations);

    let mut fast_processor = SpeedyGonzales::new(stack_inputs.clone());
    let fast_stack_outputs = fast_processor.execute(&program);

    let mut host = DefaultHost::default();
    let mut slow_processor = Process::new(
        Kernel::default(),
        StackInputs::new(stack_inputs).unwrap(),
        ExecutionOptions::default(),
    );
    let slow_stack_outputs = slow_processor.execute(&program, &mut host);

    match (&fast_stack_outputs, &slow_stack_outputs) {
        (Ok(fast_stack_outputs), Ok(slow_stack_outputs)) => {
            assert_eq!(fast_stack_outputs, slow_stack_outputs);
        },
        (Err(fast_error), Err(slow_error)) => {
            assert_eq!(fast_error.to_string(), slow_error.to_string());
        },
        _ => panic!(
            "Fast processor: {:?}. Slow processor: {:?}",
            fast_stack_outputs, slow_stack_outputs
        ),
    }
}

// TEST HELPERS
// -----------------------------------------------------------------------------------------------

fn simple_program_with_ops(ops: Vec<Operation>) -> Program {
    let program: Program = {
        let mut program = MastForest::new();
        let root_id = program.add_block(ops, None).unwrap();
        program.make_root(root_id);

        Program::new(program.into(), root_id)
    };

    program
}
