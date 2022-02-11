use super::{
    build_inputs, compile, execute, test_compilation_failure, test_execution_failure,
    test_op_execution, test_param_out_of_bounds, test_unsafe_execution, U32_BOUND,
};
use proptest::prelude::*;
use rand_utils::rand_value;

// U32 OPERATIONS TESTS - MANUAL - ARITHMETIC OPERATIONS
// ================================================================================================

#[test]
fn u32add() {
    let asm_op = "u32add";

    // --- simple case ----------------------------------------------------------------------------
    test_op_execution(asm_op, &[1, 2], &[3]);

    // --- random values --------------------------------------------------------------------------
    // test using u16 values to ensure there's no overflow so the result is valid
    let a = rand_value::<u64>() as u16;
    let b = rand_value::<u64>() as u16;
    let expected = a as u64 + b as u64;

    test_op_execution(asm_op, &[a as u64, b as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(asm_op, &[c, a as u64, b as u64], &[expected, c]);
}

#[test]
fn u32add_fail() {
    let asm_op = "u32add";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if a + b >= 2^32
    let a = u32::MAX;
    let b = 1_u64;
    test_execution_failure(asm_op, &[a as u64, b], "FailedAssertion");
}

#[test]
fn u32add_b() {
    let build_asm_op = |param: u16| format!("u32add.{}", param);

    // --- simple case ----------------------------------------------------------------------------
    test_op_execution(build_asm_op(2).as_str(), &[1], &[3]);

    // --- random values --------------------------------------------------------------------------
    // test using u16 values to ensure there's no overflow so the result is valid
    let a = rand_value::<u64>() as u16;
    let b = rand_value::<u64>() as u16;
    let expected = a as u64 + b as u64;
    test_op_execution(build_asm_op(b).as_str(), &[a as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(build_asm_op(b).as_str(), &[c, a as u64], &[expected, c]);
}

#[test]
fn u32add_b_fail() {
    let build_asm_op = |param: u64| format!("u32add.{}", param);

    // should fail during execution if a >= 2^32
    test_execution_failure(build_asm_op(0).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail if a + b >= 2^32
    let a = u32::MAX;
    let b = 1_u64;
    test_execution_failure(build_asm_op(b).as_str(), &[a as u64], "FailedAssertion");
}

#[test]
fn u32add_full() {
    let asm_op = "u32add.full";

    // should push c = (a + b) % 2^32 onto the stack
    // should push overflow flag d, where d = 1 if (a + b) >= 2^32 and d = 0 otherwise
    test_add_full(asm_op);
}

#[test]
fn u32add_full_fail() {
    let asm_op = "u32add.full";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");
}

#[test]
fn u32add_unsafe() {
    let asm_op = "u32add.unsafe";

    // should push c = (a + b) % 2^32 onto the stack
    // should push overflow flag d, where d = 1 if (a + b) >= 2^32 and d = 0 otherwise
    test_add_full(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32addc() {
    let asm_op = "u32addc";

    // should push d = (a + b + c) % 2^32 onto the stack, where c is 1 or 0
    // should push overflow flag e, where e = 1 if (a + b + c) >= 2^32 and e = 0 otherwise
    test_addc(asm_op);
}

#[test]
fn u32addc_fail() {
    let asm_op = "u32addc";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, 0, U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND, 0], "FailedAssertion");

    // should fail if c > 1
    test_execution_failure(asm_op, &[2, 0, 0], "NotBinaryValue");
}

#[test]
fn u32addc_unsafe() {
    let asm_op = "u32addc.unsafe";

    // --- test correct execution -----------------------------------------------------------------
    // should push d = (a + b + c) % 2^32 onto the stack, where c is 1 or 0
    // should push overflow flag e, where e = 1 if (a + b + c) >= 2^32 and e = 0 otherwise
    test_addc(asm_op);

    // --- test that out of bounds inputs do not cause a failure ----------------------------------
    let script = compile(format!("begin {} end", asm_op).as_str());

    // should not fail if a >= 2^32
    let inputs = build_inputs(&[0, 0, U32_BOUND]);
    assert!(execute(&script, &inputs).is_ok());

    // should not fail if b >= 2^32
    let inputs = build_inputs(&[0, U32_BOUND, 0]);
    assert!(execute(&script, &inputs).is_ok());
}

#[test]
fn u32addc_unsafe_fail() {
    let asm_op = "u32addc.unsafe";

    // should fail if c > 1
    test_execution_failure(asm_op, &[2, U32_BOUND, 0], "NotBinaryValue");
}

#[test]
fn u32sub() {
    let asm_op = "u32sub";

    // --- simple cases ---------------------------------------------------------------------------
    test_op_execution(asm_op, &[1, 1], &[0]);
    test_op_execution(asm_op, &[2, 1], &[1]);

    // --- random u32 values ----------------------------------------------------------------------
    let val1 = rand_value::<u64>() as u32;
    let val2 = rand_value::<u64>() as u32;
    // assign the larger value to a and the smaller value to b
    let (a, b) = if val1 >= val2 {
        (val1, val2)
    } else {
        (val2, val1)
    };
    let expected = a - b;

    test_op_execution(asm_op, &[a as u64, b as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(asm_op, &[c, a as u64, b as u64], &[expected as u64, c]);
}

#[test]
fn u32sub_fail() {
    let asm_op = "u32sub";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if a < b
    let a = 1_u64;
    let b = 2_u64;
    test_execution_failure(asm_op, &[a, b], "FailedAssertion");
}

#[test]
fn u32sub_b() {
    let build_asm_op = |param: u32| format!("u32sub.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_op_execution(build_asm_op(1).as_str(), &[2], &[1]);
    test_op_execution(build_asm_op(1).as_str(), &[1], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let val1 = rand_value::<u64>() as u32;
    let val2 = rand_value::<u64>() as u32;
    // assign the larger value to a and the smaller value to b
    let (a, b) = if val1 >= val2 {
        (val1, val2)
    } else {
        (val2, val1)
    };
    let expected = a - b;
    test_op_execution(build_asm_op(b).as_str(), &[a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(
        build_asm_op(b).as_str(),
        &[c, a as u64],
        &[expected as u64, c],
    );
}

#[test]
fn u32sub_b_fail() {
    let build_asm_op = |param: u64| format!("u32sub.{}", param);

    // should fail during execution if a >= 2^32
    test_execution_failure(build_asm_op(0).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail if a < b
    let a = 1_u64;
    let b = 2_u64;
    test_execution_failure(build_asm_op(b).as_str(), &[a], "FailedAssertion");
}

#[test]
fn u32sub_full() {
    let asm_op = "u32sub.full";

    // should push c = (a - b) % 2^32 onto the stack
    // should push underflow flag d, where d = 1 if a < b and d = 0 otherwise
    test_sub_full(asm_op);
}

#[test]
fn u32sub_full_fail() {
    let asm_op = "u32sub.full";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");
}

#[test]
fn u32sub_unsafe() {
    let asm_op = "u32sub.unsafe";

    // should push c = (a - b) % 2^32 onto the stack
    // should push underflow flag d, where d = 1 if a < b and d = 0 otherwise
    test_sub_full(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32mul() {
    let asm_op = "u32mul";

    // --- simple cases ---------------------------------------------------------------------------
    test_op_execution(asm_op, &[1, 0], &[0]);
    test_op_execution(asm_op, &[5, 1], &[5]);
    test_op_execution(asm_op, &[2, 5], &[10]);

    // --- random values --------------------------------------------------------------------------
    // test using u16 values to ensure there's no overflow so the result is valid
    let a = rand_value::<u64>() as u16;
    let b = rand_value::<u64>() as u16;

    let expected: u64 = a as u64 * b as u64;
    test_op_execution(asm_op, &[a as u64, b as u64], &[expected]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(asm_op, &[c, a as u64, b as u64], &[expected, c]);
}

#[test]
fn u32mul_fail() {
    let asm_op = "u32mul";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");

    // should fail if a * b  >= 2^32
    let a = u32::MAX as u64;
    let b = 2_u64;
    test_execution_failure(asm_op, &[a, b], "FailedAssertion");
}

#[test]
fn u32mul_b() {
    let build_asm_op = |param: u16| format!("u32mul.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_op_execution(build_asm_op(0).as_str(), &[1], &[0]);
    test_op_execution(build_asm_op(1).as_str(), &[5], &[5]);
    test_op_execution(build_asm_op(5).as_str(), &[2], &[10]);

    // --- random values --------------------------------------------------------------------------
    // test using u16 values to ensure there's no overflow so the result is valid
    let a = rand_value::<u64>() as u16;
    let b = rand_value::<u64>() as u16;

    let expected: u64 = a as u64 * b as u64;
    test_op_execution(build_asm_op(b).as_str(), &[a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(build_asm_op(5).as_str(), &[c, 10], &[50, c]);
}

#[test]
fn u32mul_b_fail() {
    let build_asm_op = |param: u64| format!("u32mul.{}", param);

    // should fail during execution if a >= 2^32
    test_execution_failure(build_asm_op(0).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail if a * b >= 2^32
    let a = u32::MAX as u64;
    let b = u32::MAX as u64;
    test_execution_failure(build_asm_op(b).as_str(), &[a], "FailedAssertion");
}

#[test]
fn u32mul_full() {
    let asm_op = "u32mul.full";

    // should push c = (a * b) % 2^32 onto the stack
    // should push d = (a * b) / 2^32 onto the stack
    test_mul_full(asm_op);
}

#[test]
fn u32mul_full_fail() {
    let asm_op = "u32mul.full";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND], "FailedAssertion");
}

#[test]
fn u32mul_unsafe() {
    let asm_op = "u32mul.unsafe";

    // should push c = (a * b) % 2^32 onto the stack
    // should push d = (a * b) / 2^32 onto the stack
    test_mul_full(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
fn u32madd() {
    let asm_op = "u32madd";

    // should push d = (a * b + c) % 2^32 onto the stack
    // should push e = (a * b + c) / 2^32 onto the stack
    test_madd(asm_op);
}

#[test]
fn u32madd_fail() {
    let asm_op = "u32madd";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[0, 0, U32_BOUND], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[0, U32_BOUND, 0], "FailedAssertion");

    // should fail if c  >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 0, 0], "FailedAssertion");
}

#[test]
fn u32madd_unsafe() {
    let asm_op = "u32madd.unsafe";

    // should push d = (a * b + c) % 2^32 onto the stack
    // should push e = (a * b + c) / 2^32 onto the stack
    test_madd(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 3);
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/94
fn u32div() {
    let asm_op = "u32div";

    // --- simple cases ---------------------------------------------------------------------------
    test_op_execution(asm_op, &[0, 1], &[0]);
    // division with no remainder
    test_op_execution(asm_op, &[2, 1], &[2]);
    // division with remainder
    test_op_execution(asm_op, &[1, 2], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = a / b;
    test_op_execution(asm_op, &[a as u64, b as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(asm_op, &[c, a as u64, b as u64], &[expected as u64, c]);
}

#[test]
fn u32div_fail() {
    let asm_op = "u32div";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 1], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[1, U32_BOUND], "FailedAssertion");
}

#[test]
#[should_panic = "divide by zero"]
fn u32div_panic() {
    let script = compile("begin u32div end");
    let inputs = build_inputs(&[1, 0]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/94
fn u32div_b() {
    let build_asm_op = |param: u32| format!("u32div.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_op_execution(build_asm_op(1).as_str(), &[0], &[0]);
    // division with no remainder
    test_op_execution(build_asm_op(1).as_str(), &[2], &[2]);
    // division with remainder
    test_op_execution(build_asm_op(2).as_str(), &[1], &[0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let expected = a / b;
    test_op_execution(build_asm_op(b).as_str(), &[a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(
        build_asm_op(b).as_str(),
        &[c, a as u64],
        &[expected as u64, c],
    );
}

#[test]
// issue: https://github.com/maticnetwork/miden/issues/94
fn u32div_b_fail() {
    let build_asm_op = |param: u64| format!("u32div.{}", param);

    // should fail during execution if a >= 2^32
    test_execution_failure(build_asm_op(1).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail during compilation if b = 0
    test_compilation_failure(build_asm_op(0).as_str(), "parameter");
}

#[test]
fn u32div_full() {
    let asm_op = "u32div.full";

    // should push the quotient c = a / b onto the stack
    // should push the remainder d = a % b onto the stack
    test_div_full(asm_op);
}

#[test]
fn u32div_full_fail() {
    let asm_op = "u32div.full";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 1], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[1, U32_BOUND], "FailedAssertion");
}

#[test]
#[should_panic = "divide by zero"]
fn u32div_full_panic() {
    let script = compile("begin u32div.full end");
    let inputs = build_inputs(&[1, 0]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

#[test]
fn u32div_unsafe() {
    let asm_op = "u32div.unsafe";

    // should push c = (a * b) % 2^32 onto the stack
    // should push d = (a * b) / 2^32 onto the stack
    test_div_full(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
#[should_panic = "divide by zero"]
fn u32div_unsafe_panic() {
    let script = compile("begin u32div.unsafe end");
    let inputs = build_inputs(&[1, 0]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

#[test]
fn u32mod() {
    let asm_op = "u32mod";

    // should pop b, a off the stack and push the result of a % b onto the stack
    test_mod(asm_op);
}

#[test]
fn u32mod_fail() {
    let asm_op = "u32mod";

    // should fail if a >= 2^32
    test_execution_failure(asm_op, &[U32_BOUND, 1], "FailedAssertion");

    // should fail if b >= 2^32
    test_execution_failure(asm_op, &[1, U32_BOUND], "FailedAssertion");
}

#[test]
#[should_panic = "divide by zero"]
fn u32mod_panic() {
    let script = compile("begin u32mod end");
    let inputs = build_inputs(&[1, 0]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

#[test]
fn u32mod_b() {
    let build_asm_op = |param: u32| format!("u32mod.{}", param);

    // --- simple cases ---------------------------------------------------------------------------
    test_op_execution(build_asm_op(5).as_str(), &[10], &[0]);
    test_op_execution(build_asm_op(5).as_str(), &[11], &[1]);
    test_op_execution(build_asm_op(11).as_str(), &[5], &[5]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let mut b = rand_value::<u64>() as u32;
    if b == 0 {
        // ensure we're not using a failure case
        b += 1;
    }
    let expected = a % b;
    test_op_execution(build_asm_op(b).as_str(), &[a as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(
        build_asm_op(b).as_str(),
        &[c, a as u64],
        &[expected as u64, c],
    );
}

#[test]
fn u32mod_b_fail() {
    let build_asm_op = |param: u64| format!("u32mod.{}", param);

    // should fail during exeuction if a >= 2^32
    test_execution_failure(build_asm_op(1).as_str(), &[U32_BOUND], "FailedAssertion");

    // should fail during compilation if b >= 2^32
    test_param_out_of_bounds(build_asm_op(U32_BOUND).as_str(), U32_BOUND);

    // should fail during compilation if b = 0
    test_compilation_failure(build_asm_op(0).as_str(), "parameter");
}

#[test]
fn u32mod_unsafe() {
    let asm_op = "u32mod.unsafe";

    // should pop b, a off the stack and push the result of a % b onto the stack
    test_mod(asm_op);

    // should not fail when inputs are out of bounds
    test_unsafe_execution(asm_op, 2);
}

#[test]
#[should_panic = "divide by zero"]
fn u32mod_unsafe_panic() {
    let script = compile("begin u32mod.unsafe end");
    let inputs = build_inputs(&[1, 0]);

    // should panic if b = 0
    execute(&script, &inputs).unwrap();
}

// U32 OPERATIONS TESTS - RANDOMIZED - ARITHMETIC OPERATIONS
// ================================================================================================
proptest! {
    #[test]
    fn u32add_proptest(a in any::<u16>(), b in any::<u16>()) {
        let asm_op = "u32add";

        let expected = a as u64 + b as u64;

        // b provided via the stack
        test_op_execution(asm_op, &[b as u64, a as u64], &[expected]);
        // b provided as a parameter
        test_op_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected]);
    }

    #[test]
    fn u32add_full_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32add";

        let (c, overflow) = a.overflowing_add(b);
        let d = if overflow { 1 } else { 0 };

        // full and unsafe should produce the same result for valid values
        test_op_execution(format!("{}.full", asm_op).as_str(), &[a as u64, b as u64], &[d, c as u64]);
        test_op_execution(format!("{}.unsafe", asm_op).as_str(), &[a as u64, b as u64], &[d, c as u64]);
    }

    #[test]
    fn u32addc_proptest(a in any::<u32>(), b in any::<u32>(), c in 0_u32..1) {
        let asm_op = "u32addc";

        let (d, overflow_b) = a.overflowing_add(b);
        let (d, overflow_c) = d.overflowing_add(c);
        let e = if overflow_b || overflow_c { 1_u64 } else { 0_u64 };

        // safe and unsafe should produce the same result for valid values
        test_op_execution(asm_op, &[c as u64, a as u64, b as u64], &[e, d as u64]);
        test_op_execution(format!("{}.unsafe", asm_op).as_str(), &[c as u64, a as u64, b as u64], &[e, d as u64]);
    }

    #[test]
    fn u32sub_proptest(val1 in any::<u32>(), val2 in any::<u32>()) {
        let asm_op = "u32sub";

        // assign the larger value to a and the smaller value to b so all parameters are valid
        let (a, b) = if val1 >= val2 {
            (val1, val2)
        } else {
            (val2, val1)
        };

        let expected = a - b;
        // b provided via the stack
        test_op_execution(asm_op, &[a as u64, b as u64], &[expected as u64]);
        // b provided as a parameter
        test_op_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected as u64]);
    }

    #[test]
    fn u32sub_full_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32sub";

        // assign the larger value to a and the smaller value to b so all parameters are valid
        let (c, overflow) = a.overflowing_sub(b);
        let d = if overflow { 1 } else { 0 };

        // full and unsafe should produce the same result for valid values
        test_op_execution(format!("{}.full", asm_op).as_str(), &[a as u64, b as u64], &[d, c as u64]);
        test_op_execution(format!("{}.unsafe", asm_op).as_str(), &[a as u64, b as u64], &[d, c as u64]);
    }

    #[test]
    fn u32mul_proptest(a in any::<u16>(), b in any::<u16>()) {
        let asm_op = "u32mul";

        let expected = a as u64 * b as u64;

        // b provided via the stack
        test_op_execution(asm_op, &[b as u64, a as u64], &[expected]);
        // b provided as a parameter
        test_op_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected]);
    }

    #[test]
    fn u32mul_full_proptest(a in any::<u32>(), b in any::<u32>()) {
        let asm_op = "u32mul";

        let (c, overflow) = a.overflowing_mul(b);
        let d = if !overflow {
            0
        } else {
            (a as u64 * b as u64) / U32_BOUND
        };

        // full and unsafe should produce the same result for valid values
        test_op_execution(format!("{}.full", asm_op).as_str(), &[a as u64, b as u64], &[d, c as u64]);
        test_op_execution(format!("{}.unsafe", asm_op).as_str(), &[a as u64, b as u64], &[d, c as u64]);
    }

    #[test]
    fn u32madd_proptest(a in any::<u32>(), b in any::<u32>(), c in any::<u32>()) {
        let asm_op = "u32madd";

        let madd = a as u64 * b as u64 + c as u64;
        let d = madd % U32_BOUND;
        let e = madd / U32_BOUND;

        // safe and unsafe should produce the same result for valid values
        test_op_execution(asm_op, &[c as u64, a as u64, b as u64], &[e, d as u64]);
        test_op_execution(format!("{}.unsafe", asm_op).as_str(), &[c as u64, a as u64, b as u64], &[e, d as u64]);
    }

    #[test]
    // issue: https://github.com/maticnetwork/miden/issues/94
    fn u32div_proptest(a in any::<u32>(), b in 1..u32::MAX) {
        let asm_op = "u32div";

        let expected = a / b;

        // b provided via the stack
        test_op_execution(asm_op, &[a as u64, b as u64], &[expected as u64]);
        // b provided as a parameter
        test_op_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected as u64]);
    }

    #[test]
    fn u32div_full_proptest(a in any::<u32>(), b in 1..u32::MAX) {
        let asm_op = "u32div";

        let quot = (a / b) as u64;
        let rem = (a % b) as u64;

        // full and unsafe should produce the same result for valid values
        test_op_execution(format!("{}.full", asm_op).as_str(), &[a as u64, b as u64], &[rem, quot]);
        test_op_execution(format!("{}.unsafe", asm_op).as_str(), &[a as u64, b as u64], &[rem, quot]);
    }

    #[test]
    fn u32mod_proptest(a in any::<u32>(), b in 1..u32::MAX) {
        let asm_op = "u32mod";

        let expected = a % b;

        // b provided via the stack
        test_op_execution(asm_op, &[a as u64, b as u64], &[expected as u64]);
        // b provided as a parameter
        test_op_execution(format!("{}.{}", asm_op, b).as_str(), &[a as u64], &[expected as u64]);
        // safe and unsafe should produce the same result for valid values
        test_op_execution(format!("{}.unsafe", asm_op).as_str(), &[a as u64, b as u64], &[expected as u64]);
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// This helper function tests overflowing addition for two u32 inputs for a number of simple cases
/// as well as for random values. It checks that a result of (a + b) % 2^32 is pushed to the stack,
/// as well as a flag indicating whether or not arithmetic overflow occurred. Finally, it ensures
/// that the rest of the stack was unaffected.
fn test_add_full(asm_op: &str) {
    // --- (a + b) < 2^32 -------------------------------------------------------------------------
    // c = a + b and d should be unset, since there was no overflow
    test_op_execution(asm_op, &[1, 2], &[0, 3]);

    // --- (a + b) = 2^32 -------------------------------------------------------------------------
    let a = u32::MAX;
    let b = 1_u64;
    // c should be the sum mod 2^32 and d should be set to signal overflow
    test_op_execution(asm_op, &[a as u64, b], &[1, 0]);

    // --- (a + b) > 2^32 -------------------------------------------------------------------------
    let a = 2_u64;
    let b = u32::MAX;
    // c should be the sum mod 2^32 and d should be set to signal overflow
    test_op_execution(asm_op, &[a, b as u64], &[1, 1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let (c, overflow) = a.overflowing_add(b);
    let d = if overflow { 1 } else { 0 };
    test_op_execution(asm_op, &[a as u64, b as u64], &[d, c as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let e = rand_value::<u64>();
    test_op_execution(asm_op, &[e, a as u64, b as u64], &[d, c as u64, e]);
}

/// This helper function tests overflowing add with carry for two u32 inputs a, b and one binary
/// value c. It tests a number of simple cases as well as random u32 values. It checks that a
/// result of (a + b + c) % 2^32 is pushed to the stack, along with a flag indicating whether or
/// not arithmetic overflow occurred. Finally, it ensures that the rest of the stack was
/// unaffected.
fn test_addc(asm_op: &str) {
    // --- (a + b + c) < 2^32 where c = 0 ---------------------------------------------------------
    // d = a + b + c and e should be unset, since there was no overflow
    test_op_execution(asm_op, &[0, 1, 2], &[0, 3]);

    // --- (a + b + c) < 2^32 where c = 1 ---------------------------------------------------------
    // d = a + b + c and e should be unset, since there was no overflow
    test_op_execution(asm_op, &[1, 2, 3], &[0, 6]);

    // --- (a + b + c) = 2^32 ---------------------------------------------------------------------
    let a = u32::MAX;
    let b = 1_u64;
    // d should be the sum mod 2^32 and e should be set to signal overflow
    test_op_execution(asm_op, &[0, a as u64, b], &[1, 0]);

    // --- (a + b + c) > 2^32 ---------------------------------------------------------------------
    let a = 1_u64;
    let b = u32::MAX;
    // d should be the sum mod 2^32 and e should be set to signal overflow
    test_op_execution(asm_op, &[1, a, b as u64], &[1, 1]);

    // --- random u32 values with c = 0 -----------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let c = 0_u64;
    let (d, overflow) = a.overflowing_add(b);
    let e = if overflow { 1 } else { 0 };
    test_op_execution(asm_op, &[c, a as u64, b as u64], &[e, d as u64]);

    // --- random u32 values with c = 1 -----------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let c = 1_u32;
    let (d, overflow_b) = a.overflowing_add(b);
    let (d, overflow_c) = d.overflowing_add(c);
    let e = if overflow_b || overflow_c { 1 } else { 0 };
    test_op_execution(asm_op, &[c as u64, a as u64, b as u64], &[e, d as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let f = rand_value::<u64>();
    test_op_execution(
        asm_op,
        &[f, c as u64, a as u64, b as u64],
        &[e, d as u64, f],
    );
}

/// This helper function tests overflowing subtraction for two u32 inputs for a number of simple
/// cases as well as for random values. It checks that a result of (a + b) % 2^32 is pushed to the
/// stack, as well as a flag indicating whether or not arithmetic overflow occurred. Finally, it
/// ensures that the rest of the stack was unaffected.
fn test_sub_full(asm_op: &str) {
    // --- a > b -------------------------------------------------------------------------
    // c = a - b and d should be unset, since there was no arithmetic overflow
    test_op_execution(asm_op, &[2, 1], &[0, 1]);

    // --- a = b -------------------------------------------------------------------------
    // c = a - b and d should be unset, since there was no arithmetic overflow
    test_op_execution(asm_op, &[1, 1], &[0, 0]);

    // --- a < b -------------------------------------------------------------------------
    // c = a - b % 2^32 and d should be set, since there was arithmetic overflow
    test_op_execution(asm_op, &[1, 2], &[1, u32::MAX as u64]);

    // --- random u32 values: a >= b --------------------------------------------------------------
    let val1 = rand_value::<u64>() as u32;
    let val2 = rand_value::<u64>() as u32;
    let (a, b) = if val1 >= val2 {
        (val1, val2)
    } else {
        (val2, val1)
    };
    let c = a - b;
    test_op_execution(asm_op, &[a as u64, b as u64], &[0, c as u64]);

    // --- random u32 values: a < b ---------------------------------------------------------------
    let val1 = rand_value::<u64>() as u32;
    let val2 = rand_value::<u64>() as u32;
    let (a, b) = if val1 >= val2 {
        (val2, val1)
    } else {
        (val1, val2)
    };
    let (c, _) = a.overflowing_sub(b);
    let d = 1;
    test_op_execution(asm_op, &[a as u64, b as u64], &[d, c as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let e = rand_value::<u64>();
    test_op_execution(asm_op, &[e, a as u64, b as u64], &[d, c as u64, e]);
}

/// This helper function tests overflowing multiplication for two u32 inputs for a number of simple
/// cases as well as for random values. It checks that a result of (a * b) % 2^32 is pushed to the
/// stack, as well as a value of (a * b) / 2^32 indicating the number of times multiplication
/// overflowed. Finally, it ensures that the rest of the stack was unaffected.
fn test_mul_full(asm_op: &str) {
    // --- no overflow ----------------------------------------------------------------------------
    // c = a * b and d should be unset, since there was no arithmetic overflow
    test_op_execution(asm_op, &[1, 2], &[0, 2]);

    // --- overflow once --------------------------------------------------------------------------
    // c = a * b and d = 1, since it overflows once
    test_op_execution(asm_op, &[U32_BOUND / 2, 2], &[1, 0]);

    // --- multiple overflows ---------------------------------------------------------------------
    // c = a * b and d = 2, since it overflows twice
    test_op_execution(asm_op, &[U32_BOUND / 2, 4], &[2, 0]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let (c, overflow) = a.overflowing_mul(b);
    let d = if !overflow {
        0
    } else {
        (a as u64 * b as u64) / U32_BOUND
    };
    test_op_execution(asm_op, &[a as u64, b as u64], &[d, c as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let e = rand_value::<u64>();
    test_op_execution(asm_op, &[e, a as u64, b as u64], &[d, c as u64, e]);
}

/// This helper function tests multiply and add for three u32 inputs for a number of simple cases
/// as well as for random values. It checks that a result of (a * b + c) % 2^32 is pushed to the
/// stack, along with a value of (a * b + c) / 2^32 indicating the number of times the operation
/// overflowed. Finally, it ensures that the rest of the stack was unaffected.
fn test_madd(asm_op: &str) {
    // --- no overflow ----------------------------------------------------------------------------
    // d = a * b + c and e should be unset, since there was no arithmetic overflow
    test_op_execution(asm_op, &[1, 0, 0], &[0, 1]);
    test_op_execution(asm_op, &[3, 1, 2], &[0, 5]);

    // --- overflow once --------------------------------------------------------------------------
    // c = a * b and d = 1, since it overflows once
    test_op_execution(asm_op, &[1, U32_BOUND / 2, 2], &[1, 1]);

    // --- multiple overflows ---------------------------------------------------------------------
    // c = a * b and d = 2, since it overflows twice
    test_op_execution(asm_op, &[1, U32_BOUND / 2, 4], &[2, 1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let c = rand_value::<u64>() as u32;
    let madd = a as u64 * b as u64 + c as u64;
    let d = madd % U32_BOUND;
    let e = madd / U32_BOUND;
    test_op_execution(asm_op, &[c as u64, a as u64, b as u64], &[e, d]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let f = rand_value::<u64>();
    test_op_execution(asm_op, &[f, c as u64, a as u64, b as u64], &[e, d, f]);
}

/// This helper function tests division with remainder for two u32 inputs for a number of simple
/// cases as well as for random values. It checks that the floor of a / b is pushed to the
/// stack, along with the remainder a % b. Finally, it ensures that the rest of the stack was
/// unaffected.
fn test_div_full(asm_op: &str) {
    // --- simple cases ---------------------------------------------------------------------------
    // division with no remainder
    test_op_execution(asm_op, &[2, 1], &[0, 2]);
    // division with remainder
    test_op_execution(asm_op, &[1, 2], &[1, 0]);
    test_op_execution(asm_op, &[3, 2], &[1, 1]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let b = rand_value::<u64>() as u32;
    let quot = (a / b) as u64;
    let rem = (a % b) as u64;
    test_op_execution(asm_op, &[a as u64, b as u64], &[rem, quot]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let e = rand_value::<u64>();
    test_op_execution(asm_op, &[e, a as u64, b as u64], &[rem, quot, e]);
}

/// This helper function tests the modulus operation for two u32 inputs for a number of simple
/// cases as well as for random values. It checks that a % b is pushed to the stack. Finally, it
/// ensures that the rest of the stack was unaffected.
fn test_mod(asm_op: &str) {
    // --- simple cases ---------------------------------------------------------------------------
    test_op_execution(asm_op, &[10, 5], &[0]);
    test_op_execution(asm_op, &[11, 5], &[1]);
    test_op_execution(asm_op, &[5, 11], &[5]);

    // --- random u32 values ----------------------------------------------------------------------
    let a = rand_value::<u64>() as u32;
    let mut b = rand_value::<u64>() as u32;
    if b == 0 {
        // ensure we're not using a failure case
        b += 1;
    }
    let expected = a % b;
    test_op_execution(asm_op, &[a as u64, b as u64], &[expected as u64]);

    // --- test that the rest of the stack isn't affected -----------------------------------------
    let c = rand_value::<u64>();
    test_op_execution(asm_op, &[c, a as u64, b as u64], &[expected as u64, c]);
}
