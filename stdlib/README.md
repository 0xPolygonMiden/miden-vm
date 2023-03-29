# Miden stdlib
Standard library for Miden VM.

Miden standard library provides a set of procedures which can be used by any Miden program. These procedures build on the core instruction set of [Miden assembly](../assembly) expanding the functionality immediately available to the user.

The goals of Miden standard library are:
* Provide highly-optimized and battle-tested implementations of commonly-used primitives.
* Reduce the amount of code that needs to be shared between parties for proving and verifying program execution.

The second goal can be achieved because calls to procedures in the standard library can always be serialized as 32 bytes, regardless of how large the procedure is.

## Available modules
Currently, Miden standard library contains just a few modules, which are listed below. Over time, we plan to add many more modules which will include various cryptographic primitives, additional numeric data types and operations, and many others.

- [std::crypto::hashes::blake3](./docs/blake3_hashes.md)
- [std::crypto::hashes::keccak256](./docs/keccak256_hashes.md)
- [std::crypto::hashes::sha256](./docs/sha256_hashes.md)
- [std::crypto::fri::frie2f4](./docs/ext2fri_fri.md)
- [std::math::u256](./docs/u256_math.md)
- [std::math::u64](./docs/u64_math.md)
- [std::math::secp256k1](./docs/secp256k1_math.md)
- [std::mem](./docs/mem_std.md)
- [std::sys](./docs/sys_std.md)

## Status
At this point, all implementations listed above are considered to be experimental and are subject to change.

## License
This project is [MIT licensed](../LICENSE).
