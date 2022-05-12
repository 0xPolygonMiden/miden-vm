use miden::tools::{analyze, ProgramInfo};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Script Source File Path
    path: String,
}

fn main() {
    let args = Args::from_args();
    //reads input masm file
    let program = std::fs::read_to_string(&args.path).expect("Could not read source file");
    let program_info: ProgramInfo = analyze(program);
    let total_vm_cycles = program_info.total_vm_cycles();
    println!("Total Number of VM Cycles: {total_vm_cycles}");
}
