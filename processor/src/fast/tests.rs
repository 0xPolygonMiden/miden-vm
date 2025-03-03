use alloc::string::ToString;

use miden_air::ExecutionOptions;
use rstest::rstest;
use vm_core::{assert_matches, Kernel, StackInputs};

use super::*;
use crate::{system::FMP_MAX, DefaultHost, Process};

// TODO(plafer): add prop tests to try to make the stack overflow logic fail (e.g. an out of bounds
// stack access).

/// Test a number of combinations of stack inputs and operations to ensure that the fast processor
/// produces the same results as `Process`.
///
/// This creates a test for each element of the cross product of the given stack inputs and
/// operations.
#[rstest]
fn test_basic_block(
    #[values(
        vec![],
        vec![0_u32.into()],
        vec![0_u32.into(), 1_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into(), 9_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into(), 9_u32.into(), 10_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into(), 9_u32.into(), 10_u32.into(), 11_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into(), 9_u32.into(), 10_u32.into(), 11_u32.into(), 12_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into(), 9_u32.into(), 10_u32.into(), 11_u32.into(), 12_u32.into(), 13_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into(), 9_u32.into(), 10_u32.into(), 11_u32.into(), 12_u32.into(), 13_u32.into(), 14_u32.into()],
        vec![0_u32.into(), 1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into(), 9_u32.into(), 10_u32.into(), 11_u32.into(), 12_u32.into(), 13_u32.into(), 14_u32.into(), 15_u32.into()],
    )]
    stack_inputs: Vec<Felt>,
    #[values(
        // clk
        vec![Operation::Noop, Operation::Noop, Operation::Clk, Operation::MovUp8, Operation::Drop],
        vec![Operation::Add],
        vec![Operation::Swap],
        // We want SDepth to output "17", and then drop 2 elements from somewhere else in the stack.
        vec![Operation::Dup0, Operation::SDepth, Operation::MovUp8, Operation::Drop,Operation::MovUp8, Operation::Drop],
        vec![Operation::Neg],
        vec![Operation::Mul],
        vec![Operation::Inv],
        vec![Operation::Incr],
        vec![Operation::And],
        vec![Operation::Or],
        vec![Operation::Not],
        vec![Operation::Eq],
        vec![Operation::Eqz],
        vec![Operation::Expacc],
        vec![Operation::Ext2Mul],
        vec![Operation::U32split, Operation::MovUp8, Operation::Drop],
        vec![Operation::U32add],
        vec![Operation::U32add3],
        vec![Operation::U32mul],
        vec![Operation::U32sub],
        vec![Operation::U32div],
        vec![Operation::U32and],
        vec![Operation::U32xor],
        vec![Operation::U32madd],
        vec![Operation::U32assert2(5)],
        vec![Operation::Pad, Operation::MovUp8, Operation::Drop],
        // for the dups, we drop an element that was not duplicated, and hence we are still testing
        // that the `dup` works as expected
        vec![Operation::Dup0, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup1, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup2, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup3, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup4, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup5, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup6, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup7, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup9, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup11, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup13, Operation::MovUp8, Operation::Drop],
        vec![Operation::Dup15, Operation::MovUp8, Operation::Drop],

        vec![Operation::SwapW],
        vec![Operation::SwapW2],
        vec![Operation::SwapW3],
        vec![Operation::SwapDW],
        vec![Operation::MovUp2],
        vec![Operation::MovUp3],
        vec![Operation::MovUp4],
        vec![Operation::MovUp5],
        vec![Operation::MovUp6],
        vec![Operation::MovUp7],
        vec![Operation::MovUp8],
        vec![Operation::MovDn2],
        vec![Operation::MovDn3],
        vec![Operation::MovDn4],
        vec![Operation::MovDn5],
        vec![Operation::MovDn6],
        vec![Operation::MovDn7],
        vec![Operation::MovDn8],
        vec![Operation::CSwap],
        vec![Operation::CSwapW],
        vec![Operation::Push(42_u32.into()), Operation::MovUp8, Operation::Drop],
        // the memory operations here are more to ensure e.g. that unaligned word accesses are
        // reported correctly.
        vec![Operation::MLoadW],
        vec![Operation::MStoreW],
        vec![Operation::MLoad],
        vec![Operation::MStore],
        vec![Operation::MStream],
        // crypto ops
        vec![Operation::HPerm],
        // Note: we have another more specific test for `FriE2F4`
        vec![Operation::FriE2F4],
    )]
    operations: Vec<Operation>,
) {
    let program = simple_program_with_ops(operations);

    let mut host = DefaultHost::default();
    let fast_processor = SpeedyGonzales::<512>::new(stack_inputs.clone());
    let fast_stack_outputs = fast_processor.execute(&program, &mut host);

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

            // Make sure that we're not getting an output stack overflow error, as it indicates that
            // the sequence of operations makes the stack end with a non-16 depth, and doesn't tell
            // us if the stack outputs are actually the same.
            if matches!(fast_error, ExecutionError::OutputStackOverflow(_)) {
                panic!("we don't want to be testing this output stack overflow error");
            }
        },
        _ => panic!(
            "Fast processor: {:?}. Slow processor: {:?}",
            fast_stack_outputs, slow_stack_outputs
        ),
    }
}

