use std::vec;

use crate::build_test;

use miden::utils::IntoBytes;
use rand_utils::rand_array;
use vm_core::{tiered_smt, AdviceSet, Felt, FieldElement, StarkField, Word};

#[test]
fn test_insert_16() {
    let source = "
    use.std::collections::tieredsmt
        begin
            exec.tieredsmt::insert
        end
    ";

    let depth = 16;
    let prefix = 1303;
    let key = rand_prefix_word(prefix as u64, depth);
    let value = rand_prefix_word(prefix as u64, depth);

    let t_smt = AdviceSet::new_tiered_smt().unwrap();
    let root = t_smt.root();

    let mut stack = vec![];
    stack.extend_from_slice(&to_stack(root));
    stack.extend_from_slice(&to_stack(value));
    stack.extend_from_slice(&to_stack(key));

    let test = build_test!(source, &stack, vec![], vec![t_smt]);

    let mut t_smt = tiered_smt::new();

    let expected_root = t_smt.insert(key, value).expect("should return new root");
    let mut empty_16 = vec![
        17483286922353768131,
        353378057542380712,
        1935183237414585408,
        4820339620987989650,
    ];
    empty_16.reverse();
    let mut final_stack = to_stack_reversed(*expected_root);
    final_stack.extend_from_slice(&empty_16);
    test.expect_stack(&final_stack);
}

#[test]
fn test_insert_complex() {
    let source = "
    use.std::collections::tieredsmt
        begin
            exec.tieredsmt::insert
        end
    ";

    let depth = 16;
    let prefix = 1303;
    let key0 = rand_prefix_word(prefix as u64, depth);
    let value0 = random_word();
    let key1 = rand_prefix_word(prefix as u64, depth);
    let value1 = random_value();

    let mut t_smt = AdviceSet::new_tiered_smt().unwrap();
    t_smt.insert_tiered_smt(key0, value0);
    let root = t_smt.root();

    let mut stack = vec![];
    stack.extend_from_slice(&to_stack(root));
    stack.extend_from_slice(&to_stack(value1));
    stack.extend_from_slice(&to_stack(key1));

    let test = build_test!(source, &stack, vec![], vec![t_smt]);

    let mut t_smt = tiered_smt::new();

    let expected_root = t_smt.insert(key1, value1).expect("should return new root");
    let mut empty_16 = vec![
        17483286922353768131,
        353378057542380712,
        1935183237414585408,
        4820339620987989650,
    ];
    empty_16.reverse();
    let mut final_stack = to_stack_reversed(*expected_root);
    final_stack.extend_from_slice(&empty_16);
    test.expect_stack(&final_stack);
}

fn rand_prefix_word(key: u64, depth: u32) -> Word {
    let mut v = rand_utils::rand_array::<Felt, 4>();

    let mut limb = v[3].as_int();
    limb /= 2u64.pow(depth);
    limb += key * 2u64.pow(64 - depth);
    v[3] = Felt::new(limb);
    v
}
fn random_word() -> Word {
    rand_utils::rand_array::<Felt, 4>()
}

const fn int_to_word(value: u64) -> Word {
    [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
}

fn to_stack(val: Word) -> Vec<u64> {
    vec![val[0].as_int(), val[1].as_int(), val[2].as_int(), val[3].as_int()]
}

fn to_stack_reversed(val: Word) -> Vec<u64> {
    vec![val[3].as_int(), val[2].as_int(), val[1].as_int(), val[0].as_int()]
}
