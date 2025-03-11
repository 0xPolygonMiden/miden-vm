use alloc::{string::ToString, sync::Arc};

use assembly::{Assembler, DefaultSourceManager};
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
        // Note: we have more specific tests for these below
        vec![Operation::FriE2F4],
        vec![Operation::HornerBase],
        vec![Operation::HornerExt],
    )]
    operations: Vec<Operation>,
) {
    let program = simple_program_with_ops(operations);

    let mut host = DefaultHost::default();
    let fast_processor = SpeedyGonzales::new(stack_inputs.clone());
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

// TODO(plafer): add tests for pipe, adv_pop{w}, emit, mpverify, mrupdate

// TODO(plafer): this test no longer works since we fixed `STACK_BUFFER_SIZE`. Rewrite it.
/// Makes sure that the bounds checking fails when expected.
#[test]
#[ignore]
fn test_stack_overflow_bounds_failure() {
    let mut host = DefaultHost::default();

    // dup1 grows the stack by one, which makes us reach the limit. The subsequent add operation
    // fails.
    {
        let program = simple_program_with_ops(vec![Operation::Dup1, Operation::Add]);
        let err = SpeedyGonzales::new(vec![]).execute(&program, &mut host);
        assert_matches!(err, Err(ExecutionError::FailedToExecuteProgram(_)));
    }

    // add decreases the stack by one, which makes us reach the limit. The subsequent dup1 operation
    // fails.
    {
        let program = simple_program_with_ops(vec![Operation::Add, Operation::Dup1]);
        let err = SpeedyGonzales::new(vec![]).execute(&program, &mut host);
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
        let err = SpeedyGonzales::new(vec![]).execute(&program, &mut host);
        assert_matches!(err, Err(ExecutionError::FailedToExecuteProgram(_)));
    }
}

/// In all these cases, the stack only grows or shrink by 1, and we have 2 spots on each side, so
/// execution never fails.
#[test]
fn test_stack_overflow_bounds_success() {
    let mut host = DefaultHost::default();

    // dup1, add
    {
        let program = simple_program_with_ops(vec![Operation::Dup1, Operation::Add]);
        SpeedyGonzales::new(vec![]).execute(&program, &mut host).unwrap();
    }

    // the first add doesn't change the stack size, but the subsequent dup1 does
    {
        let program =
            simple_program_with_ops(vec![Operation::Add, Operation::Dup1, Operation::Add]);
        SpeedyGonzales::new(vec![]).execute(&program, &mut host).unwrap();
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
        SpeedyGonzales::new(vec![]).execute(&program, &mut host).unwrap();
    }
}

#[test]
fn test_memory_word_access_alignment() {
    let mut host = DefaultHost::default();

    // mloadw
    {
        let program = simple_program_with_ops(vec![Operation::MLoadW]);

        // loadw at address 40 is allowed
        SpeedyGonzales::new(vec![40_u32.into()]).execute(&program, &mut host).unwrap();

        // but loadw at address 43 is not allowed
        let err = SpeedyGonzales::new(vec![43_u32.into()])
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
        SpeedyGonzales::new(vec![40_u32.into()]).execute(&program, &mut host).unwrap();

        // but storew at address 43 is not allowed
        let err = SpeedyGonzales::new(vec![43_u32.into()])
            .execute(&program, &mut host)
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            "word memory access at address 43 in context 0 is unaligned at clock cycle 1"
        );
    }
}

#[test]
fn test_mloadw_success() {
    let mut host = DefaultHost::default();
    let addr = Felt::from(40_u32);
    let word_at_addr = [1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into()];
    let ctx = 0_u32.into();
    let dummy_clk: RowIndex = 0_u32.into();

    // load the contents of address 40
    {
        let mut processor = SpeedyGonzales::new(vec![addr]);
        processor.memory.write_word(ctx, addr, dummy_clk, word_at_addr).unwrap();

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
        let mut processor = SpeedyGonzales::new(vec![100_u32.into()]);
        processor.memory.write_word(ctx, addr, dummy_clk, word_at_addr).unwrap();

        let program = simple_program_with_ops(vec![Operation::MLoadW]);
        let stack_outputs = processor.execute_impl(&program, &mut host).unwrap();

        assert_eq!(stack_outputs.stack_truncated(16), &vec![ZERO; 16]);
    }
}

