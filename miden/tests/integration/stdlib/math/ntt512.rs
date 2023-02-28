use super::build_test;
use std::fmt::Write;

#[test]
fn test_ntt512() {
    let source = generate_test_script_ntt512();

    let test = build_test!(source, &[]);
    let _ = test.get_last_stack_state();
}

fn generate_test_script_ntt512() -> String {
    const POLYNOMIAL_LENGTH: usize = 512;
    const WORDS: usize = 128;
    const Q: u64 = (((1u64 << 32) - 1) << 32) + 1; // Miden Field Prime

    let polynomial = rand_utils::rand_array::<u64, POLYNOMIAL_LENGTH>().map(|v| v % Q);

    let mut polynomial_script = String::new();
    let mut check_result_script = String::new();

    for i in 0..WORDS {
        let _ = writeln!(
            polynomial_script,
            "push.{}.{}.{}.{}",
            polynomial[4 * i + 3],
            polynomial[4 * i + 2],
            polynomial[4 * i + 1],
            polynomial[4 * i]
        );
        let _ = writeln!(polynomial_script, "loc_storew.{i}");
        polynomial_script.push_str("dropw\n");

        check_result_script.push_str("dup\n");
        check_result_script.push_str("push.0.0.0.0\n");
        check_result_script.push_str("movup.4\n");
        check_result_script.push_str("mem_loadw\n");
        let _ = writeln!(check_result_script, "push.{}", polynomial[4 * i]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", polynomial[4 * i + 1]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", polynomial[4 * i + 2]);
        check_result_script.push_str("assert_eq\n");
        let _ = writeln!(check_result_script, "push.{}", polynomial[4 * i + 3]);
        check_result_script.push_str("assert_eq\n");
        check_result_script.push_str("add.1\n");
    }

    let script = format!(
        "
    use.std::math::ntt512

    proc.wrapper.128
        # prepare input vector

        {polynomial_script}

        # place starting absolute memory addresses on stack, where input vector is kept,
        # next addresses are computable using `add.1` instruction.

        locaddr.0

        exec.ntt512::forward  # apply forward NTT
        exec.ntt512::backward # apply inverse NTT

        # test that v == v' | v -> forward -> backward -> v'
        # where v = input vector
        #       v' = output vector holding result of iNTT(NTT(v))

        {check_result_script}

        drop
    end

    begin
        exec.wrapper
    end
    "
    );
    script
}
