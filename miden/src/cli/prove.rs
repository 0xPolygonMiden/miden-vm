use super::data::{instrument, Debug, InputFile, Libraries, OutputFile, ProgramFile, ProofFile};
use assembly::diagnostics::{IntoDiagnostic, Report, WrapErr};
use clap::Parser;
use miden_vm::ProvingOptions;
use processor::{DefaultHost, ExecutionOptions, ExecutionOptionsError, Program};

use std::{path::PathBuf, time::Instant};

#[derive(Debug, Clone, Parser)]
#[clap(about = "Prove a miden program")]
pub struct ProveCmd {
    /// Path to .masm assembly file
    #[clap(short = 'a', long = "assembly", value_parser)]
    assembly_file: PathBuf,

    /// Number of cycles the program is expected to consume
    #[clap(short = 'e', long = "exp-cycles", default_value = "64")]
    expected_cycles: u32,

    /// Path to input file
    #[clap(short = 'i', long = "input", value_parser)]
    input_file: Option<PathBuf>,

    /// Paths to .masl library files
    #[clap(short = 'l', long = "libraries", value_parser)]
    library_paths: Vec<PathBuf>,

    /// Maximum number of cycles a program is allowed to consume
    #[clap(short = 'm', long = "max-cycles", default_value = "4294967295")]
    max_cycles: u32,

    /// Number of outputs
    #[clap(short = 'n', long = "num-outputs", default_value = "16")]
    num_outputs: usize,

    /// Path to output file
    #[clap(short = 'o', long = "output", value_parser)]
    output_file: Option<PathBuf>,

    /// Path to proof file
    #[clap(short = 'p', long = "proof", value_parser)]
    proof_file: Option<PathBuf>,

    /// Enable generation of proofs suitable for recursive verification
    #[clap(short = 'r', long = "recursive")]
    recursive: bool,

    /// Security level for execution proofs generated by the VM
    #[clap(short = 's', long = "security", default_value = "96bits")]
    security: String,

    /// Enable tracing to monitor execution of the VM
    #[clap(short = 't', long = "tracing")]
    tracing: bool,
}

impl ProveCmd {
    pub fn get_proof_options(&self) -> Result<ProvingOptions, ExecutionOptionsError> {
        let exec_options =
            ExecutionOptions::new(Some(self.max_cycles), self.expected_cycles, self.tracing)?;
        Ok(match self.security.as_str() {
            "96bits" => ProvingOptions::with_96_bit_security(self.recursive),
            "128bits" => ProvingOptions::with_128_bit_security(self.recursive),
            other => panic!("{} is not a valid security setting", other),
        }
        .with_execution_options(exec_options))
    }

    pub fn execute(&self) -> Result<(), Report> {
        println!("===============================================================================");
        println!("Prove program: {}", self.assembly_file.display());
        println!("-------------------------------------------------------------------------------");

        let (program, input_data) = load_data(self)?;

        let program_hash: [u8; 32] = program.hash().into();
        println!("Proving program with hash {}...", hex::encode(program_hash));
        let now = Instant::now();

        // fetch the stack and program inputs from the arguments
        let stack_inputs = input_data.parse_stack_inputs().map_err(Report::msg)?;
        let host = DefaultHost::new(input_data.parse_advice_provider().map_err(Report::msg)?);

        let proving_options =
            self.get_proof_options().map_err(|err| Report::msg(format!("{err}")))?;

        // execute program and generate proof
        let (stack_outputs, proof) = prover::prove(&program, stack_inputs, host, proving_options)
            .into_diagnostic()
            .wrap_err("Failed to prove program")?;

        println!(
            "Program with hash {} proved in {} ms",
            hex::encode(program_hash),
            now.elapsed().as_millis()
        );

        // write proof to file
        ProofFile::write(proof, &self.proof_file, &self.assembly_file).map_err(Report::msg)?;

        // provide outputs
        if let Some(output_path) = &self.output_file {
            // write all outputs to specified file.
            OutputFile::write(&stack_outputs, output_path).map_err(Report::msg)?;
        } else {
            // if no output path was provided, get the stack outputs for printing to the screen.
            let stack = stack_outputs.stack_truncated(self.num_outputs).to_vec();

            // write all outputs to default location if none was provided
            OutputFile::write(&stack_outputs, &self.assembly_file.with_extension("outputs"))
                .map_err(Report::msg)?;

            // print stack outputs to screen.
            println!("Output: {:?}", stack);
        }

        Ok(())
    }
}

// HELPER FUNCTIONS
// ================================================================================================

#[instrument(skip_all)]
fn load_data(params: &ProveCmd) -> Result<(Program, InputFile), Report> {
    // load libraries from files
    let libraries = Libraries::new(&params.library_paths)?;

    // load program from file and compile
    let program =
        ProgramFile::read(&params.assembly_file)?.compile(&Debug::Off, &libraries.libraries)?;

    // load input data from file
    let input_data = InputFile::read(&params.input_file, &params.assembly_file)?;

    Ok((program, input_data))
}
