use test_utils::{
    math::fft, push_inputs, rand::rand_vector, test_case, Felt, FieldElement, QuadFelt, StarkField,
    ONE,
};

#[test_case(8, 1; "poly_8 |> evaluated_8 |> interpolated_8")]
#[test_case(8, 2; "poly_8 |> evaluated_16 |> interpolated_8")]
#[test_case(8, 4; "poly_8 |> evaluated_32 |> interpolated_8")]
#[test_case(8, 8; "poly_8 |> evaluated_64 |> interpolated_8")]
#[test_case(16, 1; "poly_16 |> evaluated_16 |> interpolated_16")]
#[test_case(16, 2; "poly_16 |> evaluated_32 |> interpolated_16")]
#[test_case(16, 4; "poly_16 |> evaluated_64 |> interpolated_16")]
#[test_case(16, 8; "poly_16 |> evaluated_128 |> interpolated_16")]
fn test_decorator_ext2intt(in_poly_len: usize, blowup: usize) {
    // requirements
    assert!((in_poly_len > 1) && in_poly_len.is_power_of_two());
    assert!((blowup > 0) && blowup.is_power_of_two());

    let eval_len = in_poly_len * blowup;
    let eval_mem_req = (eval_len * 2) / 4;
    let out_mem_req = (in_poly_len * 2) / 4;

    let poly = rand_vector::<QuadFelt>(in_poly_len);
    let twiddles = fft::get_twiddles(poly.len());
    let evals = fft::evaluate_poly_with_offset(&poly, &twiddles, ONE, blowup);

    let ifelts = QuadFelt::slice_as_base_elements(&evals);
    let iu64s = ifelts.iter().map(|v| v.as_int()).collect::<Vec<u64>>();
    let ou64s = QuadFelt::slice_as_base_elements(&poly)
        .iter()
        .rev()
        .map(|v| v.as_int())
        .collect::<Vec<u64>>();

    let source = format!(
        "
    use.std::sys
    
    proc.helper.{}
        locaddr.{}
        repeat.{}
            movdn.4
            dup.4
            mem_storew
            dropw
            sub.1
        end
        drop

        locaddr.0
        push.{}
        push.{}

        adv.push_ext2intt

        push.0
        dropw

        repeat.{}
            push.0.0.0.0
            adv_loadw
        end
    end

    begin
        {inputs}
        exec.helper

        exec.sys::truncate_stack
    end
    ",
        eval_mem_req,
        eval_mem_req - 1,
        eval_mem_req,
        eval_len,
        in_poly_len,
        out_mem_req,
        inputs = push_inputs(&iu64s)
    );

    let test = build_test!(&source, &[]);
    test.expect_stack(&ou64s);
}

#[test]
fn test_verify_remainder_64() {
    let poly = rand_vector::<QuadFelt>(8);
    let twiddles = fft::get_twiddles(poly.len());
    let evals = fft::evaluate_poly_with_offset(&poly, &twiddles, Felt::GENERATOR, 8);
    let tau = rand_vector::<QuadFelt>(1);

    let mut ifelts = QuadFelt::slice_as_base_elements(&tau).to_vec();
    ifelts.extend_from_slice(QuadFelt::slice_as_base_elements(&evals));
    ifelts.extend_from_slice(QuadFelt::slice_as_base_elements(&poly));
    let iu64s = ifelts.iter().map(|v| v.as_int()).collect::<Vec<u64>>();

    let source = format!(
        "
    use.std::crypto::fri::ext2fri

    proc.helper.36
        locaddr.35
        repeat.36
            movdn.4
            dup.4
            mem_storew
            dropw
            sub.1
        end
        drop

        locaddr.0 movdn.2
        exec.ext2fri::verify_remainder_64
    end

    begin
        {inputs}
        exec.helper
    end
    ",
        inputs = push_inputs(&iu64s)
    );

    let test = build_test!(source, &[]);
    test.expect_stack(&[]);
}

#[test]
fn test_verify_remainder_32() {
    let poly = rand_vector::<QuadFelt>(4);
    let twiddles = fft::get_twiddles(poly.len());
    let evals = fft::evaluate_poly_with_offset(&poly, &twiddles, Felt::GENERATOR, 8);
    let tau = rand_vector::<QuadFelt>(1);

    let mut ifelts = QuadFelt::slice_as_base_elements(&tau).to_vec();
    ifelts.extend_from_slice(QuadFelt::slice_as_base_elements(&evals));
    ifelts.extend_from_slice(QuadFelt::slice_as_base_elements(&poly));
    let iu64s = ifelts.iter().map(|v| v.as_int()).collect::<Vec<u64>>();

    let source = format!(
        "
    use.std::crypto::fri::ext2fri

    proc.helper.18
        locaddr.17
        repeat.18
            movdn.4
            dup.4
            mem_storew
            dropw
            sub.1
        end
        drop

        locaddr.0 movdn.2
        exec.ext2fri::verify_remainder_32
    end

    begin
        {inputs}
        exec.helper
    end
    ",
        inputs = push_inputs(&iu64s)
    );

    let test = build_test!(source, &[]);
    test.expect_stack(&[]);
}
