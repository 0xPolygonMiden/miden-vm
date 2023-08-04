use crate::repl::start_repl;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(about = "Initiates the Miden REPL tool")]
pub struct ReplCmd {}

impl ReplCmd {
    pub fn execute(&self) -> Result<(), String> {
        // initiates repl tool.
        start_repl();
        Ok(())
    }
}
