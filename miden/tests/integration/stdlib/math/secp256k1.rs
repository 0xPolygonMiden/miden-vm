use super::{build_test, Felt};
use test_case::test_case;

#[test]
fn test_to_and_from_mont_repr() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::to_mont
        exec.secp256k1::from_mont
    end";

    let mut num = [0u32; 8];
    for i in 0..4 {
        let a = rand_utils::rand_value::<u32>();
        let b = rand_utils::rand_value::<u32>();

        num[i] = a;
        num[i ^ 4] = b;
    }

    let mut stack = [0u64; 8];
    for i in 0..4 {
        stack[i] = num[i] as u64;
        stack[i ^ 4] = num[i ^ 4] as u64;
    }
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(num[i] as u64), strace[i]);
    }
}

#[test]
fn test_u256_mod_mul() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::u256_mod_mul
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u32>() as u64;
        let b = rand_utils::rand_value::<u32>() as u64;

        stack[i] = a;
        stack[i ^ 8] = b;
    }

    let mut a = [0u32; 8];
    let mut b = [0u32; 8];

    for i in 0..8 {
        a[i] = stack[i] as u32;
        b[i] = stack[i ^ 8] as u32;
    }

    let expected = u256xu256_mod_mult(&a, &b);

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(expected[i] as u64), strace[i]);
    }
}

#[test]
fn test_u256_mod_add() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::u256_mod_add
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u32>() as u64;
        let b = rand_utils::rand_value::<u32>() as u64;

        stack[i] = a;
        stack[i ^ 8] = b;
    }

    let mut a = [0u32; 8];
    let mut b = [0u32; 8];

    for i in 0..8 {
        a[i] = stack[i] as u32;
        b[i] = stack[i ^ 8] as u32;
    }

    let expected = u256_mod_add(a, b);

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(expected[i] as u64), strace[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn test_u256_mod_neg() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::u256_mod_neg
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        stack[i] = rand_utils::rand_value::<u32>() as u64;
    }

    let mut a = [0u32; 8];
    for i in 0..8 {
        a[i] = stack[i] as u32;
    }

    let expected = u256_mod_neg(a);

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(expected[i] as u64), strace[i]);
    }
}

#[test]
fn test_u256_mod_sub() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::u256_mod_sub
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u32>() as u64;
        let b = rand_utils::rand_value::<u32>() as u64;

        stack[i] = a;
        stack[i ^ 8] = b;
    }

    let mut a = [0u32; 8];
    let mut b = [0u32; 8];

    for i in 0..8 {
        a[i] = stack[i] as u32;
        b[i] = stack[i ^ 8] as u32;
    }

    let expected = u256_mod_sub(a, b);

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(expected[i] as u64), strace[i]);
    }
}

#[test]
fn test_u256_mod_add_sub_cycle() {
    let source_add = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::u256_mod_add
    end";

    let source_sub = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::u256_mod_sub
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u32>() as u64;
        let b = rand_utils::rand_value::<u32>() as u64;

        stack[i] = a;
        stack[i ^ 8] = b;
    }

    let mut a = [0u32; 8];
    let mut b = [0u32; 8];

    // randomly generate a, b --- two secp256k1 field elements
    for i in 0..8 {
        a[i] = stack[i] as u32;
        b[i] = stack[i ^ 8] as u32;
    }

    // compute c = a + b
    let c = {
        stack.reverse();

        let c = u256_mod_add(a, b);

        let test = build_test!(source_add, &stack);
        let strace = test.get_last_stack_state();

        for i in 0..8 {
            assert_eq!(Felt::new(c[i] as u64), strace[i]);
        }

        c
    };

    for i in 0..8 {
        stack[i] = c[i] as u64;
        stack[i ^ 8] = a[i] as u64;
    }

    // compute d = c - a
    let d = {
        stack.reverse();

        let d = u256_mod_sub(c, a);

        let test = build_test!(source_sub, &stack);
        let strace = test.get_last_stack_state();

        for i in 0..8 {
            assert_eq!(Felt::new(d[i] as u64), strace[i]);
        }

        d
    };

    // check b == d | (d = c - a) & (c = a + b)
    for i in 0..8 {
        assert_eq!(b[i] ^ d[i], 0);
    }
}

