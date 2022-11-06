use crate::repl::start_repl;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Repl", about = "Initiates the Miden REPL tool")]
pub struct ReplCmd {}

impl ReplCmd {
    pub fn execute(&self) -> Result<(), String> {
        // initiates repl tool.
        start_repl();
        Ok(())
    }
}
