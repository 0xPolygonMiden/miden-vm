use std::{path::PathBuf, time::Instant};
use std::path::Path;

use assembly::diagnostics::{IntoDiagnostic, Report, WrapErr};
use clap::Parser;
use miden_vm::{Kernel, ProgramInfo};

use super::data::{InputFile, OutputFile, ProgramHash, ProofFile};

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
    proof_file: Option<PathBuf>,
    /// Program hash (hex)
    #[clap(short = 'x', long = "hash")]
    program_hash: String,
}

impl VerifyCmd {
    pub fn execute(&mut self) -> Result<(), Report> {

        let (proof_file,output_file)=self.infer_defaults();

        self.proof_file=Some(proof_file);
        self.output_file=Some(output_file);

        println!("===============================================================================");
        println!("Verifying proof: {}", self.proof_file.as_ref().unwrap().display());
        println!("-------------------------------------------------------------------------------");

        // read program hash from input
        let program_hash = ProgramHash::read(&self.program_hash).map_err(Report::msg)?;

        // load input data from file
        let input_data = InputFile::read(&self.input_file, &self.proof_file.as_ref().unwrap().clone())?;

        // fetch the stack inputs from the arguments
        let stack_inputs = input_data.parse_stack_inputs().map_err(Report::msg)?;

        // load outputs data from file
        let outputs_data =
            OutputFile::read(&self.output_file, &self.proof_file.as_ref().unwrap()).map_err(Report::msg)?;

        // load proof from file
        let proof = ProofFile::read(&Some(self.proof_file.as_ref().unwrap().to_path_buf()), &self.proof_file.as_ref().unwrap())
            .map_err(Report::msg)?;

        let now = Instant::now();

        // TODO accept kernel as CLI argument
        let kernel = Kernel::default();
        let program_info = ProgramInfo::new(program_hash, kernel);

        // verify proof
        let stack_outputs = outputs_data.stack_outputs().map_err(Report::msg)?;
        verifier::verify(program_info, stack_inputs, stack_outputs, proof)
            .into_diagnostic()
            .wrap_err("Program failed verification!")?;

        println!("Verification complete in {} ms", now.elapsed().as_millis());

        Ok(())
    }

    pub fn infer_defaults(&self)->(PathBuf,PathBuf){
        let input_file = self.input_file.clone().unwrap_or_else(|| {    
                PathBuf::from("default_input_file.txt")
        });

        let base_name = input_file.file_stem().expect("Invalid input file").to_str().unwrap();

        let proof_file = self.proof_file.clone().unwrap_or_else(|| {
            let mut proof_path = input_file.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
            proof_path.push(format!("{}.proof", base_name));
            proof_path
        });

        let output_file = self.output_file.clone().unwrap_or_else(|| {
            let mut output_path = input_file.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
            output_path.push(format!("{}.outputs", base_name));
            output_path
        });
        
        return (proof_file,output_file);
    }
}