#[test]
fn test_secp256k1_point_doubling() {
    let source = "
    use.std::math::secp256k1

    # Given generator point of secp256k1 elliptic curve, this routine first computes
    # point doubling of generator point in projective coordinate & then asserts
    # each coordinate limb-by-limb for ensuring correctness.
    #
    # Note, this test is not yet very generic i.e. it can't be generalized to work
    # with any point generated from curve generator & test for correctness of execution
    # of point doubling assembly routine. This is what I'd like to make it, in sometime future.
    proc.point_doubling_test_wrapper.12
        # push X -coordinate to memory
        push.589179219.700212955.3610652250.1216225431
        loc_storew.0
        dropw
        push.2575427139.3909656392.2543798464.872223388
        loc_storew.1
        dropw

        # push Y -coordinate to memory
        push.2382126429.522045005.2975770322.3554388962
        loc_storew.2
        dropw
        push.3477046559.3567616726.1891022234.2887369014
        loc_storew.3
        dropw

        # push Z -coordinate to memory
        push.0.0.1.977
        loc_storew.4
        dropw
        push.0.0.0.0
        loc_storew.5
        dropw

        # input/ output memory addresses for point doubling purpose
        locaddr.11
        locaddr.10
        locaddr.9
        locaddr.8
        locaddr.7
        locaddr.6

        locaddr.5
        locaddr.4
        locaddr.3
        locaddr.2
        locaddr.1
        locaddr.0

        # elliptic curve point doubling
        exec.secp256k1::point_doubling

        # --- start asserting X3 ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.474728642
        assert
        u32checked_eq.4256012599
        assert
        u32checked_eq.2072183026
        assert
        u32checked_eq.3437933890
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.4191201175
        assert
        u32checked_eq.1644336685
        assert
        u32checked_eq.3276311816
        assert
        u32checked_eq.617223735
        assert
        # --- end asserting X3 ---

        # --- start asserting Y3 ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.3875396767
        assert
        u32checked_eq.483526712
        assert
        u32checked_eq.3043178571
        assert
        u32checked_eq.2826781693
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.2758035882
        assert
        u32checked_eq.3425160008
        assert
        u32checked_eq.524996660
        assert
        u32checked_eq.1440660280
        assert
        # --- end asserting Y3 ---

        # --- start asserting Z3 ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.2545792257
        assert
        u32checked_eq.4082826636
        assert
        u32checked_eq.1673463056
        assert
        u32checked_eq.2688095969
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.2687252166
        assert
        u32checked_eq.3884180958
        assert
        u32checked_eq.1848170264
        assert
        u32checked_eq.579919648
        assert
        # --- end asserting Z3 ---
    end

    begin
        exec.point_doubling_test_wrapper
    end";

    let test = build_test!(source, &[]);
    let _ = test.get_last_stack_state();
}

