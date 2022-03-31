use super::{build_test, TestError};
use proptest::prelude::*;
use rand_utils::rand_value;

#[test]
fn add_unsafe() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a.wrapping_add(b);

    let source = "
        use.std::math::u64
        begin
            exec.u64::add_unsafe
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);
}

#[test]
fn sub_unsafe() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a.wrapping_sub(b);

    let source = "
        use.std::math::u64
        begin
            exec.u64::sub_unsafe
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);
}

#[test]
fn mul_unsafe() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a.wrapping_mul(b);

    let source = "
        use.std::math::u64
        begin
            exec.u64::mul_unsafe
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);
}

// COMPARISONS
// ------------------------------------------------------------------------------------------------

#[test]
fn lt_unsafe() {
    // test a few manual cases; randomized tests are done using proptest
    let source = "
        use.std::math::u64
        begin
            exec.u64::lt_unsafe
        end";

    // a = 0, b = 0
    build_test!(source, &[0, 0, 0, 0]).expect_stack(&[0]);

    // a = 0, b = 1
    build_test!(source, &[0, 0, 1, 0]).expect_stack(&[1]);

    // a = 1, b = 0
    build_test!(source, &[1, 0, 0, 0]).expect_stack(&[0]);
}

#[test]
fn lte_unsafe() {
    let source = "
        use.std::math::u64
        begin
            exec.u64::lte_unsafe
        end";

    // a = 0, b = 0
    build_test!(source, &[0, 0, 0, 0]).expect_stack(&[1]);

    // a = 0, b = 1
    build_test!(source, &[0, 0, 1, 0]).expect_stack(&[1]);

    // a = 1, b = 0
    build_test!(source, &[1, 0, 0, 0]).expect_stack(&[0]);

    // randomized test
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = (a <= b) as u64;

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    build_test!(source, &[a0, a1, b0, b1]).expect_stack(&[c]);
}

#[test]
fn gt_unsafe() {
    // test a few manual cases; randomized tests are done using proptest
    let source = "
        use.std::math::u64
        begin
            exec.u64::gt_unsafe
        end";

    // a = 0, b = 0
    build_test!(source, &[0, 0, 0, 0]).expect_stack(&[0]);

    // a = 0, b = 1
    build_test!(source, &[0, 0, 1, 0]).expect_stack(&[0]);

    // a = 1, b = 0
    build_test!(source, &[1, 0, 0, 0]).expect_stack(&[1]);
}

#[test]
fn gte_unsafe() {
    let source = "
        use.std::math::u64
        begin
            exec.u64::gte_unsafe
        end";

    // a = 0, b = 0
    build_test!(source, &[0, 0, 0, 0]).expect_stack(&[1]);

    // a = 0, b = 1
    build_test!(source, &[0, 0, 1, 0]).expect_stack(&[0]);

    // a = 1, b = 0
    build_test!(source, &[1, 0, 0, 0]).expect_stack(&[1]);

    // randomized test
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = (a >= b) as u64;

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    build_test!(source, &[a0, a1, b0, b1]).expect_stack(&[c]);
}

#[test]
fn eq_unsafe() {
    let source = "
        use.std::math::u64
        begin
            exec.u64::eq_unsafe
        end";

    // a = 0, b = 0
    build_test!(source, &[0, 0, 0, 0]).expect_stack(&[1]);

    // a = 0, b = 1
    build_test!(source, &[0, 0, 1, 0]).expect_stack(&[0]);

    // a = 1, b = 0
    build_test!(source, &[1, 0, 0, 0]).expect_stack(&[0]);

    // randomized test
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = (a == b) as u64;

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    build_test!(source, &[a0, a1, b0, b1]).expect_stack(&[c]);
}

#[test]
fn eqz_unsafe() {
    let source = "
        use.std::math::u64
        begin
            exec.u64::eqz_unsafe
        end";

    // a = 0
    build_test!(source, &[0, 0]).expect_stack(&[1]);

    // a = 1
    build_test!(source, &[1, 0]).expect_stack(&[0]);

    // randomized test
    let a: u64 = rand_value();
    let c = (a == 0) as u64;

    let (a1, a0) = split_u64(a);
    build_test!(source, &[a0, a1]).expect_stack(&[c]);
}

// DIVISION
// ------------------------------------------------------------------------------------------------

#[test]
fn div_unsafe() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a / b;

    let source = "
        use.std::math::u64
        begin
            exec.u64::div_unsafe
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);

    let d = a / b0;
    let (d1, d0) = split_u64(d);

    let test = build_test!(source, &[a0, a1, b0, 0]);
    test.expect_stack(&[d1, d0]);
}

