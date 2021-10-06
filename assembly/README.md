# Distaff assembly
This crate contains Distaff assembler and description of the Distaff assembly language (located [here](doc/assembly.md)).

The purpose of the assembler is to compile Distaff assembly source code into a Distaff VM program (represented by `Program` struct). The program can then be executed on Distaff VM [processor](../processor).

## Compiling assembly code
To compile Distaff assembly source code into a program for Distaff VM, you can use the `compile()` function exposed by this crate. This function takes the following parameters:

* `source: &str` - a reference to a string containing Distaff assembly source code.

The `compile()` function returns `Result<Program, AssemblyError>` which will contain the compiled program if the compilation was successful, or if the source code contained errors, description of the first encountered error.

For example:
```Rust
use distaff_assembly::compile;

// the program pushes values 3 and 5 onto the stack and adds them
let program = compile("begin push.3 push.5 add end").unwrap();
```

## License
This project is [MIT licensed](../LICENSE).