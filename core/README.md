# Miden core
This crate contains core components used by Miden VM. These components include:

* Instruction set architecture (ISA) defined [here](src/opcodes.rs) and described [here](doc/isa.md).
* Program structure defined [here](src/programs/mod.rs) and described [here](doc/programs.md).
* Implementations of Rescue hash function used by the VM.
* Various minor utility functions used by other VM crates.

## License
This project is [MIT licensed](../LICENSE).