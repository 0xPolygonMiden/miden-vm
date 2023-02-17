use core::fmt;
use miden::{AssemblyError, ExecutionError};
use structopt::StructOpt;

mod cli;
mod examples;
mod repl;
mod tools;

/// Root CLI struct
#[derive(StructOpt, Debug)]
#[structopt(name = "Miden", about = "Miden CLI")]
pub struct Cli {
    #[structopt(subcommand)]
    action: Actions,
}

/// CLI actions
#[derive(StructOpt, Debug)]
pub enum Actions {
    Analyze(tools::Analyze),
    Compile(cli::CompileCmd),
    Debug(cli::DebugCmd),
    Example(examples::ExampleOptions),
    Prove(cli::ProveCmd),
    Run(cli::RunCmd),
    Verify(cli::VerifyCmd),
    #[cfg(feature = "std")]
    Repl(cli::ReplCmd),
}

/// CLI entry point
impl Cli {
    pub fn execute(&self) -> Result<(), String> {
        match &self.action {
            Actions::Analyze(analyze) => analyze.execute(),
            Actions::Compile(compile) => compile.execute(),
            Actions::Debug(debug) => debug.execute(),
            Actions::Example(example) => example.execute(),
            Actions::Prove(prove) => prove.execute(),
            Actions::Run(run) => run.execute(),
            Actions::Verify(verify) => verify.execute(),
            #[cfg(feature = "std")]
            Actions::Repl(repl) => repl.execute(),
        }
    }
}

/// Executable entry point
pub fn main() {
    // read command-line args
    let cli = Cli::from_args();

    // execute cli action
    if let Err(error) = cli.execute() {
        println!("{}", error);
    }
}

// PROGRAM ERROR
// ================================================================================================

/// This is used to specify the error type returned from analyze.
#[derive(Debug)]
pub enum ProgramError {
    AssemblyError(AssemblyError),
    ExecutionError(ExecutionError),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramError::AssemblyError(e) => write!(f, "Assembly Error: {:?}", e),
            ProgramError::ExecutionError(e) => write!(f, "Execution Error: {:?}", e),
        }
    }
}

impl std::error::Error for ProgramError {}
