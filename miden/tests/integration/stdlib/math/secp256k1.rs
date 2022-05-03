use super::build_test;

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
        stack[0] as u32,
        stack[1] as u32,
        stack[2] as u32,
        stack[3] as u32,
    );

    let test = build_test!(source, &stack);
    test.expect_stack(&[hi as u64, lo as u64]);
}
