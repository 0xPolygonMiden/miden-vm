
## std::crypto::stark::public_inputs
| Procedure | Description |
| ----------- | ------------- |
| load | Load the public inputs in memory starting from the address referenced by `public_inputs_ptr`.<br />In parallel, compute the hash of the public inputs being loaded. The hashing starts with<br />capacity registers of the hash function set to `C` resulting from hashing the proof context.<br />The output D is the digest of the hashing.<br /><br />Input: [public_inputs_ptr, C]<br />Output: [D]<br />Cycles: 38<br /> |
