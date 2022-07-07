use assembly::{Assembler, AssemblyError};
use core::fmt;
use processor::ExecutionError;
use structopt::StructOpt;
use vm_core::{utils::collections::Vec, AsmOpInfo, Operation, ProgramInputs};

// CLI
// ================================================================================================

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
        println!("{}", program_info);
        Ok(())
    }
}

// PROGRAM INFO
// ================================================================================================

/// Contains info of a program. Used for program analysis. Contains the following fields:
/// - total_vm_cycles: vm cycles it takes to execute the entire script
/// - total_noops: total noops executed as part of a program
/// - asmop_info_list: vector of [AsmOpStats] that contains assembly instructions and
///   the number of vm cycles it takes to execute the instruction and the number of times the
///   instruction is run as part of the given program.
#[derive(Debug, Eq, PartialEq)]
pub struct ProgramInfo {
    total_vm_cycles: usize,
    total_noops: usize,
    asmop_info_list: Vec<AsmOpStats>,
}

impl ProgramInfo {
    /// Creates a new ProgramInfo object from the specified `total_vm_cycles`, `total_noops` and
    /// `asmop_info_list`.
    /// * total_vm_cycles: Total number of VM cycles required to run the given program.
    /// * total_noops: Total number of NOOPs executed as part of the program.
    /// * asmop_info_list: Vector of [AsmOpStats] that contains assembly instructions and
    ///   the number of vm cycles it takes to execute the instruction and the number of times the
    ///   instruction is run as part of the given program.
    pub fn new(
        total_vm_cycles: usize,
        total_noops: usize,
        asmop_info_list: Vec<AsmOpStats>,
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

    /// Returns [AsmOpStats] that contains assembly instructions and the number of vm cycles
    /// it takes to execute them and the number of times they are run as part of the given program.
    pub fn asmop_info_list(&self) -> &Vec<AsmOpStats> {
        &self.asmop_info_list
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Increments the noop count by one
    pub fn incr_noop_count(&mut self) {
        self.total_noops += 1;
    }

    /// Sets the total vm cycles to the provided value
    pub fn set_total_vm_cycles(&mut self, total_vm_cycles: usize) {
        self.total_vm_cycles = total_vm_cycles;
    }

    /// Records a new occurence of asmop in the sorted asmop info list of this program info.
    /// If the asmop is already in the list, increments its frequency by one.
    /// If the asmop is not already in the list, add it at the appropriate index to keep the
    /// list sorted alphabetically.
    pub fn record_asmop(&mut self, asmop_info: AsmOpInfo) {
        let asmop_with_params = asmop_info.get_op().clone();
        let asmop_vec: Vec<&str> = asmop_with_params.split(".").collect();
        let op = if asmop_vec.last().unwrap().parse::<usize>().is_ok() {
            asmop_vec.split_last().unwrap().1.join(".")
        } else {
            asmop_with_params
        };
        match &mut self
            .asmop_info_list
            .binary_search_by_key(&(op), |asmop: &AsmOpStats| asmop.op().to_string())
        {
            Ok(pos) => {
                self.asmop_info_list[*pos].incr_frequency();
            }
            Err(pos) => {
                self.asmop_info_list
                    .insert(*pos, AsmOpStats::new(op, asmop_info.get_num_cycles(), 1));
            }
        }
    }
}

impl Default for ProgramInfo {
    fn default() -> Self {
        Self::new(0, 0, Vec::new())
    }
}

impl fmt::Display for ProgramInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_vm_cycles = self.total_vm_cycles();
        let total_noops = self.total_noops();
        let asmop_info_list = self.asmop_info_list();
        write!(f, "Total Number of VM Cycles: {}\n\n", total_vm_cycles)?;
        write!(f, "Total Number of NOOPs executed: {}\n\n", total_noops)?;
        write!(
            f,
            "{0: <20} | {1: <20} | {2: <20} | {3: <20}\n",
            "AsmOp", "Instruction Cycles", "Frequency", "Total Cycles"
        )?;
        for op_info in asmop_info_list {
            write!(
                f,
                "{0: <20} | {1: <20} | {2: <20} | {3: <20}\n",
                op_info.op(),
                op_info.num_cycles(),
                op_info.frequency(),
                op_info.frequency() * op_info.num_cycles() as usize
            )?;
        }
        Ok(())
    }
}

/// Returns program analysis of a given program.
pub fn analyze(program: &str, inputs: ProgramInputs) -> Result<ProgramInfo, ProgramError> {
    let assembler = Assembler::new(true);
    let script = assembler
        .compile_script(program)
        .map_err(ProgramError::AssemblyError)?;
    let vm_state_iterator = processor::execute_iter(&script, &inputs);
    let mut program_info = ProgramInfo::default();

    for state in vm_state_iterator {
        let vm_state = state.map_err(ProgramError::ExecutionError)?;
        if matches!(vm_state.op, Some(Operation::Noop)) {
            program_info.incr_noop_count();
        }
        if let Some(asmop_info) = vm_state.asmop {
            program_info.record_asmop(asmop_info);
        }
        program_info.set_total_vm_cycles(vm_state.clk);
    }

    Ok(program_info)
}

// PROGRAM ERROR
// ================================================================================================

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

// ASMOP STATS
// ================================================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct AsmOpStats {
    op: String,
    num_cycles: u8,
    frequency: usize,
}

impl AsmOpStats {
    /// Returns [AsmOpStats] instantiated with the specified assembly instruction string,
    /// number of cycles it takes to execute the assembly instruction and the number of times
    /// the assembly instruction is executed.
    pub fn new(op: String, num_cycles: u8, frequency: usize) -> Self {
        Self {
            op,
            num_cycles,
            frequency,
        }
    }

    /// Returns the assembly instruction corresponding to this decorator.
    pub fn op(&self) -> &String {
        &self.op
    }

    /// Returns the number of VM cycles taken to execute the assembly instruction of this decorator.
    pub fn num_cycles(&self) -> u8 {
        self.num_cycles
    }

    /// Returns the number of times this AsmOp is executed as part of a program.
    pub fn frequency(&self) -> usize {
        self.frequency
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Increments the frequency of this AsmOp.
    pub fn incr_frequency(&mut self) {
        self.frequency += 1;
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::AsmOpStats;

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
                AsmOpStats::new("pop.local".to_string(), 10, 1),
                AsmOpStats::new("popw.mem".to_string(), 6, 1),
                AsmOpStats::new("push".to_string(), 1, 1),
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
