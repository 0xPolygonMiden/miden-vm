// SIMPLE SCRIPTS
// ================================================================================================

#[test]
fn single_span() {
    let assembler = super::Assembler::new();
    let source = "begin push.1 push.2 add end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "begin span pad incr push(2) add end end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn span_and_simple_if() {
    let assembler = super::Assembler::new();

    // if with else
    let source = "begin push.2 push.3 if.true add else mul end end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "\
        begin \
            join \
                span push(2) push(3) end \
                if.true span add end else span mul end end \
            end \
        end";
    assert_eq!(expected, format!("{}", script));

    // if without else
    let source = "begin push.2 push.3 if.true add end end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "\
        begin \
            join \
                span push(2) push(3) end \
                if.true span add end else span noop end end \
            end \
        end";
    assert_eq!(expected, format!("{}", script));
}

// NESTED CONTROL BLOCKS
// ================================================================================================

#[test]
fn nested_control_blocks() {
    let assembler = super::Assembler::new();

    // if with else
    let source = "begin \
        push.2 push.3 \
        if.true \
            add while.true push.7 push.11 add end \
        else \
            mul repeat.2 push.8 end if.true mul end  \
        end
        push.3 add
        end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "\
        begin \
            join \
                join \
                    span push(2) push(3) end \
                    if.true \
                        join \
                            span add end \
                            while.true span push(7) push(11) add end end \
                        end \
                    else \
                        join \
                            span mul push(8) push(8) end \
                            if.true span mul end else span noop end end \
                        end \
                    end \
                end \
            span push(3) add end \
            end \
        end";
    assert_eq!(expected, format!("{}", script));
}

// SCRIPTS WITH PROCEDURES
// ================================================================================================

#[test]
fn script_with_one_procedure() {
    let assembler = super::Assembler::new();
    let source = "proc.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "begin span push(2) push(3) add push(3) push(7) mul end end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn script_with_nested_procedure() {
    let assembler = super::Assembler::new();
    let source = "\
        proc.foo push.3 push.7 mul end \
        proc.bar push.5 exec.foo add end \
        begin push.2 push.4 add exec.foo push.11 exec.bar sub end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "begin \
        span push(2) push(4) add push(3) push(7) mul \
        push(11) push(5) push(3) push(7) mul add neg add \
        end end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn script_with_proc_locals() {
    let assembler = super::Assembler::new();
    let source = "\
        proc.foo.1 \
            pop.local.0 \
            add \
            push.local.0 \
            mul \
        end \
        begin \
            push.4 push.3 push.2 \
            exec.foo \
        end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "\
        begin \
            span \
                push(4) push(3) push(2) \
                push(1) fmpupdate \
                pad pad pad pad fmpadd storew drop drop drop drop \
                add \
                pad pad pad pad pad fmpadd loadw drop drop drop \
                mul \
                push(18446744069414584320) fmpupdate \
            end \
        end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn script_with_exported_procedure() {
    let assembler = super::Assembler::new();
    let source = "export.foo push.3 push.7 mul end begin push.2 push.3 add exec.foo end";
    assert!(assembler.compile_script(source).is_err());
}

// IMPORTS
// ================================================================================================

#[test]
fn script_with_one_import() {
    let assembler = super::Assembler::new();
    let source = "\
        use.std::math::u256
        begin \
            push.4 push.3 \
            exec.u256::iszero_unsafe \
        end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "\
        begin \
            span \
                push(4) push(3) \
                eqz \
                swap eqz and \
                swap eqz and \
                swap eqz and \
                swap eqz and \
                swap eqz and \
                swap eqz and \
                swap eqz and \
            end \
        end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn script_with_import_errors() {
    // --- non-existent import ------------------------------------------------
    let assembler = super::Assembler::new();
    let source = "\
        use.std::math::u512
        begin \
            push.4 push.3 \
            exec.u256::iszero_unsafe \
        end";
    assert!(assembler.compile_script(source).is_err());

    // --- non-existent procedure in import -----------------------------------
    let assembler = super::Assembler::new();
    let source = "\
        use.std::math::u256
        begin \
            push.4 push.3 \
            exec.u256::foo \
        end";
    assert!(assembler.compile_script(source).is_err());
}

// COMMENTS
// ================================================================================================

#[test]
fn comment_simple() {
    let assembler = super::Assembler::new();
    let source = "begin # simple comment \n push.1 push.2 add end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "begin span pad incr push(2) add end end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn comment_in_nested_control_blocks() {
    let assembler = super::Assembler::new();

    // if with else
    let source = "begin \
        push.1 push.2 \
        if.true \
            # nested comment \n\
            add while.true push.7 push.11 add end \
        else \
            mul repeat.2 push.8 end if.true mul end  \
            # nested comment \n\
        end
        push.3 add
        end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "\
        begin \
            join \
                join \
                    span pad incr push(2) end \
                    if.true \
                        join \
                            span add end \
                            while.true span push(7) push(11) add end end \
                        end \
                    else \
                        join \
                            span mul push(8) push(8) end \
                            if.true span mul end else span noop end end \
                        end \
                    end \
                end \
            span push(3) add end \
            end \
        end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn comment_before_script() {
    let assembler = super::Assembler::new();
    let source = " # starting comment \n begin push.1 push.2 add end";
    let script = assembler.compile_script(source).unwrap();
    let expected = "begin span pad incr push(2) add end end";
    assert_eq!(expected, format!("{}", script));
}

#[test]
fn comment_after_script() {
    let assembler = super::Assembler::new();
    let source = "begin push.1 push.2 add end # closing comment";
    let script = assembler.compile_script(source).unwrap();
    let expected = "begin span pad incr push(2) add end end";
    assert_eq!(expected, format!("{}", script));
}

// ERRORS
// ================================================================================================

#[test]
fn invalid_script() {
    let assembler = super::Assembler::new();
    let source = "";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "source code cannot be an empty string");
    }

    let source = " ";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "source code cannot be an empty string");
    }

    let source = "none";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(
            error.message(),
            "unexpected token: expected 'begin' but was 'none'"
        );
    }

    let source = "begin add";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "begin without matching end");
    }

    let source = "begin end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(
            error.message(),
            "a code block must contain at least one instruction"
        );
    }

    let source = "begin add end mul";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "dangling instructions after script end");
    }
}

