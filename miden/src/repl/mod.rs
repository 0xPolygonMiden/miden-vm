use air::StarkField;
use processor::{ExecutionError, Process};
use rustyline::{error::ReadlineError, Editor};
use vm_core::{Felt, ProgramInputs, Word};

/// This work is in continuation to the amazing work done by team `Scribe`
/// [here](https://github.com/ControlCplusControlV/Scribe/blob/main/transpiler/src/repl.rs#L8)
///
/// The Miden Read–eval–print loop or repl for short is a Miden shell that allows for quick and
/// easy debugging with Miden assembly. To use the repl, simply type "miden repl" after building it
/// with feature "executable" (cargo build --release --feature executable) when in the miden home
/// crate and the repl will launch. Now that you have the repl launched, there are a bunch of
/// awesome things an user can do like execute any Miden instruction, use procedures, undo
/// executed instructions, check the stack at anytime and more! Check out the list of commands
/// that one can use below. After exiting the repl, a history.txt file will be saved.
///
/// Miden Instructions
///  All the Miden instruction mentioned in the Miden Assembly is valid. (Ex. push.0, drop, dropw,
///  swap, cswap, u32checked_add, mem_loadw, mtree_get etc.)
///  One can input instructions one by one or multiple instructions in one input.
///  Ex.
///  `push.1`
///  `push.2`
///  `push.3`
///  Is the same as
///  `push.1 push.2 push.3`
///
/// In order to execute a control flow operation, one needs to write the entire flow operation in
/// a single line with spaces between individual operations.
/// Ex.
/// `repeat.20
///     pow2
/// end`
/// should be written as
/// `repeat.20 pow2 end`
///
/// The Repl tool doesn't support multi-line inputs for control flow operations yet.
///
/// `!stack`
///  Use the `stack` command to check the state of the whole stack(including overflow) at anytime.
///  The stack will be empty on initiation.
///  The state of the stack after executing the below instruction is as follow:
///  Ex.
///  push.1 push.2 push.3
///  !stack
///  >> 3 2 1 0 0 0 0 0 0 0 0 0 0 0 0 0
///
/// `!undo`
/// Use the `undo` command at anytime to revert to the last state of the stack and memory before a
/// command or Miden instruction. One could use `undo` as many times as you want to restore the state
/// of a stack and memory n instructions ago.
/// Ex.
/// push.1 push.2 push.3
/// !stack
/// >> 3 2 1 0 0 0 0 0 0 0 0 0 0 0 0 0
/// push.4
/// !stack
/// >> 4 3 2 1 0 0 0 0 0 0 0 0 0 0 0 0
/// push.5
/// !stack
/// >> 5 4 3 2 1 0 0 0 0 0 0 0 0 0 0 0
/// !undo
/// >> 4 3 2 1 0 0 0 0 0 0 0 0 0 0 0 0
/// !undo
/// >> 3 2 1 0 0 0 0 0 0 0 0 0 0 0 0 0
///
///`!program`
/// Use the `program` command at anytime to see the full Miden assembly that you have input to that point
/// as a Miden program
/// Ex.
/// push.1
/// push.2
/// push.3
/// add
/// add
/// !program
/// >> begin
///    push.1
///    push.2
///    push.3
///    add
///    add
/// end
///
/// `!help`
/// Use the `help` command at any time to see a list of available commands.
///
/// `!mem`
/// Use the `mem` command to see the full state of the memory at the latest clock cycle.
/// Ex.
/// `!mem`
/// >> 7: [1, 2, 0, 3]
/// >> 8: [5, 7, 3, 32]
/// >> 9: [9, 10, 2, 0]
///
/// `!mem[i]`
/// Use the `mem[i]` command to see the state of the memory at address `i` at the latest clock cycle.
/// If memory at the specified address has been initialized:
/// `!mem[9]`
/// >> 9: [9, 10, 2, 0]
///
/// If memory at the specified address has not been initialized:
/// `!mem[8]`
/// >> Memory at address 8 is empty

