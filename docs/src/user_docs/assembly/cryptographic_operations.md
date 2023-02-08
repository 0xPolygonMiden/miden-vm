## Cryptographic operations
Miden assembly provides a set of instructions for performing common cryptographic operations. These instructions are listed in the table below.

### Hashing and Merkle trees
[Rescue Prime Optimized](https://eprint.iacr.org/2022/1577) is the native hash function of Miden VM. The parameters of the hash function were chosen to provide 128-bit security level against preimage and collision attacks. The function operates over a state of 12 field elements, and requires 7 rounds for a single permutation. However, due to its special status within the VM, computing Rescue Prime Optimized hashes can be done very efficiently. For example, applying a permutation of the hash function can be done in a single VM cycle. 

| Instruction    | Stack_input     | Stack_output   | Notes                                      |
| -------------- | --------------- | -------------- | ------------------------------------------ |
| hperm  <br> - *(1 cycle)*      | [C, B, A, ...]  | [F, E, D, ...] | $\{D, E, F\} \leftarrow permute(A, B, C)$ <br> where, $permute()$ computes a Rescue Prime Optimized permutation. |
| hmerge  <br> - *(16 cycles)*        | [B, A, ...]     | [C, ...]       | $C \leftarrow hash(A,B)$ <br> where, $hash()$ computes a 2-to-1 Rescue Prime Optimized hash. |
| mtree_get  <br> - *(9 cycles)*     | [d, i, R, ...]  | [V, R, ...] |  Verifies that a Merkle tree with root $R$ opens to node $V$ at depth $d$ and index $i$. Merkle tree with root $R$ must be present in the advice provider, otherwise execution fails. |
| mtree_set <br> - *(29 cycles)*      | [d, i, R, V', ...] | [V, R', ...] | Updates a node in the Merkle tree with root $R$ at depth $d$ and index $i$ to value $V'$. $R'$ is the Merkle root of the resulting tree and $V$ is old value of the node. Merkle tree with root $R$ must be present in the advice provider, otherwise execution fails. At the end of the operation Merkle tree in the advice provider with root $R$ is replaced with the Merkle tree with root $R'$. |
| mtree_cwm <br> - *(29 cycles)*      | [d, i, R, V', ...] | [V, R', ...] | Copies a Merkle tree with root $R$ and updates a node at depth $d$ and index $i$ in the copied tree to value $V'$. $R'$ is the Merkle root of the new tree and $V$ is old value of the node. Merkle tree with root $R$ must be present in the advice provider, otherwise execution fails. At the end of the operation the advice provider will contain both Merkle trees. |
