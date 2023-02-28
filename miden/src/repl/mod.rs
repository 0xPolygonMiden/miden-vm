use super::ProgramError;
use miden::{
    math::{Felt, StarkField},
    MemAdviceProvider, StackInputs, Word,
};
use processor::Process;
use rustyline::{error::ReadlineError, Editor};

/// This work is in continuation to the amazing work done by team `Scribe`
/// [here](https://github.com/ControlCplusControlV/Scribe/blob/main/transpiler/src/repl.rs#L8)
///
/// The Miden Read–eval–print loop (REPL) is a Miden shell that allows for quick and easy debugging
/// of Miden assembly. To use the repl, simply type "miden repl" after building it with feature
/// "executable" (cargo build --release --feature executable) when in the miden home
/// crate and the repl will launch. After the REPL gets initialized, you can execute any Miden
/// instruction, undo executed instructions, check the state of the stack and memory at a given point,
/// and do many other useful things! When the REPL is exited, a `history.txt` file is saved. One
/// thing to note is that all the REPL native commands start with an `!` to differentiate them from
/// regular assembly instructions.
///
/// Miden Instructions
/// All Miden instructions mentioned in the
/// [Miden Assembly section](https://0xpolygonmiden.github.io/miden-vm/user_docs/assembly/main.html)
/// are valid.
/// One can either input instructions one by one or multiple instructions in one input.
/// For example, the below two commands will result in the same output.
/// >> push.1
/// >> push.2
/// >> push.3
///
///  >> push.1 push.2 push.3
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
/// To execute a control flow operation, one must write the entire statement in a single line with
/// spaces between individual operations.
/// >> repeat.20
///       pow2
///    end
///
/// The above example should be written as follows in the REPL tool:
/// >> repeat.20 pow2 end
///
/// `!stack`
/// The `!stack` command prints out the state of the stack at the last executed instruction. Since
/// the stack always contains at least 16 elements, 16 or more elements will be printed out (even
/// if all of them are zeros).
/// >> push.1 push.2 push.3 push.4 push.5
/// >> exp
/// >> u32checked_mul
/// >> swap
/// >> eq.2
/// >> assert
///
/// The `!stack` command will print out the following state of the stack:
/// >> !stack
/// 3072 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
///
/// `!undo`
/// The `!undo` command reverts to the previous state of the stack and memory by dropping off the
/// last executed assembly instruction from the program. One could use `!undo` as often as they want
/// to restore the state of a stack and memory $n$ instructions ago (provided there are $n$ instructions
/// in the program). The `!undo` command will result in an error if no remaining instructions are left in
///  the miden program.
/// >> push.1 push.2 push.3
/// >> push.4
/// >> !stack
/// 4 3 2 1 0 0 0 0 0 0 0 0 0 0 0 0
/// >> push.5
/// >> !stack
/// 5 4 3 2 1 0 0 0 0 0 0 0 0 0 0 0
/// >> !undo
/// 4 3 2 1 0 0 0 0 0 0 0 0 0 0 0 0
/// >> !undo
/// 3 2 1 0 0 0 0 0 0 0 0 0 0 0 0 0
///
///`!program`
/// The `!program` command prints out the entire miden program getting executed. E.g., in the below scenario:
/// >> push.1
/// >> push.2
/// >> push.3
/// >> add
/// >> add
/// >> !program
/// begin
///    push.1
///    push.2
///    push.3
///    add
///    add
/// end
///
/// `!help`
/// The `!help` command prints out all the available commands in the REPL tool.
///
/// `!mem`
/// The `!mem` command prints out the contents of all initialized memory locations. For each such location,
/// the address, along with its memory values, is printed. Recall that four elements are stored at each memory
/// address.
/// If the memory has at least one value that has been initialized:
/// >> !mem
/// 7: [1, 2, 0, 3]
/// 8: [5, 7, 3, 32]
/// 9: [9, 10, 2, 0]
///
/// If the memory is not yet been initialized:
/// >> !mem
/// The memory has not been initialized yet
///
/// `!mem[addr]`
/// The `!mem[addr]` command prints out memory contents at the address specified by `addr`.
/// If the `addr` has been initialized:
/// >> !mem[9]
/// 9: [9, 10, 2, 0]
///
/// If the `addr` has not been initialized:
/// >> !mem[87]
/// Memory at address 87 is empty

