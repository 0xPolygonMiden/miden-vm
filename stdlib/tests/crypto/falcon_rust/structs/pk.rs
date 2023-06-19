use super::sig::Signature;
use crate::crypto::falcon_rust::{binder::*, param::*, Polynomial};
use libc::c_void;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PublicKey(pub(crate) [u8; PK_LEN]);

impl PublicKey {
    // Verify using the reference C implementation
    pub fn verify_c(&self, message: &[u8], signature: &Signature) -> bool {
        let signature_type = 2;
        let mut buffer = [0u8; VERIFY_BUF_LEN];

        let res = unsafe {
            falcon_verify(
                signature.0.as_ptr() as *const c_void,
                signature.0.len() as u64,
                signature_type,
                self.0.as_ptr() as *const c_void,
                self.0.len() as u64,
                message.as_ptr() as *const c_void,
                message.len() as u64,
                buffer.as_mut_ptr() as *mut c_void,
                VERIFY_BUF_LEN as u64,
            )
        };

        res == 0
    }

    // Unpack the public key into a polynomial in Z_q[x]
    pub fn unpack(&self) -> [u16; N] {
        assert!(self.0[0] == LOG_N as u8);
        mod_q_decode(self.0[1..].as_ref())
    }

    // Verify using naive Rust implementation
    pub fn verify_rs(&self, message: &[u8], signature: &Signature) -> bool {
        let h: Polynomial = self.into();
        let s2: Polynomial = signature.into();
        let c = Polynomial::from_hash_of_message(message, signature.0[1..41].as_ref());

        let s1 = c - s2 * h;

        let sq_norm = s1.sq_norm() + s2.sq_norm();
        sq_norm <= SIG_L2_BOUND
    }
}

impl From<&PublicKey> for Polynomial {
    fn from(pk: &PublicKey) -> Self {
        Polynomial(pk.unpack())
    }
}

fn mod_q_decode(input: &[u8]) -> [u16; N] {
    if input.len() != (N * 14 + 7) / 8 {
        panic!("Decoding failure: input length is incorrect")
    }

    let mut input_pt = 0;
    let mut acc = 0u32;
    let mut acc_len = 0;

    let mut output_ptr = 0;
    let mut output = [0u16; N];

    while output_ptr < N {
        acc = (acc << 8) | (input[input_pt] as u32);
        input_pt += 1;
        acc_len += 8;

        if acc_len >= 14 {
            acc_len -= 14;
            let w = (acc >> acc_len) & 0x3FFF;
            assert!(w < 12289, "Coefficient out of range: {}", w);
            output[output_ptr] = w as u16;
            output_ptr += 1;
        }
    }

    if (acc & ((1u32 << acc_len) - 1)) != 0 {
        panic!("Decoding failure: input not fully consumed")
    }

    output
}
