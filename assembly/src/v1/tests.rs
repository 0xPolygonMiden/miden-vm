#[test]
fn test_parser() {
    let source = "push.1 push.2 add if.true add else mul end push.3 push.4";
    let script = super::compile_script(source).unwrap();
    println!("{}", script.root());
    assert!(false);
}
