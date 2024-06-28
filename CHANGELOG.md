# Changelog

## 0.9.0 (TBD)

#### Packaging
- [BREAKING] The package `miden-vm` crate was renamed from `miden` to `miden-vm`. Now the package and crate names match (#1271).

#### Stdlib
- Added `init_no_padding` procedure to `std::crypto::hashes::native` (#1313).
- [BREAKING] `native` module was renamed to the `pro`, `hash_memory` procedure was renamed to the `hash_memory_words` (#1368).
- Added `hash_memory` procedure to `std::crypto::hashes::rpo` (#1368).

#### VM Internals
- Removed unused `find_lone_leaf()` function from the Advice Provider (#1262).
- [BREAKING] Changed fields type of the `StackOutputs` struct from `Vec<u64>` to `Vec<Felt>` (#1268).
- [BREAKING] Migrated to `miden-crypto` v0.9.0 (#1287).

## 0.8.0 (02-26-2024)

#### Assembly
- Expanded capabilities of the `debug` decorator. Added `debug.mem` and `debug.local` variations (#1103).
- Introduced the `emit.<event_id>` assembly instruction (#1119).
- Introduced the `procref.<proc_name>` assembly instruction (#1113).
- Added the ability to use constants as counters in `repeat` loops (#1124).
- [BREAKING] Removed all `checked` versions of the u32 instructions. Renamed all `unchecked` versions (#1115).
- Introduced the `u32clz`, `u32ctz`, `u32clo`, `u32cto` and `ilog2` assembly instructions (#1176).
- Added support for hexadecimal values in constants (#1199).
- Added the `RCombBase` instruction (#1216).

#### Stdlib
- Introduced `std::utils` module with `is_empty_word` procedure.  Refactored `std::collections::smt`
  and `std::collections::smt64` to use the procedure (#1107).
- [BREAKING] Removed `checked` versions of the instructions in the `std::math::u64` module (#1142).
- Introduced `clz`, `ctz`, `clo` and `cto` instructions in the `std::math::u64` module (#1179).
- [BREAKING] Refactored `std::collections::smt` to use `SimpleSmt`-based implementation (#1215).
- [BREAKING] Removed `std::collections::smt64` (#1249)

#### VM Internals
- Introduced the `Event` decorator and an associated `on_event` handler on the `Host` trait (#1119).
- Added methods `StackOutputs::get_stack_item()` and `StackOutputs::get_stack_word()` (#1155).
- Added [Tracing](https://crates.io/crates/tracing) logger to the VM (#1139).
- Refactored auxiliary trace construction (#1140).
- [BREAKING] Optimized `u32lt` instruction (#1193)
- Added `on_assert_failed()` method to the Host trait (#1197).
- Added support for handling `trace` instruction in the `Host` interface (#1198).
- Updated Winterfell dependency to v0.8 (#1234).
- Increased min version of `rustc` to 1.75.

#### CLI
- Introduced the `!use` command for the Miden REPL (#1162).
- Introduced a `BLAKE3` hashing example (#1180).

## 0.7.0 (2023-10-11)

#### Assembly
- Added ability to attach doc comments to re-exported procedures (#994).
- Added support for nested modules (#992).
- Added support for the arithmetic expressions in constant values (#1026).
- Added support for module aliases (#1037).
- Added `adv.insert_hperm` decorator (#1042).
- Added `adv.push_smtpeek` decorator (#1056).
- Added `debug` decorator (#1069).
- Refactored `push` instruction so now it parses long hex string in little-endian (#1076).

#### CLI
- Implemented ability to output compiled `.masb` files to disk (#1102).

#### VM Internals
- Simplified range checker and removed 1 main and 1 auxiliary trace column (#949).
- Migrated range checker lookups to use LogUp and reduced the number of trace columns to 2 main and
  1 auxiliary (#1027).
- Added `get_mapped_values()` and `get_store_subset()` methods to the `AdviceProvider` trait (#987).
- [BREAKING] Added options to specify maximum number of cycles and expected number of cycles for a program (#998).
- Improved handling of invalid/incomplete parameters in `StackOutputs` constructors (#1010).
- Allowed the assembler to produce programs with "phantom" calls (#1019).
- Added `TraceLenSummary` struct which holds information about traces lengths to the `ExecutionTrace` (#1029).
- Imposed the 2^32 limit for the memory addresses used in the memory chiplet (#1049).
- Supported `PartialMerkleTree` as a secret input in `.input` file (#1072).
- [BREAKING] Refactored `AdviceProvider` interface into `Host` interface (#1082).

#### Stdlib
- Completed `std::collections::smt` module by implementing `insert` and `set` procedures (#1036, #1038, #1046).
- Added new module `std::crypto::dsa::rpo_falcon512` to support Falcon signature verification (#1000, #1094)

## 0.6.1 (2023-06-29)

- Fixed `no-std` compilation for `miden-core`, `miden-assembly`, and `miden-processor` crates.

## 0.6.0 (2023-06-28)

#### Assembly
- Added new instructions: `mtree_verify`.
- [BREAKING] Refactored `adv.mem` decorator to use parameters from operand stack instead of immediate values.
- [BREAKING] Refactored `mem_stream` and `adv_pipe` instructions.
- Added constant support for memory operations.
- Enabled incremental compilation via `compile_in_context()` method.
- Exposed ability to compile individual modules publicly via `compile_module()` method.
- [BREAKING] Refactored advice injector instructions.
- Implemented procedure re-exports from modules.

#### CLI
- Implemented support for all types of nondeterministic inputs (advice stack, advice map, and Merkle store).
- Implemented ability to generate proofs suitable for recursion.

#### Stdlib
- Added new module: `std::collections::smt` (only `smt::get` available).
- Added new module: `std::collections::mmr`.
- Added new module: `std::collections::smt64`.
- Added several convenience procedures to `std::mem` module.
- [BREAKING] Added procedures to compute 1-to-1 hashes in `std::crypto::hashes` module and renamed existing procedures to remove ambiguity.
- Greatly optimized recursive STARK verifier (reduced number of cycles by 6x - 8x).

#### VM Internals
- Moved test framework from `miden-vm` crate to `miden-test-utils` crate.
- Updated Winterfell dependency to v0.6.4.
- Added support for GPU acceleration on Apple silicon (Metal).
- Added source locations to all AST nodes.
- Added 8 more instruction slots to the VM (not yet used).
- Completed kernel ROM trace generation.
- Implemented ability to record advice provider requests to the initial dataset via `RecAdviceProvider`.

## 0.5.0 (2023-03-29)

#### CLI
- Renamed `ProgramInfo` to `ExecutionDetails` since there is another `ProgramInfo` struct in the source code.
- [BREAKING] renamed `stack_init` and `advice_tape` to `operand_stack` and `advice_stack` in input files.
- Enabled specifying additional advice provider inputs (i.e., advice map and Merkle store) via the input files.

#### Assembly
- Added new instructions: `is_odd`, `assert_eqw`, `mtree_merge`.
- [BREAKING] Removed `mtree_cwm` instruction.
- Added `breakpoint` instruction to help with debugging.

#### VM Internals
- [BREAKING] Renamed `Read`, `ReadW` operations into `AdvPop`, `AdvPopW`.
- [BREAKING] Replaced `AdviceSet` with `MerkleStore`.
- Updated Winterfell dependency to v0.6.0.
- [BREAKING] Renamed `Read/ReadW` operations into `AdvPop/AdvPopW`.

## 0.4.0 (2023-02-27)

#### Advice provider
- [BREAKING] Converted `AdviceProvider` into a trait which can be provided to the processor.
- Added a decorator for interpolating polynomials over degree 2 extension field (`ext2intt`).
- Added `AdviceSource` enum for greater future flexibility of advice injectors.

#### CLI
- Added `debug` subcommand to enable stepping through program execution forward/backward.
- Added cycle count to the output of program execution.

#### Assembly
- Added support for constant declarations.
- Added new instructions: `clk`, `ext2*`, `fri_ext2fold4`, `hash`, `u32checked_popcnt`, `u32unchecked_popcnt`.
- [BREAKING] Renamed `rpperm` to `hperm` and `rphash` to `hmerge`.
- Removed requirement that code blocks must be non-empty (i.e., allowed empty blocks).
- [BREAKING] Refactored `mtree_set` and `mtree_cwm` instructions to leave the old value on the stack.
- [BREAKING] Replaced `ModuleProvider` with `Library` to improve 3rd party library support.

#### Processor, Prover, and Verifier
- [BREAKING] Refactored `execute()`, `prove()`, `verify()` functions to take `StackInputs` as one of the parameters.
- [BREAKING] Refactored `prove()` function to return `ExecutionProof` (which is a wrapper for `StarkProof`).
- [BREAKING] Refactored `verify()` function to take `ProgramInfo`, `StackInputs`, and `ExecutionProof` as parameters and return a `u32` indicating security level of the verified proof.

#### Stdlib
- Added `std::mem::memcopy` procedure for copying regions of memory.
- Added `std::crypto::fri::frie2f4::verify` for verifying FRI proofs over degree 2 extension field.

#### VM Internals
- [BREAKING] Migrated to Rescue Prime Optimized hash function.
- Updated Winterfell backend to v0.5.1

## 0.3.0 (2022-11-23)

- Implemented `call` operation for context-isolated function calls.
- Added support for custom kernels.
- Implemented `syscall` operation for kernel calls, and added a new `caller` instruction for accessing the hash of the calling function.
- Implemented `mem_stream` operation for fast hashing of memory regions.
- Implemented `adv_pipe` operation for fast "unhashing" of inputs into memory.
- Added support for unlimited number of stack inputs/outputs.
- [BREAKING] Redesigned Miden assembly input/output instructions for environment, random access memory, local memory, and non-deterministic "advice" inputs.
- [BREAKING] Reordered the output stack for Miden assembly cryptographic operations `mtree_set` and `mtree_get` to improve efficiency.
- Refactored the advice provider to add support for advice maps, and added the `adv.mem` decorator for copying memory regions into the advice map.
- [BREAKING] Refactored the Assembler and added support for module providers. (Standard library is no longer available by default.)
- Implemented AIR constraints for the stack component.
- Added Miden REPL tool.
- Improved performance with various internal refactorings and optimizations.

## 0.2.0 (2022-08-09)

- Implemented new decoder which removes limitations on the depth of control flow logic.
- Introduced chiplet architecture to offload complex computations to specialized modules.
- Added read-write random access memory.
- Added support for operations with 32-bit unsigned integers.
- Redesigned advice provider to include Merkle path advice sets.
- Changed base field of the VM to the prime field with modulus 2^64 - 2^32 + 1.

## 0.1.0 (2021-11-16)

- Initial release (migration of the original [Distaff VM](https://github.com/GuildOfWeavers/distaff) codebase to [Winterfell](https://github.com/novifinancial/winterfell) backend).
