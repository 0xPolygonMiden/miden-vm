
## std::collections::smt
| Procedure | Description |
| ----------- | ------------- |
| get | Returns the value stored under the specified key in a Sparse Merkle Tree with the specified root.<br /><br />If the value for a given key has not been set, the returned `V` will consist of all zeroes.<br /><br />Input:  [K, R, ...]<br /><br />Output: [V, R, ...]<br /><br />Depth 16: 92 cycles<br /><br />Depth 32: 95 cycles<br /><br />Depth 48: 93 cycles |
