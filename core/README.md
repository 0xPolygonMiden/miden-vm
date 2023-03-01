# Miden core
This crate contains core components used by Miden VM. These components include:

* Miden VM instruction set, defined in the [Operation](/../main/core/src/operations/mod.rs) struct.
* Miden VM program kernel, defined in [Kernel](/../main/core/src/program/mod.rs) struct which contains a set of roots of kernel routines.
* Miden VM program structure, defined in [Program](/../main/core/src/program/mod.rs) struct and described [here](https://0xpolygonmiden.github.io/miden-vm/design/programs.html).
* Miden VM program metadata, defined in [ProgramInfo](/../main/core/src/program/info.rs) struct which contains a program's MAST root and the kernel used by the program.
* Input and output containers for Miden VM programs, defined in [StackInputs](/../main/core/src/stack/inputs.rs) and [StackOutputs](/../main/core/src/stack/outputs.rs) structs.
* Constants describing the shape of the VM's execution trace.
* Various minor utility functions used by other VM crates.

## License
This project is [MIT licensed](../LICENSE).
