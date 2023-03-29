# Miden assembly
This crate contains Miden assembler.

The purpose of the assembler is to compile [Miden assembly](https://0xpolygonmiden.github.io/miden-vm/user_docs/assembly/main.html) source code into a Miden VM program (represented by `Program` struct). The program can then be executed on Miden VM [processor](../processor).

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

## Assembler options
By default, the assembler is instantiated in the most minimal form. To extend the capabilities of the assembler, you can apply a chain of `with_*` methods to the default instance in a builder pattern. The set of currently available options is described below.

### Libraries
To enable calls to procedures from external modules, the assembler must be supplied with libraries containing these modules. This can be done via `with_library()` or `with_libraries()` methods.

 A library can be anything that implements the `Library` trait. We have implemented this trait for the Miden [standard library](../stdlib). Thus, for example, to make Miden stdlib available to programs during compilation, the assembler can be instantiated as follows:

```Rust
use miden_assembly::Assembler;
use miden_stdlib::StdLibrary;

// instantiate the assembler with access to Miden stdlib
let assembler = Assembler::default().with_library(&StdLibrary::default()).unwrap();
```
Programs compiled with this assembler can invoke any procedure from Miden `stdlib`. For example, something like this will be possible:
```
use.std::math::u64

begin
    push.1.0
    push.2.0
    exec.u64::checked_add
end
```

We also provide a concrete implementation of the `Library` trait called `MaslLibrary`. This implementation can be used to instantiate libraries from `.masl` files.

### Program kernels
A *program kernel* defines a set of procedures which can be invoked via `syscall` instructions. Miden programs are always compiled against some kernel, and by default this kernel is empty (i.e., no `syscall`'s are possible).

Instantiating the assembler with a non-empty kernel can be done like so:
```Rust
use miden_assembly::Assembler;

// define a kernel with a single exported procedure
let kernel_source = "export.foo add end";

// instantiate the assembler with a kernel
let assembler = Assembler::default().with_kernel(kernel_source).unwrap();
```

Programs compiled with this assembler will be able to make calls to `foo` procedure by executing `syscall.foo` instruction.

### Debug mode
The assembler can be instantiated in debug mode. Compiling a program with such an assembler retains source mappings between assembly instructions and VM operations. Thus, when such a program is executed using the `execute_iter()` function of the [processor](../processor), is it possible to tell exactly which assembly instruction is being executed at a specific VM cycle.

Instantiating the assembler in debug mode can be done like so:
```Rust
use miden_assembly::Assembler;

// instantiate the assembler in debug mode
let assembler = Assembler::default().with_debug_mode(true);
```

### Instantiating assembler with multiple options
As mentioned previously, a builder pattern can be used to chain multiple `with_*` method together. For example, an assembler can be instantiated with all available options like so:

```Rust
use miden_assembly::Assembler;
use miden_stdlib::StdLibrary;

// source code of the kernel module
let kernel_source = "export.foo add end";

// instantiate the assembler
let assembler = Assembler::default()
    .with_debug_mode(true)
    .with_library(&StdLibrary::default())
    .and_then(|a| a.with_kernel(kernel_source))
    .unwrap();
```

## License
This project is [MIT licensed](../LICENSE).
