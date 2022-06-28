use super::{InputsFile, OutputsFile, Parser, ProgramHash, ProofFile};
use std::path::PathBuf;

// use crypto::Digest as DigestT;

#[derive(Debug, Parser)]
pub struct VerifyCmd {
    #[clap(short, parse(from_os_str))]
    script_file: PathBuf,
    #[clap(short, parse(from_os_str))]
    input_file: Option<PathBuf>,
    #[clap(short, parse(from_os_str))]
    output_file: Option<PathBuf>,
    #[clap(long, parse(from_os_str))]
    proof_file: Option<PathBuf>,
    #[clap(long, value_parser)]
    program_hash: String,
}

impl VerifyCmd {
    pub fn execute(&self) -> Result<(), String> {
        // read program hash from input
        let program_hash = ProgramHash::read(&self.program_hash)?;

        // load input data from file
        let input_data = InputsFile::read(&self.input_file, &self.script_file)?;

        // load outputs data from file
        let outputs_data = OutputsFile::read(&self.output_file, &self.script_file)?;

        // load proof from file
        let proof = ProofFile::read(&self.proof_file, &self.script_file)?;

        // verify proof
        verifier::verify(
            program_hash,
            &input_data.stack_inputs,
            &outputs_data.outputs,
            proof,
        )
        .map_err(|err| format!("Program failed verification! - {}", err))?;

        Ok(())
    }
}
