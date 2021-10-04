// GROUP BLOCKS
// ================================================================================================
#[test]
fn single_block() {
    let source = "begin push.1 push.2 add end";
    let program = super::compile(source).unwrap();

    let expected = "\
        begin noop noop noop noop noop noop noop \
        push(1) noop noop noop noop noop noop noop \
        push(2) add noop noop noop noop noop noop \
        noop noop noop noop noop noop noop end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn sequence_of_blocks() {
    let source = "begin block push.1 push.2 add end block push.3 push.4 add end end";
    let program = super::compile(source).unwrap();

    let expected = "\
        begin noop noop noop noop noop noop noop \
        noop noop noop noop noop noop noop block \
        push(1) noop noop noop noop noop noop noop \
        push(2) add noop noop noop noop noop end \
        block push(3) noop noop noop noop noop noop \
        noop push(4) add noop noop noop noop noop \
        end end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn sequence_of_blocks_with_prefix() {
    let source = "begin read read add block push.1 push.2 add end block push.3 push.4 sub end end";
    let program = super::compile(source).unwrap();

    let expected = "\
        begin read read add noop noop noop noop \
        noop noop noop noop noop noop noop block \
        push(1) noop noop noop noop noop noop noop \
        push(2) add noop noop noop noop noop end \
        block push(3) noop noop noop noop noop noop \
        noop push(4) neg add noop noop noop noop \
        end end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn sequence_of_blocks_with_prefix_and_suffix() {
    let source =
        "begin read read add block push.1 push.2 add end block push.3 push.4 sub end hash.2 end";
    let program = super::compile(source).unwrap();

    let expected = "\
        begin read read add noop noop noop noop \
        noop noop noop noop noop noop noop block \
        push(1) noop noop noop noop noop noop noop \
        push(2) add noop noop noop noop noop end \
        block push(3) noop noop noop noop noop noop \
        noop push(4) neg add noop noop noop noop \
        end pad2 pad2 noop noop noop noop noop \
        noop noop noop noop noop noop noop noop \
        noop rescr rescr rescr rescr rescr rescr rescr \
        rescr rescr rescr drop4 noop noop noop noop \
        end";

    assert_eq!(expected, format!("{:?}", program));
}

// SWITCH BLOCKS
// ================================================================================================

