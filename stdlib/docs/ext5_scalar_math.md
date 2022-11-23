
## std::math::ext5_scalar
| Procedure | Description |
| ----------- | ------------- |
| mont_mul | Montgomery multiplication of two radix-2^32 scalar field elements s.t. each<br /><br />number can be represented using 10 limbs, each of 32 -bit width, returning<br /><br />r = (a * b) / 2^320 (mod N) \| N = 319 -bit prime ( See https://github.com/itzmeanjan/miden/blob/6a611e693601577864da3e43e745525b83c0030d/miden/tests/integration/stdlib/math/ext5_scalar.rs#L24-L35 )<br /><br />Expected stack state<br /><br />[a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, ...]<br /><br />Final stack state<br /><br />[r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, ...]<br /><br />Adapted from equivalent Rust implementation https://github.com/itzmeanjan/miden/blob/6a611e693601577864da3e43e745525b83c0030d/miden/tests/integration/stdlib/math/ext5_scalar.rs#L92-L132 |
