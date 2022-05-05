use super::{build_test, Felt};

fn mac(a: u32, b: u32, c: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 + (b as u64 * c as u64) + carry as u64;
    ((tmp >> 32) as u32, tmp as u32)
}

#[test]
fn test_mac() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::mac
    end";

    let stack = [
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
    ];

    let (hi, lo) = mac(
        stack[3] as u32,
        stack[2] as u32,
        stack[1] as u32,
        stack[0] as u32,
    );

    let test = build_test!(source, &stack);
    test.expect_stack(&[hi as u64, lo as u64]);
}

fn adc(a: u32, b: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 + b as u64 + carry as u64;
    return ((tmp >> 32) as u32, tmp as u32);
}

#[test]
fn test_adc() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::adc
    end";

    let stack = [
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
    ];

    let (hi, lo) = adc(stack[2] as u32, stack[1] as u32, stack[0] as u32);

    let test = build_test!(source, &stack);
    test.expect_stack(&[hi as u64, lo as u64]);
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

#[test]
fn test_u256xu32() {
    let source = "
    use.std::math::secp256k1

    proc.wrapper.2
        push.3102021493.1265174470.1329925018.4146020526
        popw.local.0
        push.1873376618.982499173.390191265.153760297
        popw.local.1

        push.env.locaddr.0
        movdn.9
        push.env.locaddr.1
        movdn.10

        exec.secp256k1::u256xu32
    end

    begin
        exec.wrapper
    end";

    let stack = [
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
    ];

    let mut a = [
        stack[8] as u32,
        stack[7] as u32,
        stack[6] as u32,
        stack[5] as u32,
        stack[4] as u32,
        stack[3] as u32,
        stack[2] as u32,
        stack[1] as u32,
        0,
    ];
    let b = stack[0] as u32;
    let c = [
        4146020526, 1329925018, 1265174470, 3102021493, 153760297, 390191265, 982499173, 1873376618,
    ];

    u256xu32(&mut a, b, &c);

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..9 {
        assert_eq!(Felt::new(a[i] as u64), strace[i]);
    }
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

#[test]
fn test_u288_reduce() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::u288_reduce
    end";

    let stack = [
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
    ];

    let c: [u32; 9] = [
        stack[9] as u32,
        stack[8] as u32,
        stack[7] as u32,
        stack[6] as u32,
        stack[5] as u32,
        stack[4] as u32,
        stack[3] as u32,
        stack[2] as u32,
        stack[1] as u32,
    ];
    let pc = stack[0] as u32;

    let d = u288_reduce(&c, pc);

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..9 {
        assert_eq!(Felt::new(d[i] as u64), strace[i])
    }
}

fn u256xu256_mod_mult(a: &[u32], b: &[u32]) -> [u32; 8] {
    assert_eq!(a.len(), 8);
    assert_eq!(a.len(), b.len());

    let mut c = [0u32; 16];
    let mut pc = 0u32;

    u256xu32(&mut c[0..9], b[0], &a);

    let d = u288_reduce(&c[0..9], pc);
    pc = d[8];
    c[1..9].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[1..10], b[1], &a);

    let d = u288_reduce(&c[1..10], pc);
    pc = d[8];
    c[2..10].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[2..11], b[2], &a);

    let d = u288_reduce(&c[2..11], pc);
    pc = d[8];
    c[3..11].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[3..12], b[3], &a);

    let d = u288_reduce(&c[3..12], pc);
    pc = d[8];
    c[4..12].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[4..13], b[4], &a);

    let d = u288_reduce(&c[4..13], pc);
    pc = d[8];
    c[5..13].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[5..14], b[5], &a);

    let d = u288_reduce(&c[5..14], pc);
    pc = d[8];
    c[6..14].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[6..15], b[6], &a);

    let d = u288_reduce(&c[6..15], pc);
    pc = d[8];
    c[7..15].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[7..16], b[7], &a);

    let d = u288_reduce(&c[7..16], pc);
    pc = d[8];
    c[8..16].copy_from_slice(&d[0..8]);

    c[8] += pc * 977;
    c[9] += pc;

    c[8..16].try_into().expect("incorrect length")
}

#[test]
fn test_u256xu256_mod_mult() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::u256xu256_mod_mult
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u64>() as u32 as u64;
        let b = rand_utils::rand_value::<u64>() as u32 as u64;

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