#[test]
fn test_mstorew_success() {
    let mut host = DefaultHost::default();
    let addr = Felt::from(40_u32);
    let word_to_store = [1_u32.into(), 2_u32.into(), 3_u32.into(), 4_u32.into()];
    let ctx = 0_u32.into();
    let clk = 0_u32.into();

    // Store the word at address 40
    let mut processor = SpeedyGonzales::new(vec![
        word_to_store[0],
        word_to_store[1],
        word_to_store[2],
        word_to_store[3],
        addr,
    ]);
    let program = simple_program_with_ops(vec![Operation::MStoreW]);
    processor.execute_impl(&program, &mut host).unwrap();

    // Ensure that the memory was correctly modified
    assert_eq!(processor.memory.read_word(ctx, addr, clk).unwrap(), &word_to_store);
}

#[rstest]
#[case(40_u32, 42_u32)]
#[case(41_u32, 42_u32)]
#[case(42_u32, 42_u32)]
#[case(43_u32, 42_u32)]
fn test_mstore_success(#[case] addr: u32, #[case] value_to_store: u32) {
    let mut host = DefaultHost::default();
    let ctx = 0_u32.into();
    let clk = 1_u32.into();
    let value_to_store = Felt::from(value_to_store);

    // Store the value at address 40
    let mut processor = SpeedyGonzales::new(vec![value_to_store, addr.into()]);
    let program = simple_program_with_ops(vec![Operation::MStore]);
    processor.execute_impl(&program, &mut host).unwrap();

    // Ensure that the memory was correctly modified
    let word_addr = addr - (addr % WORD_SIZE as u32);
    let word = processor.memory.read_word(ctx, word_addr.into(), clk).unwrap();
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
    let dummy_clk = 0_u32.into();

    // Initialize processor with a word at address 40
    let mut processor = SpeedyGonzales::new(vec![addr_to_access.into()]);
    processor
        .memory
        .write_word(ctx, addr_with_word.into(), dummy_clk, word_at_addr)
        .unwrap();

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

    let mut processor = SpeedyGonzales::new(stack_inputs.clone());
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

    let mut processor = SpeedyGonzales::new(stack_inputs.clone());
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

    let mut processor = SpeedyGonzales::new(stack_inputs.clone());
    processor.fmp = initial_fmp;

    let err = processor.execute(&program, &mut host).unwrap_err();

    // Check that the error is due to the FMP exceeding FMP_MAX
    assert_matches!(err, ExecutionError::InvalidFmpValue(_, _));

    // set the initial FMP to a value close to FMP_MIN
    let initial_fmp = Felt::new(FMP_MIN + 4);
    let stack_inputs = vec![-Felt::new(5_u64)];
    let program = simple_program_with_ops(vec![Operation::FmpUpdate]);

    let mut processor = SpeedyGonzales::new(stack_inputs.clone());
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

        let processor = SpeedyGonzales::new(stack_inputs);
        let result = processor.execute(&program, &mut host);

        // Check that the execution succeeds
        assert!(result.is_ok());
    }

    // Case 2: the stack top is not ONE
    {
        let stack_inputs = vec![ZERO];
        let program = simple_program_with_ops(vec![Operation::Assert(0)]);

        let processor = SpeedyGonzales::new(stack_inputs);
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
    let processor = SpeedyGonzales::new(stack_inputs);
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
    let processor = SpeedyGonzales::new(stack_inputs);
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
    let clk = 1_u32.into();

    let mut processor = {
        let stack_init = {
            let mut stack = vec![ZERO; 16];
            stack[MIN_STACK_DEPTH - 1 - 12] = addr.into();
            stack
        };
        SpeedyGonzales::new(stack_init)
    };
    // Store values at addresses 40 and 44
    processor.memory.write_word(ctx, addr.into(), clk, word_at_addr_40).unwrap();
    processor
        .memory
        .write_word(ctx, (addr + 4).into(), clk, word_at_addr_44)
        .unwrap();

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
    let fast_processor = SpeedyGonzales::new(stack_inputs.clone());
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

// EXECUTION CONTEXT TESTS
// -----------------------------------------------------------------------------------------------

#[test]
fn test_call_node_preserves_stack_overflow() {
    let mut host = DefaultHost::default();

    // equivalent to:
    // proc.foo
    //   add
    // end
    //
    // begin
    //   # stack: [1, 2, 3, 4, ..., 16]
    //   push.10 push.20
    //   # stack: [10, 20, 1, 2, ..., 15, 16], 15 and 16 on overflow
    //   call.foo
    //   # => stack: [30, 1, 2, 3, 4, 5, ..., 14, 0, 15, 16]
    //   swap drop swap drop
    //   # => stack: [30, 3, 4, 5, 6, ..., 14, 0, 15, 16]
    // end
    let program = {
        let mut program = MastForest::new();
        // foo proc
        let foo_id = program.add_block(vec![Operation::Add], None).unwrap();

        // before call
        let push10_push20_id = program
            .add_block(vec![Operation::Push(10_u32.into()), Operation::Push(20_u32.into())], None)
            .unwrap();

        // call
        let call_node_id = program.add_call(foo_id).unwrap();
        // after call
        let swap_drop_swap_drop = program
            .add_block(
                vec![Operation::Swap, Operation::Drop, Operation::Swap, Operation::Drop],
                None,
            )
            .unwrap();

        // joins
        let join_call_swap = program.add_join(call_node_id, swap_drop_swap_drop).unwrap();
        let root_id = program.add_join(push10_push20_id, join_call_swap).unwrap();

        program.make_root(root_id);

        Program::new(program.into(), root_id)
    };

    // initial stack: (top) [1, 2, 3, 4, ..., 16] (bot)
    let mut processor = SpeedyGonzales::new(vec![
        16_u32.into(),
        15_u32.into(),
        14_u32.into(),
        13_u32.into(),
        12_u32.into(),
        11_u32.into(),
        10_u32.into(),
        9_u32.into(),
        8_u32.into(),
        7_u32.into(),
        6_u32.into(),
        5_u32.into(),
        4_u32.into(),
        3_u32.into(),
        2_u32.into(),
        1_u32.into(),
    ]);

    // Execute the program
    let result = processor.execute_impl(&program, &mut host).unwrap();

    assert_eq!(
        result.stack_truncated(16),
        &[
            // the sum from the call to foo
            30_u32.into(),
            // rest of the stack
            3_u32.into(),
            4_u32.into(),
            5_u32.into(),
            6_u32.into(),
            7_u32.into(),
            8_u32.into(),
            9_u32.into(),
            10_u32.into(),
            11_u32.into(),
            12_u32.into(),
            13_u32.into(),
            14_u32.into(),
            // the 0 shifted in during `foo`
            0_u32.into(),
            // the preserved overflow from before the call
            15_u32.into(),
            16_u32.into(),
        ]
    );
}

#[rstest]
// ---- syscalls --------------------------------

// check stack is preserved after syscall
#[case(Some("export.foo add end"), "begin push.1 syscall.foo swap.8 drop end", vec![16_u32.into(); 16])]
// check that `fn_hash` register is updated correctly
#[case(Some("export.foo caller end"), "begin syscall.foo end", vec![16_u32.into(); 16])]
#[case(Some("export.foo caller end"), "proc.bar syscall.foo end begin call.bar end", vec![16_u32.into(); 16])]
// check that clk works correctly through syscalls
#[case(Some("export.foo clk add end"), "begin syscall.foo end", vec![16_u32.into(); 16])]
// check that fmp register is updated correctly after syscall
#[case(Some("export.foo.2 locaddr.0 locaddr.1 swap.8 drop swap.8 drop end"), "proc.bar syscall.foo end begin call.bar end", vec![16_u32.into(); 16])]
// check that memory context is updated correctly across a syscall (i.e. anything stored before the
// syscall is retrievable after, but not during)
#[case(Some("export.foo add end"), "proc.bar push.100 mem_store.44 syscall.foo mem_load.44 swap.8 drop end begin call.bar end", vec![16_u32.into(); 16])]
// check that syscalls share the same memory context
#[case(Some("export.foo push.100 mem_store.44 end export.baz mem_load.44 swap.8 drop end"), 
    "proc.bar syscall.foo syscall.baz end begin call.bar end", vec![16_u32.into(); 16])]
