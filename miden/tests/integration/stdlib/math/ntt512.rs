use super::build_test;

#[test]
fn test_ntt512() {
    let source = generate_random_test_script();

    let test = build_test!(source, &[]);
    let _ = test.get_last_stack_state();
}

fn generate_random_test_script() -> String {
    const N: usize = 512;
    const WORDS: usize = N >> 2;

    const Q: u64 = (((1u64 << 32) - 1) << 32) + 1; // Miden Field Prime

    let mut input_instructions = String::new();
    let mut test_instructions = String::new();

    for _ in 0..WORDS {
        let v0 = rand_utils::rand_value::<u64>() % Q;
        let v1 = rand_utils::rand_value::<u64>() % Q;
        let v2 = rand_utils::rand_value::<u64>() % Q;
        let v3 = rand_utils::rand_value::<u64>() % Q;

        input_instructions.push_str(&format!("push.{}.{}.{}.{}\n", v3, v2, v1, v0));
        input_instructions.push_str("dup.4\n");
        input_instructions.push_str("mem_storew\n");
        input_instructions.push_str("dropw\n");
        input_instructions.push_str("add.1\n\n");

        test_instructions.push_str("dup\n");
        test_instructions.push_str("push.0.0.0.0\n");
        test_instructions.push_str("movup.4\n");
        test_instructions.push_str("mem_loadw\n");
        test_instructions.push_str(&format!("push.{}\n", v0));
        test_instructions.push_str("assert_eq\n");
        test_instructions.push_str(&format!("push.{}\n", v1));
        test_instructions.push_str("assert_eq\n");
        test_instructions.push_str(&format!("push.{}\n", v2));
        test_instructions.push_str("assert_eq\n");
        test_instructions.push_str(&format!("push.{}\n", v3));
        test_instructions.push_str("assert_eq\n");
        test_instructions.push_str("add.1\n\n");
    }

    let script = format!(
        "
    use.std::math::ntt512

    proc.wrapper.128
        # prepare input vector
        
        locaddr.127

        {}

        drop

        # place starting absolute memory addresses on stack, where input vector is kept,
        # next addresses are computable using `add.1` instruction.

        locaddr.127

        exec.ntt512::forward  # apply forward NTT
        exec.ntt512::backward # apply inverse NTT

        # test that v == v' | v -> forward -> backward -> v'
        # where v = input vector
        #       v' = output vector holding result of iNTT(NTT(v))

        {}

        drop
    end

    begin
        exec.wrapper
    end
    ",
        &input_instructions, &test_instructions
    );
    script
}
