use miden::MemAdviceProvider;
use processor::{AdviceInputs, Process, StackInputs};
use stdlib::StdLibrary;
use vm_core::{ONE, ZERO};

#[test]
fn memcopy() {
    let source = "
    use.std::mem

    begin
        push.0.0.0.1.1000 mem_storew dropw
        push.0.0.1.0.1001 mem_storew dropw
        push.0.0.1.1.1002 mem_storew dropw
        push.0.1.0.0.1003 mem_storew dropw
        push.0.1.0.1.1004 mem_storew dropw

        push.2000.1000.5 exec.mem::memcopy
    end
    ";

    let assembler = assembly::Assembler::default()
        .with_library(&StdLibrary::default())
        .expect("failed to load stdlib");

    let program = assembler.compile(source).expect("Failed to compile test source.");

    let mut process = Process::new(
        program.kernel().clone(),
        StackInputs::default(),
        MemAdviceProvider::from(AdviceInputs::default()),
    );
    process.execute(&program).unwrap();

    assert_eq!(process.get_memory_value(0, 1000), Some([ZERO, ZERO, ZERO, ONE]), "Address 1000");
    assert_eq!(process.get_memory_value(0, 1001), Some([ZERO, ZERO, ONE, ZERO]), "Address 1001");
    assert_eq!(process.get_memory_value(0, 1002), Some([ZERO, ZERO, ONE, ONE]), "Address 1002");
    assert_eq!(process.get_memory_value(0, 1003), Some([ZERO, ONE, ZERO, ZERO]), "Address 1003");
    assert_eq!(process.get_memory_value(0, 1004), Some([ZERO, ONE, ZERO, ONE]), "Address 1004");

    assert_eq!(process.get_memory_value(0, 2000), Some([ZERO, ZERO, ZERO, ONE]), "Address 2000");
    assert_eq!(process.get_memory_value(0, 2001), Some([ZERO, ZERO, ONE, ZERO]), "Address 2001");
    assert_eq!(process.get_memory_value(0, 2002), Some([ZERO, ZERO, ONE, ONE]), "Address 2002");
    assert_eq!(process.get_memory_value(0, 2003), Some([ZERO, ONE, ZERO, ZERO]), "Address 2003");
    assert_eq!(process.get_memory_value(0, 2004), Some([ZERO, ONE, ZERO, ONE]), "Address 2004");
}
