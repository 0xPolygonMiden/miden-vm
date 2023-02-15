use std::vec;

use crate::build_test;

use miden::utils::IntoBytes;
use rand_utils::rand_array;
use vm_core::{tiered_smt, AdviceSet, Felt, FieldElement, StarkField, Word};
//use miden_crypto::merkle::TieredSmt;

#[test]
fn test_insert_16() {
    let source = "
    use.std::collections::tieredsmt
        begin
            exec.tieredsmt::insert
        end
    ";

    let depth = 16;
    //let prefix: u16 = rand_utils::rand_value();
    let prefix = 1303;
    let key = rand_prefix_word(prefix as u64, depth);
    let value = rand_prefix_word(prefix as u64, depth);

    let t_smt = AdviceSet::new_tiered_smt().unwrap();
    let root = t_smt.root();

    println!("root of empty tree in byte is {:?}", root.into_bytes());

    let mut stack = vec![];
    stack.extend_from_slice(&to_stack(root));
    stack.extend_from_slice(&to_stack(value));
    stack.extend_from_slice(&to_stack(key));

    //hash of inserted leaf RpoDigest([BaseElement(5551095448908990592), BaseElement(6799191866597197811), BaseElement(18419664413467872727), BaseElement(10786253155893541963)])
    //remaining of inserted leaf [BaseElement(12019865079187491325), BaseElement(17479270241248200081), BaseElement(14684243997561300020), BaseElement(15560555691038820302)]

    // Correct hash BaseElement(10554308063829101541), BaseElement(1953252548040980706), BaseElement(16251828720717561788), BaseElement(17148118962368585741)
    println!("value is {:?}", value);
    println!("key is {:?}", key);
    println!("output0 {:?}", Felt::from_mont(5845943628274199658).as_int());
    println!("output1 {:?}", Felt::from_mont(68719476720).as_int());
    println!("output2 {:?}", Felt::from_mont(5596342385385).as_int());
    println!("output3 {:?}", Felt::from_mont(10789198164517701654).as_int());

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
    //let prefix: u16 = rand_utils::rand_value();
    let prefix = 1303;
    let key0 = rand_prefix_word(prefix as u64, depth);
    let value0 = rand_prefix_word(prefix as u64, depth);
    let key1 = rand_prefix_word_1(prefix as u64, depth);
    let value1 = rand_prefix_word_1(prefix as u64, depth);

    let mut t_smt = AdviceSet::new_tiered_smt().unwrap();
    t_smt.insert_tiered_smt(key0, value0);
    let root = t_smt.root();

    let mut stack = vec![];
    stack.extend_from_slice(&to_stack(root));
    stack.extend_from_slice(&to_stack(value1));
    stack.extend_from_slice(&to_stack(key1));

    //hash of inserted leaf RpoDigest([BaseElement(5551095448908990592), BaseElement(6799191866597197811), BaseElement(18419664413467872727), BaseElement(10786253155893541963)])
    //remaining of inserted leaf [BaseElement(12019865079187491325), BaseElement(17479270241248200081), BaseElement(14684243997561300020), BaseElement(15560555691038820302)]

    // Correct hash BaseElement(10554308063829101541), BaseElement(1953252548040980706), BaseElement(16251828720717561788), BaseElement(17148118962368585741)
    println!("value0 is {:?}", value0);
    println!("key0 is {:?}", key0);
    println!("output0 {:?}", Felt::from_mont(366761894568591360).as_int());
    println!("output1 {:?}", Felt::from_mont(68719476720).as_int());
    println!("output2 {:?}", Felt::from_mont(10554308063829101541).as_int());
    println!("output3 {:?}", Felt::from_mont(1953252548040980706).as_int());

    let test = build_test!(source, &stack, vec![], vec![t_smt]);

    let mut t_smt = tiered_smt::new();
    println!("root empty smt {:?}", t_smt.root().into_bytes());

    let _expected_root = t_smt.insert(key0, value0).expect("should return new root");
    println!("root smt after first insertion {:?}", _expected_root.into_bytes());
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
    let mut v = [
        Felt::new(6059895009993237404),
        Felt::new(3370208481294604206),
        Felt::new(12567701226416302623),
        Felt::new(9600785940884098942),
    ];
    let mut limb = v[3].as_int();
    limb /= 2u64.pow(depth);
    limb += key * 2u64.pow(64 - depth);
    v[3] = Felt::new(limb);
    v
}
fn rand_prefix_word_1(key: u64, depth: u32) -> Word {
    let mut v = [
        Felt::new(2060593237404),
        Felt::new(33702940046),
        Felt::new(12078502623),
        Felt::new(960884098942),
    ];
    let mut limb = v[3].as_int();
    limb /= 2u64.pow(depth);
    limb += key * 2u64.pow(64 - depth);
    v[3] = Felt::new(limb);
    v
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
