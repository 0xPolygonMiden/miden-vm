
## std::crypto::hashes::rescue_prime
| Procedure | Description |
| ----------- | ------------- |
| hash_1to1 | Given 4 Miden field elements on stack, representing 32 -bytes input, this routine<br /><br />computes Rescue Prime hash digest, producing 4 field elements, representing 32 -bytes digest.<br /><br />Expected input stack state<br /><br />[a3, a2, a1, a0, ...] s.t. a`i` ∈ F_q \| q = 2^64 - 2^32 + 1<br /><br />Output stack state<br /><br />[b3, b2, b1, b0, ...] s.t. b`i` ∈ F_q \| q = 2^64 - 2^32 + 1<br /><br />Applying below linked routine on a slice [a0, a1, a2, a3] should result in digest [b0, b1, b2, b3].<br /><br />Notice difference between input/ output ordering in case of Miden assembly implementation and Rust implementation.<br /><br />See equivalent Rust implementation https://github.com/novifinancial/winterfell/blob/6322724/crypto/src/hash/rescue/rp64_256/mod.rs#L223-L256 |
