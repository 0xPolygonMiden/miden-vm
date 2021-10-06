use crate::Example;
use distaff::{assembly, BaseElement, FieldElement, Program, ProgramInputs, StarkField};
use winter_rand_utils::rand_vector;

pub fn get_example(num_values: usize) -> Example {
    // generate random sequence of 64-bit values
    let values = generate_values(num_values);

    // generate the program and expected results
    let program = generate_range_check_program(num_values);
    let expected_result = vec![count_63_bit_values(&values)];
    println!(
        "Generated a program to range-check {} values; expected result: {}",
        num_values, expected_result[0]
    );

    // set public inputs to the initial sum (0), and pass values to the secret tape A
    let inputs = ProgramInputs::new(&[BaseElement::ZERO], &values, &[]);

    // a single element from the top of the stack will be the output
    let num_outputs = 1;

    Example {
        program,
        inputs,
        expected_result,
        num_outputs,
    }
}

/// Generates a random sequence of 64-bit values.
fn generate_values(n: usize) -> Vec<BaseElement> {
    let mut values = rand_vector::<BaseElement>(n);
    for value in values.iter_mut() {
        *value = BaseElement::new((value.as_int() as u64) as u128);
    }
    values
}

/// Generates a program to range-check a sequence of values.
fn generate_range_check_program(n: usize) -> Program {
    let mut program = String::with_capacity(n * 80);
    program.push_str("begin ");

    // repeat the cycle of the following operations:
    // 1. read a value from secret tape A
    // 2. check if it fits into 63 bits (result is 1 if true, 0 otherwise)
    // 3. add the result into the running sum
    for _ in 0..n {
        program.push_str("read rc.63 add ");
    }
    program.push_str("end");

    assembly::compile(&program).unwrap()
}

/// Counts the number of values smaller than 63-bits in size.
fn count_63_bit_values(values: &[BaseElement]) -> BaseElement {
    let p63 = BaseElement::new(2).exp(63);

    let mut result = BaseElement::ZERO;
    for &value in values.iter() {
        if value.as_int() < p63.as_int() {
            result += BaseElement::ONE;
        }
    }
    result
}
