# Polygon Miden

<a href="https://github.com/maticnetwork/miden/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
<img src="https://github.com/maticnetwork/miden/workflows/CI/badge.svg?branch=main">
<a href="https://crates.io/crates/miden"><img src="https://img.shields.io/crates/v/miden"></a>

A STARK-based virtual machine.

**WARNING:** This project is in an alpha stage. It has not been audited and may contain bugs and security flaws. This implementation is NOT ready for production use.

## Overview
Miden VM is a zero-knowledge virtual machine written in Rust. For any program executed on Miden VM, a STARK-based proof of execution is automatically generated. This proof can then be used by anyone to verify that the program was executed correctly without the need for re-executing the program or even knowing the contents of the program.

* If you'd like to learn more about how Miden VM works, check out the [documentation](https://wiki.polygon.technology/docs/miden/intro/main/).
* If you'd like to start using Miden VM, check out the [miden](miden) crate.
* If you'd like to learn more about STARKs, check out the [references](#references) section.

### Status and features
Miden VM is currently on v0.2 release. In this release, most of the core features of the VM have been stabilized, and most of the STARK proof generation has been implemented. While we expect to keep making changes to the VM internals, the external interfaces should remain relatively stable, and we will do our best to minimize the amount of breaking changes going forward.

The next version of the VM is being developed in the [next](https://github.com/maticnetwork/miden/tree/next) branch. There is also a documentation for the latest features and changes in the next branch [documentation next branch](https://maticnetwork.github.io/miden/intro/main.html).

#### Feature highlights
Miden VM is a fully-featured virtual machine. Despite being optimized for zero-knowledge proof generation, it provides all the features one would expect from a regular VM. To highlight a few:

* **Flow control.** Miden VM is Turing-complete and supports familiar flow control structures such as conditional statements and counter/condition-controlled loops. There are no restrictions on the maximum number of loop iterations or the depth of control flow logic.
* **Procedures.** Miden assembly programs can be broken into subroutines called *procedures*. This improves code modularity and helps reduce the size of Miden VM programs.
* **Memory.** Miden VM supports read-write random-access memory. Procedures can reserve portions of global memory for easier management of local variables.
* **u32 operations.** Miden VM supports native operations with 32-bit unsigned integers. This includes basic arithmetic, comparison, and bitwise operations.
* **Cryptographic operations.** Miden assembly provides built-in instructions for computing hashes and verifying Merkle paths. These instructions use Rescue Prime hash function (which is the native hash function of the VM).
* **Standard library.** Miden VM ships with a standard library which expands the core functionality of the VM (e.g., by adding support for 64-bit unsigned integers). Currently, the standard library is quite limited, but we plan to expand it significantly in the future.
* **Nondeterminism**. Unlike traditional virtual machines, Miden VM supports nondeterministic programming. This means a prover may do additional work outside of the VM and then provide execution *hints* to the VM. These hints can be used to dramatically speed up certain types of computations, as well as to supply secret inputs to the VM.

#### Planned features
In the coming months we plan to finalize the design of the VM and implement support for the following features:

* **Custom kernels.** It will be possible to instantiate Miden VM with a custom kernel. There will also be full separation between kernel space and user space, and we will introduce a number of opcodes to facilitate safe communication between the two.
* **Persistent storage.** Support for read-write persistent storage will be provided at the kernel level. With support for custom kernels, we will be able to support multiple modes of persistent storage, such as key-value maps, index-based arrays etc.
* **Faulty execution.** Miden VM will support generating proofs for programs with faulty execution (a notoriously complex task in ZK context). That is, it will be possible to prove that execution of some program resulted in an error.

#### Compilation to WebAssembly.
Miden VM is written in pure Rust and can be compiled to WebAssembly. Rust's `std` standard library is enabled as feature by default for most crates. For WASM targets, one can compile with default features disabled by using `--no-default-features` flag.

#### Concurrent proof generation
When compiled with `concurrent` feature enabled, the prover will generate STARK proofs using multiple threads. For benefits of concurrent proof generation check out benchmarks below.

Internally, we use [rayon](https://github.com/rayon-rs/rayon) for parallel computations. To control the number of threads used to generate a STARK proof, you can use `RAYON_NUM_THREADS` environment variable.

### Project structure
The project is organized into several crates like so:

| Crate                  | Description |
| ---------------------- | ----------- |
| [core](core)           | Contains components defining Miden VM instruction set, program structure, and a set of utility functions used by other crates. |
| [assembly](assembly)   | Contains Miden assembler. The assembler is used to compile Miden assembly source code into Miden VM programs. |
| [processor](processor) | Contains Miden VM processor. The processor is used to execute Miden programs and to generate program execution traces. These traces are then used by the Miden prover to generate proofs of correct program execution. |
| [air](air)             | Contains *algebraic intermediate representation* (AIR) of Miden VM processor logic. This AIR is used by the VM during proof generation and verification processes. |
| [prover](prover)       | Contains Miden VM prover. The prover is used to generate STARK proofs attesting to correct execution of Miden VM programs. Internally, the prover uses Miden processor to execute programs. |
| [verifier](verifier)   | Contains a light-weight verifier which can be used to verify proofs of program execution generated by Miden VM. |
| [miden](miden)         | Aggregates functionality exposed by Miden VM processor, prover, and verifier in a single place, and also provide a CLI interface for Miden VM. |
| [stdlib](stdlib)       | Contains Miden standard library. The goal of Miden standard library is to provide highly-optimized and battle-tested implementations of commonly-used primitives. |

## Performance
The benchmarks below should be viewed only as a rough guide for expected future performance. The reasons for this are twofold:
1. Not all constraints have been implemented yet, and we expect that there will be some slowdown once constraint evaluation is completed.
2. Many optimizations have not been applied yet, and we expect that there will be some speedup once we dedicate some time to performance optimizations.

Overall, we don't expect the benchmarks to change significantly, but there will definitely be some deviation from the below numbers in the future.

A few general notes on performance:

* Execution time is dominated by proof generation time. In fact, the time needed to run the program is usually under 1% of the time needed to generate the proof.
* Proof verification time is really fast. In most cases it is under 1 ms, but sometimes gets as high as 2 ms or 3 ms.
* Proof generation process is dynamically adjustable. In general, there is a trade-off between execution time, proof size, and security level (i.e. for a given security level, we can reduce proof size by increasing execution time, up to a point).
* Both proof generation and proof verification times are greatly influenced by the hash function used in the STARK protocol. In the benchmarks below, we use BLAKE3, which is a really fast hash function.

### Single-core prover performance
When executed on a single CPU core, the current version of Miden VM operates at around 10 - 15 KHz. In the benchmarks below, the VM executes a [Fibonacci calculator](miden/README.md#fibonacci-calculator) program on Apple M1 Pro CPU in a single thread. The generated proofs have a target security level of 96 bits.

| VM cycles       | Execution time | Proving time | RAM consumed  | Proof size |
| :-------------: | :------------: | :----------: | :-----------: | :--------: |
| 2<sup>10</sup>  |  2 ms          | 80 ms        | 13 MB         | 49 KB      |
| 2<sup>12</sup>  |  4 ms          | 280 ms       | 41 MB         | 59 KB      |
| 2<sup>14</sup>  |  10 ms         | 1.05 sec     | 155 MB        | 70 KB      |
| 2<sup>16</sup>  |  30 ms         | 4.2 sec      | 624 MB        | 79 KB      |
| 2<sup>18</sup>  |  90 ms         | 18 sec       | 2.5 GB        | 91 KB      |
| 2<sup>20</sup>  |  330 ms        | 79 sec       | 9.5 GB        | 104 KB     |

As can be seen from the above, proving time roughly doubles with every doubling in the number of cycles, but proof size grows much slower.

We can also generate proofs at a higher security level. The cost of doing so is roughly doubling of proving time and roughly 40% increase in proof size. In the benchmarks below, the same Fibonacci calculator program was executed on Apple M1 Pro CPU at 128-bit target security level:

| VM cycles       | Execution time | Proving time | RAM consumed  | Proof size |
| :-------------: | :------------: | :----------: | :-----------: | :--------: |
| 2<sup>10</sup>  | 2 ms           | 340 ms       | 24 MB         | 72 KB      |
| 2<sup>12</sup>  | 4 ms           | 560 ms       | 86 MB         | 84 KB      |
| 2<sup>14</sup>  | 10 ms          | 2.2 sec      | 335 MB        | 97 KB      |
| 2<sup>16</sup>  | 30 ms          | 7.7 sec      | 1.4 GB        | 113 KB     |
| 2<sup>18</sup>  | 90 ms          | 34 sec       | 5.4 GB        | 130 KB     |
| 2<sup>20</sup>  | 330 ms         | 143 sec      | 19.7 GB       | 147 KB     |

### Multi-core prover performance
STARK proof generation is massively parallelizable. Thus, by taking advantage of multiple CPU cores we can dramatically reduce proof generation time. For example, when executed on a high-end 8-core CPU (Apple M1 Pro), the current version of Miden VM operates at around 80 KHz. And when executed on a high-end 64-core CPU (Amazon Graviton 3), the VM operates at around 320 KHz.

In the benchmarks below, the VM executes the same Fibonacci calculator program for 2<sup>20</sup> cycles at 96-bit target security level:

| Machine                        | Execution time | Proving time |
| ------------------------------ | :------------: | :----------: |
| Apple M1 Pro (8 threads)       | 330 ms         | 12.8 sec     |
| Amazon Graviton 3 (64 threads) | 390 ms         | 3.2 sec      |

## References
Proofs of execution generated by Miden VM are based on STARKs. A STARK is a novel proof-of-computation scheme that allows you to create an efficiently verifiable proof that a computation was executed correctly. The scheme was developed by Eli Ben-Sasson, Michael Riabzev et al. at Technion - Israel Institute of Technology. STARKs do not require an initial trusted setup, and rely on very few cryptographic assumptions.

Here are some resources to learn more about STARKs:

* STARKs whitepaper: [Scalable, transparent, and post-quantum secure computational integrity](https://eprint.iacr.org/2018/046)
* STARKs vs. SNARKs: [A Cambrian Explosion of Crypto Proofs](https://nakamoto.com/cambrian-explosion-of-crypto-proofs/)

Vitalik Buterin's blog series on zk-STARKs:
* [STARKs, part 1: Proofs with Polynomials](https://vitalik.ca/general/2017/11/09/starks_part_1.html)
* [STARKs, part 2: Thank Goodness it's FRI-day](https://vitalik.ca/general/2017/11/22/starks_part_2.html)
* [STARKs, part 3: Into the Weeds](https://vitalik.ca/general/2018/07/21/starks_part_3.html)

Alan Szepieniec's STARK tutorials:
* [Anatomy of a STARK](https://aszepieniec.github.io/stark-anatomy/)
* [BrainSTARK](https://aszepieniec.github.io/stark-brainfuck/)

StarkWare's STARK Math blog series:
* [STARK Math: The Journey Begins](https://medium.com/starkware/stark-math-the-journey-begins-51bd2b063c71)
* [Arithmetization I](https://medium.com/starkware/arithmetization-i-15c046390862)
* [Arithmetization II](https://medium.com/starkware/arithmetization-ii-403c3b3f4355)
* [Low Degree Testing](https://medium.com/starkware/low-degree-testing-f7614f5172db)
* [A Framework for Efficient STARKs](https://medium.com/starkware/a-framework-for-efficient-starks-19608ba06fbe)

StarkWare's STARK tutorial:
 * [STARK 101](https://starkware.co/stark-101/)

## License
This project is [MIT licensed](./LICENSE).
