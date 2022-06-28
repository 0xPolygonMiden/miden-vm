use super::Example;
use log::debug;
use super::{assembly::Assembler, Felt, FieldElement, ProgramInputs, StarkField};

// EXAMPLE BUILDER
// ================================================================================================

pub fn get_example(start_value: usize) -> Example {
    // convert starting value of the sequence into a field element
    let start_value = Felt::new(start_value as u64);

    // determine the expected result
    let expected_result = compute_collatz_steps(start_value).as_int();

    // construct the program which executes an unbounded loop to compute a Collatz sequence
    // which starts with the provided value; the output of the program is the number of steps
    // needed to reach the end of the sequence
    let program = Assembler::new().compile_script(
        "
    begin
        pad read dup push.1 ne
        while.true
            swap push.1 add swap dup isodd.128
            if.true
                push.3 mul push.1 add
            else
                push.2 div
            end
            dup push.1 ne
        end
        swap
    end",
    )
    .unwrap();

    debug!(
        "Generated a program to compute Collatz sequence; expected result: {}",
        expected_result
    );

    Example {
        program,
        inputs: ProgramInputs::new(&[], &[start_value.as_int()], (&[]).to_vec()).unwrap(),
        pub_inputs: vec![],
        expected_result: vec![expected_result],
        num_outputs: 1,
    }
}

/// Computes number of steps in a Collatz sequence which starts with the provided `value`.
fn compute_collatz_steps(mut value: Felt) -> Felt {
    let mut i = 0;
    while value != Felt::ONE {
        if value.as_int() & 1 == 0 {
            value /= Felt::new(2);
        } else {
            value = value * Felt::new(3) + Felt::ONE
        }
        i += 1;
    }

    Felt::new(i)
}

// EXAMPLE TESTER
// ================================================================================================

#[test]
fn test_collatz_example() {
    let example = get_example(5);
    super::test_example(example, false);
}

#[test]
fn test_collatz_example_fail() {
    let example = get_example(5);
    super::test_example(example, true);
}
