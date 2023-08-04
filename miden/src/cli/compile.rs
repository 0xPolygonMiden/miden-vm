use clap::Parser;

use super::data::{Debug, Libraries, ProgramFile};
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[clap(about = "Compile a miden program")]
pub struct CompileCmd {
    /// Path to .masm assembly file
    #[clap(short = 'a', long = "assembly", value_parser)]
    assembly_file: PathBuf,
    /// Paths to .masl library files
    #[clap(short = 'l', long = "libraries", value_parser)]
    library_paths: Vec<PathBuf>,
}

impl CompileCmd {
    pub fn execute(&self) -> Result<(), String> {
        println!("============================================================");
        println!("Compile program");
        println!("============================================================");

        // load libraries from files
        let libraries = Libraries::new(&self.library_paths)?;

        // load program from file and compile
        let program = ProgramFile::read(&self.assembly_file, &Debug::Off, libraries.libraries)?;

        // report program hash to user
        let program_hash: [u8; 32] = program.hash().into();
        println!("program hash is {}", hex::encode(program_hash));

        Ok(())
    }
}