#[test]
fn test_secp256k1_point_addition() {
    let source = "
    use.std::math::secp256k1

    # Given generator point of secp256k1 elliptic curve ( twice ), this routine first computes
    # point addition of generator point with itself ( i.e. it's equivalent to point doubling ) 
    # in projective coordinate & then asserts each coordinate limb-by-limb for ensuring correctness.
    #
    # Note, this test is not yet very generic i.e. it can't be generalized to work
    # with any points generated from curve generator & test for correctness of execution
    # of point addition assembly routine. This is what I'd like to make it, in sometime future.
    proc.point_addition_test_wrapper.18
        # push X1 -coordinate to memory
        push.589179219.700212955.3610652250.1216225431
        loc_storew.0
        dropw
        push.2575427139.3909656392.2543798464.872223388
        loc_storew.1
        dropw

        # push Y1 -coordinate to memory
        push.2382126429.522045005.2975770322.3554388962
        loc_storew.2
        dropw
        push.3477046559.3567616726.1891022234.2887369014
        loc_storew.3
        dropw

        # push Z1 -coordinate to memory
        push.0.0.1.977
        loc_storew.4
        dropw
        push.0.0.0.0
        loc_storew.5
        dropw

        # push X2 -coordinate to memory
        push.589179219.700212955.3610652250.1216225431
        loc_storew.6
        dropw
        push.2575427139.3909656392.2543798464.872223388
        loc_storew.7
        dropw

        # push Y2 -coordinate to memory
        push.2382126429.522045005.2975770322.3554388962
        loc_storew.8
        dropw
        push.3477046559.3567616726.1891022234.2887369014
        loc_storew.9
        dropw

        # push Z2 -coordinate to memory
        push.0.0.1.977
        loc_storew.10
        dropw
        push.0.0.0.0
        loc_storew.11
        dropw

        # input/ output memory addresses for point doubling purpose
        locaddr.17
        locaddr.16
        locaddr.15
        locaddr.14
        locaddr.13
        locaddr.12

        locaddr.11
        locaddr.10
        locaddr.9
        locaddr.8
        locaddr.7
        locaddr.6

        locaddr.5
        locaddr.4
        locaddr.3
        locaddr.2
        locaddr.1
        locaddr.0

        # elliptic curve point addition
        exec.secp256k1::point_addition

        # --- start asserting X3 ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.474728642
        assert
        u32checked_eq.4256012599
        assert
        u32checked_eq.2072183026
        assert
        u32checked_eq.3437933890
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.4191201175
        assert
        u32checked_eq.1644336685
        assert
        u32checked_eq.3276311816
        assert
        u32checked_eq.617223735
        assert
        # --- end asserting X3 ---

        # --- start asserting Y3 ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.3875396767
        assert
        u32checked_eq.483526712
        assert
        u32checked_eq.3043178571
        assert
        u32checked_eq.2826781693
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.2758035882
        assert
        u32checked_eq.3425160008
        assert
        u32checked_eq.524996660
        assert
        u32checked_eq.1440660280
        assert
        # --- end asserting Y3 ---

        # --- start asserting Z3 ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.2545792257
        assert
        u32checked_eq.4082826636
        assert
        u32checked_eq.1673463056
        assert
        u32checked_eq.2688095969
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.2687252166
        assert
        u32checked_eq.3884180958
        assert
        u32checked_eq.1848170264
        assert
        u32checked_eq.579919648
        assert
        # --- end asserting Z3 ---
    end

    begin
        exec.point_addition_test_wrapper
    end";

    let test = build_test!(source, &[]);
    let _ = test.get_last_stack_state();
}

struct FieldElement([u32; 8]);
struct Point([FieldElement; 3]);

