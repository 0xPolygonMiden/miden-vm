use std::sync::Arc;

use assembly::{Assembler, DefaultSourceManager};
use processor::{ExecutionOptions, Program};
use prover::StackInputs;

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
    processor::execute(
        &program,
        StackInputs::default(),
        &mut host,
        ExecutionOptions::default(),
        Arc::new(DefaultSourceManager::default()),
    )
    .unwrap();

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
    processor::execute(
        &program,
        StackInputs::default(),
        &mut host,
        ExecutionOptions::default(),
        Arc::new(DefaultSourceManager::default()),
    )
    .unwrap();
    let expected = Vec::<u32>::new();
    assert_eq!(host.trace_handler, expected);

    // execute program with enabled tracing
    processor::execute(
        &program,
        StackInputs::default(),
        &mut host,
        ExecutionOptions::default().with_tracing(),
        Arc::new(DefaultSourceManager::default()),
    )
    .unwrap();
    let expected = vec![1, 2];
    assert_eq!(host.trace_handler, expected);
}

#[test]
fn test_debug_with_debugging() {
    let source: &str = "\
    begin
        push.1
        debug.stack
        debug.mem
        drop
    end";

    // compile and execute program
    let program: Program =
        Assembler::default().with_debug_mode(true).assemble_program(source).unwrap();
    let mut host = TestHost::default();
    processor::execute(
        &program,
        StackInputs::default(),
        &mut host,
        ExecutionOptions::default().with_debugging(true),
        Arc::new(DefaultSourceManager::default()),
    )
    .unwrap();

    // Expect to see the debug.stack and debug.mem commands
    let expected = vec!["stack", "mem"];
    assert_eq!(host.debug_handler, expected);
}

#[test]
fn test_debug_without_debugging() {
    let source: &str = "\
    begin
        push.1
        debug.stack
        debug.mem
        drop
    end";

    // compile and execute program
    let program: Program = Assembler::default().assemble_program(source).unwrap();
    let mut host = TestHost::default();
    processor::execute(
        &program,
        StackInputs::default(),
        &mut host,
        ExecutionOptions::default(),
        Arc::new(DefaultSourceManager::default()),
    )
    .unwrap();

    // Expect to see no debug commands
    let expected: Vec<String> = vec![];
    assert_eq!(host.debug_handler, expected);
}
