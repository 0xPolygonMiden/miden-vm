use crate::crypto::falcon_rust::{Polynomial, MODULUS, N, SIG_LEN};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Signature(pub(crate) [u8; SIG_LEN]);

impl Signature {
    /// Unpack the signature into a vector of integers
    /// within the range of [0, MODULUS)
    pub fn unpack(&self) -> [u16; N] {
        let res = comp_decode(self.0[41..].as_ref());
        res
    }

    /// Return the nonce component of the signature
    pub fn nonce(&self) -> &[u8] {
        self.0[1..41].as_ref()
    }
}

impl From<&Signature> for Polynomial {
    fn from(sig: &Signature) -> Self {
        let mut res = Self::default();
        res.0
            .copy_from_slice(sig.unpack().iter().map(|x| *x as u16).collect::<Vec<u16>>().as_ref());
        res
    }
}

fn comp_decode(input: &[u8]) -> [u16; N] {
    let mut input_pt = 0;
    let mut acc = 0u32;
    let mut acc_len = 0;
    let mut output = [0u16; N];

    for e in output.iter_mut() {
        // Get next eight bits: sign and low seven bits of the
        // absolute value.

        acc = (acc << 8) | (input[input_pt] as u32);
        input_pt += 1;
        let b = acc >> acc_len;
        let s = b & 128;
        let mut m = b & 127;

        // Get next bits until a 1 is reached.
        loop {
            if acc_len == 0 {
                acc = (acc << 8) | (input[input_pt] as u32);
                input_pt += 1;
                acc_len = 8;
            }
            acc_len -= 1;
            if ((acc >> acc_len) & 1) != 0 {
                break;
            }
            m += 128;
            assert!(m < 2048, "Incorrect input: {}", m);
        }

        if s != 0 && m == 0 {
            panic!("Incorrect remaining data")
        }
        *e = if s != 0 { (MODULUS as u32 - m) as u16 } else { m as u16 };
    }

    // Unused bits in the last byte must be zero.
    if (acc & ((1 << acc_len) - 1)) != 0 {
        panic!("Incorrect remaining data")
    }

    output
}
