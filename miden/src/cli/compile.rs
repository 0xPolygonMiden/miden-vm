use clap::Parser;

use super::data::{Debug, Libraries, ProgramFile};
use assembly::diagnostics::Report;
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
    /// Path to output file
    #[clap(short = 'o', long = "output", value_parser)]
    output_file: Option<PathBuf>,
}

impl CompileCmd {
    pub fn execute(&self) -> Result<(), Report> {
        println!("============================================================");
        println!("Compile program");
        println!("============================================================");

        // load the program from file and parse it
        let program = ProgramFile::read(&self.assembly_file)?;

        // load libraries from files
        let libraries = Libraries::new(&self.library_paths)?;

        // compile the program
        let compiled_program = program.compile(&Debug::Off, &libraries.libraries)?;

        // report program hash to user
        let program_hash: [u8; 32] = compiled_program.hash().into();
        println!("program hash is {}", hex::encode(program_hash));

        // write the compiled file
        program.write(self.output_file.clone())
    }
}