// ---- calls ------------------------

// check stack is preserved after call
#[case(None, "proc.foo add end begin push.1 call.foo swap.8 drop end", vec![16_u32.into(); 16])]
// check that `clk` works correctly though calls
#[case(None, "
    proc.foo clk add end 
    begin push.1 
    if.true call.foo else swap end 
    clk swap.8 drop
    end", 
    vec![16_u32.into(); 16]
)]
// check that fmp register is updated correctly after call
#[case(None,"
    proc.foo.2 locaddr.0 locaddr.1 swap.8 drop swap.8 drop end
    begin call.foo end", 
    vec![16_u32.into(); 16]
)]
// check that 2 functions creating different memory contexts don't interfere with each other
#[case(None,"
    proc.foo push.100 mem_store.44 end
    proc.bar mem_load.44 assertz end
    begin call.foo mem_load.44 assertz call.bar end", 
    vec![16_u32.into(); 16]
)]
// check that memory context is updated correctly across a call (i.e. anything stored before the
// call is retrievable after, but not during)
#[case(None,"
    proc.foo mem_load.44 assertz end
    proc.bar push.100 mem_store.44 call.foo mem_load.44 swap.8 drop end
    begin call.bar end", 
    vec![16_u32.into(); 16]
)]
// ---- dyncalls ------------------------

