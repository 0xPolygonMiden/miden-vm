
## std::collections::mmr
| Procedure | Description |
| ----------- | ------------- |
| ilog2_checked | Computes the `ilog2(number)` and `2^(ilog2(number))`.<br /><br />number must be non-zero, otherwise this will error<br /><br />Stack transition:<br /><br />Input: [number, ...]<br /><br />Output: [ilog2, power_of_two, ...]<br /><br />Cycles:  12 + 9 * leading_zeros |
| get | Loads the leaf at the absolute `pos` in the MMR.<br /><br />This MMR implementation supports only u32 positions.<br /><br />Stack transition:<br /><br />Input: [pos, mmr_ptr, ...]<br /><br />Output: [N, ...] where `N` is the leaf and `R` is the MMR peak that owns the leaf.<br /><br />Cycles: 65 + 9 * tree_position (where `tree_position` is 0-indexed bit position from most to least significant) |
