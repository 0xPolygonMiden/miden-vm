use assembly::{Assembler, AssemblyError};
use core::fmt;
use processor::ExecutionError;
use structopt::StructOpt;
use vm_core::{utils::collections::Vec, Decorator, Operation, ProgramInputs};

/// Defines cli interace
#[derive(StructOpt, Debug)]
#[structopt(about = "Analyze a miden program")]
pub struct Analyze {
    /// Script Source File Path
    masm_path: String,
}

/// Implements CLI execution logic
impl Analyze {
    pub fn execute(&self) -> Result<(), String> {
        let program = std::fs::read_to_string(&self.masm_path).expect("Could not read masm file");
        let program_inputs = ProgramInputs::none();
        let program_info: ProgramInfo =
            analyze(program.as_str(), program_inputs).expect("Could not retreive program info");

        print_stats(&program_info);
        Ok(())
    }
}

/// Utility function to print the following stats about the program being run
/// - Total Number of VM Cycles
/// - Total Number of NOOPs executed:
/// - Number of cycles per assembly instruction
fn print_stats(program_info: &ProgramInfo) {
    let total_vm_cycles = program_info.total_vm_cycles();
    let total_noops = program_info.total_noops();
    let asmop_info_list = program_info.asmop_info_list();
    println!("Total Number of VM Cycles: {}\n", total_vm_cycles);
    println!("Total Number of NOOPs executed: {}\n", total_noops);
    println!(
        "{0: <20} | {1: <20} | {2: <20} | {3: <20}",
        "AsmOp", "Instruction Cycles", "Frequency", "Total Cycles"
    );
    println!("{:-<1$}", "", 80);
    for op_info in asmop_info_list {
        println!(
            "{0: <20} | {1: <20} | {2: <20} | {3: <20}",
            op_info.0,
            op_info.1,
            op_info.2,
            op_info.2 * op_info.1 as usize
        );
    }
}

/// Contains info of a program. Used for program analysis. Contains the following fields:
/// - total_vm_cycles (vm cycles it takes to execute the entire script)
/// - total_noops (total noops executed as part of a program)
/// - asmop_info_list (maps an assembly instruction to the number of vm cycles it takes to execute
/// the instruction and the number of times the instruction is run as part of the given program.)
#[derive(Debug, Eq, PartialEq)]
pub struct ProgramInfo {
    total_vm_cycles: usize,
    total_noops: usize,
    asmop_info_list: Vec<(String, u8, usize)>,
}

impl ProgramInfo {
    /// Creates a new ProgramInfo object from the specified `total_vm_cycles`, `total_noops` and
    /// `op_info_list`.
    /// * total_vm_cycles: Total number of VM cycles required to run the given program.
    /// * total_noops: Total number of NOOPs executed as part of the program.
    /// * op_info_list: Vector of tuples that maps an assembly instruction to the number of vm cycles
    /// it takes to execute it and the number of times it is run as part of the given program.
    pub fn new(
        total_vm_cycles: usize,
        total_noops: usize,
        asmop_info_list: Vec<(String, u8, usize)>,
    ) -> ProgramInfo {
        Self {
            total_vm_cycles,
            total_noops,
            asmop_info_list,
        }
    }

    /// Returns total vm cycles to execute a program
    pub fn total_vm_cycles(&self) -> usize {
        self.total_vm_cycles
    }

    /// Returns total noops executed as part of a program
    pub fn total_noops(&self) -> usize {
        self.total_noops
    }

    /// Returns a Vector of tuples that maps an assembly instruction to the number of vm cycles it takes to
    /// execute it and the number of times it is run as part of the given program.
    pub fn asmop_info_list(&self) -> &Vec<(String, u8, usize)> {
        &self.asmop_info_list
    }
}

/// Returns program analysis of a given program.
pub fn analyze(program: &str, inputs: ProgramInputs) -> Result<ProgramInfo, ProgramError> {
    let assembler = Assembler::new(true);
    let script = assembler
        .compile_script(program)
        .map_err(ProgramError::AssemblyError)?;
    let vm_state_iterator = processor::execute_iter(&script, &inputs);

    let mut total_vm_cycles = 0;
    let mut noop_count = 0;
    let mut asmop_info_list = Vec::new();

    for state in vm_state_iterator {
        let vm_state = state.map_err(ProgramError::ExecutionError)?;
        if matches!(vm_state.op, Some(Operation::Noop)) {
            noop_count += 1;
        }
        if let Some(decorators) = vm_state.decorators {
            for decorator in decorators {
                match decorator {
                    Decorator::AsmOp(asmop_info) => {
                        match &mut asmop_info_list.binary_search_by_key(
                            &(asmop_info.get_op()),
                            |(a, _, _): &(String, u8, usize)| &a,
                        ) {
                            Ok(pos) => {
                                asmop_info_list[*pos].2 += 1;
                            }
                            Err(pos) => asmop_info_list.insert(
                                *pos,
                                (asmop_info.get_op().clone(), asmop_info.get_num_cycles(), 1),
                            ),
                        }
                    }
                    _ => (),
                }
            }
        }
        total_vm_cycles = vm_state.clk;
    }

    Ok(ProgramInfo::new(
        total_vm_cycles,
        noop_count,
        asmop_info_list,
    ))
}

/// This is used to specify the error type returned from analyze.
#[derive(Debug)]
pub enum ProgramError {
    AssemblyError(AssemblyError),
    ExecutionError(ExecutionError),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramError::AssemblyError(e) => write!(f, "Assembly Error: {:?}", e),
            ProgramError::ExecutionError(e) => write!(f, "Execution Error: {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn analyze_test() {
        let source = "proc.foo.1 pop.local.0 end begin popw.mem.1 push.17 exec.foo end";
        let program_inputs = super::ProgramInputs::none();
        let program_info =
            super::analyze(source, program_inputs).expect("analyze_test: Unexpected Error");
        let expected_program_info = super::ProgramInfo::new(
            24,
            1,
            vec![
                ("pop.local.0".to_string(), 10, 1),
                ("popw.mem.1".to_string(), 6, 1),
                ("push.17".to_string(), 1, 1),
            ],
        );
        assert_eq!(program_info, expected_program_info);
    }

    #[test]
    fn analyze_test_execution_error() {
        let source = "begin div end";
        let stack_input = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let program_inputs = super::ProgramInputs::new(&stack_input, &[], vec![]).unwrap();
        let program_info = super::analyze(source, program_inputs);
        let expected_error = "Execution Error: DivideByZero(1)";
        assert_eq!(program_info.err().unwrap().to_string(), expected_error);
    }

    #[test]
    fn analyze_test_assembly_error() {
        let source = "proc.foo.1 pop.local.0 end popw.mem.1 push.17 exec.foo end";
        let program_inputs = super::ProgramInputs::none();
        let program_info = super::analyze(source, program_inputs);
        let expected_error = "Assembly Error: assembly error at 3: unexpected token: expected 'begin' but was 'popw.mem.1'";
        assert_eq!(program_info.err().unwrap().to_string(), expected_error);
    }
}