#[test_case(Point([FieldElement([1725045020, 1243934934, 83748696, 1271163719, 2490525753, 3709155749, 1579712529, 1757200845]), FieldElement([258440691, 3022796594, 2607846704, 163567449, 1396660245, 61235791, 73386979, 3569717]), FieldElement([628236075, 1776096883, 1596640373, 1237597377, 2238764922, 2503475385, 3619273576, 3366549089])]), FieldElement([2301743426,2075099376, 2969588298, 1793611799, 2457684815, 3951838026, 2737387451, 3754378978]), Point([FieldElement([1557300347, 3826368586, 2537306948, 1194350582, 2206313690, 2155850976, 910320597, 3536848074]), FieldElement([124257772, 3353686949, 2778858866, 3272416768, 3192211612, 670334657, 2786774514, 1334286332]), FieldElement([2312297066, 2925488368, 3267009695, 2498870966, 1732427718, 4239428087, 1550410695, 627716766])]); "0")]
#[test_case(Point([FieldElement([1557300347, 3826368586, 2537306948, 1194350582, 2206313690, 2155850976, 910320597, 3536848074]), FieldElement([124257772, 3353686949, 2778858866, 3272416768, 3192211612, 670334657, 2786774514, 1334286332]), FieldElement([2312297066, 2925488368, 3267009695, 2498870966, 1732427718, 4239428087, 1550410695, 627716766])]), FieldElement([2301743426,2075099376, 2969588298, 1793611799, 2457684815, 3951838026, 2737387451, 3754378978]), Point([FieldElement([3527372762, 3507857639, 1594370824, 3718082544, 2518725024, 2545775599, 1088522187, 1093635599]),FieldElement([3614258408, 1260438099, 1063020787, 456123286, 4107569356, 1151599969, 3890268877, 1968252526]), FieldElement([3558741386, 268995358, 367673520, 1545535419, 2508499329, 1109236387, 895079977, 1740167655])]); "1")]
#[test_case(Point([FieldElement([3527372762, 3507857639, 1594370824, 3718082544, 2518725024, 2545775599, 1088522187, 1093635599]), FieldElement([3614258408, 1260438099, 1063020787, 456123286, 4107569356, 1151599969, 3890268877, 1968252526]), FieldElement([3558741386, 268995358, 367673520, 1545535419, 2508499329, 1109236387, 895079977, 1740167655])]), FieldElement([2301743426,2075099376, 2969588298, 1793611799, 2457684815, 3951838026, 2737387451, 3754378978]), Point([FieldElement([3575801888, 1578089417, 2395624562, 564065581, 2066984214, 2348140603, 765785243, 3808292373]), FieldElement([361245020, 2527203120, 3484075690, 3129019989, 661091683, 2687745598, 4167871392, 778426466]), FieldElement([3338036891, 4208971587, 1993683533, 4189224997, 2780649411, 2819629975, 3646250205, 1195817501])]); "2")]
fn test_secp256k1_point_multiplication(src_point: Point, scalar: FieldElement, dst_point: Point) {
    let source = format!(
        "
    use.std::math::secp256k1

    # Given an elliptic curve point ( in projective coordinate system ) and a 256 -bit scalar 
    # in radix-2^32 form ( i.e. 8 limbs, each of 32 -bit width ), this routine first multiplies
    # the EC point with provided scalar and then asserts for correctness with known answer.
    proc.point_multiplication_test_wrapper.12
        # resulting point
        locaddr.11
        locaddr.10
        locaddr.9
        locaddr.8
        locaddr.7
        locaddr.6

        # scalar
        push.{}.{}.{}.{}
        push.{}.{}.{}.{}

        # EC point
        push.{}.{}.{}.{}
        loc_storew.0
        dropw

        push.{}.{}.{}.{}
        loc_storew.1
        dropw

        push.{}.{}.{}.{}
        loc_storew.2
        dropw

        push.{}.{}.{}.{}
        loc_storew.3
        dropw
        
        push.{}.{}.{}.{}
        loc_storew.4
        dropw

        push.{}.{}.{}.{}
        loc_storew.5
        dropw

        locaddr.5
        locaddr.4
        locaddr.3
        locaddr.2
        locaddr.1
        locaddr.0

        # elliptic curve point multiplication
        exec.secp256k1::point_mul

        # --- start asserting X ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        # --- end asserting X ---

        # --- start asserting Y ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        # --- end asserting Y ---

        # --- start asserting Z ---
        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert

        push.0.0.0.0
        movup.4
        mem_loadw

        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        u32checked_eq.{}
        assert
        # --- end asserting Z ---
    end

    begin
        exec.point_multiplication_test_wrapper
    end",
        scalar.0[7],
        scalar.0[6],
        scalar.0[5],
        scalar.0[4],
        scalar.0[3],
        scalar.0[2],
        scalar.0[1],
        scalar.0[0],
        src_point.0[0].0[3],
        src_point.0[0].0[2],
        src_point.0[0].0[1],
        src_point.0[0].0[0],
        src_point.0[0].0[7],
        src_point.0[0].0[6],
        src_point.0[0].0[5],
        src_point.0[0].0[4],
        src_point.0[1].0[3],
        src_point.0[1].0[2],
        src_point.0[1].0[1],
        src_point.0[1].0[0],
        src_point.0[1].0[7],
        src_point.0[1].0[6],
        src_point.0[1].0[5],
        src_point.0[1].0[4],
        src_point.0[2].0[3],
        src_point.0[2].0[2],
        src_point.0[2].0[1],
        src_point.0[2].0[0],
        src_point.0[2].0[7],
        src_point.0[2].0[6],
        src_point.0[2].0[5],
        src_point.0[2].0[4],
        dst_point.0[0].0[0],
        dst_point.0[0].0[1],
        dst_point.0[0].0[2],
        dst_point.0[0].0[3],
        dst_point.0[0].0[4],
        dst_point.0[0].0[5],
        dst_point.0[0].0[6],
        dst_point.0[0].0[7],
        dst_point.0[1].0[0],
        dst_point.0[1].0[1],
        dst_point.0[1].0[2],
        dst_point.0[1].0[3],
        dst_point.0[1].0[4],
        dst_point.0[1].0[5],
        dst_point.0[1].0[6],
        dst_point.0[1].0[7],
        dst_point.0[2].0[0],
        dst_point.0[2].0[1],
        dst_point.0[2].0[2],
        dst_point.0[2].0[3],
        dst_point.0[2].0[4],
        dst_point.0[2].0[5],
        dst_point.0[2].0[6],
        dst_point.0[2].0[7],
    );

    let test = build_test!(source, &[]);
    test.execute().unwrap();
}

