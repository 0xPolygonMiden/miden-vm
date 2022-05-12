use assembly::Assembler;
use vm_core::ProgramInputs;

#[derive(Debug, PartialEq)]
pub struct ProgramInfo {
    total_vm_cycles: usize,
}

impl ProgramInfo {
    ///Creates a new ProgramInfo object
    pub fn new(total_vm_cycles: usize) -> ProgramInfo {
        Self { total_vm_cycles }
    }

    ///Get total vm cycles to execute a program
    pub fn total_vm_cycles(&self) -> usize {
        self.total_vm_cycles
    }
}

/// Returns program analysis for a given script string. Returns ProgramInfo with following fields:
/// - total_vm_cycles (vm cycles it takes to execute the entire script)
pub fn analyze(program: String) -> ProgramInfo {
    let assembler = Assembler::new();
    let script = assembler.compile_script(&program).unwrap();
    let inputs = ProgramInputs::none();
    let count = processor::execute_iter(&script, &inputs).count() - 1;
    ProgramInfo::new(count)
}

#[cfg(test)]
mod tests {

    #[test]
    fn analyze_test() {
        let source = "proc.foo.1 pop.local.0 end begin popw.mem.1 push.17 exec.foo end";
        let program_info: super::ProgramInfo = super::analyze(source.to_string());
        let expected_program_info = super::ProgramInfo::new(24);
        assert_eq!(program_info, expected_program_info);
    }
}
