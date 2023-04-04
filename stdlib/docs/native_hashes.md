
## std::crypto::hashes::native
| Procedure | Description |
| ----------- | ------------- |
| hash_memory | Hashes the memory `start_addr` to `end_addr`, handles odd number of elements.<br /><br />Requires `start_addr < end_addr`, `end_addr` is not inclusive.<br /><br />Stack transition:<br /><br />Input: [start_addr, end_addr, ...]<br /><br />Output: [H, ...]<br /><br />Cycles:<br /><br />even words: 48 cycles + 3 * words<br /><br />odd words: 60 cycles + 3 * words |