fn mac(a: u32, b: u32, c: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 + (b as u64 * c as u64) + carry as u64;
    ((tmp >> 32) as u32, tmp as u32)
}

fn adc(a: u32, b: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 + b as u64 + carry as u64;
    ((tmp >> 32) as u32, tmp as u32)
}

fn sbb(a: u32, b: u32, borrow: u32) -> (u32, u32) {
    let tmp = (a as u64).wrapping_sub(b as u64 + (borrow >> 31) as u64);
    ((tmp >> 32) as u32, tmp as u32)
}

fn u256xu32(a: &mut [u32], b: u32, c: &[u32]) {
    assert_eq!(a.len(), 9);
    assert_eq!(c.len(), 8);

    let mut carry: u32;

    let v = mac(a[0], b, c[0], 0);
    carry = v.0;
    a[0] = v.1;

    let v = mac(a[1], b, c[1], carry);
    carry = v.0;
    a[1] = v.1;

    let v = mac(a[2], b, c[2], carry);
    carry = v.0;
    a[2] = v.1;

    let v = mac(a[3], b, c[3], carry);
    carry = v.0;
    a[3] = v.1;

    let v = mac(a[4], b, c[4], carry);
    carry = v.0;
    a[4] = v.1;

    let v = mac(a[5], b, c[5], carry);
    carry = v.0;
    a[5] = v.1;

    let v = mac(a[6], b, c[6], carry);
    carry = v.0;
    a[6] = v.1;

    let v = mac(a[7], b, c[7], carry);
    a[8] = v.0;
    a[7] = v.1;
}

fn u288_reduce(c: &[u32], pc: u32) -> [u32; 9] {
    assert_eq!(c.len(), 9);

    let prime: [u32; 8] = [
        4294966319, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295,
    ];
    let mu: u32 = 3525653809;

    let q: u32 = mu.wrapping_mul(c[0]);
    let mut carry: u32;
    let mut d = [0u32; 9];

    let v = mac(c[0], q, prime[0], 0);
    carry = v.0;

    let v = mac(c[1], q, prime[1], carry);
    carry = v.0;
    d[0] = v.1;

    let v = mac(c[2], q, prime[2], carry);
    carry = v.0;
    d[1] = v.1;

    let v = mac(c[3], q, prime[3], carry);
    carry = v.0;
    d[2] = v.1;

    let v = mac(c[4], q, prime[4], carry);
    carry = v.0;
    d[3] = v.1;

    let v = mac(c[5], q, prime[5], carry);
    carry = v.0;
    d[4] = v.1;

    let v = mac(c[6], q, prime[6], carry);
    carry = v.0;
    d[5] = v.1;

    let v = mac(c[7], q, prime[7], carry);
    carry = v.0;
    d[6] = v.1;

    let v = adc(c[8], pc, carry);
    d[8] = v.0;
    d[7] = v.1;

    d
}