#[test]
fn single_if_else() {
    let source = "
    begin
        push.3
        push.5
        read
        if.true
            add dup mul
        else
            mul dup add
        end
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
        begin noop noop noop noop noop noop noop \
        push(3) noop noop noop noop noop noop noop \
        push(5) read noop noop noop noop noop noop \
        noop noop noop noop noop noop noop if \
        assert add dup mul noop noop noop noop \
        noop noop noop noop noop noop noop else \
        not assert mul dup add noop noop noop \
        noop noop noop noop noop noop noop end \
        end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn single_if_else_with_suffix() {
    let source = "
    begin
        push.3
        push.5
        read
        if.true
            add dup mul
        else
            mul dup add
        end
        rc.16
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
        begin \
            noop noop noop noop noop noop noop \
            push(3) noop noop noop noop noop noop noop \
            push(5) read noop noop noop noop noop noop \
            noop noop noop noop noop noop noop \
            if \
                assert add dup mul noop noop noop noop \
                noop noop noop noop noop noop noop \
            else \
                not assert mul dup add noop noop noop \
                noop noop noop noop noop noop noop \
            end \
            pad2 noop noop noop noop noop noop noop \
            push(1) swap dup binacc.16 binacc binacc binacc binacc \
            binacc binacc binacc binacc binacc binacc binacc binacc \
            binacc binacc binacc dup drop4 read::eq eq \
        end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn nested_if_else() {
    let source = "
    begin
        push.3
        push.5
        read
        if.true
            add dup mul eq
            if.true
                not push.6 mul
            end
        else
            mul dup add
        end
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
    begin noop noop noop noop noop noop noop \
        push(3) noop noop noop noop noop noop noop \
        push(5) read noop noop noop noop noop noop \
        noop noop noop noop noop noop noop \
        if \
            assert add dup mul read::eq eq noop noop \
            noop noop noop noop noop noop noop \
            if \
                assert not noop noop noop noop noop noop \
                push(6) mul noop noop noop noop noop \
            else \
                not assert noop noop noop noop noop noop \
                noop noop noop noop noop noop noop \
            end \
        else \
            not assert mul dup add noop noop noop \
            noop noop noop noop noop noop noop \
        end \
    end";

    assert_eq!(expected, format!("{:?}", program));
}

// LOOP BLOCKS
// ================================================================================================
#[test]
fn single_loop() {
    let source = "
    begin
        push.3
        push.5
        read
        while.true
            add dup mul read.ab
        end
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
    begin noop noop noop noop noop noop noop \
        push(3) noop noop noop noop noop noop noop \
        push(5) read noop noop noop noop noop noop \
        noop noop noop noop noop noop noop \
        while \
            assert add dup mul read2 noop noop noop \
            noop noop noop noop noop noop noop \
        end \
    end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn loop_with_suffix_and_nested_if_else() {
    let source = "
    begin
        push.3
        push.5
        read
        while.true
            add dup mul read.ab
            if.true
                push.6 sub
            end
            push.7 add
        end
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
    begin noop noop noop noop noop noop noop \
        push(3) noop noop noop noop noop noop noop \
        push(5) read noop noop noop noop noop noop \
        noop noop noop noop noop noop noop \
        while \
            assert add dup mul read2 noop noop noop \
            noop noop noop noop noop noop noop \
            if \
                assert noop noop noop noop noop noop noop \
                push(6) neg add noop noop noop noop \
            else \
                not assert noop noop noop noop noop noop \
                noop noop noop noop noop noop noop \
            end \
            push(7) add noop noop noop noop noop noop \
            noop noop noop noop noop noop noop \
        end \
    end";

    assert_eq!(expected, format!("{:?}", program));
}

// REPEAT BLOCKS
// ================================================================================================

#[test]
fn repeat_2_spans() {
    let source = "
    begin
        read read add read eq
        repeat.2
            push.3 add
        end
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
    begin \
        read read add read read::eq eq noop \
        noop noop noop noop noop noop noop \
        block \
            push(3) add noop noop noop noop noop noop \
            noop noop noop noop noop noop noop noop \
            push(3) add noop noop noop noop noop noop \
            noop noop noop noop noop noop noop \
        end \
    end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn repeat_5_spans() {
    let source = "
    begin
        read read add read eq
        repeat.5
            push.3 add
        end
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
    begin \
        read read add read read::eq eq noop \
        noop noop noop noop noop noop noop \
        block \
            push(3) add noop noop noop noop noop noop \
            noop noop noop noop noop noop noop noop \
            push(3) add noop noop noop noop noop noop \
            noop noop noop noop noop noop noop noop \
            push(3) add noop noop noop noop noop noop \
            noop noop noop noop noop noop noop noop \
            push(3) add noop noop noop noop noop noop \
            noop noop noop noop noop noop noop noop \
            push(3) add noop noop noop noop noop noop \
            noop noop noop noop noop noop noop \
        end \
    end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn repeat_2_blocks() {
    let source = "
    begin
        read read add read eq
        repeat.2
            read
            if.true
                push.3 add mul
            end
        end
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
    begin \
        read read add read read::eq eq noop \
        noop noop noop noop noop noop noop \
        block \
            read noop noop noop noop noop noop noop \
            noop noop noop noop noop noop noop \
            if \
                assert noop noop noop noop noop noop noop \
                push(3) add mul noop noop noop noop \
            else \
                not assert noop noop noop noop noop noop \
                noop noop noop noop noop noop noop \
            end \
            read noop noop noop noop noop noop noop \
            noop noop noop noop noop noop noop \
            if \
                assert noop noop noop noop noop noop noop \
                push(3) add mul noop noop noop noop \
            else \
                not assert noop noop noop noop noop noop \
                noop noop noop noop noop noop noop \
            end \
        end \
    end";

    assert_eq!(expected, format!("{:?}", program));
}

#[test]
fn repeat_2_blocks_with_suffix() {
    let source = "
    begin
        read read add read eq
        repeat.2
            read
            if.true
                push.3 add mul
            end
            sub inv
        end
    end";
    let program = super::compile(source).unwrap();

    let expected = "\
    begin \
        read read add read read::eq eq noop \
        noop noop noop noop noop noop noop \
        block \
            read noop noop noop noop noop noop noop \
            noop noop noop noop noop noop noop \
            if \
                assert noop noop noop noop noop noop noop \
                push(3) add mul noop noop noop noop \
            else \
                not assert noop noop noop noop noop noop \
                noop noop noop noop noop noop noop \
            end \
            neg add inv noop noop noop noop noop \
            noop noop noop noop noop noop noop noop \
            read noop noop noop noop noop noop noop \
            noop noop noop noop noop noop noop \
            if \
                assert noop noop noop noop noop noop noop \
                push(3) add mul noop noop noop noop \
            else \
                not assert noop noop noop noop noop noop \
                noop noop noop noop noop noop noop \
            end \
            neg add inv noop noop noop noop noop \
            noop noop noop noop noop noop noop \
        end \
    end";

    assert_eq!(expected, format!("{:?}", program));
}
