use super::build_test;

#[test]
fn test_ntt512() {
    let source = "
    use.std::math::ntt512

    begin
        exec.ntt512::forward
        exec.ntt512::backward
    end
    ";

    let test = build_test!(source, &[]);
    let _ = test.get_last_stack_state();
}
