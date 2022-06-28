use super::{load_and_compile_script, InputsFile, Parser, ProofFile};
use air::ProofOptions;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct ProveCmd {
    #[clap(short, parse(from_os_str))]
    script_file: PathBuf,
    #[clap(short, parse(from_os_str))]
    input_file: Option<PathBuf>,
    #[clap(short, value_parser, default_value_t = 16)]
    num_outputs: usize,
    #[clap(short, parse(from_os_str))]
    proof_file: Option<PathBuf>,
    #[clap(long, value_parser, default_value = "96bits")]
    security: String,
}

impl ProveCmd {
    pub fn get_proof_security(&self) -> ProofOptions {
        match self.security.as_str() {
            "96bits" => ProofOptions::with_96_bit_security(),
            "128bits" => ProofOptions::with_128_bit_security(),
            other => panic!("{} is not a valid security setting", other),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        // load script from file and compile
        let script = load_and_compile_script(&self.script_file)?;

        // load input data from file
        let input_data = InputsFile::read(&self.input_file, &self.script_file)?;

        // execute program and generate proof
        let (_, proof) = prover::prove(
            &script,
            &input_data.get_program_inputs()?,
            self.num_outputs,
            &self.get_proof_security(),
        )
        .map_err(|err| format!("Failed to prove program - {:?}", err))?;

        // write proof to file
        ProofFile::write(proof, &self.proof_file, &self.script_file)?;

        Ok(())
    }
}
