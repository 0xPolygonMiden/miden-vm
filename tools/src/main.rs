use miden::{tools::{analyze, ProgramInfo}, ProgramInputs};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Script Source File Path
    masm_path: String,
    /// Stack Input file (Optional)
    stack_input_path: Option<String>,
}

fn main() {
    let args = Args::from_args();
    //reads input masm file
    let program = std::fs::read_to_string(&args.masm_path).expect("Could not read masm file");
    let mut program_inputs = ProgramInputs::none();
    if args.stack_input_path.is_some() {
        //reads input stack file
        let stack_input_str = std::fs::read_to_string(&args.stack_input_path.unwrap()).expect("Could not read stack input file");
        let stack_input: Vec<u64> = stack_input_str.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
        program_inputs = ProgramInputs::new(&stack_input, &[], vec![]).unwrap();
    }
    
    let program_info: ProgramInfo = analyze(program, program_inputs);
    let total_vm_cycles = program_info.total_vm_cycles();
    let total_noops = program_info.total_noops();
    println!("Total Number of VM Cycles: {}", total_vm_cycles);
    println!(
        "Total Number of NOOPs executed: {}",
        total_noops
    );
}
