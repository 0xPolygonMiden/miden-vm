
## std::crypto::hashes::blake3
| Procedure | Description |
| ----------- | ------------- |
| hash.4 |  blake3 2-to-1 hash function<br /><br /> Input: First 16 elements of stack ( i.e. stack top ) holds 64 -bytes input digest, <br />   which is two blake3 digests concatenated next to each other<br />  <br /> Output: First 8 elements of stack holds 32 -bytes blake3 digest, <br />   while remaining 8 elements of stack top are zeroed |
