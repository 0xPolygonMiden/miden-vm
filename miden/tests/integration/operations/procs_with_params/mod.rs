use super::super::build_test;

#[test]
fn field_ops_proc_with_params() {
    let source = "
        proc.foo<alpha,beta,gamma> 
            add.alpha
            mul.beta
            eq.gamma
        end 
        begin
            exec.foo.2.3.18
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [1, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);
}