/// Initiates the Miden Repl tool.
pub fn start_repl() {
    let mut program_lines: Vec<String> = Vec::new();

    println!("========================== Miden REPL ============================");
    println!();
    // prints out all the available commands in the Miden Repl tool.
    print_instructions();

    // flag to determine if the stack should be printed or not post the execution of the
    // last command.
    let mut should_print_stack = false;

    // state of the entire memory at the latest clock cycle.
    let mut memory: Vec<(u64, Word)> = Vec::new();

    // initializing readline.
    let mut rl = Editor::<()>::new().expect("Readline couldn't be initialized");
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
                    if should_print_stack {
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
            if should_print_stack {
                println!("{}", str::repeat("0 ", 16));
            }
        }
        match rl.readline(">> ") {
            Ok(line) => {
                if line == "!program" {
                    println!("{}", program);
                    should_print_stack = false;
                } else if line == "!help" {
                    // prints out all the available commands in the Miden Repl tool.
                    print_instructions();
                    should_print_stack = false;
                } else if line == "!mem" {
                    should_print_stack = false;
                    if memory.len() == 0 {
                        println!("The memory has not been initialized yet");
                        continue;
                    }
                    for (addr, mem) in &memory {
                        // prints out the address and memory value at that address.
                        print_mem_address(*addr, mem);
                    }
                } else if line.len() > 6 && &line[..5] == "!mem[" {
                    // if user wants to see the state of a particular address in a memory, the input should be atleast
                    // of length 5.

                    // flag to determine if the memory at an address has been initialized or not
                    let mut mem_at_addr_present = false;

                    // extracts the address from user input.
                    match read_mem_address(&line) {
                        Ok(addr) => {
                            for (i, memory_value) in &memory {
                                if *i == addr {
                                    // prints the address and memory value at that address.
                                    print_mem_address(addr, memory_value);
                                    // sets the flag to true as the address has been initialized.
                                    mem_at_addr_present = true;
                                    break;
                                }
                            }
                            // incase the flag has not been initialized.
                            if !mem_at_addr_present {
                                println!("Memory at address {} is empty", addr);
                            }
                        }
                        Err(msg) => println!("{}", msg),
                    }

                    should_print_stack = false;
                } else if line == "!undo" {
                    match program_lines.pop() {
                        Some(last_line) => {
                            println!("Undoing {}", last_line);
                            should_print_stack = true;
                        }
                        None => {
                            println!("There's no previously executed command");
                            should_print_stack = false;
                        }
                    };
                } else if line == "!stack" {
                    should_print_stack = true;
                } else {
                    rl.add_history_entry(line.clone());
                    program_lines.push(line.clone());
                    should_print_stack = true;
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
fn execute(program: String) -> Result<(Vec<(u64, Word)>, Vec<Felt>), ProgramError> {
    let program = assembly::Assembler::default()
        .compile(&program)
        .map_err(ProgramError::AssemblyError)?;

    let stack_inputs = StackInputs::default();
    let advice_provider = MemAdviceProvider::default();

    let mut process = Process::new_debug(program.kernel().clone(), stack_inputs, advice_provider);
    let _program_outputs = process.execute(&program).map_err(ProgramError::ExecutionError);

    let (sys, _, stack, _, chiplets, _) = process.into_parts();

    // loads the memory at the latest clock cycle.
    let mem = chiplets.get_mem_state_at(0, sys.clk());

    // loads the stack along with the overflow values at the latest clock cycle.
    let stack_state = stack.get_state_at(sys.clk());

    Ok((mem, stack_state))
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
    println!();
}

/// Returns the state of the stack along with its overflown part in a string format.
fn print_stack(stack: Vec<Felt>) {
    // converts the stack which is a vector of felt into string and prints it.
    println!("{}", stack.iter().map(|f| format!("{}", f)).collect::<Vec<_>>().join(" "),)
}

/// Accepts and returns a memory at an address by converting its register into integer
/// from Felt.
fn print_mem_address(addr: u64, mem: &Word) {
    let mem_int = mem.iter().map(|&x| x.as_int()).collect::<Vec<_>>();
    println!("{} {:?}", addr, mem_int)
}