// MODULO OPERATION
// ------------------------------------------------------------------------------------------------

#[test]
fn mod_unsafe() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a % b;

    let source = "
        use.std::math::u64
        begin
            exec.u64::mod_unsafe
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);

    let d = a % b0;
    let (d1, d0) = split_u64(d);

    let test = build_test!(source, &[a0, a1, b0, 0]);
    test.expect_stack(&[d1, d0]);
}

// BITWISE OPERATIONS
// ------------------------------------------------------------------------------------------------

#[test]
fn and() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a & b;

    let source = "
        use.std::math::u64
        begin
            exec.u64::and
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);
}

#[test]
fn and_fail() {
    let a0: u64 = rand_value();
    let b0: u64 = rand_value();

    let a1: u64 = u32::MAX as u64 + 1;
    let b1: u64 = u32::MAX as u64 + 1;

    let source = "
        use.std::math::u64
        begin
            exec.u64::and
        end";

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));
}

#[test]
fn or() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a | b;

    let source = "
        use.std::math::u64
        begin
            exec.u64::or
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);
}

#[test]
fn or_fail() {
    let a0: u64 = rand_value();
    let b0: u64 = rand_value();

    let a1: u64 = u32::MAX as u64 + 1;
    let b1: u64 = u32::MAX as u64 + 1;

    let source = "
        use.std::math::u64
        begin
            exec.u64::or
        end";

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));
}

#[test]
fn xor() {
    let a: u64 = rand_value();
    let b: u64 = rand_value();
    let c = a ^ b;

    let source = "
        use.std::math::u64
        begin
            exec.u64::xor
        end";

    let (a1, a0) = split_u64(a);
    let (b1, b0) = split_u64(b);
    let (c1, c0) = split_u64(c);

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_stack(&[c1, c0]);
}

#[test]
fn xor_fail() {
    let a0: u64 = rand_value();
    let b0: u64 = rand_value();

    let a1: u64 = u32::MAX as u64 + 1;
    let b1: u64 = u32::MAX as u64 + 1;

    let source = "
        use.std::math::u64
        begin
            exec.u64::xor
        end";

    let test = build_test!(source, &[a0, a1, b0, b1]);
    test.expect_error(TestError::ExecutionError("NotU32Value"));
}

