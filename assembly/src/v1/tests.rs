// SIMPLE SCRIPTS
// ================================================================================================

#[test]
fn single_span() {
    let assembler = super::Assembler::new();
    let source = "begin push.1 push.2 add end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "begin span push(1) push(2) add end end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn span_and_simple_if() {
    let assembler = super::Assembler::new();

    // if with else
    let source = "begin push.1 push.2 if.true add else mul end end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "\
        begin \
            join \
                span push(1) push(2) end \
                if.true span add end else span mul end end \
            end \
        end";
    assert_eq!(expected, format!("{}", script));

    // if without else
    let source = "begin push.1 push.2 if.true add end end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "\
        begin \
            join \
                span push(1) push(2) end \
                if.true span add end else span noop end end \
            end \
        end";
    assert_eq!(expected, format!("{}", script));
}

// SCRIPTS WITH PROCEDURES
// ================================================================================================

#[test]
fn script_with_one_procedure() {
    let assembler = super::Assembler::new();
    let source = "proc.foo push.3 push.7 mul end begin push.1 push.2 add exec.foo end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "begin span push(1) push(2) add push(3) push(7) mul end end";
    assert_eq!(expected, format!("{}", script));
}
