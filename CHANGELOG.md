# Changelog

## 0.16.0 (TBD)

#### Changes

- Update lalrpop dependency to 0.22 (#1865)
- Removed the obsolete `RpoFalcon512` decorator and associated structs (#1872).
- Fixed instructions with errors print without quotes (#1882).
- Licensed the project under the Apache 2.0 license (in addition to the MIT) (#1883).
- [BREAKING] Renamed `Assembler::add_module` to `Assembler::compile_and_statically_link` (#1881)
- [BREAKING] Renamed `Assembler::add_modules` to `Assembler::compile_and_statically_link_all` (#1881)
- [BREAKING] Renamed `Assembler::add_modules_from_dir` to `Assembler::compile_and_statically_link_from_dir` (#1881)
- [BREAKING] Removed `Assembler::add_module_with_options` (#1881)
- [BREAKING] Removed `Assembler::add_modules_with_options` (#1881)
- [BREAKING] Renamed `Assembler::add_library` to `Assembler::link_dynamic_library` (#1881)
- [BREAKING] Renamed `Assembler::add_vendored_library` to `Assembler::link_static_library` (#1881)
- [BREAKING] `AssemblyError` was removed, and all uses replaced with `Report` (#1881).
- [BREAKING] `Compile` trait was renamed to `Parse`.
- [BREAKING] `CompileOptions` was renamed to `ParseOptions`.
- Uniform chiplet bus message flag encoding (#1887).
- [BREAKING] Updated dependencies Winterfell to v0.13 and Crypto to v0.15 (#1896).
- [BREAKING] Convert `AdviceProvider` into a struct ([#1904](https://github.com/0xMiden/miden-vm/issues/1904), [#1905](https://github.com/0xMiden/miden-vm/issues/1905))
- [BREAKING] `Host::get_mast_forest` takes `&mut self` ([#1902](https://github.com/0xMiden/miden-vm/issues/1902)
- [BREAKING] `ProcessState` returns `MemoryError` instead of `ExecutionError` ([#1912](https://github.com/0xMiden/miden-vm/issues/1912)
- [BREAKING] `AdviceProvider` should return its own error type ([#1907](https://github.com/0xMiden/miden-vm/issues/1907)
- Split out the syntax-related aspects of the `miden-assembly` crate into a new crate called `miden-assembly-syntax` ([#1921](https://github.com/0xMiden/miden-vm/pull/1921))
- Removed the dependency on `miden-assembly` from `miden-mast-package` ([#1921](https://github.com/0xMiden/miden-vm/pull/1921))
- [BREAKING] Removed `Library::from_dir` in favor of `Assembler::assemble_library_from_dir` ([#1921](https://github.com/0xMiden/miden-vm/pull/1921))
- [BREAKING] Removed `KernelLibrary::from_dir` in favor of `Assembler::assemble_kernel_from_dir` ([#1921](https://github.com/0xMiden/miden-vm/pull/1921))
- [BREAKING] Fixed incorrect namespace being set on modules parsed using the `lib_dir` parameter of `KernelLibrary::from_dir`. Previously, modules would be namespaced under `kernel`, but this should have been `$kernel`. Downstream kernels using this option should make sure that any references to the `kernel` namespace are replaced with `$kernel` instead. ([#1921](https://github.com/0xMiden/miden-vm/pull/1921)).
- [BREAKING] The signature of `SourceManager::load` has changed, and now requires a `SourceLanguage` and `Uri` parameter. ([#1937](https://github.com/0xMiden/miden-vm/pull/1937))
- [BREAKING] The signature of `SourceManager::load_from_raw_parts` has changed, and now requires a `Uri` parameter in place of `&str`. ([#1937](https://github.com/0xMiden/miden-vm/pull/1937))
- [BREAKING] The signature of `SourceManager::find` has changed, and now requires a `Uri` parameter in place of `&str`. ([#1937](https://github.com/0xMiden/miden-vm/pull/1937))
- [BREAKING] `SourceManager::get_by_path` was renamed to `get_by_uri`, and now requires a `&Uri` instead of a `&str` for the URI/path parameter ([#1937](https://github.com/0xMiden/miden-vm/pull/1937))
- [BREAKING] The `path` parameter of `Location` and `FileLineCol` debuginfo types was renamed to `uri`, and changed from `Arc<str>` to `Uri` type. ([#1937](https://github.com/0xMiden/miden-vm/pull/1937))
- [BREAKING] Move `AdviceProvider` from `Host` to `ProcessState` ([#1923](https://github.com/0xMiden/miden-vm/issues/1923)))
- Remove decorator for interpolating polynomials over degree 2 extension field ([#1875](https://github.com/0xMiden/miden-vm/issues/1875)).
- Remove MASM code for probabilistic NTT ([#1875](https://github.com/0xMiden/miden-vm/issues/1875)).
- [BREAKING] Disallow usage of the field modulus as an immediate value ([#1938](https://github.com/0xMiden/miden-vm/pull/1938)).

#### Enhancements

- Allow constants to be declared as words and to be arguments of the `push` instruction (#1855).
- Allow definition of Advice Map data in MASM programs. The data is loaded by the host before execution (#1862).
- The documentation for the `Assembler` and its APIs was improved, to better explain how each affects the final assembled artifact (#1881).
- It is now possible to assemble kernels with multiple modules while allowing those modules to perform kernel-like actions, such as using the `caller` instruction. (#1893)
- Optimize handling of variable length public inputs in the recursive verifier (#1842).
- Simplify processing of OOD evaluations in the recursive verifier (#1848).
- Make `ErrorContext` zero-cost ([#1910](https://github.com/0xMiden/miden-vm/issues/1910)).
- Make `FastProcessor` output rich error diagnostics ([#1914](https://github.com/0xMiden/miden-vm/issues/1914)).
- The `SourceManager` API was improved to be more precise about source file locations (URIs) and language type. This is intended to support the LSP server implementation. ([#1937](https://github.com/0xMiden/miden-vm/pull/1937))
- `SourceManager::update` was added to allow for the LSP server to update documents stored in the source manager based on edits made by the user. ([#1937](https://github.com/0xMiden/miden-vm/pull/1937))
- [BREAKING] Make `FastProcessor::execute()` async ([#1933](https://github.com/0xMiden/miden-vm/issues/1933)).

#### Fixes

- Modules can now be provided in any order to the `Assembler`, see #1669 (#1881)
- Addressed bug which caused references to re-exported procedures whose definition internally referred to an aliased module import, to produce an "undefined module" error, see #1451 (#1892)
- The special identifiers for kernel, executable, and anonymous namespaces were not valid MASM syntax (#1893)
- `AdviceProvider`: replace `SimpleAdviceMap` with `AdviceMap` struct from `miden-core` & add `merge_advice_map` to `AdviceProvider` ([#1924](https://github.com/0xMiden/miden-vm/issues/1924) & [#1922](https://github.com/0xMiden/miden-vm/issues/1922))

## 0.15.0 (2025-06-06)

#### Enhancements

- Add `debug.stack_adv` and `debug.stack_adv.<n>` to help debug the advice stack (#1828).
- Add a complete description of the constraints for `horner_eval_base` and `horner_eval_ext` (#1817).
- Add documentation for ACE chiplet (#1766)
- Add support for setting debugger breakpoints via `breakpoint` instruction (#1860)
- Improve error messages for some procedure locals-related errors (#1863)
- Add range checks to the `push_falcon_mod_result` advice injector to make sure that the inputs are `u32` (#1819).

#### Changes

- [BREAKING] Rename `miden` executable to `miden-vm`
- Improve error messages for some assembler instruction (#1785)
- Remove `idx` column from Kernel ROM chiplet and use chiplet bus for initialization. (#1818)
- [BREAKING] Make `Assembler::source_manager()` be `Send + Sync` (#1822)
- Refactored `ProcedureName` validation logic to improve readability (#1663)
- Simplify and optimize the recursive verifier (#1801).
- Simplify auxiliary randomness generation (#1810).
- Add handling of variable length public inputs to the recursive verifier (#1813).

#### Fixes

- `miden debug` rewind command no longer panics at clock 0 (#1751)
- Prevent overflow in ACE circuit evaluation (#1820)
- `debug.local` decorators no longer panic or print incorrect values (#1859)

## 0.14.0 (2025-05-07)

#### Enhancements

- Add kernel procedures digests as public inputs to the recursive verifier (#1724).
- add optional `Package::account_component_metadata_bytes` to store serialized `AccountComponentMetadata` (#1731).
- Add `executable` feature to the `make test` and `make test-build` Make commands (#1762).
- Allow asserts instruction to take error messages as strings instead of error codes as Felts (#1771).
- Add arithmetic evaluation chiplet (#1759).
- Update the recursive verifier to use arithmetic evaluation chiplet (#1760).

#### Changes

- Replace deprecated #[clap(...)] with #[command(...)] and #[arg(.…)] (#1794)
- Add pull request template to guide contributors (#1795)
- [BREAKING] `ExecutionOptions::with_debugging()` now takes a boolean parameter (#1761)
- Use `MemoryAddress(u32)` for `VmState` memory addresses instead of plain `u64` (#1758).
- [BREAKING] Improve processor errors for memory and calls (#1717)
- Implement a new fast processor that doesn't generate a trace (#1668)
- `ProcessState::get_stack_state()` now only returns the state of the active context (#1753)
- Change `MastForestBuilder::set_after_exit()` for `append_after_exit()` (#1775)
- Improve processor error diagnostics (#1765)
- Fix source spans associated with assert* and mtree_verify instructions (#1789)
- [BREAKING] Improve the layout of the memory used by the recursive verifier (#1857)

## 0.13.2 (2025-04-02)

#### Changes

- Relaxed rules for identifiers created via `Ident::new`, `ProcedureName::new`, `LibraryNamespace::new`, and `Library::new_from_components` (#1735)
- [BREAKING] Renamed `Ident::new_unchecked` and `ProcedureName::new_unchecked` to `from_raw_parts` (#1735).

#### Fixes

- Fixed various issues with pretty printing of Miden Assembly (#1740).

## 0.13.1 (2025-03-21) - `stdlib` crate only

#### Enhancements

- Added `prepare_hasher_state` and `hash_memory_with_state` procedures to the `stdlib::crypto::hashes::rpo` module (#1718).

## 0.13.0 (2025-03-20)

#### Enhancements

- Added to the `Assembler` the ability to vendor a compiled library.
- [BREAKING] Update CLI to accept masm or masp files as input for all commands (#1683, #1692).
- [BREAKING] Introduced `HORNERBASE`, `HORNEREXT` and removed `RCOMBBASE` instructions (#1656).

#### Changes

- Update minimum supported Rust version to 1.85.
- Change Chiplet Fields to Public (#1629).
- [BREAKING] Updated Winterfell dependency to v0.12 (#1658).
- Introduce `BusDebugger` to facilitate debugging buses (#1664).
- Update Falcon verification procedure to use `HORNERBASE` (#1661).
- Update recursive verifier to use `HORNERBASE` (#1665).
- Fix the docs and implementation of `EXPACC` (#1676).
- Running a call/syscall/dyncall while processing a syscall now results in an error (#1680).
- Using a non-binary value as a loop condition now results in an error (#1685).
- [BREAKING] Remove `Assembler::assemble_common()` from the public interface (#1689).
- Fix `Horner{Base, Ext}` bus requests to memory chiplet (#1689).
- Fix docs on the layout of the auxiliary segment trace (#1694).
- Optimize FRI remainder polynomial check (#1670).
- Remove `FALCON_SIG_TO_STACK` event (#1703).
- Prevent `U64Div` event from crashing processor (#1710).

## 0.12.0 (2025-01-22)

#### Highlights

- [BREAKING] Refactored memory to be element-addressable (#1598).

#### Changes

- [BREAKING] Resolved flag collision in `--verify` command and added functionality for optional input/output files (#1513).
- [BREAKING] Refactored `MastForest` serialization/deserialization to put decorator data at the end of the binary (#1531).
- [BREAKING] Refactored `Process` struct to no longer take ownership of the `Host` (#1571).
- [BREAKING] Converted `ProcessState` from a trait to a struct (#1571).
- [BREAKING] Simplified `Host` and `AdviceProvider` traits (#1572).
- [BREAKING] Updated Winterfell dependency to v0.11 (#1586).
- [BREAKING] Cleaned up benchmarks and examples in the `miden-vm` crate (#1587)
- [BREAKING] Switched to `thiserror` 2.0 derive errors and refactored errors (#1588).
- Moved handling of `FalconSigToStack` event from system event handlers to the `DefaultHost` (#1630).

#### Enhancements

- Added options `--kernel`, `--debug` and `--output` to `miden bundle` (#1447).
- Added `miden_core::mast::MastForest::advice_map` to load it into the advice provider before the `MastForest` execution (#1574).
- Optimized the computation of the DEEP queries in the recursive verifier (#1594).
- Added validity checks for the inputs to the recursive verifier (#1596).
- Allow multiple memory reads in the same clock cycle (#1626)
- Improved Falcon signature verification (#1623).
- Added `miden-mast-package` crate with `Package` type to represent a compiled Miden program/library (#1544).

## 0.11.0 (2024-11-04)

#### Enhancements

- Added `miden_core::utils::sync::racy_lock` module (#1463).
- Updated `miden_core::utils` to re-export `std::sync::LazyLock` and `racy_lock::RacyLock as LazyLock` for std and no_std environments, respectively (#1463).
- Debug instructions can be enabled in the cli `run` command using `--debug` flag (#1502).
- Added support for procedure annotation (attribute) syntax to Miden Assembly (#1510).
- Make `miden-prover::prove()` method conditionally asynchronous (#1563).
- Update and sync the recursive verifier (#1575).

#### Changes

- [BREAKING] Wrapped `MastForest`s in `Program` and `Library` structs in `Arc` (#1465).
- `MastForestBuilder`: use `MastNodeId` instead of MAST root to uniquely identify procedures (#1473).
- Made the undocumented behavior of the VM with regard to undefined behavior of u32 operations, stricter (#1480).
- Introduced the `Emit` instruction (#1496).
- [BREAKING] ExecutionOptions::new constructor requires a boolean to explicitly set debug mode (#1502).
- [BREAKING] The `run` and the `prove` commands in the cli will accept `--trace` flag instead of `--tracing` (#1502).
- Migrated to new padding rule for RPO (#1343).
- Migrated to `miden-crypto` v0.11.0 (#1343).
- Implemented `MastForest` merging (#1534).
- Rename `EqHash` to `MastNodeFingerprint` and make it `pub` (#1539).
- Updated Winterfell dependency to v0.10 (#1533).
- [BREAKING] `DYN` operation now expects a memory address pointing to the procedure hash (#1535).
- [BREAKING] `DYNCALL` operation fixed, and now expects a memory address pointing to the procedure hash (#1535).
- Permit child `MastNodeId`s to exceed the `MastNodeId`s of their parents (#1542).
- Don't validate export names on `Library` deserialization (#1554)
- Compile advice injectors down to `Emit` operations (#1581)

#### Fixes

- Fixed an issue with formatting of blocks in Miden Assembly syntax
- Fixed the construction of the block hash table (#1506)
- Fixed a bug in the block stack table (#1511) (#1512) (#1557)
- Fixed the construction of the chiplets virtual table (#1514) (#1556)
- Fixed the construction of the chiplets bus (#1516) (#1525)
- Decorators are now allowed in empty basic blocks (#1466)
- Return an error if an instruction performs 2 memory accesses at the same memory address in the same cycle (#1561)

## 0.10.6 (2024-09-12) - `miden-processor` crate only

#### Enhancements

- Added `PartialEq`, `Eq`, `Serialize` and `Deserialize` to `AdviceMap` and `AdviceInputs` structs (#1494).

## 0.10.5 (2024-08-21)

#### Enhancements

- Updated `MastForest::read_from` to deserialize without computing node hashes unnecessarily (#1453).
- Assembler: Merge contiguous basic blocks (#1454).
- Assembler: Add a threshold number of operations after which we stop merging more in the same block (#1461).

#### Changes

- Added `new_unsafe()` constructors to MAST node types which do not compute node hashes (#1453).
- Consolidated `BasicBlockNode` constructors and converted assert flow to `MastForestError::EmptyBasicBlock` (#1453).

#### Fixes

- Fixed an issue with registering non-local procedures in `MemMastForestStore` (#1462).
- Added a check for circular external node lookups in the processor (#1464).

## 0.10.4 (2024-08-15) - `miden-processor` crate only

#### Enhancements

- Added support for executing `Dyn` nodes from external MAST forests (#1455).

## 0.10.3 (2024-08-12)

#### Enhancements

- Added `with-debug-info` feature to `miden-stdlib` (#1445).
- Added `Assembler::add_modules_from_dir()` method (#1445).
- [BREAKING] Implemented building of multi-module kernels (#1445).

#### Changes

- [BREAKING] Replaced `SourceManager` parameter with `Assembler` in `Library::from_dir` (#1445).
- [BREAKING] Moved `Library` and `KernelLibrary` exports to the root of the `miden-assembly` crate. (#1445).
- [BREAKING] Depth of the input and output stack was restricted to 16 (#1456).

## 0.10.2 (2024-08-10)

#### Enhancements

- Removed linear search of trace rows from `BlockHashTableRow::table_init()` (#1439).
- Exposed some pretty printing internals for `MastNode` (#1441).
- Made `KernelLibrary` impl `Clone` and `AsRef<Library>` (#1441).
- Added serialization to the `Program` struct (#1442).

#### Changes

- [BREAKING] Removed serialization of AST structs (#1442).

## 0.10.0 (2024-08-06)

#### Features

- Added source location tracking to assembled MAST (#1419).
- Added error codes support for the `mtree_verify` instruction (#1328).
- Added support for immediate values for `lt`, `lte`, `gt`, `gte` comparison instructions (#1346).
- Added support for immediate values for `u32lt`, `u32lte`, `u32gt`, `u32gte`, `u32min` and `u32max` comparison instructions (#1358).
- Added support for the `nop` instruction, which corresponds to the VM opcode of the same name, and has the same semantics.
- Added support for the `if.false` instruction, which can be used in the same manner as `if.true`
- Added support for immediate values for `u32and`, `u32or`, `u32xor` and `u32not` bitwise instructions (#1362).
- [BREAKING] Assembler: add the ability to compile MAST libraries, and to assemble a program using compiled libraries (#1401)

#### Enhancements

- Changed MAST to a table-based representation (#1349).
- Introduced `MastForestStore` (#1359).
- Adjusted prover's metal acceleration code to work with 0.9 versions of the crates (#1357).
- Relaxed the parser to allow one branch of an `if.(true|false)` to be empty.
- Optimized `std::sys::truncate_stuck` procedure (#1384).
- Updated CI and Makefile to standardize it across Miden repositories (#1342).
- Add serialization/deserialization for `MastForest` (#1370).
- Updated CI to support `CHANGELOG.md` modification checking and `no changelog` label (#1406).
- Introduced `MastForestError` to enforce `MastForest` node count invariant (#1394).
- Added functions to `MastForestBuilder` to allow ensuring of nodes with fewer LOC (#1404).
- [BREAKING] Made `Assembler` single-use (#1409).
- Removed `ProcedureCache` from the assembler (#1411).
- Added functions to `MastForest` and `MastForestBuilder` to add and ensure nodes with fewer LOC (#1404, #1412).
- Added `Assembler::assemble_library()` and `Assembler::assemble_kernel()`  (#1413, #1418).
- Added `miden_core::prettier::pretty_print_csv` helper, for formatting of iterators over `PrettyPrint` values as comma-separated items.
- Added source code management primitives in `miden-core` (#1419).
- Added `make test-fast` and `make test-skip-proptests` Makefile targets for faster testing during local development.
- Added `ProgramFile::read_with` constructor that takes a `SourceManager` impl to use for source management.
- Added `RowIndex(u32)` (#1408).

#### Changed

- When using `if.(true|false) .. end`, the parser used to emit an empty block for the branch that was elided. The parser now emits a block containing a single `nop` instruction instead.
- [BREAKING] `internals` configuration feature was renamed to `testing` (#1399).
- The `AssemblyOp` decorator now contains an optional `Location` (#1419)
- The `Assembler` now requires passing in a `Arc<dyn SourceManager>`, for use in rendering diagnostics.
- The `Module::parse_file` and `Module::parse_str` functions have been removed in favor of calling `Module::parser` and then using the `ModuleParser` methods.
- The `Compile` trait now requires passing a `SourceManager` reference along with the item to be compiled.
- Update minimum supported Rust version to 1.80 (#1425).
- Made `debug` mode the default in the CLI. Added `--release` flag to disable debugging instead of having to enable it. (#1728)

## 0.9.2 (2024-05-22) - `stdlib` crate only

- Skip writing MASM documentation to file when building on docs.rs (#1341).

## 0.9.2 (2024-05-09) - `assembly` crate only

- Remove usage of `group_vector_elements()` from `combine_blocks()` (#1331).

## 0.9.2 (2024-04-25) - `air` and `processor` crates only

- Allowed enabling debug mode via `ExecutionOptions` (#1316).

## 0.9.1 (2024-04-04)

- Added additional trait implementations to error types (#1306).

## 0.9.0 (2024-04-03)

#### Packaging

- [BREAKING] The package `miden-vm` crate was renamed from `miden` to `miden-vm`. Now the package and crate names match (#1271).

#### Stdlib

- Added `init_no_padding` procedure to `std::crypto::hashes::native` (#1313).
- [BREAKING] `native` module was renamed to the `rpo`, `hash_memory` procedure was renamed to the `hash_memory_words` (#1368).
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

- Introduced `std::utils` module with `is_empty_word` procedure. Refactored `std::collections::smt`
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