#[test]
fn shl() {
    let source = "
        use.std::math::u64
        begin
            exec.u64::shl
        end";

    // shift by 0
    let a: u64 = rand_value();
    let (a1, a0) = split_u64(a);
    let b: u32 = 0;

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[a1, a0, 5]);

    // shift by 31 (max lower limb of b)
    let b: u32 = 31;
    let c = a.wrapping_shl(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift by 32 (min for upper limb of b)
    let a = 1_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 32;
    let c = a.wrapping_shl(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift by 33
    let a = 1_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 33;
    let c = a.wrapping_shl(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift 64 by 58
    let a = 64_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 58;
    let c = a.wrapping_shl(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);
}

#[test]
fn shr() {
    let source = "
        use.std::math::u64
        begin
            exec.u64::shr
        end";

    // shift by 0
    let a: u64 = rand_value();
    let (a1, a0) = split_u64(a);
    let b: u32 = 0;

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[a1, a0, 5]);

    // simple right shift
    build_test!(source, &[5, 1, 1, 1]).expect_stack(&[0, 2_u64.pow(31), 5]);

    // simple right shift
    build_test!(source, &[5, 3, 3, 1]).expect_stack(&[1, 2_u64.pow(31) + 1, 5]);

    // shift by 31 (max lower limb of b)
    let b: u32 = 31;
    let c = a.wrapping_shr(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift by 32 (min for upper limb of b)
    let a = 1_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 32;
    let c = a.wrapping_shr(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift by 33
    let a = 1_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 33;
    let c = a.wrapping_shr(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift 4294967296 by 2
    let a = 4294967296;
    let (a1, a0) = split_u64(a);
    let b: u32 = 2;
    let c = a.wrapping_shr(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);
}

#[test]
fn rotl() {
    let source = "
        use.std::math::u64
        begin
            exec.u64::rotl
        end";

    // shift by 0
    let a: u64 = rand_value();
    let (a1, a0) = split_u64(a);
    let b: u32 = 0;

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[a1, a0, 5]);

    // shift by 31 (max lower limb of b)
    let b: u32 = 31;
    let c = a.rotate_left(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift by 32 (min for upper limb of b)
    let a = 1_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 32;
    let c = a.rotate_left(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift by 33
    let a = 1_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 33;
    let c = a.rotate_left(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift 64 by 58
    let a = 64_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 58;
    let c = a.rotate_left(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);
}

#[test]
fn rotr() {
    let source = "
        use.std::math::u64
        begin
            exec.u64::rotr
        end";

    // shift by 0
    let a: u64 = rand_value();
    let (a1, a0) = split_u64(a);
    let b: u32 = 0;

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[a1, a0, 5]);

    // shift by 31 (max lower limb of b)
    let b: u32 = 31;
    let c = a.rotate_right(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift by 32 (min for upper limb of b)
    let a = 1_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 32;
    let c = a.rotate_right(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift by 33
    let a = 1_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 33;
    let c = a.rotate_right(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);

    // shift 64 by 58
    let a = 64_u64;
    let (a1, a0) = split_u64(a);
    let b: u32 = 58;
    let c = a.rotate_right(b);
    let (c1, c0) = split_u64(c);

    build_test!(source, &[5, a0, a1, b as u64]).expect_stack(&[c1, c0, 5]);
}

// RANDOMIZED TESTS
// ================================================================================================

proptest! {
    #[test]
    fn lt_unsafe_proptest(a in any::<u64>(), b in any::<u64>()) {

        let (a1, a0) = split_u64(a);
        let (b1, b0) = split_u64(b);
        let c = (a < b) as u64;

        let source = "
            use.std::math::u64
            begin
                exec.u64::lt_unsafe
            end";

        build_test!(source, &[a0, a1, b0, b1]).prop_expect_stack(&[c])?;
    }

    #[test]
    fn gt_unsafe_proptest(a in any::<u64>(), b in any::<u64>()) {

        let (a1, a0) = split_u64(a);
        let (b1, b0) = split_u64(b);
        let c = (a > b) as u64;

        let source = "
            use.std::math::u64
            begin
                exec.u64::gt_unsafe
            end";

        build_test!(source, &[a0, a1, b0, b1]).prop_expect_stack(&[c])?;
    }

    #[test]
    fn div_unsafe_proptest(a in any::<u64>(), b in any::<u64>()) {

        let c = a / b;

        let (a1, a0) = split_u64(a);
        let (b1, b0) = split_u64(b);
        let (c1, c0) = split_u64(c);

        let source = "
            use.std::math::u64
            begin
                exec.u64::div_unsafe
            end";

        build_test!(source, &[a0, a1, b0, b1]).prop_expect_stack(&[c1, c0])?;
    }

    #[test]
    fn mod_unsafe_proptest(a in any::<u64>(), b in any::<u64>()) {

        let c = a % b;

        let (a1, a0) = split_u64(a);
        let (b1, b0) = split_u64(b);
        let (c1, c0) = split_u64(c);

        let source = "
            use.std::math::u64
            begin
                exec.u64::mod_unsafe
            end";

        build_test!(source, &[a0, a1, b0, b1]).prop_expect_stack(&[c1, c0])?;
    }

    fn shl_proptest(a in any::<u64>(), b in 0_u32..64) {

        let c = a.wrapping_shl(b);

        let (a1, a0) = split_u64(a);
        let (c1, c0) = split_u64(c);

        let source = "
        use.std::math::u64
        begin
            exec.u64::shl
        end";

        build_test!(source, &[5, a0, a1, b as u64]).prop_expect_stack(&[c1, c0, 5])?;
    }

    #[test]
    fn shr_proptest(a in any::<u64>(), b in 0_u32..64) {

        let c = a.wrapping_shr(b);

        let (a1, a0) = split_u64(a);
        let (c1, c0) = split_u64(c);

        let source = "
        use.std::math::u64
        begin
            exec.u64::shr
        end";

        build_test!(source, &[5, a0, a1, b as u64]).prop_expect_stack(&[c1, c0, 5])?;
    }

    #[test]
    fn rotl_proptest(a in any::<u64>(), b in 0_u32..64) {

        let c = a.rotate_left(b);

        let (a1, a0) = split_u64(a);
        let (c1, c0) = split_u64(c);

        let source = "
        use.std::math::u64
        begin
            exec.u64::rotl
        end";

        build_test!(source, &[5, a0, a1, b as u64]).prop_expect_stack(&[c1, c0, 5])?;
    }

    #[test]
    fn rotr_proptest(a in any::<u64>(), b in 0_u32..64) {

        let c = a.rotate_right(b);

        let (a1, a0) = split_u64(a);
        let (c1, c0) = split_u64(c);

        let source = "
        use.std::math::u64
        begin
            exec.u64::rotr
        end";

        build_test!(source, &[5, a0, a1, b as u64]).prop_expect_stack(&[c1, c0, 5])?;
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Split the provided u64 value into 32 high and low bits.
fn split_u64(value: u64) -> (u64, u64) {
    (value >> 32, value as u32 as u64)
}
