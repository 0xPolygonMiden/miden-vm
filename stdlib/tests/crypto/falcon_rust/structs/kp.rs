use crate::crypto::falcon_rust::{binder::*, param::*};
use libc::c_void;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use zeroize::Zeroize;

use super::{PublicKey, SecretKey};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl KeyPair {
    /// Generate a (pk, sk) keypair from a cryptographic PRNG
    pub fn keygen() -> Self {
        let mut seed = [0u8; 32];
        let mut rng = ChaCha20Rng::from_entropy();
        rng.fill_bytes(&mut seed);

        Self::keygen_with_seed(seed.as_ref())
    }

    /// generate a pair of public and secret keys from a seed
    pub fn keygen_with_seed(seed: &[u8]) -> Self {
        let mut shake256_context = shake256_context::init_with_seed(seed);
        let mut pk = [0u8; PK_LEN];
        let mut sk = [0u8; SK_LEN];
        let mut buf = vec![0u8; KEYGEN_BUF_LEN];

        unsafe {
            assert!(
                falcon_keygen_make(
                    &mut shake256_context as *mut shake256_context,
                    LOG_N as u32,
                    sk.as_mut_ptr() as *mut c_void,
                    SK_LEN as u64,
                    pk.as_mut_ptr() as *mut c_void,
                    PK_LEN as u64,
                    buf.as_mut_ptr() as *mut c_void,
                    KEYGEN_BUF_LEN as u64
                ) == 0
            );
        }
        buf.zeroize();

        Self {
            public_key: PublicKey(pk),
            secret_key: SecretKey(sk),
        }
    }
}
