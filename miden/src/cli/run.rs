use super::data::{Debug, InputFile, Libraries, OutputFile, ProgramFile};
use clap::Parser;
use processor::ExecutionOptions;
use std::{path::PathBuf, time::Instant};

#[derive(Debug, Clone, Parser)]
#[clap(about = "Run a miden program")]
pub struct RunCmd {
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

    /// Number of ouptuts
    #[clap(short = 'n', long = "num-outputs", default_value = "16")]
    num_outputs: usize,

    /// Path to output file
    #[clap(short = 'o', long = "output", value_parser)]
    output_file: Option<PathBuf>,
}

impl RunCmd {
    pub fn execute(&self) -> Result<(), String> {
        println!("============================================================");
        println!("Run program");
        println!("============================================================");

        // load libraries from files
        let libraries = Libraries::new(&self.library_paths)?;

        // load program from file and compile
        let program = ProgramFile::read(&self.assembly_file, &Debug::Off, libraries.libraries)?;

        // load input data from file
        let input_data = InputFile::read(&self.input_file, &self.assembly_file)?;

        // get execution options
        let execution_options = ExecutionOptions::new(Some(self.max_cycles), self.expected_cycles)
            .map_err(|err| format!("{err}"))?;

        // fetch the stack and program inputs from the arguments
        let stack_inputs = input_data.parse_stack_inputs()?;
        let advice_provider = input_data.parse_advice_provider()?;

        let program_hash: [u8; 32] = program.hash().into();
        print!("Executing program with hash {}... ", hex::encode(program_hash));
        let now = Instant::now();

        // execute program and generate outputs
        let trace = processor::execute(&program, stack_inputs, advice_provider, execution_options)
            .map_err(|err| format!("Failed to generate execution trace = {:?}", err))?;

        println!("done ({} steps in {} ms)", trace.get_trace_len(), now.elapsed().as_millis());

        if let Some(output_path) = &self.output_file {
            // write outputs to file if one was specified
            OutputFile::write(trace.stack_outputs(), output_path)?;
        } else {
            // write the stack outputs to the screen.
            println!("Output: {:?}", trace.stack_outputs().stack_truncated(self.num_outputs));
        }

        Ok(())
    }
}