// check stack is preserved after dyncall
#[case(None, "
    proc.foo add end 
    begin 
        procref.foo mem_storew.100 dropw push.100
        dyncall swap.8 drop 
    end", 
    vec![16_u32.into(); 16]
)]
// check that `clk` works correctly though dyncalls
#[case(None, "
    proc.foo clk add end 
    begin 
        push.1 
        if.true 
            procref.foo mem_storew.100 dropw
            push.100 dyncall
            push.100 dyncall
        else 
            swap 
        end 
        clk swap.8 drop
    end", 
    vec![16_u32.into(); 16]
)]
// check that fmp register is updated correctly after dyncall
#[case(None,"
    proc.foo.2 locaddr.0 locaddr.1 swap.8 drop swap.8 drop end
    begin 
        procref.foo mem_storew.100 dropw push.100
        dyncall
    end", 
    vec![16_u32.into(); 16]
)]
// check that 2 functions creating different memory contexts don't interfere with each other
#[case(None,"
    proc.foo push.100 mem_store.44 end
    proc.bar mem_load.44 assertz end
    begin 
        procref.foo mem_storew.100 dropw push.100 dyncall
        mem_load.44 assertz 
        procref.bar mem_storew.104 dropw push.104 dyncall
    end", 
    vec![16_u32.into(); 16]
)]
// check that memory context is updated correctly across a dyncall (i.e. anything stored before the
// call is retrievable after, but not during)
#[case(None,"
    proc.foo mem_load.44 assertz end
    proc.bar 
        push.100 mem_store.44 
        procref.foo mem_storew.104 dropw push.104 dyncall
        mem_load.44 swap.8 drop 
    end
    begin 
        procref.bar mem_storew.104 dropw push.104 dyncall
    end", 
    vec![16_u32.into(); 16]
)]
// ---- dyn ------------------------

// check stack is preserved after dynexec
#[case(None, "
    proc.foo add end 
    begin 
        procref.foo mem_storew.100 dropw push.100
        dynexec swap.8 drop 
    end", 
    vec![16_u32.into(); 16]
)]
// check that `clk` works correctly though dynexecs
#[case(None, "
    proc.foo clk add end 
    begin 
        push.1 
        if.true 
            procref.foo mem_storew.100 dropw
            push.100 dynexec
            push.100 dynexec
        else 
            swap 
        end 
        clk swap.8 drop
    end", 
    vec![16_u32.into(); 16]
)]
// check that fmp register is updated correctly after dynexec
#[case(None,"
    proc.foo.2 locaddr.0 locaddr.1 swap.8 drop swap.8 drop end
    begin 
        procref.foo mem_storew.100 dropw push.100
        dynexec
    end", 
    vec![16_u32.into(); 16]
)]
// check that dynexec doesn't create a new memory context
#[case(None,"
    proc.foo push.100 mem_store.44 end
    proc.bar mem_load.44 sub.100 assertz end
    begin 
        procref.foo mem_storew.104 dropw push.104 dynexec
        mem_load.44 sub.100 assertz 
        procref.bar mem_storew.108 dropw push.108 dynexec
    end", 
    vec![16_u32.into(); 16]
)]
// ---- loop --------------------------------

