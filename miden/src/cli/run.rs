use super::{load_and_compile_script, InputsFile, OutputsFile, Parser};
use air::StarkField;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct RunCmd {
    #[clap(short, parse(from_os_str))]
    script_file: PathBuf,
    #[clap(short, parse(from_os_str))]
    input_file: Option<PathBuf>,
    #[clap(short, value_parser, default_value_t = 16)]
    num_outputs: usize,
    #[clap(short, parse(from_os_str))]
    output_file: Option<PathBuf>,
}

impl RunCmd {
    pub fn execute(&self) -> Result<(), String> {
        // load script from file and compile
        let script = load_and_compile_script(&self.script_file)?;

        // load input data from file
        let input_data = InputsFile::read(&self.input_file, &self.script_file)?;

        // generate execution trace
        let trace = processor::execute(&script, &input_data.get_program_inputs()?)
            .map_err(|err| format!("Failed to generate exection trace = {:?}", err))?;

        // extract outputs from execution trace
        let outputs = trace.last_stack_state()[..self.num_outputs]
            .iter()
            .rev() // TODO investigate inconsistency - no reversal performed in comparison repo (see conditional.rs with n_outputs = 2)
            .map(|&v| v.as_int())
            .collect::<Vec<_>>();

        // write outputs to file
        OutputsFile::write(outputs, &self.output_file)?;

        Ok(())
    }
}
