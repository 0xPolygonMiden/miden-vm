use super::build_test;
use std::fmt::Write;
use vm_core::{polynom, Felt, StarkField};

const POLYNOMIAL_LENGTH: usize = 512;
const WORDS: usize = 128;
const Q: u32 = 12289; // Prime Number

#[test]
fn test_poly512_add_zq() {
    let source = generate_test_script_add_zq();

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

fn generate_test_script_add_zq() -> String {
    let polynomial_1 = rand_utils::rand_array::<u32, POLYNOMIAL_LENGTH>().map(|v| v % Q);
    let polynomial_2 = rand_utils::rand_array::<u32, POLYNOMIAL_LENGTH>().map(|v| v % Q);

    let result_polynomial: Vec<u32> = (0..POLYNOMIAL_LENGTH)
        .map(|i| (polynomial_1[i] + polynomial_2[i]) % Q)
        .collect();

    let mut polynomial_1_script = String::new();
    let mut polynomial_2_script = String::new();
    let mut check_result_script = String::new();

    for i in 0..WORDS {
        // fill script for polynomial 1
        writeln!(
            polynomial_1_script,
            "push.{}.{}.{}.{}",
            polynomial_1[4 * i + 3],
            polynomial_1[4 * i + 2],
            polynomial_1[4 * i + 1],
            polynomial_1[4 * i]
        )
        .unwrap();
        writeln!(polynomial_1_script, "loc_storew.{i}").unwrap();
        writeln!(polynomial_1_script, "dropw").unwrap();

        // fill script for polynomial 2
        writeln!(
            polynomial_2_script,
            "push.{}.{}.{}.{}",
            polynomial_2[4 * i + 3],
            polynomial_2[4 * i + 2],
            polynomial_2[4 * i + 1],
            polynomial_2[4 * i]
        )
        .unwrap();
        writeln!(polynomial_2_script, "loc_storew.{}", i + 128).unwrap();
        writeln!(polynomial_2_script, "dropw").unwrap();

        // fill script for checking the result
        writeln!(check_result_script, "push.0.0.0.0").unwrap();
        writeln!(check_result_script, "loc_loadw.{}", i + 256).unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 1]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 2]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 3]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
    }

    let script = format!(
        "
        use.std::math::poly512

        proc.wrapper.384
            {polynomial_1_script}

            {polynomial_2_script}

            locaddr.256 # output
            locaddr.128 # input 1
            locaddr.0 # input 0

            exec.poly512::add_zq

            {check_result_script}
        end

        begin
            exec.wrapper
        end
    "
    );
    script
}

#[test]
fn test_poly512_neg_zq() {
    let source = generate_test_script_neg_zq();

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

fn generate_test_script_neg_zq() -> String {
    let polynomial_1 = rand_utils::rand_array::<u32, POLYNOMIAL_LENGTH>().map(|v| v % Q);

    let result_polynomial: Vec<u32> = (0..POLYNOMIAL_LENGTH).map(|i| Q - polynomial_1[i]).collect();

    let mut polynomial_1_script = String::new();
    let mut check_result_script = String::new();

    for i in 0..WORDS {
        // fill script for polynomial 1
        writeln!(
            polynomial_1_script,
            "push.{}.{}.{}.{}",
            polynomial_1[4 * i + 3],
            polynomial_1[4 * i + 2],
            polynomial_1[4 * i + 1],
            polynomial_1[4 * i]
        )
        .unwrap();
        writeln!(polynomial_1_script, "loc_storew.{i}").unwrap();
        writeln!(polynomial_1_script, "dropw").unwrap();

        // fill script for checking the result
        writeln!(check_result_script, "push.0.0.0.0").unwrap();
        writeln!(check_result_script, "loc_loadw.{}", i + 128).unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 1]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 2]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 3]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
    }

    let script = format!(
        "
        use.std::math::poly512

        proc.wrapper.256
            {polynomial_1_script}

            locaddr.128 # output
            locaddr.0 # input 0

            exec.poly512::neg_zq

            {check_result_script}
        end

        begin
            exec.wrapper
        end
    "
    );
    script
}

