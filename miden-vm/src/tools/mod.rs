use core::fmt;
use std::{path::PathBuf, sync::Arc};

use assembly::{
    DefaultSourceManager, SourceManager,
    diagnostics::{Report, WrapErr},
};
use clap::Parser;
use miden_vm::{DefaultHost, Operation, StackInputs, SyncHost, internal::InputFile};
use processor::{AsmOpInfo, TraceLenSummary};
use prover::AdviceInputs;
use stdlib::StdLibrary;
use vm_core::Program;

use super::cli::data::Libraries;
use crate::cli::utils::{get_masm_program, get_masp_program};

// CLI
// ================================================================================================

/// Defines cli interface
#[derive(Debug, Clone, Parser)]
#[command(about = "Analyze a miden program")]
pub struct Analyze {
    /// Path to a .masm assembly file or a .masp package file
    #[arg(value_parser)]
    program_file: PathBuf,

    /// Path to .inputs file
    #[arg(short = 'i', long = "input", value_parser)]
    input_file: Option<PathBuf>,

    /// Paths to .masl library files
    #[arg(short = 'l', long = "libraries", value_parser)]
    library_paths: Vec<PathBuf>,
}

/// Implements CLI execution logic
impl Analyze {
    pub fn execute(&self) -> Result<(), Report> {
        // load libraries from files
        let libraries = Libraries::new(&self.library_paths)?;

        // Determine file type based on extension.
        let ext = self
            .program_file
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Use a single match expression to load the program.
        let (program, source_manager) = match ext.as_str() {
            "masp" => (
                get_masp_program(&self.program_file)?,
                Arc::new(DefaultSourceManager::default()) as Arc<dyn SourceManager>,
            ),
            "masm" => get_masm_program(&self.program_file, &libraries, true)?,
            _ => return Err(Report::msg("The provided file must have a .masm or .masp extension")),
        };

        // load input data from file
        let input_data = InputFile::read(&self.input_file, &self.program_file)?;

        // fetch the stack and program inputs from the arguments
        let stack_inputs = input_data.parse_stack_inputs().map_err(Report::msg)?;
        let advice_inputs = input_data.parse_advice_inputs().map_err(Report::msg)?;
        let mut host = DefaultHost::default();
        host.load_mast_forest(StdLibrary::default().mast_forest().clone())?;

        let execution_details: ExecutionDetails =
            analyze(&program, stack_inputs, advice_inputs, host, source_manager)
                .expect("Could not retrieve execution details");
        let program_name = self
            .program_file
            .file_name()
            .expect("provided file path is incorrect")
            .to_str()
            .unwrap();

        println!("============================================================");
        print!("Analyzed {program_name} program");
        if let Some(input_path) = &self.input_file {
            let input_name = input_path
                .file_name()
                .expect("provided input path is incorrect")
                .to_str()
                .unwrap();
            println!(" with {input_name}");
        }

        println!("{execution_details}");

        Ok(())
    }
}

// EXECUTION DETAILS
// ================================================================================================

/// Contains details of executing a program, used for program analysis.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ExecutionDetails {
    /// Number of noops executed as part of a program.
    total_noops: usize,
    /// Statistics about individual assembly operations executed by the VM, see [AsmOpStats].
    asm_op_stats: Vec<AsmOpStats>,
    /// Information about VM components trace lengths.
    trace_len_summary: TraceLenSummary,
}

impl ExecutionDetails {
    /// Returns total noops executed as part of a program
    pub fn total_noops(&self) -> usize {
        self.total_noops
    }

    /// Returns [AsmOpStats] that contains assembly instructions and the number of vm cycles
    /// it takes to execute them and the number of times they are run as part of the given program.
    pub fn asm_op_stats(&self) -> &[AsmOpStats] {
        &self.asm_op_stats
    }

    /// Returns [TraceLenSummary] that contains the data about lengths of the trace parts.
    pub fn trace_len_summary(&self) -> TraceLenSummary {
        self.trace_len_summary
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Increments the noop count by one
    pub fn incr_noop_count(&mut self) {
        self.total_noops += 1;
    }

    /// Records a new occurrence of asmop in the sorted asmop stats vector of this program info.
    /// If the asmop is already in the list, increments its frequency by one.
    /// If the asmop is not already in the list, add it at the appropriate index to keep the
    /// list sorted alphabetically.
    pub fn record_asmop(&mut self, asmop_info: AsmOpInfo) {
        match &mut self
            .asm_op_stats
            .binary_search_by_key(&(asmop_info.op_generalized()), |asmop: &AsmOpStats| {
                asmop.op().to_string()
            }) {
            Ok(pos) => {
                if asmop_info.cycle_idx() == 1 {
                    self.asm_op_stats[*pos].incr_frequency();
                    self.asm_op_stats[*pos].add_vm_cycles(asmop_info.num_cycles());
                }
            },
            Err(pos) => {
                self.asm_op_stats.insert(
                    *pos,
                    AsmOpStats::new(
                        asmop_info.op_generalized(),
                        1,
                        asmop_info.num_cycles() as usize,
                    ),
                );
            },
        }
    }

    /// Sets the information about lengths of the trace parts.
    pub fn set_trace_len_summary(&mut self, extended_cycles_info: &TraceLenSummary) {
        self.trace_len_summary = *extended_cycles_info;
    }
}

impl fmt::Display for ExecutionDetails {
    #[allow(clippy::write_literal)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // calculate the percentage of padded rows
        let padding_percentage = (self.trace_len_summary().padded_trace_len()
            - self.trace_len_summary().trace_len())
            * 100
            / self.trace_len_summary().padded_trace_len();

