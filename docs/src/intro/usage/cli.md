# CLI interface

## Compiling Miden VM
To compile Miden VM into a binary, we have a [Makefile](https://www.gnu.org/software/make/manual/make.html) with the following tasks:
```
make exec
```
This will place an optimized, multi-threaded `miden` executable in the `./target/release` directory. It is equivalent to executing:
```
cargo build --profile optimized --features concurrent,executable
```
If you would like to enable single-threaded mode, you can compile Miden VM using the following command:
```
cargo build --profile optimized --features executable
```
For a faster build, you can compile with less optimizations, replacing `--profile optimized` by `--release`. Example:
```
cargo build --release --features concurrent,executable
```
## Controlling parallelism
Internally, Miden VM uses [rayon](https://github.com/rayon-rs/rayon) for parallel computations. To control the number of threads used to generate a STARK proof, you can use `RAYON_NUM_THREADS` environment variable.

## Running Miden VM
Once the executable has been compiled, you can run Miden VM like so:
```
./target/release/miden [subcommand] [parameters]
```
Currently, Miden VM can be executed with the following subcommands:
* `run` - this will execute a Miden assembly program and output the result, but will not generate a proof of execution.
* `prove` - this will execute a Miden assembly program, and will also generate a STARK proof of execution.
* `verify` - this will verify a previously generated proof of execution for a given program.
* `compile` - this will compile a Miden assembly program (i.e., build a program [MAST](../../design/programs.md)) and outputs stats about the compilation process.
* `debug` - this will instantiate a CLI debugger against the specified Miden assembly program and inputs.
* `analyze` - this will run a Miden assembly program against specific inputs and will output stats about its execution.
* `repl` - this will initiate the [Miden REPL](development_tooling.md#repl) tool.

All of the above subcommands require various parameters to be provided. To get more detailed help on what is needed for a given subcommand, you can run the following:
```
./target/release/miden [subcommand] --help
```
For example:
```
./target/release/miden prove --help
```

## Fibonacci example
In the `miden/examples/fib` directory, we provide a very simple Fibonacci calculator example. This example computes the 1000th term of the Fibonacci sequence. You can execute this example on Miden VM like so:
```
./target/release/miden run -a miden/examples/fib/fib.masm -n 1
```
This will run the example code to completion and will output the top element remaining on the stack.

If you want the output of the program in a file, you can use the `--output` or `-o` flag and specify the path to the output file. For example:
```
./target/release/miden run -a miden/examples/fib/fib.masm -o fib.out
```
This will dump the output of the program into the `fib.out` file. The output file will contain the state of the stack at the end of the program execution.