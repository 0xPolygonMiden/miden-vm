use miden::{
    tools::{analyze, ProgramInfo},
    ProgramInputs,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Script Source File Path
    masm_path: String,
}

fn main() {
    let args = Args::from_args();
    //reads input masm file
    let program = std::fs::read_to_string(&args.masm_path).expect("Could not read masm file");
    let program_inputs = ProgramInputs::none();
    let program_info: ProgramInfo =
        analyze(program.as_str(), program_inputs).expect("Could not retreive program info");

    let total_vm_cycles = program_info.total_vm_cycles();
    let total_noops = program_info.total_noops();

    println!("Total Number of VM Cycles: {}", total_vm_cycles);
    println!("Total Number of NOOPs executed: {}", total_noops);
}