/// Makes sure that the bounds checking fails when expected.
#[test]
fn test_stack_overflow_bounds_failure() {
    const STACK_DEPTH: usize = MIN_STACK_DEPTH + 2;
    let mut host = DefaultHost::default();

    // dup1 grows the stack by one, which makes us reach the limit. The subsequent add operation
    // fails.
    {
        let program = simple_program_with_ops(vec![Operation::Dup1, Operation::Add]);
        let err = SpeedyGonzales::<STACK_DEPTH>::new(vec![]).execute(&program, &mut host);
        assert_matches!(err, Err(ExecutionError::FailedToExecuteProgram(_)));
    }

    // add decreases the stack by one, which makes us reach the limit. The subsequent dup1 operation
    // fails.
    {
        let program = simple_program_with_ops(vec![Operation::Add, Operation::Dup1]);
        let err = SpeedyGonzales::<STACK_DEPTH>::new(vec![]).execute(&program, &mut host);
        assert_matches!(err, Err(ExecutionError::FailedToExecuteProgram(_)));
    }

    // The initial swaps don't change the stack size, but the subsequent add operation makes us
    // reach the limit, such that dup1 fails.
    {
        let program = simple_program_with_ops(vec![
            Operation::Swap,
            Operation::Swap,
            Operation::Swap,
            Operation::Swap,
            Operation::Add,
            Operation::Dup1,
        ]);
        let err = SpeedyGonzales::<STACK_DEPTH>::new(vec![]).execute(&program, &mut host);
        assert_matches!(err, Err(ExecutionError::FailedToExecuteProgram(_)));
    }
}

/// In all these cases, the stack only grows or shrink by 1, and we have 2 spots on each side, so
/// execution never fails.
#[test]
fn test_stack_overflow_bounds_success() {
    const STACK_DEPTH: usize = MIN_STACK_DEPTH + 4;
    let mut host = DefaultHost::default();

    // dup1, add
    {
        let program = simple_program_with_ops(vec![Operation::Dup1, Operation::Add]);
        SpeedyGonzales::<STACK_DEPTH>::new(vec![]).execute(&program, &mut host).unwrap();
    }

    // the first add doesn't change the stack size, but the subsequent dup1 does
    {
        let program =
            simple_program_with_ops(vec![Operation::Add, Operation::Dup1, Operation::Add]);
        SpeedyGonzales::<STACK_DEPTH>::new(vec![]).execute(&program, &mut host).unwrap();
    }

    // alternating add/dup1, with some swaps which don't change the stack size.
    // Note that add when stack size is 16 doesn't reduce the stack size.
    {
        let program = simple_program_with_ops(vec![
            // stack depth after: 16
            Operation::Add,
            // stack depth after: 17
            Operation::Dup1,
            // stack depth after: 16
            Operation::Add,
            // stack depth after: 17
            Operation::Dup1,
            // stack depth after: 17
            Operation::Swap,
            // stack depth after: 16
            Operation::Add,
            // stack depth after: 17
            Operation::Dup1,
            // stack depth after: 16
            Operation::Add,
            // stack depth after: 16
            Operation::Swap,
        ]);
        SpeedyGonzales::<STACK_DEPTH>::new(vec![]).execute(&program, &mut host).unwrap();
    }
}

