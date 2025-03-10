use std::{fs, path::PathBuf, sync::Arc, time::Instant};

use assembly::{
    diagnostics::{IntoDiagnostic, Report, WrapErr},
    utils::Deserializable,
};
use clap::Parser;
use miden_vm::{internal::InputFile, ProvingOptions};
use package::{MastArtifact, Package};
use processor::{DefaultHost, ExecutionOptions, ExecutionOptionsError, Program};
use stdlib::StdLibrary;
use tracing::instrument;

use super::data::{Debug, Libraries, OutputFile, ProgramFile, ProofFile};

#[derive(Debug, Clone, Parser)]
#[clap(about = "Prove a miden program")]
pub struct ProveCmd {
    /// Path to a .masm assembly file or a .masp package file
    #[clap(value_parser)]
    program_file: PathBuf,

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

    /// Specifies if the RPX Hash should be used. Conflicts with the recursive flag
    #[clap(long = "rpx", conflicts_with("recursive"))]
    rpx: bool,

    /// Security level for execution proofs generated by the VM
    #[clap(short = 's', long = "security", default_value = "96bits")]
    security: String,

    /// Enable tracing to monitor execution of the VM
    #[clap(short = 't', long = "trace")]
    trace: bool,
}

impl ProveCmd {
    pub fn get_proof_options(&self) -> Result<ProvingOptions, ExecutionOptionsError> {
        let exec_options =
            ExecutionOptions::new(Some(self.max_cycles), self.expected_cycles, self.trace, false)?;
        Ok(match self.security.as_str() {
            "96bits" => {
                if self.rpx {
                    ProvingOptions::with_96_bit_security_rpx()
                } else {
                    ProvingOptions::with_96_bit_security(self.recursive)
                }
            },
            "128bits" => {
                if self.rpx {
                    ProvingOptions::with_128_bit_security_rpx()
                } else {
                    ProvingOptions::with_128_bit_security(self.recursive)
                }
            },
            other => panic!("{} is not a valid security setting", other),
        }
        .with_execution_options(exec_options))
    }
    pub fn execute(&self) -> Result<(), Report> {
        println!("===============================================================================");
        println!("Prove program: {}", self.program_file.display());
        println!("-------------------------------------------------------------------------------");

        // determine file type based on extension
        let ext = self
            .program_file
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let (program, input_data) = match ext.as_str() {
            "masp" => load_masp_data(self)?,
            "masm" => load_masm_data(self)?,
            _ => return Err(Report::msg("File must have a .masm or .masp extension")),
        };

        let program_hash: [u8; 32] = program.hash().into();
        println!("Proving program with hash {}...", hex::encode(program_hash));
        let now = Instant::now();

        // fetch the stack and program inputs from the arguments
        let stack_inputs = input_data.parse_stack_inputs().map_err(Report::msg)?;
        let mut host = DefaultHost::new(input_data.parse_advice_provider().map_err(Report::msg)?);
        host.load_mast_forest(StdLibrary::default().mast_forest().clone()).unwrap();

        let proving_options =
            self.get_proof_options().map_err(|err| Report::msg(format!("{err}")))?;

        // execute program and generate proof
        let (stack_outputs, proof) =
            prover::prove(&program, stack_inputs, &mut host, proving_options)
                .into_diagnostic()
                .wrap_err("Failed to prove program")?;

        println!("Program proved in {} ms", now.elapsed().as_millis());

        // write proof to file
        ProofFile::write(proof, &self.proof_file, &self.program_file).map_err(Report::msg)?;

        // provide outputs
        if let Some(output_path) = &self.output_file {
            // write all outputs to specified file.
            OutputFile::write(&stack_outputs, output_path).map_err(Report::msg)?;
        } else {
            // if no output path was provided, get the stack outputs for printing to the screen.
            let stack = stack_outputs.stack_truncated(self.num_outputs).to_vec();

            // write all outputs to default location if none was provided
            let default_output_path = self.program_file.with_extension("outputs");
            OutputFile::write(&stack_outputs, &default_output_path).map_err(Report::msg)?;

            // print stack outputs to screen.
            println!("Output: {:?}", stack);
        }

        Ok(())
    }
}

// HELPER FUNCTIONS
// ================================================================================================

#[instrument(skip_all)]
fn load_masp_data(params: &ProveCmd) -> Result<(Program, InputFile), Report> {
    let bytes = fs::read(&params.program_file)
        .into_diagnostic()
        .wrap_err("Failed to read package file")?;
    let package = Package::read_from_bytes(&bytes)
        .into_diagnostic()
        .wrap_err("Failed to deserialize package")?;

    let program_arc: Arc<vm_core::Program> = match package.into_mast_artifact() {
        MastArtifact::Executable(program_arc) => program_arc,
        _ => return Err(Report::msg("The provided package is not a program package.")),
    };
    let program = &*program_arc;

    let input_data = InputFile::read(&params.input_file, &params.program_file)?;

    Ok((program.clone(), input_data))
}

#[instrument(skip_all)]
fn load_masm_data(params: &ProveCmd) -> Result<(Program, InputFile), Report> {
    let libraries = Libraries::new(&params.library_paths)?;
    let program =
        ProgramFile::read(&params.program_file)?.compile(Debug::Off, &libraries.libraries)?;
    let input_data = InputFile::read(&params.input_file, &params.program_file)?;
    Ok((program, input_data))
}
