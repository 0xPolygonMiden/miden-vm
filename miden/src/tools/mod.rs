use assembly::Assembler;
use vm_core::{Operation, ProgramInputs};

#[derive(Debug, PartialEq)]
pub struct ProgramInfo {
    total_vm_cycles: usize,
    total_noops: usize,
}

impl ProgramInfo {
    ///Creates a new ProgramInfo object
    pub fn new(total_vm_cycles: usize, total_noops: usize) -> ProgramInfo {
        Self {
            total_vm_cycles,
            total_noops,
        }
    }

    ///Get total vm cycles to execute a program
    pub fn total_vm_cycles(&self) -> usize {
        self.total_vm_cycles
    }

    ///Get total noops executed as part of a program
    pub fn total_noops(&self) -> usize {
        self.total_noops
    }
}

/// Returns program analysis for a given script string. Returns ProgramInfo with following fields:
/// - total_vm_cycles (vm cycles it takes to execute the entire script)
/// - total_noops (total noops executed as part of a program)
pub fn analyze(program: String) -> ProgramInfo {
    let assembler = Assembler::new();
    let script = assembler.compile_script(&program).unwrap();
    let inputs = ProgramInputs::none();
    let vm_state_iterator = processor::execute_iter(&script, &inputs);

    let mut total_vm_cycles = 0;
    let mut noop_count = 0;

    for vm_state in vm_state_iterator {
        if vm_state.as_ref().unwrap().op == Operation::Noop {
            noop_count += 1;
        }
        total_vm_cycles = vm_state.unwrap().clk;
    }

    ProgramInfo::new(total_vm_cycles, noop_count)
}

#[cfg(test)]
mod tests {

    #[test]
    fn analyze_test() {
        let source = "proc.foo.1 pop.local.0 end begin popw.mem.1 push.17 exec.foo end";
        let program_info: super::ProgramInfo = super::analyze(source.to_string());
        let expected_program_info = super::ProgramInfo::new(24, 1);
        assert_eq!(program_info, expected_program_info);
    }
}