#[test]
fn test_memory_word_access_alignment() {
    let mut host = DefaultHost::default();

    // mloadw
    {
        let program = simple_program_with_ops(vec![Operation::MLoadW]);

        // loadw at address 40 is allowed
        SpeedyGonzales::<512>::new(vec![40_u32.into()])
            .execute(&program, &mut host)
            .unwrap();

        // but loadw at address 43 is not allowed
        let err = SpeedyGonzales::<512>::new(vec![43_u32.into()])
            .execute(&program, &mut host)
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            "word memory access at address 43 in context 0 is unaligned at clock cycle 1"
        );
    }

    // mstorew
    {
        let program = simple_program_with_ops(vec![Operation::MStoreW]);

        // storew at address 40 is allowed
        SpeedyGonzales::<512>::new(vec![40_u32.into()])
            .execute(&program, &mut host)
            .unwrap();

        // but storew at address 43 is not allowed
        let err = SpeedyGonzales::<512>::new(vec![43_u32.into()])
            .execute(&program, &mut host)
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            "word memory access at address 43 in context 0 is unaligned at clock cycle 1"
        );
    }
}

// TODO(plafer): test that memory operations work just like Processor (i.e. like basic_block_test,
// but for memory things)

#[test]
fn test_mloadw_success() {
    let mut host = DefaultHost::default();
    let addr = 40_u32;
    let word_at_addr = [1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into()];
    let ctx = 0_u32.into();

    // load the contents of address 40
    {
        let mut processor = SpeedyGonzales::<512>::new(vec![addr.into()]);
        processor.memory.insert((ctx, addr), word_at_addr);

        let program = simple_program_with_ops(vec![Operation::MLoadW]);
        let stack_outputs = processor.execute_impl(&program, &mut host).unwrap();

        assert_eq!(
            stack_outputs.stack_truncated(4),
            // memory order is the reverse from the stack order
            &[word_at_addr[3], word_at_addr[2], word_at_addr[1], word_at_addr[0]]
        );
    }

    // load the contents of address 100 (should yield the ZERO word)
    {
        let mut processor = SpeedyGonzales::<512>::new(vec![100_u32.into()]);
        processor.memory.insert((ctx, addr), word_at_addr);

        let program = simple_program_with_ops(vec![Operation::MLoadW]);
        let stack_outputs = processor.execute_impl(&program, &mut host).unwrap();

        assert_eq!(stack_outputs.stack_truncated(16), &vec![ZERO; 16]);
    }
}

#[test]
fn test_mstorew_success() {
    let mut host = DefaultHost::default();
    let addr = 40_u32;
    let word_to_store = [1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into()];
    let ctx = 0_u32.into();

    // Store the word at address 40
    let mut processor = SpeedyGonzales::<512>::new(vec![
        word_to_store[0],
        word_to_store[1],
        word_to_store[2],
        word_to_store[3],
        addr.into(),
    ]);
    let program = simple_program_with_ops(vec![Operation::MStoreW]);
    processor.execute_impl(&program, &mut host).unwrap();

    // Ensure that the memory was correctly modified
    assert_eq!(processor.memory.get(&(ctx, addr)).copied().unwrap(), word_to_store);
}

#[rstest]
#[case(40_u32, 42_u32)]
#[case(41_u32, 42_u32)]
#[case(42_u32, 42_u32)]
#[case(43_u32, 42_u32)]
fn test_mstore_success(#[case] addr: u32, #[case] value_to_store: u32) {
    let mut host = DefaultHost::default();
    let ctx = 0_u32.into();
    let value_to_store = Felt::from(value_to_store);

    // Store the value at address 40
    let mut processor = SpeedyGonzales::<512>::new(vec![value_to_store, addr.into()]);
    let program = simple_program_with_ops(vec![Operation::MStore]);
    processor.execute_impl(&program, &mut host).unwrap();

    // Ensure that the memory was correctly modified
    let word_addr = addr - (addr % WORD_SIZE as u32);
    let word = processor.memory.get(&(ctx, word_addr)).copied().unwrap();
    assert_eq!(word[addr as usize % WORD_SIZE], value_to_store);
}

#[rstest]
#[case(40_u32)]
#[case(41_u32)]
#[case(42_u32)]
#[case(43_u32)]
fn test_mload_success(#[case] addr_to_access: u32) {
    let mut host = DefaultHost::default();
    let addr_with_word = 40_u32;
    let word_at_addr = [1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into()];
    let ctx = 0_u32.into();

    // Initialize processor with a word at address 40
    let mut processor = SpeedyGonzales::<512>::new(vec![addr_to_access.into()]);
    processor.memory.insert((ctx, addr_with_word), word_at_addr);

    let program = simple_program_with_ops(vec![Operation::MLoad]);
    let stack_outputs = processor.execute_impl(&program, &mut host).unwrap();

    // Ensure that Operation::MLoad correctly reads the value on the stack
    assert_eq!(
        stack_outputs.stack_truncated(1)[0],
        word_at_addr[addr_to_access as usize % WORD_SIZE]
    );
}