        writeln!(
            f,
            "\nVM cycles: {} extended to {} steps ({}% padding).
├── Stack rows: {}
├── Range checker rows: {}
└── Chiplets rows: {}
    ├── Hash chiplet rows: {}
    ├── Bitwise chiplet rows: {}
    ├── Memory chiplet rows: {}
    └── Kernel ROM rows: {}\n",
            self.trace_len_summary().trace_len(),
            self.trace_len_summary().padded_trace_len(),
            padding_percentage,
            self.trace_len_summary().main_trace_len(),
            self.trace_len_summary().range_trace_len(),
            self.trace_len_summary().chiplets_trace_len().trace_len(),
            self.trace_len_summary().chiplets_trace_len().hash_chiplet_len(),
            self.trace_len_summary().chiplets_trace_len().bitwise_chiplet_len(),
            self.trace_len_summary().chiplets_trace_len().memory_chiplet_len(),
            self.trace_len_summary().chiplets_trace_len().kernel_rom_len(),
        )?;
        let total_noops = self.total_noops();
        let asm_op_stats = self.asm_op_stats();

        // calculate the total length of pading for the `AsmOp` column
        let padding =
            asm_op_stats.iter().try_fold(20, |max, value| Ok(value.op().len().max(max)))?;

        writeln!(
            f,
            "{0: <width$} | {1: <20} | {2: <20} | {3:}",
            "Assembly instruction",
            "Frequency",
            "Total Cycles",
            "Avg Instruction Cycles",
            width = padding,
        )?;

        let delimeter = "-".repeat(padding + 71);
        writeln!(f, "{delimeter}")?;

        for op_info in asm_op_stats {
            writeln!(
                f,
                "{0: <width$} | {1: <20} | {2: <20} | {3:.2}",
                op_info.op(),
                op_info.frequency(),
                op_info.total_vm_cycles(),
                op_info.total_vm_cycles() as f64 / op_info.frequency() as f64,
                width = padding,
            )?;
        }

        writeln!(f, "\nTotal number of NOOPs executed: {total_noops}")?;

        Ok(())
    }
}

/// Returns program analysis of a given program.
fn analyze<H>(
    program: &Program,
    stack_inputs: StackInputs,
    advice_inputs: AdviceInputs,
    mut host: H,
    source_manager: Arc<dyn SourceManager>,
) -> Result<ExecutionDetails, Report>
where
    H: SyncHost,
{
    let mut execution_details = ExecutionDetails::default();

    let vm_state_iterator =
        processor::execute_iter(program, stack_inputs, advice_inputs, &mut host, source_manager);
    execution_details.set_trace_len_summary(vm_state_iterator.trace_len_summary());

    for state in vm_state_iterator {
        let vm_state = state.wrap_err("execution error")?;
        if matches!(vm_state.op, Some(Operation::Noop)) {
            execution_details.incr_noop_count();
        }
        if let Some(asmop_info) = vm_state.asmop {
            execution_details.record_asmop(asmop_info);
        }
    }

    Ok(execution_details)
}

// ASMOP STATS
// ================================================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct AsmOpStats {
    op: String,
    frequency: usize,
    total_vm_cycles: usize,
}

impl AsmOpStats {
    /// Returns [AsmOpStats] instantiated with the specified assembly instruction string,
    /// number of cycles it takes to execute the assembly instruction and the number of times
    /// the assembly instruction is executed.
    pub fn new(op: String, frequency: usize, total_vm_cycles: usize) -> Self {
        Self { op, frequency, total_vm_cycles }
    }

    /// Returns the assembly instruction corresponding to this decorator.
    pub fn op(&self) -> &String {
        &self.op
    }

    /// Returns the number of times this AsmOp is executed as part of a program.
    pub fn frequency(&self) -> usize {
        self.frequency
    }

    /// Returns the combined vm cycles all occurrences of this AsmOp take.
    pub fn total_vm_cycles(&self) -> usize {
        self.total_vm_cycles
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Increments the frequency of this AsmOp.
    pub fn incr_frequency(&mut self) {
        self.frequency += 1;
    }

    /// Increments the total vm cycles of this AsmOp by the specified number of vm cycles.
    pub fn add_vm_cycles(&mut self, num_cycles: u8) {
        self.total_vm_cycles += num_cycles as usize;
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use assembly::DefaultSourceManager;
    use miden_vm::Assembler;
    use processor::{ChipletsLengths, DefaultHost, TraceLenSummary};

    use super::{AsmOpStats, ExecutionDetails, StackInputs, *};

    #[test]
    fn analyze_test() {
        let source = "proc.foo.1 loc_store.0 end begin mem_storew.4 dropw push.17 push.1 movdn.2 exec.foo drop end";
        let stack_inputs = StackInputs::default();
        let advice_inputs = AdviceInputs::default();
        let host = DefaultHost::default();
        let program = Assembler::default().with_debug_mode(true).assemble_program(source).unwrap();
        let execution_details = super::analyze(
            &program,
            stack_inputs,
            advice_inputs,
            host,
            Arc::new(DefaultSourceManager::default()),
        )
        .expect("analyze_test: Unexpected Error");
        let expected_details = ExecutionDetails {
            total_noops: 0,
            asm_op_stats: vec![
                AsmOpStats::new("drop".to_string(), 1, 1),
                AsmOpStats::new("dropw".to_string(), 1, 4),
                AsmOpStats::new("loc_store".to_string(), 1, 4),
                AsmOpStats::new("mem_storew".to_string(), 1, 2),
                AsmOpStats::new("movdn.2".to_string(), 1, 1),
                AsmOpStats::new("push".to_string(), 2, 3),
            ],
            trace_len_summary: TraceLenSummary::new(
                21,
                39,
                ChipletsLengths::from_parts(8, 0, 2, 0),
            ),
        };
        assert_eq!(execution_details, expected_details);
    }
}