// check that the loop is never entered if the condition is false (and that clk is properly updated)
#[case(None, "begin while.true push.1 assertz end clk swap.8 drop end", vec![3_u32.into(), 2_u32.into(), 1_u32.into(), ZERO])]
// check that the loop is entered if the condition is true, and that the stack and clock are managed
// properly
#[case(None,
    "begin 
        while.true 
            clk swap.15 drop
        end 
        clk swap.8 drop 
        end",
    vec![42_u32.into(), ZERO, ONE, ONE, ONE, ONE]
)]
// ---- horner ops --------------------------------
#[case(None,
    "begin 
        push.1.2.3.4 mem_storew.40 dropw
        horner_eval_base
        end",
    // first 3 addresses in the vec are the alpha_ptr, acc_high and acc_low, respectively.
    vec![100_u32.into(), 4_u32.into(), 40_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(),
        8_u32.into(), 9_u32.into(), 10_u32.into(), 11_u32.into(), 12_u32.into(), 13_u32.into(),
        14_u32.into(), 15_u32.into(), 16_u32.into()]
)]
#[case(None,
    "begin 
        push.1.2.3.4 mem_storew.40 dropw
        horner_eval_ext
        end",
    // first 3 addresses in the vec are the alpha_ptr, acc_high and acc_low, respectively.
    vec![100_u32.into(), 4_u32.into(), 40_u32.into(), 4_u32.into(), 5_u32.into(), 6_u32.into(), 7_u32.into(),
        8_u32.into(), 9_u32.into(), 10_u32.into(), 11_u32.into(), 12_u32.into(), 13_u32.into(),
        14_u32.into(), 15_u32.into(), 16_u32.into()]
)]
fn test_masm_consistency(
    #[case] kernel_source: Option<&'static str>,
    #[case] program_source: &'static str,
    #[case] stack_inputs: Vec<Felt>,
) {
    let (program, kernel_lib) = {
        let source_manager = Arc::new(DefaultSourceManager::default());

        match kernel_source {
            Some(kernel_source) => {
                let kernel_lib =
                    Assembler::new(source_manager.clone()).assemble_kernel(kernel_source).unwrap();
                let program = Assembler::with_kernel(source_manager, kernel_lib.clone())
                    .assemble_program(program_source)
                    .unwrap();

                (program, Some(kernel_lib))
            },
            None => {
                let program =
                    Assembler::new(source_manager).assemble_program(program_source).unwrap();
                (program, None)
            },
        }
    };

    let mut host = DefaultHost::default();
    if let Some(kernel_lib) = &kernel_lib {
        host.load_mast_forest(kernel_lib.mast_forest().clone()).unwrap();
    }

    // fast processor
    let processor = SpeedyGonzales::new(stack_inputs.clone());
    let fast_stack_outputs = processor.execute(&program, &mut host).unwrap();

    // slow processor
    let mut slow_processor = Process::new(
        kernel_lib.map(|k| k.kernel().clone()).unwrap_or_default(),
        StackInputs::new(stack_inputs).unwrap(),
        ExecutionOptions::default(),
    );
    let slow_stack_outputs = slow_processor.execute(&program, &mut host).unwrap();

    assert_eq!(fast_stack_outputs, slow_stack_outputs);
}

/// Tests that emitted errors are consistent between the fast and slow processors.
#[rstest]
// check that error is returned if condition is not a boolean
#[case(None, "begin while.true swap end end", vec![2_u32.into(); 16])]
#[case(None, "begin while.true push.100 end end", vec![ONE; 16])]
// check that dynamically calling a hash that doesn't exist fails
#[case(None,"
    begin 
        dyncall
    end", 
    vec![16_u32.into(); 16]
)]
// check that dynamically calling a hash that doesn't exist fails
#[case(None,"
    begin 
        dynexec
    end", 
    vec![16_u32.into(); 16]
)]
fn test_masm_errors_consistency(
    #[case] kernel_source: Option<&'static str>,
    #[case] program_source: &'static str,
    #[case] stack_inputs: Vec<Felt>,
) {
    let (program, kernel_lib) = {
        let source_manager = Arc::new(DefaultSourceManager::default());

        match kernel_source {
            Some(kernel_source) => {
                let kernel_lib =
                    Assembler::new(source_manager.clone()).assemble_kernel(kernel_source).unwrap();
                let program = Assembler::with_kernel(source_manager, kernel_lib.clone())
                    .assemble_program(program_source)
                    .unwrap();

                (program, Some(kernel_lib))
            },
            None => {
                let program =
                    Assembler::new(source_manager).assemble_program(program_source).unwrap();
                (program, None)
            },
        }
    };

    let mut host = DefaultHost::default();
    if let Some(kernel_lib) = &kernel_lib {
        host.load_mast_forest(kernel_lib.mast_forest().clone()).unwrap();
    }

    // fast processor
    let processor = SpeedyGonzales::new(stack_inputs.clone());
    let fast_stack_outputs = processor.execute(&program, &mut host).unwrap_err();

    // slow processor
    let mut slow_processor = Process::new(
        kernel_lib.map(|k| k.kernel().clone()).unwrap_or_default(),
        StackInputs::new(stack_inputs).unwrap(),
        ExecutionOptions::default(),
    );
    let slow_stack_outputs = slow_processor.execute(&program, &mut host).unwrap_err();

    assert_eq!(fast_stack_outputs.to_string(), slow_stack_outputs.to_string());
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
