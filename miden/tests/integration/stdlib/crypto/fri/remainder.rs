use super::build_test;
use processor::math::fft;
use test_case::test_case;
use vm_core::{Felt, FieldElement, QuadExtension, StarkField};

type QuadFelt = QuadExtension<Felt>;

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

    let source = format!(
        "
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

        adv.ext2intt

        push.0
        dropw

        repeat.{}
            push.0.0.0.0
            adv_loadw
        end
    end

    begin
        exec.helper
    end
    ",
        eval_mem_req,
        eval_mem_req - 1,
        eval_mem_req,
        eval_len,
        in_poly_len,
        out_mem_req
    );

    let poly = rand_utils::rand_vector::<QuadExtension<Felt>>(in_poly_len);
    let twiddles = fft::get_twiddles(poly.len());
    let evals = fft::evaluate_poly_with_offset(&poly, &twiddles, Felt::ONE, blowup);

    let ifelts = QuadFelt::slice_as_base_elements(&evals);
    let iu64s = ifelts.iter().map(|v| v.as_int()).collect::<Vec<u64>>();
    let ou64s = QuadFelt::slice_as_base_elements(&poly)
        .iter()
        .rev()
        .map(|v| v.as_int())
        .collect::<Vec<u64>>();

    let test = build_test!(source, &iu64s);
    test.expect_stack(&ou64s);
}

#[test]
fn test_verify_remainder_64() {
    let source = "
    use.std::crypto::fri::ext2fri

    proc.helper.32
        locaddr.31
        repeat.32
            movdn.4
            dup.4
            mem_storew
            dropw
            sub.1
        end
        drop

        locaddr.0
        exec.ext2fri::verify_remainder_64
    end

    begin
        exec.helper
    end
    ";

    let poly = rand_utils::rand_vector::<QuadFelt>(8);
    let twiddles = fft::get_twiddles(poly.len());
    let evals = fft::evaluate_poly_with_offset(&poly, &twiddles, Felt::ONE, 8);

    let ifelts = QuadFelt::slice_as_base_elements(&evals);
    let iu64s = ifelts.iter().map(|v| v.as_int()).collect::<Vec<u64>>();

    let test = build_test!(source, &iu64s);
    let res = test.execute();
    assert!(res.is_ok());
}

#[test]
fn test_verify_remainder_32() {
    let source = "
    use.std::crypto::fri::ext2fri

    proc.helper.16
        locaddr.15
        repeat.16
            movdn.4
            dup.4
            mem_storew
            dropw
            sub.1
        end
        drop

        locaddr.0
        exec.ext2fri::verify_remainder_32
    end

    begin
        exec.helper
    end
    ";

    let poly = rand_utils::rand_vector::<QuadFelt>(4);
    let twiddles = fft::get_twiddles(poly.len());
    let evals = fft::evaluate_poly_with_offset(&poly, &twiddles, Felt::ONE, 8);

    let ifelts = QuadFelt::slice_as_base_elements(&evals);
    let iu64s = ifelts.iter().map(|v| v.as_int()).collect::<Vec<u64>>();

    let test = build_test!(source, &iu64s);
    let res = test.execute();
    assert!(res.is_ok());
}
