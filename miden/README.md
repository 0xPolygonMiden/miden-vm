# Miden VM
This crate aggregates all components of Miden VM in a single place. Specifically, it re-exports functionality from [processor](../processor/), [prover](../prover/), and [verifier](../verifier/) crates. Additionally, when compiled as an executable, this crate can be used via a [CLI interface](#cli-interface) to execute Miden VM programs and to verify correctness of their execution.

## Basic concepts
An in-depth description of Miden VM is available in the full Miden VM [documentation](https://maticnetwork.github.io/miden/). In this section we cover only the basics to make the included examples easier to understand.

### Writing programs
Our goal is to make Miden VM an easy compilation target for high-level blockchain-centric languages such as Solidity, Move, Sway, and others. We believe it is important to let people write programs in the languages of their choice. However, compilers to help with this have not been developed yet. Thus, for now, the primary way to write programs for Miden VM is to use [Miden assembly](../assembly).

Miden assembler compiles assembly source code in a [program MAST](https://maticnetwork.github.io/miden/design/programs.html), which is represented by a `Program` struct. It is possible to construct a `Program` struct manually, but we don't recommend this approach because it is tedious, error-prone, and requires an in-depth understanding of VM internals. All examples throughout these docs use assembly syntax.

#### Program hash
All Miden programs can be reduced to a single 32-byte value, called program hash. Once a `Program` object is constructed, you can access this hash via `Program::hash()` method. This hash value is used by a verifier when they verify program execution. This ensures that the verifier verifies execution of a specific program (e.g. a program which the prover had committed to previously). The methodology for computing program hash is described [here](https://maticnetwork.github.io/miden/design/programs.html#program-hash-computation).

### Inputs / outputs
Currently, there are 3 ways to get values onto the stack:

1. You can use `push` instruction to push values onto the stack. These values become a part of the program itself, and, therefore, cannot be changed between program executions. You can think of them as constants.
2. The stack can be initialized to some set of values at the beginning of the program. These inputs are public and must be shared with the verifier for them to verify a proof of the correct execution of a Miden program. The number of elements at the top of the stack which can receive an initial value is limited to 16.
3. The program may request nondeterministic advice inputs from the prover. These inputs are secret inputs. This means that the prover does not need to share them with the verifier. There are two types of advice inputs: (1) a single advice tape which can contain any number of elements and (2) a list of advice sets, which are used to provide nondeterministic inputs for instructions which work with Merkle trees. There are no restrictions on the number of advice inputs a program can request.

Stack and advice inputs are provided to Miden VM via `ProgramInputs` struct. To instantiate this struct, you can use `ProgramInputs::new()` constructor, as well as `ProgramInputs::from_stack_inputs()` and `ProgramInputs:none()` convenience constructors.

Values remaining on the stack after a program is executed can be returned as program outputs. You can specify exactly how many values (from the top of the stack) should be returned. Currently, the maximum number of outputs is limited to 16.

Having only 16 elements to describe public inputs and outputs of a program may seem limiting, however, just 4 elements are sufficient to represent a root of a Merkle tree or a sequential hash of elements. Both of these can be expanded into an arbitrary number of values by supplying the actual values non-deterministically via the advice provider.

## Usage
Miden crate exposes several functions which can be used to execute programs, generate proofs of their correct execution, and verify the generated proofs. How to do this is explained below, but you can also take a look at working examples [here](examples) and find instructions for running them via CLI [here](#fibonacci-example).

### Executing programs
To execute a program on Miden VM, you can use either `execute()` or `execute_iter()` functions. Both of these functions take the same arguments:

* `program: &Program` - a reference to a Miden program to be executed.
* `inputs: &ProgramInputs` - a reference to a set of public and secret inputs with which to execute the program.

The `execute()` function returns a `Result<ExecutionTrace, ExecutionError>` which will contain the execution trace of the program if the execution was successful, or an error, if the execution failed. You can inspect the trace to get the final state of the VM out of it, but generally, this trace is intended to be used internally by the prover during proof generation process.

The `execute_iter()` function returns a `VmStateIterator` which can be used to iterate over the cycles of the executed program for debug purposes. In fact, when we execute a program using this function, a lot of the debug information is retained and we can get a precise picture of the VM's state at any cycle. Moreover, if the execution results in an error, the `VmStateIterator` can still be used to inspect VM states right up to the cycle at which the error occurred.

For example:
```Rust
use miden::{Assembler, ProgramInputs};

// instantiate the assembler
let assembler = Assembler::default();

// compile Miden assembly source code into a program
let program = assembler.compile("begin push.3 push.5 add end").unwrap();

// execute the program with no inputs
let trace = miden::execute(&program, &ProgramInputs::none()).unwrap();

// now, execute the same program in debug mode and iterate over VM states
for vm_state in miden::execute_iter(&program, &ProgramInputs::none()) {
    match vm_state {
        Ok(vm_state) => println!("{:?}", vm_state),
        Err(_) => println!("something went terribly wrong!"),
    }
}
```

### Proving program execution
To execute a program on Miden VM and generate a proof that the program was executed correctly, you can use the `prove()` function. This function takes the following arguments:

* `program: &Program` - a reference to a Miden program to be executed.
* `inputs: &ProgramInputs` - a reference to a set of public and secret inputs with which to execute the program.
* `num_stack_outputs: usize` - number of items on the stack to be returned as program output.
* `options: &ProofOptions` - config parameters for proof generation. The default options target 96-bit security level.

If the program is executed successfully, the function returns a tuple with 2 elements:

* `outputs: Vec<u64>` - the outputs generated by the program. The number of elements in the vector will be equal to the `num_stack_outputs` parameter.
* `proof: StarkProof` - proof of program execution. `StarkProof` can be easily serialized and deserialized using `to_bytes()` and `from_bytes()` functions respectively.

#### Proof generation example
Here is a simple example of executing a program which pushes two numbers onto the stack and computes their sum:
```Rust
use miden::{Assembler, ProgramInputs, ProofOptions};

// instantiate the assembler
let assembler = Assembler::default();

// this is our program, we compile it from assembly code
let program = assembler.compile("begin push.3 push.5 add end").unwrap();

// let's execute it and generate a STARK proof
let (outputs, proof) = miden::prove(
    &program,
    &ProgramInputs::none(),   // we won't provide any inputs
    1,                        // we'll return one item from the stack
    &ProofOptions::default(), // we'll be using default options
)
.unwrap();

// the output should be 8
assert_eq!(vec![8], outputs);
```

### Verifying program execution
To verify program execution, you can use the `verify()` function. The function takes the following parameters:

* `program_hash: Digest` - a hash of the program to be verified (represented as a 32-byte digest).
* `stack_inputs: &[u64]` - a list of the values with which the stack was initialized prior to the program's execution..
* `stack_outputs: &[u64]` - a list of the values returned from the stack after the program completed execution.
* `proof: StarkProof` - the proof generated during program execution.

Stack inputs are expected to be ordered as if they would be pushed onto the stack one by one. Thus, their expected order on the stack will be the reverse of the order in which they are provided, and the last value in the `stack_inputs` slice is expected to be the value at the top of the stack.

Stack outputs are expected to be ordered as if they would be popped off the stack one by one. Thus, the value at the top of the stack is expected to be in the first position of the `stack_outputs` slice, and the order of the rest of the output elements will also match the order on the stack. This is the reverse of the order of the `stack_inputs` slice.

The function returns `Result<(), VerificationError>` which will be `Ok(())` if verification passes, or `Err(VerificationError)` if verification fails, with `VerificationError` describing the reason for the failure.

> If a program with the provided hash is executed against some secret inputs and the provided public inputs, it will produce the provided outputs.

Notice how the verifier needs to know only the hash of the program - not what the actual program was.

#### Proof verification example
Here is a simple example of verifying execution of the program from the previous example:
```Rust
use miden;

let program =   /* value from previous example */;
let proof =     /* value from previous example */;

// let's verify program execution
match miden::verify(program.hash(), &[], &[8], proof) {
    Ok(_) => println!("Execution verified!"),
    Err(msg) => println!("Something went terribly wrong: {}", msg),
}
```

## Fibonacci calculator
Let's write a simple program for Miden VM (using [Miden assembly](../assembly)). Our program will compute the 5-th [Fibonacci number](https://en.wikipedia.org/wiki/Fibonacci_number):

```
push.0      // stack state: 0
push.1      // stack state: 1 0
swap        // stack state: 0 1
dup.1       // stack state: 1 0 1
add         // stack state: 1 1
swap        // stack state: 1 1
dup.1       // stack state: 1 1 1
add         // stack state: 2 1
swap        // stack state: 1 2
dup.1       // stack state: 2 1 2
add         // stack state: 3 2
```
Notice that except for the first 2 operations which initialize the stack, the sequence of `swap dup.1 add` operations repeats over and over. In fact, we can repeat these operations an arbitrary number of times to compute an arbitrary Fibonacci number. In Rust, it would look like this (this is actually a simplified version of the example in [fibonacci.rs](src/examples/src/fibonacci.rs)):
```Rust
use miden::{Assembler, ProgramInputs, ProofOptions};

// set the number of terms to compute
let n = 50;

// instantiate the default assembler and compile the program
let source = format!(
    "
    begin 
        repeat.{}
            swap dup.1 add
        end
    end",
    n - 1
);
let program = Assembler::default().compile(&source).unwrap();

// initialize the stack with values 0 and 1
let inputs = ProgramInputs::from_stack_inputs(&[0, 1]).unwrap();

// execute the program
let (outputs, proof) = miden::prove(
    &program,
    &inputs,
    1,                        // top stack item is the output
    &ProofOptions::default(), // use default proof options
)
.unwrap();

// the output should be the 50th Fibonacci number
assert_eq!(vec![12586269025], outputs);
```
Above, we used public inputs to initialize the stack rather than using `push` operations. This makes the program a bit simpler, and also allows us to run the program from arbitrary starting points without changing program hash.

## CLI interface
If you want to execute, prove, and verify programs on Miden VM, but don't want to write Rust code, you can use Miden CLI. It also contains a number of useful tools to help analyze and debug programs.

### Compiling Miden VM
First, make sure you have Rust [installed](https://www.rust-lang.org/tools/install). The current version of Miden VM requires Rust version **1.62** or greater.

Then, to compile Miden VM into a binary, run the following command:
```
cargo build --release --features executable
```
This will place `miden` executable in the `./target/release` directory.

By default, the executable will be compiled in the single-threaded mode. If you would like to enable multi-threaded proof generation, you can compile Miden VM using the following command:
```
cargo build --release --features "executable concurrent"
```

### Running Miden VM
Once the executable has been compiled, you can run Miden VM like so:
```
./target/release/miden [subcommand] [parameters]
```
Currently, Miden VM can be executed with the following subcommands:
* `run` - this will execute a Miden assembly program and output the result, but will not generate a proof of execution.
* `prove` - this will execute a Miden assembly program, and will also generate a STARK proof of execution.
* `verify` - this will verify a previously generated proof of execution for a given program.
* `compile` - this will compile a Miden assembly program and outputs stats about the compilation process.
* `analyze` - this will run a Miden assembly program against specific inputs and will output stats about its execution.

All of the above subcommands require various parameters to be provided. To get more detailed help on what is needed for a given subcommand, you can run the following:
```
./target/release/miden [subcommand] --help
```
For example:
```
./target/release/miden prove --help
```

### Fibonacci example
In the `miden/examples/fib` directory, we provide a very simple Fibonacci calculator example. This example computes the 1000th term of the Fibonacci sequence. You can execute this example on Miden VM like so:
```
./target/release/miden run -a miden/examples/fib/fib.masm -n 1
```
This will run the example code to completion and will output the top element remaining on the stack.

## Crate features
Miden VM can be compiled with the following features:

* `std` - enabled by default and relies on the Rust standard library.
* `concurrent` - implies `std` and also enables multi-threaded proof generation.
* `executable` - required for building Miden VM binary as described above. Implies `std`.
* `no_std` does not rely on the Rust standard library and enables compilation to WebAssembly.

To compile with `no_std`, disable default features via `--no-default-features` flag.

### Concurrent proof generation
When compiled with `concurrent` feature enabled, the VM will generate STARK proofs using multiple threads. For benefits of concurrent proof generation check out these [benchmarks](../README.md#Performance).

Internally, we use [rayon](https://github.com/rayon-rs/rayon) for parallel computations. To control the number of threads used to generate a STARK proof, you can use `RAYON_NUM_THREADS` environment variable.

## License
This project is [MIT licensed](../LICENSE).