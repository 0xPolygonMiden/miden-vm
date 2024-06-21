use super::{Example, ONE, ZERO};
use miden_vm::{math::Felt, Assembler, DefaultHost, MemAdviceProvider, Program, StackInputs};
use processor::MemMastForestStore;

// EXAMPLE BUILDER
// ================================================================================================

pub fn get_example(n: usize) -> Example<DefaultHost<MemAdviceProvider, MemMastForestStore>> {
    // generate the program and expected results
    let program = generate_fibonacci_program(n);
    let expected_result = vec![compute_fibonacci(n)];
    println!(
        "Generated a program to compute {}-th Fibonacci term; expected result: {}",
        n, expected_result[0]
    );

    Example {
        program,
        stack_inputs: StackInputs::try_from_ints([0, 1]).unwrap(),
        host: DefaultHost::default(),
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
        "begin
            repeat.{}
                swap dup.1 add
            end
        end",
        n - 1
    );

    Assembler::default().assemble(program).unwrap()
}

/// Computes the `n`-th term of Fibonacci sequence
fn compute_fibonacci(n: usize) -> Felt {
    let mut t0 = ZERO;
    let mut t1 = ONE;

    for _ in 0..n {
        t1 = t0 + t1;
        core::mem::swap(&mut t0, &mut t1);
    }
    t0
}

// EXAMPLE TESTER
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::{test_example, test_example_with_options};
    use prover::ProvingOptions;

    #[test]
    fn test_fib_example() {
        let example = get_example(16);
        test_example(example, false);
    }

    #[test]
    fn test_fib_example_fail() {
        let example = get_example(16);
        test_example(example, true);
    }

    #[test]
    fn test_fib_example_rpo() {
        let example = get_example(16);
        test_example_with_options(example, false, ProvingOptions::with_96_bit_security(true));
    }
}
