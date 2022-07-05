use assembly::Assembler;
use core::fmt;
use miden::AssemblyError;
use processor::ExecutionError;
use structopt::StructOpt;
use vm_core::{Operation, ProgramInputs};

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

        let total_vm_cycles = program_info.total_vm_cycles();
        let total_noops = program_info.total_noops();

        println!("Total Number of VM Cycles: {}", total_vm_cycles);
        println!("Total Number of NOOPs executed: {}", total_noops);

        Ok(())
    }
}

/// Contains info of a program. Used for program analysis. Contains the following fields:
/// - total_vm_cycles (vm cycles it takes to execute the entire script)
/// - total_noops (total noops executed as part of a program)
#[derive(Debug, Eq, PartialEq)]
pub struct ProgramInfo {
    total_vm_cycles: usize,
    total_noops: usize,
}

impl ProgramInfo {
    /// Creates a new ProgramInfo object
    pub fn new(total_vm_cycles: usize, total_noops: usize) -> ProgramInfo {
        Self {
            total_vm_cycles,
            total_noops,
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

    for state in vm_state_iterator {
        let vm_state = state.map_err(ProgramError::ExecutionError)?;
        if matches!(vm_state.op, Some(Operation::Noop)) {
            noop_count += 1;
        }
        total_vm_cycles = vm_state.clk;
    }

    Ok(ProgramInfo::new(total_vm_cycles, noop_count))
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
        let expected_program_info = super::ProgramInfo::new(24, 1);
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
