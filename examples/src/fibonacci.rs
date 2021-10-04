use super::{assembly, utils::parse_args, Example, Program, ProgramInputs};
use distaff::{BaseElement, FieldElement};

pub fn get_example(args: &[String]) -> Example {
    // get the length of Fibonacci sequence and proof options from the arguments
    let (n, options) = parse_args(args);

    // generate the program and expected results
    let program = generate_fibonacci_program(n);
    let expected_result = vec![compute_fibonacci(n)];
    println!(
        "Generated a program to compute {}-th Fibonacci term; expected result: {}",
        n, expected_result[0]
    );

    // initialize stack with 2 values; 1 will be at the top
    let inputs = ProgramInputs::from_public(&[BaseElement::ONE, BaseElement::ZERO]);

    // a single element from the top of the stack will be the output
    let num_outputs = 1;

    Example {
        program,
        inputs,
        options,
        expected_result,
        num_outputs,
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
