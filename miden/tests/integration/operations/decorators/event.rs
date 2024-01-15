use super::TestHost;
use assembly::Assembler;

#[test]
fn test_event_handling() {
    let source = "\
    begin
        push.1
        emit.1
        push.2
        emit.2
    end";

    // compile and execute program
    let program = Assembler::default().compile(source).unwrap();
    let mut host = TestHost::default();
    processor::execute(&program, Default::default(), &mut host, Default::default()).unwrap();

    // make sure events were handled correctly
    let expected = vec![1, 2];
    assert_eq!(host.event_handler, expected);
}
