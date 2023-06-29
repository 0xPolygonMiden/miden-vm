
## std::crypto::hashes::native
| Procedure | Description |
| ----------- | ------------- |
| state_to_digest | Given the hasher state, returns the hash output<br /><br />Input: [C, B, A, ...]<br /><br />Ouptut: [HASH, ...]<br /><br />Where: For the native RPO hasher HASH is B.<br /><br />Cycles: 9 |
| hash_memory_even | Hashes the memory `start_addr` to `end_addr`.<br /><br />This requires that `end_addr=start_addr + 2n + 1`, otherwise the procedure will enter an infinite<br /><br />loop. `end_addr` is not inclusive.<br /><br />Stack transition:<br /><br />Input: [C, B, A, start_addr, end_addr, ...]<br /><br />Output: [C', B', A', end_addr, end_addr ...]<br /><br />Cycles: 4 + 3 * words, where `words` is the `start_addr - end_addr - 1`<br /><br />Where `A` is the capacity word that will be used by the hashing function, and `B'` the hash output. |
| hash_memory | Hashes the memory `start_addr` to `end_addr`, handles odd number of elements.<br /><br />Requires `start_addr < end_addr`, `end_addr` is not inclusive.<br /><br />Stack transition:<br /><br />Input: [start_addr, end_addr, ...]<br /><br />Output: [H, ...]<br /><br />Cycles:<br /><br />even words: 48 cycles + 3 * words<br /><br />odd words: 60 cycles + 3 * words |
