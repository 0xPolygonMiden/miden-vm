use crate::Example;
use log::debug;
use miden::{assembly, BaseElement, FieldElement, Program, ProgramInputs, StarkField};

// EXAMPLE BUILDER
// ================================================================================================

pub fn get_example(n: usize) -> Example {
    // generate the program and expected results
    let program = generate_fibonacci_program(n);
    let expected_result = vec![compute_fibonacci(n).as_int()];
    debug!(
        "Generated a program to compute {}-th Fibonacci term; expected result: {}",
        n, expected_result[0]
    );

    Example {
        program,
        inputs: ProgramInputs::from_public(&[1, 0]),
        pub_inputs: vec![1, 0],
        expected_result,
        num_outputs: 1,
    }
}

/// Generates a program to compute the `n`-th term of Fibonacci sequence
fn generate_fibonacci_program(n: usize) -> Program {
    // the program is a simple repetition of 4 stack operations:
    // the first operation moves the 2nd stack item to the top,
    // the second operation duplicates the top 2 stack items,
    // the third operation removes the top item from the stack
    // the last operation pops top 2 stack items, adds them, and pushes
    // the result back onto the stack
    let program = format!(
        "
    begin 
        repeat.{}
            swap dup.2 drop add
        end
    end",
        n - 1
    );

    assembly::compile(&program).unwrap()
}

/// Computes the `n`-th term of Fibonacci sequence
fn compute_fibonacci(n: usize) -> BaseElement {
    let mut n1 = BaseElement::ZERO;
    let mut n2 = BaseElement::ONE;

    for _ in 0..(n - 1) {
        let n3 = n1 + n2;
        n1 = n2;
        n2 = n3;
    }

    n2
}

// EXAMPLE TESTER
// ================================================================================================

#[test]
fn test_fib_example() {
    let example = get_example(16);
    super::test_example(example, false);
}

#[test]
fn test_fib_example_fail() {
    let example = get_example(16);
    super::test_example(example, true);
}
