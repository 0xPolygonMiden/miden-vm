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

#[test]
fn u32_ops_proc_with_params() {
    let source = "
        proc.foo<alpha,beta> 
            u32checked_add.alpha
            u32checked_sub.beta
        end 
        begin
            exec.foo.2.3
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [3, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta> 
            u32checked_mul.alpha
            u32checked_div.beta
        end 
        begin
            exec.foo.2.3
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta> 
            u32checked_divmod.beta    
            u32checked_mod.alpha
        end 
        begin
            exec.foo.2.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [0, 2, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta> 
            u32checked_shr.alpha    
            u32checked_shl.beta
        end 
        begin
            exec.foo.2.1
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta> 
            u32checked_rotr.alpha    
            u32checked_rotl.beta
        end 
        begin
            exec.foo.2.1
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta,gamma> 
            u32wrapping_add.alpha
            u32wrapping_sub.beta
            u32wrapping_mul.gamma
        end 
        begin
            exec.foo.2.3.4
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [12, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta,gamma> 
            u32overflowing_add.alpha
            drop
            u32overflowing_sub.beta
            drop
            u32overflowing_mul.gamma
            drop
        end 
        begin
            exec.foo.2.3.4
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [12, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta,gamma> 
            u32unchecked_div.alpha
            u32unchecked_mod.beta
            u32unchecked_divmod.gamma
        end 
        begin
            exec.foo.2.3.4
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 0, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta> 
            u32unchecked_shr.alpha    
            u32unchecked_shl.beta
        end 
        begin
            exec.foo.2.1
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta> 
            u32unchecked_rotr.alpha    
            u32unchecked_rotl.beta
        end 
        begin
            exec.foo.2.1
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha,beta> 
            u32checked_eq.alpha
            drop
            u32checked_neq.beta
        end 
        begin
            exec.foo.4.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [1, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);
}

#[test]
fn stack_ops_proc_with_params() {
    let source = "
        proc.foo<alpha> 
            dup.alpha
        end 
        begin
            exec.foo.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 4, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha> 
            dupw.alpha
        end 
        begin
            exec.foo.0
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [4, 3, 2, 1, 4, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha>
            swap.alpha
        end
        begin
            exec.foo.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 3, 4, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha>
            swapw.alpha
        end
        begin
            exec.foo.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha>
            movup.alpha
        end
        begin
            exec.foo.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [2, 4, 3, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha>
            movupw.alpha
        end
        begin
            exec.foo.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [0, 0, 0, 0, 4, 3, 2, 1, 0, 0, 0, 0];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha>
            movdn.alpha
        end
        begin
            exec.foo.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [3, 2, 4, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);

    let source = "
        proc.foo<alpha>
            movdnw.alpha
        end
        begin
            exec.foo.2
        end";
    let inputs = [1, 2, 3, 4];
    let final_stack = [0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 2, 1];
    build_test!(source, &inputs).expect_stack(&final_stack);
}
