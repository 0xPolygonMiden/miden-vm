use assembly::Assembler;
use processor::{ExecutionOptions, Program};

use super::TestHost;

#[test]
fn test_event_handling() {
    let source = "\
    begin
        push.1
        emit.1
        push.2
        emit.2
        swapw dropw
    end";

    // compile and execute program
    let program: Program = Assembler::default().assemble_program(source).unwrap();
    let mut host = TestHost::default();
    processor::execute(&program, Default::default(), &mut host, Default::default()).unwrap();

    // make sure events were handled correctly
    let expected = vec![1, 2];
    assert_eq!(host.event_handler, expected);
}

#[test]
fn test_trace_handling() {
    let source = "\
    begin
        push.1
        trace.1
        push.2
        trace.2
        swapw dropw
    end";

    // compile program
    let program: Program = Assembler::default().assemble_program(source).unwrap();
    let mut host = TestHost::default();

    // execute program with disabled tracing
    processor::execute(&program, Default::default(), &mut host, Default::default()).unwrap();
    let expected = Vec::<u32>::new();
    assert_eq!(host.trace_handler, expected);

    // execute program with enabled tracing
    processor::execute(
        &program,
        Default::default(),
        &mut host,
        ExecutionOptions::default().with_tracing(),
    )
    .unwrap();
    let expected = vec![1, 2];
    assert_eq!(host.trace_handler, expected);
}
