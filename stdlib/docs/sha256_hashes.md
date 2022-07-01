
## std::crypto::hashes::sha256
| Procedure | Description |
| ----------- | ------------- |
| hash.16 |  Computes SHA256 2-to-1 hash function; see https://github.com/itzmeanjan/merklize-sha/blob/8a2c006a2ffe1e6e8e36b375bc5a570385e9f0f2/include/sha2_256.hpp#L121-L196<br /><br /> Input: First 16 elements of stack ( i.e. stack top ) holds 64 -bytes input digest, <br />   which is two sha256 digests ( each digest 32 -bytes i.e. 8 stack elements ) concatenated <br />   next to each other<br />  <br /> Output: First 8 elements of stack holds 32 -bytes blake3 digest, <br />   while remaining 8 elements of stack top are zeroed |
