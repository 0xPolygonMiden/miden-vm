use super::data::{Debug, InputFile, Libraries, ProgramFile};
use air::ExecutionOptions;
use rustyline::{error::ReadlineError, Config, EditMode, Editor};
use std::path::PathBuf;
use structopt::StructOpt;

mod command;
use command::DebugCommand;

mod executor;
use executor::DebugExecutor;

#[derive(StructOpt, Debug)]
#[structopt(name = "Debug", about = "Debug a miden program")]
pub struct DebugCmd {
    /// Path to .masm assembly file
    #[structopt(short = "a", long = "assembly", parse(from_os_str))]
    assembly_file: PathBuf,
    /// Path to input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: Option<PathBuf>,
    /// Enable vi edit mode
    #[structopt(short = "vi", long = "vim_edit_mode")]
    vim_edit_mode: Option<String>,
    /// Paths to .masl library files
    #[structopt(short = "l", long = "libraries", parse(from_os_str))]
    library_paths: Vec<PathBuf>,
    /// Maximum number of cycles a program is allowed to consume
    #[structopt(short = "m", long = "max-cycles")]
    max_cycles: Option<u32>,
    /// Expected number of cycles
    #[structopt(short = "e", long = "exp-cycles", default_value = "64")]
    expected_cycles: u32,
}

impl DebugCmd {
    pub fn execute(&self) -> Result<(), String> {
        println!("============================================================");
        println!("Debug program");
        println!("============================================================");

        // load libraries from files
        let libraries = Libraries::new(&self.library_paths)?;

        // load program from file and compile
        let program = ProgramFile::read(&self.assembly_file, &Debug::On, libraries.libraries)?;

        let program_hash: [u8; 32] = program.hash().into();
        println!("Debugging program with hash {}... ", hex::encode(program_hash));

        // load input data from file
        let input_data = InputFile::read(&self.input_file, &self.assembly_file)?;

        // get execution options
        let execution_options = ExecutionOptions::new(self.max_cycles, self.expected_cycles);

        // fetch the stack and program inputs from the arguments
        let stack_inputs = input_data.parse_stack_inputs()?;
        let advice_provider = input_data.parse_advice_provider()?;

        // Instantiate DebugExecutor
        let mut debug_executor =
            DebugExecutor::new(program, stack_inputs, advice_provider, execution_options)?;

        // build readline config
        let mut rl_config = Config::builder().auto_add_history(true);
        if self.vim_edit_mode.is_some() {
            rl_config = rl_config.edit_mode(EditMode::Vi);
        }
        let rl_config = rl_config.build();

        // initialize readline
        let mut rl =
            Editor::<()>::with_config(rl_config).expect("Readline couldn't be initialized");

        println!("Welcome! Enter `h` for help.");

        loop {
            match rl.readline(">> ") {
                Ok(command) => match DebugCommand::parse(&command) {
                    Ok(Some(command)) => {
                        if !debug_executor.execute(command) {
                            println!("Debugging complete");
                            break;
                        }
                    }
                    Ok(None) => (),
                    Err(err) => eprintln!("{err}"),
                },
                Err(ReadlineError::Interrupted) => {
                    // ctrl+c is a transparent interruption and should provide not feedback or
                    // action.
                }
                Err(ReadlineError::Eof) => {
                    eprintln!("CTRL-D");
                    break;
                }
                Err(err) => eprintln!("malformed command - failed to read user input: {}", err),
            }
        }

        Ok(())
    }
}
