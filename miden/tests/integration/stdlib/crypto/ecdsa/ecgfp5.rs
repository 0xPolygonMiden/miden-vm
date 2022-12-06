use crate::stdlib::math::ecgfp5::group::ECExt5;
use crate::stdlib::math::ecgfp5::scalar_field::Scalar;

use vm_core::StarkField;

/// Eliiptic curve digital signature algorithm over ecgfp5 curve, following https://cryptobook.nakov.com/digital-signatures/ecdsa-sign-verify-messages
///
/// Right now this test is not run until explicitly asked for because it fails.
#[test]
#[ignore]
fn test_ecgfp5_dsa() {
    // keygen
    let skey = Scalar::rand();
    let gen = ECExt5::generator();
    let pkey = gen.scalar_mul(&skey.limbs);
    // keypair: (skey, pkey)

    // sign
    let h = Scalar::rand();
    let k = Scalar::rand();
    let r = Scalar {
        limbs: {
            let r = gen.scalar_mul(&k.limbs).x;

            let r0 = r.a0.as_int();
            let r1 = r.a1.as_int();
            let r2 = r.a2.as_int();
            let r3 = r.a3.as_int();
            let r4 = r.a4.as_int();

            [
                r0 as u32,
                (r0 >> 32) as u32,
                r1 as u32,
                (r1 >> 32) as u32,
                r2 as u32,
                (r2 >> 32) as u32,
                r3 as u32,
                (r3 >> 32) as u32,
                r4 as u32,
                (r4 >> 32) as u32,
            ]
        },
    };
    let k_inv = k.inv();
    let s = k_inv * (h + r * skey);
    // signature (r, s)

    // verify
    let s1 = s.inv();
    let t0 = h * s1;
    let t1 = r * s1;
    let t2 = gen.scalar_mul(&t0.limbs);
    let t3 = pkey.scalar_mul(&t1.limbs);
    let r_prime = Scalar {
        limbs: {
            let r = (t2 + t3).x;

            let r0 = r.a0.as_int();
            let r1 = r.a1.as_int();
            let r2 = r.a2.as_int();
            let r3 = r.a3.as_int();
            let r4 = r.a4.as_int();

            [
                r0 as u32,
                (r0 >> 32) as u32,
                r1 as u32,
                (r1 >> 32) as u32,
                r2 as u32,
                (r2 >> 32) as u32,
                r3 as u32,
                (r3 >> 32) as u32,
                r4 as u32,
                (r4 >> 32) as u32,
            ]
        },
    };

    assert_eq!(r, r_prime);
}
