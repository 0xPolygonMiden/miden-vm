# Miden core 
This crate contains core components used by Miden VM. These components include:

* Miden VM instruction set, defined in the [Operation](/../main/core/src/operations/mod.rs) struct.
* Miden VM program structure, defined in [Program](/../main/core/src/program/mod.rs) struct and described [here](https://0xpolygonmiden.github.io/miden-vm/design/programs.html).
* Initial stack containner for Miden VM programs, defined in [StackInputs](/../main/core/src/inputs/stack.rs) struct.
* Implementations of [advice sets](/../main/core/src/inputs/advice/mod.rs) which are used to provide nondeterministic inputs to the VM.
* Constants describing the shape of the VM's execution trace.
* Various minor utility functions used by other VM crates.

## License
This project is [MIT licensed](../LICENSE).
