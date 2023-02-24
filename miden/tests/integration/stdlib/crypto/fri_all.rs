use std::collections::BTreeMap;

use super::{build_test, Felt};
use vm_core::StarkField;

use math::log2;
use miden::utils::math;

mod channel;
pub use channel::*;

pub(crate) mod verifier_fri_e2f4;
pub use verifier_fri_e2f4::*;

#[test]
fn fri_fold4_ext2_remainder32() {
    let source = "
        use.std::crypto::fri::frif4e2

        begin
            exec.frif4e2::preprocess
            exec.frif4e2::verify
        end
        ";

    let trace_len_e = 14;
    let blowup_exp = 3;
    let depth = trace_len_e + blowup_exp;
    let domain_size = 1 << depth;

    let (advice_provider, tape, position_eval, alphas, commitments, remainder, num_queries) =
        fri_prove_verify_fold4_ext2(trace_len_e).expect("should not panic");

    let tape = prepare_advice(
        depth,
        domain_size,
        num_queries,
        tape,
        position_eval,
        alphas,
        commitments,
        remainder,
    );

    let advice_map: BTreeMap<[u8; 32], Vec<Felt>> = BTreeMap::from_iter(advice_provider.1);
    let domain_generator = Felt::get_root_of_unity(log2(domain_size as usize)).as_int();
    let test = build_test!(
        source,
        &[domain_generator],
        &tape,
        advice_provider.0.clone(),
        advice_map.clone()
    );

    test.expect_stack(&[]);
}

#[test]
fn fri_fold4_ext2_remainder64() {
    let source = "
        use.std::crypto::fri::frif4e2

        begin
            exec.frif4e2::preprocess
            exec.frif4e2::verify
        end
        ";

    let trace_len_e = 13;
    let blowup_exp = 3;
    let depth = trace_len_e + blowup_exp;
    let domain_size = 1 << depth;

    let (advice_provider, tape, position_eval, alphas, commitments, remainder, num_queries) =
        fri_prove_verify_fold4_ext2(trace_len_e).expect("should not panic");

    let tape = prepare_advice(
        depth,
        domain_size,
        num_queries,
        tape,
        position_eval,
        alphas,
        commitments,
        remainder,
    );

    let advice_map: BTreeMap<[u8; 32], Vec<Felt>> = BTreeMap::from_iter(advice_provider.1);
    let domain_generator = Felt::get_root_of_unity(log2(domain_size as usize)).as_int();
    let test = build_test!(
        source,
        &[domain_generator],
        &tape,
        advice_provider.0.clone(),
        advice_map.clone()
    );

    test.expect_stack(&[]);
}

fn prepare_advice(
    depth: usize,
    domain_size: u32,
    num_queries: usize,
    tape_pre: Vec<u64>,
    position_eval: Vec<u64>,
    alphas: Vec<u64>,
    com: Vec<u64>,
    remainder: Vec<u64>,
) -> Vec<u64> {
    let mut tape = vec![];
    let remainder_length = remainder.len() / 2;
    let num_layers = (com.len() / 4) - 1;

    tape.push(num_layers as u64);

    let mut current_domain_size = domain_size as u64;
    let mut current_depth = depth as u64;

    for i in 0..num_layers {
        current_domain_size /= 4;

        tape.extend_from_slice(&com[(4 * i)..(4 * i + 4)]);
        tape.extend_from_slice(&alphas[(4 * i)..(4 * i + 2)]);
        tape.extend_from_slice(&vec![current_depth - 1, current_domain_size]);
        current_depth -= 2;
    }

    tape.push(remainder_length as u64 / 2);

    for i in 0..remainder_length / 2 {
        let mut remainder_4 = vec![0; 4];
        remainder_4[0] = remainder[4 * i + 0];
        remainder_4[1] = remainder[4 * i + 1];
        remainder_4[2] = remainder[4 * i + 2];
        remainder_4[3] = remainder[4 * i + 3];

        tape.extend_from_slice(&remainder_4);
    }

    tape.push(num_queries as u64);

    tape.extend_from_slice(&position_eval[..]);

    tape.extend_from_slice(&tape_pre[..]);

    tape
}