/// See `mont_mult` routine in https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6
fn u256xu256_mod_mult(a: &[u32], b: &[u32]) -> [u32; 8] {
    assert_eq!(a.len(), 8);
    assert_eq!(a.len(), b.len());

    let mut c = [0u32; 16];
    let mut pc = 0u32;

    u256xu32(&mut c[0..9], b[0], a);

    let d = u288_reduce(&c[0..9], pc);
    pc = d[8];
    c[1..9].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[1..10], b[1], a);

    let d = u288_reduce(&c[1..10], pc);
    pc = d[8];
    c[2..10].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[2..11], b[2], a);

    let d = u288_reduce(&c[2..11], pc);
    pc = d[8];
    c[3..11].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[3..12], b[3], a);

    let d = u288_reduce(&c[3..12], pc);
    pc = d[8];
    c[4..12].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[4..13], b[4], a);

    let d = u288_reduce(&c[4..13], pc);
    pc = d[8];
    c[5..13].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[5..14], b[5], a);

    let d = u288_reduce(&c[5..14], pc);
    pc = d[8];
    c[6..14].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[6..15], b[6], a);

    let d = u288_reduce(&c[6..15], pc);
    pc = d[8];
    c[7..15].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[7..16], b[7], a);

    let d = u288_reduce(&c[7..16], pc);
    pc = d[8];
    c[8..16].copy_from_slice(&d[0..8]);

    c[8] += pc * 977;
    c[9] += pc;

    c[8..16].try_into().expect("incorrect length")
}

/// See https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6#file-test_montgomery_arithmetic-py-L236-L256
fn u256_mod_add(a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
    let mut c = [0u32; 8];
    let mut carry = 0u32;

    let v = adc(a[0], b[0], carry);
    carry = v.0;
    c[0] = v.1;

    let v = adc(a[1], b[1], carry);
    carry = v.0;
    c[1] = v.1;

    let v = adc(a[2], b[2], carry);
    carry = v.0;
    c[2] = v.1;

    let v = adc(a[3], b[3], carry);
    carry = v.0;
    c[3] = v.1;

    let v = adc(a[4], b[4], carry);
    carry = v.0;
    c[4] = v.1;

    let v = adc(a[5], b[5], carry);
    carry = v.0;
    c[5] = v.1;

    let v = adc(a[6], b[6], carry);
    carry = v.0;
    c[6] = v.1;

    let v = adc(a[7], b[7], carry);
    carry = v.0;
    c[7] = v.1;

    c[0] += carry * 977;
    c[1] += carry;

    c
}

/// See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/field.py#L77-L95
fn u256_mod_neg(a: [u32; 8]) -> [u32; 8] {
    let mut b = [0u32; 8];
    let mut borrow = 0u32;

    let prime = [
        4294966319, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295,
    ];

    let v = sbb(prime[0], a[0], borrow);
    borrow = v.0;
    b[0] = v.1;

    let v = sbb(prime[1], a[1], borrow);
    borrow = v.0;
    b[1] = v.1;

    let v = sbb(prime[2], a[2], borrow);
    borrow = v.0;
    b[2] = v.1;

    let v = sbb(prime[3], a[3], borrow);
    borrow = v.0;
    b[3] = v.1;

    let v = sbb(prime[4], a[4], borrow);
    borrow = v.0;
    b[4] = v.1;

    let v = sbb(prime[5], a[5], borrow);
    borrow = v.0;
    b[5] = v.1;

    let v = sbb(prime[6], a[6], borrow);
    borrow = v.0;
    b[6] = v.1;

    let v = sbb(prime[7], a[7], borrow);
    b[7] = v.1;

    b
}

/// See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/field.py#L97-L101
fn u256_mod_sub(a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
    u256_mod_add(a, u256_mod_neg(b))
}
