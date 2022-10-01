use super::build_test;
use math::fields::f64::BaseElement;
use math::polynom::mul;
use std::fmt::Write;
use vm_core::StarkField;

#[test]
fn test_poly512_add_zq() {
    let source = generate_test_script_add_zq();

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
}

fn generate_test_script_add_zq() -> String {
    const POLYNOMIAL_LENGTH: usize = 512;
    const WORDS: usize = 128;
    const Q: u32 = 12289; // Prime Number

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
        let _ = writeln!(
            polynomial_1_script,
            "push.{}.{}.{}.{}",
            polynomial_1[4 * i + 3],
            polynomial_1[4 * i + 2],
            polynomial_1[4 * i + 1],
            polynomial_1[4 * i]
        );
        let _ = writeln!(polynomial_1_script, "loc_storew.{}", i);
        polynomial_1_script.push_str("dropw\n");

        // fill script for polynomial 2
        let _ = writeln!(
            polynomial_2_script,
            "push.{}.{}.{}.{}",
            polynomial_2[4 * i + 3],
            polynomial_2[4 * i + 2],
            polynomial_2[4 * i + 1],
            polynomial_2[4 * i]
        );
        let _ = writeln!(polynomial_2_script, "loc_storew.{}", i + 128);
        polynomial_2_script.push_str("dropw\n");

        // fill script for checking the result
        check_result_script.push_str("push.0.0.0.0\n");
        let _ = writeln!(check_result_script, "loc_loadw.{}", i + 256);
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 1]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 2]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 3]);
        check_result_script.push_str("assert_eq\n");
    }

    let script = format!(
        "
        use.std::math::poly512

        proc.wrapper.384
            {}

            {}

            locaddr.256 # output
            locaddr.128 # input 1
            locaddr.0 # input 0

            exec.poly512::add_zq

            {}
        end

        begin
            exec.wrapper
        end
    ",
        polynomial_1_script, polynomial_2_script, check_result_script
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
    const POLYNOMIAL_LENGTH: usize = 512;
    const WORDS: usize = 128;
    const Q: u32 = 12289; // Prime Number

    let polynomial_1 = rand_utils::rand_array::<u32, POLYNOMIAL_LENGTH>().map(|v| v % Q);

    let result_polynomial: Vec<u32> = (0..POLYNOMIAL_LENGTH)
        .map(|i| Q - polynomial_1[i])
        .collect();

    let mut polynomial_1_script = String::new();
    let mut check_result_script = String::new();

    for i in 0..WORDS {
        // fill script for polynomial 1
        let _ = writeln!(
            polynomial_1_script,
            "push.{}.{}.{}.{}",
            polynomial_1[4 * i + 3],
            polynomial_1[4 * i + 2],
            polynomial_1[4 * i + 1],
            polynomial_1[4 * i]
        );
        let _ = writeln!(polynomial_1_script, "loc_storew.{}", i);
        polynomial_1_script.push_str("dropw\n");

        // fill script for checking the result
        check_result_script.push_str("push.0.0.0.0\n");
        let _ = writeln!(check_result_script, "loc_loadw.{}", i + 128);
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 1]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 2]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 3]);
        check_result_script.push_str("assert_eq\n");
    }

    let script = format!(
        "
        use.std::math::poly512

        proc.wrapper.256
            {}

            locaddr.128 # output
            locaddr.0 # input 0

            exec.poly512::neg_zq

            {}
        end

        begin
            exec.wrapper
        end
    ",
        polynomial_1_script, check_result_script
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
    const POLYNOMIAL_LENGTH: usize = 512;
    const WORDS: usize = 128;
    const Q: u32 = 12289; // Prime Number

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
        let _ = writeln!(
            polynomial_1_script,
            "push.{}.{}.{}.{}",
            polynomial_1[4 * i + 3],
            polynomial_1[4 * i + 2],
            polynomial_1[4 * i + 1],
            polynomial_1[4 * i]
        );
        let _ = writeln!(polynomial_1_script, "loc_storew.{}", i);
        polynomial_1_script.push_str("dropw\n");

        // fill script for polynomial 2
        let _ = writeln!(
            polynomial_2_script,
            "push.{}.{}.{}.{}",
            polynomial_2[4 * i + 3],
            polynomial_2[4 * i + 2],
            polynomial_2[4 * i + 1],
            polynomial_2[4 * i]
        );
        let _ = writeln!(polynomial_2_script, "loc_storew.{}", i + 128);
        polynomial_2_script.push_str("dropw\n");

        // fill script for checking the result
        check_result_script.push_str("push.0.0.0.0\n");
        let _ = writeln!(check_result_script, "loc_loadw.{}", i + 256);
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 1]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 2]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 3]);
        check_result_script.push_str("assert_eq\n");
    }

    let script = format!(
        "
        use.std::math::poly512

        proc.wrapper.384
            {}

            {}

            locaddr.256 # output
            locaddr.128 # input 1
            locaddr.0 # input 0

            exec.poly512::sub_zq

            {}
        end

        begin
            exec.wrapper
        end
    ",
        polynomial_1_script, polynomial_2_script, check_result_script
    );
    script
}

