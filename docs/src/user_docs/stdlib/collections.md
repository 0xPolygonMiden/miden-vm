# Collections
Namespace `std::collections` contains modules for commonly-used authenticated data structures. This includes:

- A Merkle Mountain range.
- A Sparse Merkle Tree with 64-bit keys.
- A Sparse Merkle Tree with 256-bit keys.

## Merkle Mountain Range
Module `std::collections::mmr` contains procedures for manipulating [Merkle Mountain Range](https://github.com/opentimestamps/opentimestamps-server/blob/master/doc/merkle-mountain-range.md) data structure which can be used as an append-only log.

The following procedures are available to read data from and make updates to a Merkle Mountain Range.

| Procedure   | Description   |
| ----------- | ------------- |
| get         | Loads the leaf at the absolute position `pos` in the MMR onto the stack.<br /><br />Valid range for `pos` is between $0$ and $2^{32} - 1$ (both inclusive).<br /><br />Inputs: `[pos, mmr_ptr, ...]`<br />Output: `[N, ...]`<br /><br />Where `N` is the leaf loaded from the MMR whose memory location starts at `mmr_ptr`. |
| add         | Adds a new leaf to the MMR.<br /><br />This will update the MMR peaks in the VM's memory and the advice provider with any merged nodes.<br /><br />Inputs: `[N, mmr_ptr, ...]`<br />Outputs: `[...]`<br /><br />Where `N` is the leaf added to the MMR whose memory locations starts at `mmr_ptr`. |
| pack        | Computes a commitment to the given MMR and copies the MMR to the Advice Map using the commitment as a key.<br /><br />Inputs: `[mmr_ptr, ...]`<br />Outputs: `[HASH, ...]`<br /><br /> |
| unpack      | Load the MMR peak data based on its hash.<br /><br />Inputs: `[HASH, mmr_ptr, ...]`<br />Outputs: `[...]`<br /><br />Where:<br />- `HASH`: is the MMR peak hash, the hash is expected to be padded to an even length and to have a minimum size of 16 elements.<br />- The advice map must contain a key with `HASH`, and its value is `num_leaves \|\| hash_data`, and hash_data is the data used to computed `HASH`<br />- `mmt_ptr`: the memory location where the MMR data will be written, starting with the MMR forest (the total count of its leaves) followed by its peaks. |

## Sparse Merkle Tree (64)

Module `std::collections::smt64` contains procedures for manipulating key-value maps with single-element keys and 4-element values. The current implementation is a thin wrapper over a simple Sparse Merkle Tree of depth 64. In the future, this will be replaced with a compact Sparse Merkle Tree implementation.

The following procedures are available to read data from and make updates to a Sparse Merkle Tree.

| Procedure   | Description |
| ----------- | ------------- |
| get         | Returns the value located under the specified key in the Sparse Merkle Tree defined by the specified root.<br /><br />If no values had been previously inserted under the specified key, an empty word is returned.<br /><br />Inputs: `[key, ROOT, ...]`<br />Outputs: `[VALUE, ROOT, ...]`<br /><br />Fails if the tree with the specified root does not exist in the VM's advice provider. |
| set         | Inserts the specified value under the specified key in a Sparse Merkle Tree defined by the specified root. If the insert is successful, the old value located under the specified key is returned via the stack.<br /><br />If `VALUE` is an empty word, the new state of the tree is guaranteed to be equivalent to the state as if the updated value was never inserted.<br /><br />Inputs: `[VALUE, key, ROOT, ...]`<br />Outputs: `[OLD_VALUE, NEW_ROOT, ...]`<br /><br />Fails if the tree with the specified root does not exits in the VM's advice provider. |
| insert      | Inserts the specified value under the specified key in a Sparse Merkle Tree defined by the specified root. If the insert is successful, the old value located under the specified key is returned via the stack.<br /><br />This procedure requires that `VALUE` be a non-empty word.<br /><br />Inputs: `[VALUE, key, ROOT, ...]`<br />Outputs: `[OLD_VALUE, NEW_ROOT, ...]`<br /><br />Fails if:<br />- The tree with the specified root does not exits in the VM's advice provider.<br />- The provided value is an empty word. |

## Sparse Merkle Tree (256)

Module `std::collections::smt` contains procedures for manipulating key-value maps with 4-element keys and 4-element values. The underlying implementation is a Tiered (compacted) Sparse Merkle where leaves can exist only at specific depths called "tiers". These depths are: 16, 32, 48, and 64. Initially, when a tree is empty, it is equivalent to an empty Sparse Merkle Tree of depth 64 (i.e., leaves at depth 64 are set to [ZERO; 4]). As non-empty values are inserted into the tree, they are added to the first available tier.

The following procedures are available to read data from and make updates to a Sparse Merkle Tree.

| Procedure   | Description   |
| ----------- | ------------- |
| get         | Returns the value located under the specified key in the Sparse Merkle Tree defined by the specified root.<br /><br />If no values had been previously inserted under the specified key, an empty word is returned.<br /><br />Inputs: `[KEY, ROOT, ...]`<br />Outputs: `[VALUE, ROOT, ...]`<br /><br />Fails if the tree with the specified root does not exist in the VM's advice provider. |
| set         | Inserts the specified value under the specified key in a Sparse Merkle Tree defined by the specified root. If the insert is successful, the old value located under the specified key is returned via the stack.<br /><br />If `VALUE` is an empty word, the new state of the tree is guaranteed to be equivalent to the state as if the updated value was never inserted.<br /><br />Inputs: `[VALUE, KEY, ROOT, ...]`<br />Outputs: `[OLD_VALUE, NEW_ROOT, ...]`<br /><br />Fails if the tree with the specified root does not exits in the VM's advice provider. |
| insert      | Inserts the specified value under the specified key in a Sparse Merkle Tree defined by the specified root. If the insert is successful, the old value located under the specified key is returned via the stack.<br /><br />This procedure requires that `VALUE` be a non-empty word.<br /><br />Inputs: `[VALUE, KEY, ROOT, ...]`<br />Outputs: `[OLD_VALUE, NEW_ROOT, ...]`<br /><br />Fails if:<br />- The tree with the specified root does not exits in the VM's advice provider.<br />- The provided value is an empty word. |