#[test]
fn test_poly512_sub_zq() {
    let source = generate_test_script_sub_zq();

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

fn generate_test_script_sub_zq() -> String {
    let polynomial_1 = rand_utils::rand_array::<u32, POLYNOMIAL_LENGTH>().map(|v| v % Q);
    let polynomial_2 = rand_utils::rand_array::<u32, POLYNOMIAL_LENGTH>().map(|v| v % Q);

    let result_polynomial: Vec<u32> = (0..POLYNOMIAL_LENGTH)
        .map(|i| (polynomial_1[i] + Q - polynomial_2[i]) % Q)
        .collect();

    let mut polynomial_1_script = String::new();
    let mut polynomial_2_script = String::new();
    let mut check_result_script = String::new();

    for i in 0..WORDS {
        // fill script for polynomial 1
        writeln!(
            polynomial_1_script,
            "push.{}.{}.{}.{}",
            polynomial_1[4 * i + 3],
            polynomial_1[4 * i + 2],
            polynomial_1[4 * i + 1],
            polynomial_1[4 * i]
        )
        .unwrap();
        writeln!(polynomial_1_script, "loc_storew.{i}").unwrap();
        writeln!(polynomial_1_script, "dropw").unwrap();

        // fill script for polynomial 2
        writeln!(
            polynomial_2_script,
            "push.{}.{}.{}.{}",
            polynomial_2[4 * i + 3],
            polynomial_2[4 * i + 2],
            polynomial_2[4 * i + 1],
            polynomial_2[4 * i]
        )
        .unwrap();
        writeln!(polynomial_2_script, "loc_storew.{}", i + 128).unwrap();
        writeln!(polynomial_1_script, "dropw").unwrap();

        // fill script for checking the result
        writeln!(check_result_script, "push.0.0.0.0").unwrap();
        writeln!(check_result_script, "loc_loadw.{}", i + 256).unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 1]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 2]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 3]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
    }

    let script = format!(
        "
        use.std::math::poly512

        proc.wrapper.384
            {polynomial_1_script}

            {polynomial_2_script}

            locaddr.256 # output
            locaddr.128 # input 1
            locaddr.0 # input 0

            exec.poly512::sub_zq

            {check_result_script}
        end

        begin
            exec.wrapper
        end
    "
    );
    script
}

#[test]
fn test_poly512_mul_zq() {
    let source = generate_test_script_mul_zq();

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

fn generate_test_script_mul_zq() -> String {
    const Q: u64 = 12289; // Prime Number

    let polynomial_1 = rand_utils::rand_array::<u64, POLYNOMIAL_LENGTH>().map(|v| Felt::new(v % Q));
    let polynomial_2 = rand_utils::rand_array::<u64, POLYNOMIAL_LENGTH>().map(|v| Felt::new(v % Q));

    let result_polynomial: Vec<u64> = polynom::mul(&polynomial_1, &polynomial_2)
        .iter()
        .map(|v| v.as_int() % Q)
        .collect();

    let (lower, upper) = result_polynomial.split_at(512);
    let mut upper = upper.to_vec();
    upper.push(0);

    let result_polynomial: Vec<u64> =
        (0..POLYNOMIAL_LENGTH).map(|i| (lower[i] + Q - upper[i]) % Q).collect();

    let mut polynomial_1_script = String::new();
    let mut polynomial_2_script = String::new();
    let mut check_result_script = String::new();

    for i in 0..WORDS {
        // fill script for polynomial 1
        writeln!(
            polynomial_1_script,
            "push.{}.{}.{}.{}",
            polynomial_1[4 * i + 3],
            polynomial_1[4 * i + 2],
            polynomial_1[4 * i + 1],
            polynomial_1[4 * i]
        )
        .unwrap();
        writeln!(polynomial_1_script, "loc_storew.{i}").unwrap();
        writeln!(polynomial_1_script, "dropw").unwrap();

        // fill script for polynomial 2
        writeln!(
            polynomial_2_script,
            "push.{}.{}.{}.{}",
            polynomial_2[4 * i + 3],
            polynomial_2[4 * i + 2],
            polynomial_2[4 * i + 1],
            polynomial_2[4 * i]
        )
        .unwrap();
        writeln!(polynomial_2_script, "loc_storew.{}", i + 128).unwrap();
        writeln!(polynomial_1_script, "dropw").unwrap();

        // fill script for checking the result
        writeln!(check_result_script, "push.0.0.0.0").unwrap();
        writeln!(check_result_script, "loc_loadw.{}", i + 256).unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 1]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 2]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
        writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 3]).unwrap();
        writeln!(check_result_script, "assert_eq").unwrap();
    }

    let script = format!(
        "
        use.std::math::poly512

        proc.wrapper.384
            {polynomial_1_script}

            {polynomial_2_script}

            locaddr.256 # output
            locaddr.128 # input 1
            locaddr.0 # input 0

            exec.poly512::mul_zq

            {check_result_script}
        end

        begin
            exec.wrapper
        end
    "
    );
    script
}
