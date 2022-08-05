# Miden assembly
This crate contains Miden assembler.

The purpose of the assembler is to compile [Miden assembly](https://maticnetwork.github.io/miden/user_docs/assembly/main.html) source code into a Miden VM program (represented by `Program` struct). The program can then be executed on Miden VM [processor](../processor).

## Compiling assembly code
To compile Miden assembly source code into a program for Miden VM, you first need to instantiate the assembler, and then call its `compile()` method. This method takes the following arguments:

* `source: &str` - a reference to a string containing Miden assembly source code.

The `compile()` function returns `Result<Program, AssemblyError>` which will contain the compiled program if the compilation was successful, or if the source code contained errors, description of the first encountered error.

For example:
```Rust
use miden_assembly::Assembler;

// instantiate a default assembler
let assembler = Assembler::default();

// compile a program which pushes values 3 and 5 onto the stack and adds them
let program = assembler.compile("begin push.3 push.5 add end").unwrap();
```

### Debug mode
It is also possible to instantiate the assembler in debug mode like so:
```Rust
use miden_assembly::Assembler;

// instantiate the assembler in debug mode
let assembler = Assembler::new(true);
```
Compiling a program with an assembler instantiated in debug mode retains source mappings between assembly instructions and VM operations. Thus, when such a program is executed using `execute_iter()` function of the [processor](../processor), is it possible to tell exactly which assembly instruction is being executed at a specific VM cycle.

## License
This project is [MIT licensed](../LICENSE).