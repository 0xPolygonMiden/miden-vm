# Collections
Namespace `std::collections` contains modules for commonly authenticated data structures.

## Merkle Mountain Range
Module `std::collections::mmr` contains procedures for manipulating [Merkle Mountain Range](https://github.com/opentimestamps/opentimestamps-server/blob/master/doc/merkle-mountain-range.md) data structure which can be used as an append-only log.

| Procedure | Description |
| ----------- | ------------- |
| get | Loads the leaf at the absolute `pos` in the MMR onto the stack.<br /><br />Valid range for `pos` is between $0$ and $2^{32} - 1$ (both inclusive).<br /><br />Inputs:<br />- Operand stack: [pos, mmr_ptr, ...]<br /><br />Output:<br />- Operand stack: [N, ...]<br /><br />Where `N` is the leaf loaded from the MMR whose memory location starts at `mmr_ptr`. |
| add | Adds a new leaf to the MMR.<br /><br />This will update the MMR peaks in the VM's memory and the advice provider with any merged nodes.<br /><br />Inputs:<br />- Operand stack: [N, mmr_ptr, ...]<br /><br />Outputs:<br />- Operand stack: [...]<br /><br />Where `N` is the leaf added to the MMR whose memory locations starts at `mmr_ptr`. |
| pack | Computes the hash of the given MMR and copies it to the Advice Map using its hash as a key.<br /><br />Inputs:<br />- Operand stack: [mmr_ptr, ...]<br /><br />Outputs:<br />- Operand stack: [HASH, ...]<br /><br /> |
| unpack | Load the MMR peak data based on its hash.<br /><br />Inputs:<br />- Operand stack: [HASH, mmr_ptr, ...]<br /><br />Outputs:<br />- Operand stack: [...]<br /><br />Where:<br />- `HASH`: is the MMR peak hash, the hash is expected to be padded to an even length and to have a minimum size of 16 elements.<br />- The advice map must contain a key with `HASH`, and its value is `num_leaves \|\| hash_data`, and hash_data is the data used to computed `HASH`<br />- `mmt_ptr`: the memory location where the MMR data will be written, starting with the MMR forest (the total count of its leaves) followed by its peaks. |

## Sparse Merkle Tree (64)

Module `std::collections::smt64` contains procedures for manipulating key-value maps with single-element keys and 4-element values. The current implementation is a thin wrapper over a simple Sparse Merkle Tree of depth 64. In the future, this will be replaced with a compact Sparse Merkle Tree implementation.

| Procedure | Description |
| ----------- | ------------- |
| get | Returns the value located under the specified key in the Sparse Merkle Tree defined by the specified root.<br /><br />If no values had been previously inserted under the specified key, an empty word (i.e., [ZERO; 4]) is returned.<br /><br />Inputs:<br />- Operand stack: [key, ROOT, ...]<br /><br />Outputs:<br />-Operand stack: [VALUE, ROOT, ...]<br /><br />Fails if the tree with the specified root does not exist in the VM's advice provider. |
| set | Inserts the specified value under the specified key in a Sparse Merkle Tree defined by the specified root. If the insert is successful, the old value located under the specified key is returned via the stack.<br /><br />If `VALUE` is an empty word (i.e., [ZERO; 4]), the new state of the tree is guaranteed to be equivalent to the state as if the updated value was never inserted.<br /><br />Inputs:<br /> - Operand stack: [VALUE, key, ROOT, ...]<br /><br />Outputs:<br />- Operand stack: [OLD_VALUE, NEW_ROOT, ...]<br /><br />Fails if the tree with the specified root does not exits in the VM's advice provider. |
| insert | Inserts the specified value under the specified key in a Sparse Merkle Tree defined by the specified root. If the insert is successful, the old value located under the specified key is returned via the stack.<br /><br />This procedure requires that `VALUE` be a non-empty word (i.e., not [ZERO; 4]).<br /><br />Inputs:<br />- Operand stack: [VALUE, key, ROOT, ...]<br /><br />Outputs:<br /> -Operand stack: [OLD_VALUE, NEW_ROOT, ...]<br /><br />Fails if:<br />- The tree with the specified root does not exits in the VM's advice provider.<br />- The provided value is an empty word. |
