use super::data::{InputFile, OutputFile, ProgramHash, ProofFile};
use clap::Parser;
use miden::{Kernel, ProgramInfo};
use std::{path::PathBuf, time::Instant};

#[derive(Debug, Clone, Parser)]
#[clap(about = "Verify a miden program")]
pub struct VerifyCmd {
    /// Path to input file
    #[clap(short = 'i', long = "input", value_parser)]
    input_file: Option<PathBuf>,
    /// Path to output file
    #[clap(short = 'o', long = "output", value_parser)]
    output_file: Option<PathBuf>,
    /// Path to proof file
    #[clap(short = 'p', long = "proof", value_parser)]
    proof_file: PathBuf,
    /// Program hash (hex)
    #[clap(short = 'h', long = "program-hash")]
    program_hash: String,
}

impl VerifyCmd {
    pub fn execute(&self) -> Result<(), String> {
        println!("============================================================");
        println!("Verify program");
        println!("============================================================");

        // read program hash from input
        let program_hash = ProgramHash::read(&self.program_hash)?;

        // load input data from file
        let input_data = InputFile::read(&self.input_file, &self.proof_file)?;

        // fetch the stack inputs from the arguments
        let stack_inputs = input_data.parse_stack_inputs()?;

        // load outputs data from file
        let outputs_data = OutputFile::read(&self.output_file, &self.proof_file)?;

        // load proof from file
        let proof = ProofFile::read(&Some(self.proof_file.clone()), &self.proof_file)?;

        println!("verifying program...");
        let now = Instant::now();

        // TODO accept kernel as CLI argument
        let kernel = Kernel::default();
        let program_info = ProgramInfo::new(program_hash, kernel);

        // verify proof
        verifier::verify(program_info, stack_inputs, outputs_data.stack_outputs()?, proof)
            .map_err(|err| format!("Program failed verification! - {}", err))?;

        println!("Verification complete in {} ms", now.elapsed().as_millis());

        Ok(())
    }
}