#[test]
fn test_fmp_add() {
    let mut host = DefaultHost::default();

    // set the initial FMP to a different value than the default
    let initial_fmp = Felt::new(FMP_MIN + 4);
    let stack_inputs = vec![1_u32.into(), 2_u32.into(), 3_u32.into()];
    let program = simple_program_with_ops(vec![Operation::FmpAdd]);

    let mut processor = SpeedyGonzales::<512>::new(stack_inputs.clone());
    processor.fmp = initial_fmp;

    let stack_outputs = processor.execute(&program, &mut host).unwrap();

    // Check that the top of the stack is the sum of the initial FMP and the top of the stack input
    let expected_top = initial_fmp + stack_inputs[2];
    assert_eq!(stack_outputs.stack_truncated(1)[0], expected_top);
}

#[test]
fn test_fmp_update() {
    let mut host = DefaultHost::default();

    // set the initial FMP to a different value than the default
    let initial_fmp = Felt::new(FMP_MIN + 4);
    let stack_inputs = vec![5_u32.into()];
    let program = simple_program_with_ops(vec![Operation::FmpUpdate]);

    let mut processor = SpeedyGonzales::<512>::new(stack_inputs.clone());
    processor.fmp = initial_fmp;

    let stack_outputs = processor.execute_impl(&program, &mut host).unwrap();

    // Check that the FMP is updated correctly
    let expected_fmp = initial_fmp + stack_inputs[0];
    assert_eq!(processor.fmp, expected_fmp);

    // Check that the top of the stack is popped correctly
    assert_eq!(stack_outputs.stack_truncated(0).len(), 0);
}

#[test]
fn test_fmp_update_fail() {
    let mut host = DefaultHost::default();

    // set the initial FMP to a value close to FMP_MAX
    let initial_fmp = Felt::new(FMP_MAX - 4);
    let stack_inputs = vec![5_u32.into()];
    let program = simple_program_with_ops(vec![Operation::FmpUpdate]);

    let mut processor = SpeedyGonzales::<512>::new(stack_inputs.clone());
    processor.fmp = initial_fmp;

    let err = processor.execute(&program, &mut host).unwrap_err();

    // Check that the error is due to the FMP exceeding FMP_MAX
    assert_matches!(err, ExecutionError::InvalidFmpValue(_, _));

    // set the initial FMP to a value close to FMP_MIN
    let initial_fmp = Felt::new(FMP_MIN + 4);
    let stack_inputs = vec![-Felt::new(5_u64)];
    let program = simple_program_with_ops(vec![Operation::FmpUpdate]);

    let mut processor = SpeedyGonzales::<512>::new(stack_inputs.clone());
    processor.fmp = initial_fmp;

    let err = processor.execute(&program, &mut host).unwrap_err();

    // Check that the error is due to the FMP being less than FMP_MIN
    assert_matches!(err, ExecutionError::InvalidFmpValue(_, _));
}

#[test]
fn test_assert() {
    let mut host = DefaultHost::default();

    // Case 1: the stack top is ONE
    {
        let stack_inputs = vec![ONE];
        let program = simple_program_with_ops(vec![Operation::Assert(0)]);

        let processor = SpeedyGonzales::<512>::new(stack_inputs);
        let result = processor.execute(&program, &mut host);

        // Check that the execution succeeds
        assert!(result.is_ok());
    }

    // Case 2: the stack top is not ONE
    {
        let stack_inputs = vec![ZERO];
        let program = simple_program_with_ops(vec![Operation::Assert(0)]);

        let processor = SpeedyGonzales::<512>::new(stack_inputs);
        let err = processor.execute(&program, &mut host).unwrap_err();

        // Check that the error is due to a failed assertion
        assert_matches!(err, ExecutionError::FailedAssertion { .. });
    }
}

