# Miden processor
This crate contains an implementation of Miden VM processor. The purpose of the processor is to execute a program and to generate a program execution trace. This trace is then used by Miden VM to generate a proof of correct execution of the program.

The processor exposes an `execute()` function which takes the following parameters:

* `program: &Program` - a reference to a Miden program to be executed.
* `inputs: &ProgramInputs` - a reference to a set of public and secret inputs with which to execute the program.

If the program is executed successfully, the function will return `ExecutionTrace<BaseElement>` struct contain the execution trace of the program. Otherwise, the function will panic.

For example:
```Rust
use miden_assembly::compile;
use miden_processor::{execute, ProgramInputs};

// compile Miden assembly source code into a program
let program = compile("begin push.3 push.5 add end").unwrap();

// execute the program with no inputs
let trace = execute(&program, &ProgramInputs::none());
```

Internally, the processor is separated into two parts:
* The decoder, which is responsible for decoding instructions and managing control flow.
* The stack, which is responsible for executing instructions against the stack.

## License
This project is [MIT licensed](../LICENSE).