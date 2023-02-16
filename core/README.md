# Miden core 
This crate contains core components used by Miden VM. These components include:

* Miden VM instruction set, defined in the [Operation](/../main/core/src/operations/mod.rs) struct.
* Miden VM kernel library, defined in [Kernel](/../main/core/src/program/mod.rs) struct that will contain the set of roots of the kernel routines.
* Miden VM program structure, defined in [Program](/../main/core/src/program/mod.rs) struct and described [here](https://0xpolygonmiden.github.io/miden-vm/design/programs.html).
* Miden VM program metadata, defined in [ProgramInfo](/../main/core/src/program/info.rs) struct that will contain the MAST root and the used Kernel.
* Initial stack containner for Miden VM programs, defined in [StackInputs](/../main/core/src/stack/inputs.rs) struct.
* Post-execution stack containner for Miden VM programs, defined in [StackOutputs](/../main/core/src/stack/outputs.rs) struct.
* Constants describing the shape of the VM's execution trace.
* Various minor utility functions used by other VM crates.

## License
This project is [MIT licensed](../LICENSE).