/// Tests all valid inputs for the `And` operation.
///
/// The `test_basic_block()` test already covers the case where the stack top doesn't contain binary
/// values.
#[rstest]
#[case(vec![ZERO, ZERO], ZERO)]
#[case(vec![ZERO, ONE], ZERO)]
#[case(vec![ONE, ZERO], ZERO)]
#[case(vec![ONE, ONE], ONE)]
fn test_valid_combinations_and(#[case] stack_inputs: Vec<Felt>, #[case] expected_output: Felt) {
    let program = simple_program_with_ops(vec![Operation::And]);

    let mut host = DefaultHost::default();
    let processor = SpeedyGonzales::<512>::new(stack_inputs);
    let stack_outputs = processor.execute(&program, &mut host).unwrap();

    assert_eq!(stack_outputs.stack_truncated(1)[0], expected_output);
}

/// Tests all valid inputs for the `Or` operation.
///
/// The `test_basic_block()` test already covers the case where the stack top doesn't contain binary
/// values.
#[rstest]
#[case(vec![ZERO, ZERO], ZERO)]
#[case(vec![ZERO, ONE], ONE)]
#[case(vec![ONE, ZERO], ONE)]
#[case(vec![ONE, ONE], ONE)]
fn test_valid_combinations_or(#[case] stack_inputs: Vec<Felt>, #[case] expected_output: Felt) {
    let program = simple_program_with_ops(vec![Operation::Or]);

    let mut host = DefaultHost::default();
    let processor = SpeedyGonzales::<512>::new(stack_inputs);
    let stack_outputs = processor.execute(&program, &mut host).unwrap();

    assert_eq!(stack_outputs.stack_truncated(1)[0], expected_output);
}

#[test]
fn test_mstream() {
    let mut host = DefaultHost::default();
    let addr = 40_u32;
    let word_at_addr_40 = [1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into()];
    let word_at_addr_44 = [5_u32.into(), 6_u32.into(), 7_u32.into(), 8_u32.into()];
    let ctx = 0_u32.into();

    let mut processor = {
        let stack_init = {
            let mut stack = vec![ZERO; 16];
            stack[MIN_STACK_DEPTH - 1 - 12] = addr.into();
            stack
        };
        SpeedyGonzales::<512>::new(stack_init)
    };
    // Store values at addresses 40 and 44
    processor.memory.insert((ctx, addr), word_at_addr_40);
    processor.memory.insert((ctx, addr + 4), word_at_addr_44);

    let program = simple_program_with_ops(vec![Operation::MStream]);
    let stack_outputs = processor.execute_impl(&program, &mut host).unwrap();

    // Ensure that Operation::MStream correctly reads the values on the stack
    assert_eq!(
        stack_outputs.stack_truncated(8),
        // memory order is the reverse from the stack order
        &[
            word_at_addr_44[3],
            word_at_addr_44[2],
            word_at_addr_44[1],
            word_at_addr_44[0],
            word_at_addr_40[3],
            word_at_addr_40[2],
            word_at_addr_40[1],
            word_at_addr_40[0]
        ]
    );
}

/// Tests a valid set of inputs for the `Frie2f4` operation. This test reuses most of the logic of
/// `op_fri_ext2fold4` in `Process`.
#[test]
fn test_frie2f4() {
    let mut host = DefaultHost::default();

    // --- build stack inputs ---------------------------------------------
    let previous_value = [10_u32.into(), 11_u32.into()];
    let stack_inputs = vec![
        1_u32.into(),
        2_u32.into(),
        3_u32.into(),
        4_u32.into(),
        previous_value[0], // 4: 3rd query value and "previous value" (idx 13) must be the same
        previous_value[1], // 5: 3rd query value and "previous value" (idx 13) must be the same
        7_u32.into(),
        2_u32.into(), //7: domain segment, < 4
        9_u32.into(),
        10_u32.into(),
        11_u32.into(),
        12_u32.into(),
        13_u32.into(),
        previous_value[0], // 13: previous value
        previous_value[1], // 14: previous value
        16_u32.into(),
    ];

    let program =
        simple_program_with_ops(vec![Operation::Push(Felt::new(42_u64)), Operation::FriE2F4]);

    // fast processor
    let fast_processor = SpeedyGonzales::<512>::new(stack_inputs.clone());
    let fast_stack_outputs = fast_processor.execute(&program, &mut host).unwrap();

    // slow processor
    let mut slow_processor = Process::new(
        Kernel::default(),
        StackInputs::new(stack_inputs).unwrap(),
        ExecutionOptions::default(),
    );
    let slow_stack_outputs = slow_processor.execute(&program, &mut host).unwrap();

    assert_eq!(fast_stack_outputs, slow_stack_outputs);
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
