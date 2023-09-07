use core::slice;
use miden_air::{Felt, StarkField};
use processor::Digest;

use std::vec;
use test_utils::{
    crypto::{KeyPair, MerkleStore},
    rand::rand_vector,
};

#[test]
fn test_falcon_final() {
    let source = "
    use.std::crypto::dsa::falcon

    begin
        exec.falcon::verify
    end
    ";

    let keypair = KeyPair::keygen();
    let sk = keypair.secret_key();
    let pk = keypair.public_key();
    let pk_exp = keypair.expanded_public_key();

    let msg = rand_vector::<u64>(4);
    let message = elements_as_bytes(&msg);
    let _signature = sk.sign(message, pk_exp);

    let pk: Digest = pk.inner().into();
    let pk_u64 = pk.as_elements().iter().map(|a| a.as_int()).collect::<Vec<u64>>();

    let mut op_stack = vec![];
    op_stack.extend_from_slice(&msg);
    op_stack.extend_from_slice(&pk_u64);

    let adv_stack = vec![];

    let to_adv_map = pk_exp
        .iter()
        .map(|a| Felt::new(*a as u64))
        .chain(sk.as_bytes().map(|a| Felt::new(a as u64)))
        .collect::<Vec<Felt>>();

    let advice_map: Vec<([u8; 32], Vec<Felt>)> = vec![(pk.as_bytes(), to_adv_map.into())];
    let store = MerkleStore::new();
    let test = build_test!(source, &op_stack, &adv_stack, store, advice_map.into_iter());

    test.expect_stack(&[]);
}

// HELPER FUNCTIONS
// ================================================================================================

fn elements_as_bytes(elements: &[u64]) -> &[u8] {
    let p = elements.as_ptr();
    let len = elements.len() * 8;
    unsafe { slice::from_raw_parts(p as *const u8, len) }
}
