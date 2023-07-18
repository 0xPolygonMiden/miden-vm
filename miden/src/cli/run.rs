use super::data::{Debug, InputFile, Libraries, OutputFile, ProgramFile};
use processor::ExecutionOptions;
use std::{path::PathBuf, time::Instant};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Run", about = "Run a miden program")]
pub struct RunCmd {
    /// Path to .masm assembly file
    #[structopt(short = "a", long = "assembly", parse(from_os_str))]
    assembly_file: PathBuf,

    /// Number of cycles the program is expected to consume
    #[structopt(short = "e", long = "exp-cycles", default_value = "64")]
    expected_cycles: u32,

    /// Path to input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: Option<PathBuf>,

    /// Paths to .masl library files
    #[structopt(short = "l", long = "libraries", parse(from_os_str))]
    library_paths: Vec<PathBuf>,

    /// Maximum number of cycles a program is allowed to consume
    #[structopt(short = "m", long = "max-cycles")]
    max_cycles: Option<u32>,

    /// Number of ouptuts
    #[structopt(short = "n", long = "num-outputs", default_value = "16")]
    num_outputs: usize,

    /// Path to output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
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
        let execution_options = ExecutionOptions::new(self.max_cycles, self.expected_cycles)
            .map_err(|err| format!("{err}"))?;

        // fetch the stack and program inputs from the arguments
        let stack_inputs = input_data.parse_stack_inputs()?;
        let advice_provider = input_data.parse_advice_provider()?;

        let program_hash: [u8; 32] = program.hash().into();
        print!("Executing program with hash {}... ", hex::encode(program_hash));
        let now = Instant::now();

        // execute program and generate outputs
        let trace = processor::execute(&program, stack_inputs, advice_provider, execution_options)
            .map_err(|err| format!("Failed to generate exection trace = {:?}", err))?;

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
