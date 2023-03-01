# Miden Standard Library
Miden standard library provides a set of procedures which can be used by any Miden program. These procedures build on the core instruction set of [Miden assembly](../assembly/main.md) expanding the functionality immediately available to the user.

The goals of Miden standard library are:
* Provide highly-optimized and battle-tested implementations of commonly-used primitives.
* Reduce the amount of code that needs to be shared between parties for proving and verifying program execution.

The second goal can be achieved because calls to procedures in the standard library can always be serialized as 32 bytes, regardless of how large the procedure is.

## Organization and usage
Procedures in the Miden Standard Library are organized into modules, each targeting a narrow set of functionality. Modules are grouped into higher-level namespaces. However, higher-level namespaces do not expose any procedures themselves. For example, `std::math::u64` is a module containing procedures for working with 64-bit unsigned integers. This module is a part of the `std::math` namespace. However, the `std::math` namespace does not expose any procedures.

For an example of how to invoke procedures from imported modules see [this section](../assembly/code_organization.md#importing-modules).

## Available modules
Currently, Miden standard library contains just a few modules, which are listed below. Over time, we plan to add many more modules which will include various cryptographic primitives, additional numeric data types and operations, and many others.

| Module | Description |
| ------ | ----------- |
| [std::crypto::fri::frie2f4](./crypto/fri.md#fri-extension-2-fold-4) | Contains procedures for verifying FRI proofs (field extension = 2, folding factor = 4). |
| [std::crypto::hashes::blake3](./crypto/hashes.md#blake3) | Contains procedures for computing hashes using BLAKE3 hash function. |
| [std::crypto::hashes::sha256](./crypto/hashes.md#sha256) | Contains procedures for computing hashes using SHA256 hash function. |
| [std::math::u64](./math/u64.md) | Contains procedures for working with 64-bit unsigned integers. |
| [std::mem](./mem.md)            | Contains procedures for working with random access memory. |
| [std::sys](./sys.md)            | Contains system-level utility procedures. |
