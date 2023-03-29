# Changelog

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

#### VM Internals
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
