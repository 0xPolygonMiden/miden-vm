use super::{load_and_compile_script, tools, InputsFile, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct AnalyzeCmd {
    #[clap(short, parse(from_os_str))]
    script_file: PathBuf,
    #[clap(short, parse(from_os_str))]
    input_file: Option<PathBuf>,
}

impl AnalyzeCmd {
    pub fn execute(&self) -> Result<(), String> {
        // load script from file and compile
        let script = load_and_compile_script(&self.script_file)?;

        // load input data from file
        let input_data = InputsFile::read(&self.input_file, &self.script_file)?;

        // analyze program info
        let program_info = tools::analyze(&script, input_data.get_program_inputs()?)
            .map_err(|err| format!("Failed to analyze program - {}", err))?;

        // print program information
        println!(
            "Total Number of VM Cycles: {}",
            program_info.total_vm_cycles()
        );
        println!(
            "Total Number of NOOPs executed: {}",
            program_info.total_noops()
        );

        Ok(())
    }
}
