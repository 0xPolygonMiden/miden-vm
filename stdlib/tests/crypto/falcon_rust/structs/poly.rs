use crate::crypto::falcon_rust::{binder::rpo128_context, MODULUS, MODULUS_MINUS_1_OVER_TWO, N};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polynomial(pub(crate) [u16; N]);

impl Default for Polynomial {
    fn default() -> Self {
        Self([0u16; N])
    }
}

impl Mul for Polynomial {
    type Output = Self;
    fn mul(self, other: Self) -> <Self as Mul<Self>>::Output {
        let mut result = [0_u16; N];
        for j in 0..N {
            for k in 0..N {
                let i = (j + k) % N;
                let a = self.0[j] as usize;
                let b = other.0[k] as usize;
                let q = MODULUS as usize;
                let mut prod = a * b % q;
                if (N - 1) < (j + k) {
                    prod = (q - prod) % q;
                }
                result[i] = ((result[i] as usize + prod) % q) as u16;
            }
        }

        Polynomial(result)
    }
}

impl Add for Polynomial {
    type Output = Self;
    fn add(self, other: Self) -> <Self as Add<Self>>::Output {
        let mut res = self;
        res.0
            .iter_mut()
            .zip(other.0.iter())
            .for_each(|(x, y)| *x = (*x + *y) % MODULUS as u16);

        res
    }
}

impl Sub for Polynomial {
    type Output = Self;
    fn sub(self, other: Self) -> <Self as Add<Self>>::Output {
        let mut res = self;
        res.0
            .iter_mut()
            .zip(other.0.iter())
            .for_each(|(x, y)| *x = (*x + MODULUS - *y) % MODULUS as u16);

        res
    }
}

impl Polynomial {
    pub fn mul_modulo_p(a: &Self, b: &Self) -> [u64; 1024] {
        let mut c = [0; 2 * N];
        for i in 0..N {
            for j in 0..N {
                c[i + j] += a.0[i] as u64 * b.0[j] as u64;
            }
        }

        c
    }

    pub fn from_hash_of_message(message: &[u8], nonce: &[u8]) -> Self {
        // Initialize the RPO state
        let mut rng = rpo128_context::init();

        // Absorb the nonce and message into the RPO state
        rng.inject(nonce);
        rng.inject(message);
        rng.finalize();

        let buffer = rng.extract((N as f32 * 2.2) as usize);
        let mut ctr = 0;

        // Squeeze the coefficients of the polynomial
        let mut res = [0u16; N];
        let mut i = 0;
        while i < N {
            let coeff = (buffer[ctr] as u16) << 8 | (buffer[ctr + 1] as u16);
            ctr += 2;
            if coeff < 61445 {
                res[i] = coeff % MODULUS;
                i += 1;
            }
        }
        Self(res)
    }

    pub fn sq_norm(&self) -> u64 {
        let mut res = 0;
        for e in self.0 {
            if e > MODULUS_MINUS_1_OVER_TWO as u16 {
                res += (MODULUS - e) as u64 * (MODULUS - e) as u64
            } else {
                res += e as u64 * e as u64
            }
        }
        res
    }
}
