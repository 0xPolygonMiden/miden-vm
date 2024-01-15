use super::TestHost;
use assembly::Assembler;

#[test]
fn test_trace_handling() {
    let source = "\
    begin
        push.1
        trace.1
        push.2
        trace.2
    end";

    // compile and execute program
    let program = Assembler::default().compile(source).unwrap();
    let mut host = TestHost::default();
    processor::execute(&program, Default::default(), &mut host, Default::default()).unwrap();

    // make sure traces were handled correctly
    let expected = vec![1, 2];
    assert_eq!(host.trace_handler, expected);
}
