pub use super::{examples, tools};
use clap::Parser;

mod analyze;
mod compile;
mod data;
mod example;
mod prove;
mod run;
mod verify;

use data::{load_and_compile_script, InputsFile, OutputsFile, ProgramHash, ProofFile};

/// Cli struct is always invoked with a subcommand
#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    action: Action,
}

/// Subcommand specifying action to be executed
#[derive(Debug, Parser)]
pub enum Action {
    Run(run::RunCmd),
    Prove(prove::ProveCmd),
    Compile(compile::CompileCmd),
    Verify(verify::VerifyCmd),
    Analyze(analyze::AnalyzeCmd),
    Example(example::ExampleCmd),
}

/// Cli entry point
pub fn execute() -> Result<(), String> {
    let cli = Cli::from_args();

    match &cli.action {
        Action::Run(cmd) => cmd.execute()?,
        Action::Prove(cmd) => cmd.execute()?,
        Action::Compile(cmd) => cmd.execute()?,
        Action::Verify(cmd) => cmd.execute()?,
        Action::Analyze(cmd) => cmd.execute()?,
        Action::Example(cmd) => cmd.execute()?,
    };

    Ok(())
}