#[test]
fn test_poly512_mul_zq() {
    let source = generate_test_script_mul_zq();

    let test = build_test!(source, &[]);
    test.get_last_stack_state();
    // test.expect_stack(&[1337, 69]);
}

fn generate_test_script_mul_zq() -> String {
    const POLYNOMIAL_LENGTH: usize = 512;
    const WORDS: usize = 128;
    const Q: u64 = 12289; // Prime Number

    let polynomial_1 =
        rand_utils::rand_array::<u64, POLYNOMIAL_LENGTH>().map(|v| BaseElement::new(v % Q));
    let polynomial_2 =
        rand_utils::rand_array::<u64, POLYNOMIAL_LENGTH>().map(|v| BaseElement::new(v % Q));

    let result_polynomial: Vec<u64> = mul(&polynomial_1, &polynomial_2)
        .iter()
        .map(|v| v.as_int() % Q)
        .collect();

    let lower = result_polynomial[..512].to_vec();
    let mut upper = result_polynomial[512..].to_vec();
    upper.push(0);

    let result_polynomial: Vec<u64> = (0..POLYNOMIAL_LENGTH)
        .map(|i| (lower[i] + Q - upper[i]) % Q)
        .collect();

    let mut polynomial_1_script = String::new();
    let mut polynomial_2_script = String::new();
    let mut check_result_script = String::new();

    for i in 0..WORDS {
        // fill script for polynomial 1
        let _ = writeln!(
            polynomial_1_script,
            "push.{}.{}.{}.{}",
            polynomial_1[4 * i + 3],
            polynomial_1[4 * i + 2],
            polynomial_1[4 * i + 1],
            polynomial_1[4 * i]
        );
        let _ = writeln!(polynomial_1_script, "loc_storew.{}", i);
        polynomial_1_script.push_str("dropw\n");

        // fill script for polynomial 2
        let _ = writeln!(
            polynomial_2_script,
            "push.{}.{}.{}.{}",
            polynomial_2[4 * i + 3],
            polynomial_2[4 * i + 2],
            polynomial_2[4 * i + 1],
            polynomial_2[4 * i]
        );
        let _ = writeln!(polynomial_2_script, "loc_storew.{}", i + 128);
        polynomial_2_script.push_str("dropw\n");

        // fill script for checking the result
        check_result_script.push_str("dup\n");
        check_result_script.push_str("push.0.0.0.0\n");
        check_result_script.push_str("movup.4\n");
        check_result_script.push_str("mem_loadw\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 1]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 2]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", result_polynomial[4 * i + 3]);
        check_result_script.push_str("assert_eq\n");
        check_result_script.push_str("add.1\n");
    }

    let script = format!(
        "
        use.std::math::poly512

        proc.wrapper.384
            {}

            {}

            locaddr.256 # output
            locaddr.128 # input 1
            locaddr.0 # input 0

            exec.poly512::mul_zq

            locaddr.256

            {}
        end

        begin
            exec.wrapper
        end
    ",
        polynomial_1_script, polynomial_2_script, check_result_script
    );
    script
}
