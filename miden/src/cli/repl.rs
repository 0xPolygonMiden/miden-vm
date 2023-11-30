use clap::Parser;
use std::path::PathBuf;

use crate::repl::start_repl;

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[clap(about = "Initiates the Miden REPL tool")]
pub struct ReplCmd {
    /// Paths to .masl library files
    #[clap(short = 'l', long = "libraries", value_parser)]
    library_paths: Vec<PathBuf>,

    /// Usage of standard library
    #[clap(short = 's', long = "stdlib")]
    use_stdlib: bool,
}

impl ReplCmd {
    pub fn execute(&self) -> Result<(), String> {
        // initiates repl tool.
        start_repl(&self.library_paths, self.use_stdlib);
        Ok(())
    }
}