#[test]
fn invalid_proc() {
    let assembler = super::Assembler::new();

    let source = "proc.foo add mul begin push.1 end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "proc without matching end");
    }

    let source = "proc.foo add mul proc.bar push.3 end begin push.1 end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "proc without matching end");
    }

    let source = "proc.foo add mul end begin push.1 exec.bar end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "undefined procedure: bar");
    }

    let source = "proc.123 add mul end begin push.1 exec.123 end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "invalid procedure label: 123");
    }

    let source = "proc.foo add mul end proc.foo push.3 end begin push.1 end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "duplicate procedure label: foo");
    }
}

#[test]
fn invalid_if_else() {
    let assembler = super::Assembler::new();

    // --- unmatched if ---------------------------------------------------------------------------
    let source = "begin push.1 add if.true mul";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "if without matching else/end");
    }

    // --- unmatched else -------------------------------------------------------------------------
    let source = "begin push.1 add else mul end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "else without matching if");
    }

    let source = "begin push.1 while.true add else mul end end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "else without matching if");
    }

    let source = "begin push.1 if.true add else mul else push.1 end end end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "else without matching if");
    }

    let source = "begin push.1 add if.true mul else add";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "else without matching end");
    }
}

#[test]
fn invalid_repeat() {
    let assembler = super::Assembler::new();

    // unmatched repeat
    let source = "begin push.1 add repeat.10 mul";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "repeat without matching end");
    }

    // invalid iter count
    let source = "begin push.1 add repeat.23x3 mul end end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(
            error.message(),
            "malformed instruction `repeat.23x3`: parameter '23x3' is invalid"
        );
    }
}

#[test]
fn invalid_while() {
    let assembler = super::Assembler::new();

    let source = "begin push.1 add while mul end end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(
            error.message(),
            "malformed instruction 'while': missing required parameter"
        );
    }

    let source = "begin push.1 add while.abc mul end end";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(
            error.message(),
            "malformed instruction `while.abc`: parameter 'abc' is invalid"
        );
    }

    let source = "begin push.1 add while.true mul";
    let script = assembler.compile_script(source);
    assert!(script.is_err());
    if let Err(error) = script {
        assert_eq!(error.message(), "while without matching end");
    }
}
