use crate::Example;
use distaff::{assembly, BaseElement, FieldElement, ProgramInputs, StarkField};

pub fn get_example(start_value: usize) -> Example {
    // convert starting value of the sequence into a field element
    let start_value = BaseElement::new(start_value as u128);

    // determine the expected result
    let expected_result = compute_collatz_steps(start_value);

    // construct the program which executes an unbounded loop to compute a Collatz sequence
    // which starts with the provided value; the output of the program is the number of steps
    // needed to reach the end of the sequence
    let program = assembly::compile(
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

    println!(
        "Generated a program to compute Collatz sequence; expected result: {}",
        expected_result
    );

    // put the starting value as the only secret input for tape A
    let inputs = ProgramInputs::new(&[], &[start_value], &[]);

    // a single element from the top of the stack will be the output
    let num_outputs = 1;

    Example {
        program,
        inputs,
        expected_result: vec![expected_result],
        num_outputs,
    }
}

/// Computes number of steps in a Collatz sequence which starts with the provided `value`.
fn compute_collatz_steps(mut value: BaseElement) -> BaseElement {
    let mut i = 0;
    while value != BaseElement::ONE {
        if value.as_int() & 1 == 0 {
            value /= BaseElement::new(2);
        } else {
            value = value * BaseElement::new(3) + BaseElement::ONE
        }
        i += 1;
    }

    BaseElement::new(i)
}