/// Initiates the Miden Repl tool.
pub fn start_repl() {
    let mut program_lines: Vec<String> = Vec::new();

    println!("========================== Miden REPL ============================");
    println!();
    // prints out all the available commands in the Miden Repl tool.
    print_instructions();

    // flag to determine if the stack should be printed or not post the execution of the
    // last command.
    let mut stack_print_flag = false;

    // state of the entire memory at the latest clock cycle.
    let mut memory: Vec<(u64, Word)> = Vec::new();

    // initialising readline.
    let mut rl = Editor::<()>::new().expect("Readline couldn't be initialised");
    loop {
        let program = format!(
            "begin\n{}\nend",
            program_lines
                .iter()
                .map(|l| format!("    {}", l))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let result = execute(program.clone());

        if !program_lines.is_empty() {
            match result {
                Ok((mem, stack_state)) => {
                    if stack_print_flag {
                        print_stack(stack_state);
                    }
                    memory = mem;
                }
                Err(e) => {
                    println!("{}", format!("Error running program: {:?}", e));
                    program_lines.pop();
                }
            }
        } else {
            if stack_print_flag {
                println!("{}", str::repeat("0 ", 16));
            }
        }
        match rl.readline(">> ") {
            Ok(line) => {
                if line == "!program" {
                    println!("{}", program);
                    stack_print_flag = false;
                } else if line == "!help" {
                    // prints out all the available commands in the Miden Repl tool.
                    print_instructions();
                    stack_print_flag = false;
                } else if line == "!mem" {
                    for (addr, mem) in &memory {
                        // prints out the address and memory value at that address.
                        print_mem_address(*addr, mem);
                    }
                    stack_print_flag = false;
                } else if line.len() > 6 && &line[..5] == "!mem[" {
                    // if user wants to see the state of a particular address in a memory, the input should be atleast
                    // of length 5.

                    // flag to determine if the memory at an address has been initialised or not
                    let mut flag = false;

                    // extracts the address from user input.
                    match read_mem_address(&line) {
                        Ok(addr) => {
                            for (i, memory_value) in &memory {
                                if *i == addr {
                                    // prints the address and memory value at that address.
                                    print_mem_address(addr, memory_value);

                                    // sets the flag to true as the address has been initialised.
                                    flag = true;
                                    break;
                                }
                            }
                            // incase the flag has not been initialised.
                            if !flag {
                                println!("Memory at address {} is empty", addr);
                            }
                        }
                        Err(msg) => println!("{}", msg),
                    }

                    stack_print_flag = false;
                } else if line == "!undo" {
                    match program_lines.pop() {
                        Some(last_line) => {
                            println!("Undoing {}", last_line);
                            stack_print_flag = true;
                        }
                        None => {
                            println!("There's no previously executed command");
                            stack_print_flag = false;
                        }
                    };
                } else if line == "!stack" {
                    stack_print_flag = true;
                } else {
                    rl.add_history_entry(line.clone());
                    program_lines.push(line.clone());
                    stack_print_flag = true;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
    }
    rl.save_history("history.txt")
        .expect("Couldn't dump the program into the history file");
}

/// HELPER METHODS
/// --------------------------------------------------------------------------------------------

/// Compiles and executes a compiled Miden program, returning the stack, memory and any Miden errors.
/// The program is passed in as a String, passed to the Miden Assembler, and then passed into the Miden
/// Processor to be executed.
fn execute(program: String) -> Result<(Vec<(u64, Word)>, Vec<Felt>), MidenError> {
    let program = assembly::Assembler::new()
        .compile(&program)
        .map_err(MidenError::AssemblyError)?;

    let pub_inputs = vec![];
    let inputs = ProgramInputs::new(&pub_inputs, &[], vec![]).unwrap();
    let mut process = Process::new_debug(inputs);
    let _program_outputs = process
        .execute(&program)
        .map_err(MidenError::ExecutionError);

    let (sys, _, stack, _, chiplets) = process.to_components();

    // loads the memory at the latest clock cycle.
    let mem = chiplets.get_mem_state_at(0, sys.clk());

    // loads the stack along with the overflow values at the latest clock cycle.
    let stack_state = stack.get_state_at(sys.clk());

    Ok((mem, stack_state))
}

/// Errors that are returned from the Miden processor during execution.
#[derive(Debug)]
pub enum MidenError {
    AssemblyError(assembly::AssemblyError),
    ExecutionError(ExecutionError),
}

/// Parses the address in integer form from "!mem[addr]" command, otherwise throws an error.
fn read_mem_address(mem_str: &str) -> Result<u64, String> {
    // the first five characters is "!mem[" and the digit character should start from 6th
    // element.
    let remainder = &mem_str[5..];
    let digits_end = remainder
        .char_indices()
        .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
        .unwrap_or(remainder.len());

    if &remainder[digits_end..] != "]" {
        return Err("Please enter memory command correctly. It should be !mem[addr]".to_string());
    }

    // convert the parsed digits into integer form.
    let addr = &remainder[..digits_end]
        .parse()
        .expect("The input address couldn't be parsed into an integer");

    Ok(*addr)
}

/// Prints out all the available command present in the Miden Repl tool.
fn print_instructions() {
    println!("Available commands:");
    println!();
    println!("!stack: displays the complete state of the stack");
    println!("!mem: displays the state of the entire memory");
    println!("!mem[i]: displays the state of the memory at address i");
    println!("!undo: remove the last instruction");
    println!("!program: display the program");
    println!("!help: prints out all the available commands");
}

/// Returns the state of the stack along with its overflown part in a string format.
fn print_stack(stack: Vec<Felt>) {
    // converts the stack which is a vector of felt into string and prints it.
    println!(
        "{}",
        stack
            .iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<_>>()
            .join(" "),
    )
}

/// Accepts and returns a memory at an address by converting its register into integer
/// from Felt.
fn print_mem_address(addr: u64, mem: &Word) {
    let mem_int = mem.iter().map(|&x| x.as_int()).collect::<Vec<_>>();
    println!("{} {:?}", addr, mem_int)
}
