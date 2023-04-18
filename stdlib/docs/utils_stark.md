
## std::crypto::stark::utils
| Procedure | Description |
| ----------- | ------------- |
| compute_lde_generator | Compute the LDE domain generator from the log2 of its size.<br /><br />Input: [log2(domain_size), ..]<br /><br />Output: [domain_gen, ..] |
| check_pow | Check that the Proof-of-Work contained in the current `SEED` is equal to the required number<br /><br />of bits prescribed by grinding bits. Currently the grinding factor is fixed to 16 bits.<br /><br />Input: [...]<br /><br />Output: [...] |
