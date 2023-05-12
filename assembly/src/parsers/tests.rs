use vm_core::Felt;

use super::{
    BTreeMap, Instruction, LocalProcMap, ModuleAst, Node, ParsingError, ProcedureAst, ProcedureId,
    ProgramAst, Token,
};
use crate::SourceLocation;

// UNIT TESTS
// ================================================================================================

/// Tests the AST parsing
#[test]
fn test_ast_parsing_program_simple() {
    let source = "begin push.0 assertz add.1 end";
    let nodes: Vec<Node> = vec![
        Node::Instruction(Instruction::PushU8(0)),
        Node::Instruction(Instruction::Assertz),
        Node::Instruction(Instruction::Incr),
    ];

    assert_program_output(source, BTreeMap::new(), nodes);
}

#[test]
fn test_ast_parsing_program_push() {
    let source = "begin push.10 push.500 push.70000 push.5000000000 push.5000000000.7000000000.9000000000.11000000000 push.5.7 push.500.700 push.70000.90000 push.5000000000.7000000000 end";
    let nodes: Vec<Node> = vec![
        Node::Instruction(Instruction::PushU8(10)),
        Node::Instruction(Instruction::PushU16(500)),
        Node::Instruction(Instruction::PushU32(70000)),
        Node::Instruction(Instruction::PushFelt(Felt::from(5000000000_u64))),
        Node::Instruction(Instruction::PushWord(
            vec![
                Felt::from(5000000000_u64),
                Felt::from(7000000000_u64),
                Felt::from(9000000000_u64),
                Felt::from(11000000000_u64),
            ]
            .try_into()
            .unwrap(),
        )),
        Node::Instruction(Instruction::PushU8List(vec![5, 7])),
        Node::Instruction(Instruction::PushU16List(vec![500, 700])),
        Node::Instruction(Instruction::PushU32List(vec![70000, 90000])),
        Node::Instruction(Instruction::PushFeltList(vec![
            Felt::from(5000000000_u64),
            Felt::from(7000000000_u64),
        ])),
    ];

    assert_program_output(source, BTreeMap::new(), nodes);
}

#[test]
fn test_ast_parsing_program_u32() {
    let source = "\
    begin
        push.3

        u32checked_add.5
        u32wrapping_add.5
        u32overflowing_add.5

        u32checked_sub.1
        u32wrapping_sub.1
        u32overflowing_sub.1

        u32checked_mul.2
        u32wrapping_mul.2
        u32overflowing_mul.2

    end";
    let nodes: Vec<Node> = vec![
        Node::Instruction(Instruction::PushU8(3)),
        Node::Instruction(Instruction::U32CheckedAddImm(5)),
        Node::Instruction(Instruction::U32WrappingAddImm(5)),
        Node::Instruction(Instruction::U32OverflowingAddImm(5)),
        Node::Instruction(Instruction::U32CheckedSubImm(1)),
        Node::Instruction(Instruction::U32WrappingSubImm(1)),
        Node::Instruction(Instruction::U32OverflowingSubImm(1)),
        Node::Instruction(Instruction::U32CheckedMulImm(2)),
        Node::Instruction(Instruction::U32WrappingMulImm(2)),
        Node::Instruction(Instruction::U32OverflowingMulImm(2)),
    ];

    assert_program_output(source, BTreeMap::new(), nodes);
}

#[test]
fn test_ast_parsing_program_proc() {
    let source = "\
    proc.foo.1
        loc_load.0
    end
    proc.bar.2
        padw
    end
    begin
        exec.foo
        exec.bar
    end";
    let proc_body1: Vec<Node> = vec![Node::Instruction(Instruction::LocLoad(0))];
    let mut procedures: LocalProcMap = BTreeMap::new();
    procedures.insert(
        String::from("foo"),
        (
            0,
            ProcedureAst::new(String::from("foo").try_into().unwrap(), 1, proc_body1, false, None),
        ),
    );
    let proc_body2: Vec<Node> = vec![Node::Instruction(Instruction::PadW)];
    procedures.insert(
        String::from("bar"),
        (
            1,
            ProcedureAst::new(String::from("bar").try_into().unwrap(), 2, proc_body2, false, None),
        ),
    );
    let nodes: Vec<Node> = vec![
        Node::Instruction(Instruction::ExecLocal(0)),
        Node::Instruction(Instruction::ExecLocal(1)),
    ];
    assert_program_output(source, procedures, nodes);
}

#[test]
fn test_ast_parsing_module() {
    let source = "\
    export.foo.1
        loc_load.0
    end";
    let mut procedures: LocalProcMap = BTreeMap::new();
    let proc_body: Vec<Node> = vec![Node::Instruction(Instruction::LocLoad(0))];
    procedures.insert(
        String::from("foo"),
        (
            0,
            ProcedureAst::new(String::from("foo").try_into().unwrap(), 1, proc_body, true, None),
        ),
    );
    ProgramAst::parse(source).expect_err("Program should contain body and no export");
    let module = ModuleAst::parse(source).unwrap();
    assert_eq!(module.local_procs.len(), procedures.len());
    for (i, proc) in module.local_procs.iter().enumerate() {
        assert_eq!(
            procedures
                .values()
                .find_map(|(idx, proc)| (*idx == i as u16).then_some(proc))
                .unwrap(),
            proc
        );
    }
}

#[test]
fn test_ast_parsing_adv_ops() {
    let source = "begin adv_push.1 adv_loadw end";
    let value = 1_u8;
    let nodes: Vec<Node> = vec![
        Node::Instruction(Instruction::AdvPush(value)),
        Node::Instruction(Instruction::AdvLoadW),
    ];

    assert_program_output(source, BTreeMap::new(), nodes);
}

#[test]
fn test_ast_parsing_adv_injection() {
    let source = "begin adv.u64div adv.keyval adv.mem adv.smtget end";
    let nodes: Vec<Node> = vec![
        Node::Instruction(Instruction::AdvU64Div),
        Node::Instruction(Instruction::AdvKeyval),
        Node::Instruction(Instruction::AdvMem),
        Node::Instruction(Instruction::AdvSmtGet),
    ];

    assert_program_output(source, BTreeMap::new(), nodes);
}

#[test]
fn test_ast_parsing_use() {
    let source = "\
    use.std::abc::foo
    begin
        exec.foo::bar
    end";
    let procedures: LocalProcMap = BTreeMap::new();
    let proc_name = "std::abc::foo::bar";
    let proc_id = ProcedureId::new(proc_name);
    let nodes: Vec<Node> = vec![Node::Instruction(Instruction::ExecImported(proc_id))];
    assert_program_output(source, procedures, nodes);
}

#[test]
fn test_ast_parsing_module_nested_if() {
    let source = "\
    proc.foo
        push.1
        if.true
            push.0
            push.1
            if.true
                push.0
                sub
            else
                push.1
                sub
            end
        end
    end";

    let mut procedures: LocalProcMap = BTreeMap::new();
    let proc_body: Vec<Node> = vec![
        Node::Instruction(Instruction::PushU8(1)),
        Node::IfElse(
            [
                Node::Instruction(Instruction::PushU8(0)),
                Node::Instruction(Instruction::PushU8(1)),
                Node::IfElse(
                    [
                        Node::Instruction(Instruction::PushU8(0)),
                        Node::Instruction(Instruction::Sub),
                    ]
                    .to_vec(),
                    [
                        Node::Instruction(Instruction::PushU8(1)),
                        Node::Instruction(Instruction::Sub),
                    ]
                    .to_vec(),
                ),
            ]
            .to_vec(),
            vec![],
        ),
    ];
    procedures.insert(
        String::from("foo"),
        (
            0,
            ProcedureAst::new(String::from("foo").try_into().unwrap(), 0, proc_body, false, None),
        ),
    );
    ProgramAst::parse(source).expect_err("Program should contain body and no export");
    let module = ModuleAst::parse(source).unwrap();
    assert_eq!(module.local_procs.len(), procedures.len());
    for (i, proc) in module.local_procs.iter().enumerate() {
        assert_eq!(
            procedures
                .values()
                .find_map(|(idx, proc)| (*idx == i as u16).then_some(proc))
                .unwrap(),
            proc
        );
    }
}

#[test]
fn test_ast_parsing_module_sequential_if() {
    let source = "\
    proc.foo
        push.1
        if.true
            push.5
            push.1
        end
        if.true
            push.0
            sub
        else
            push.1
            sub
        end
    end";

    let mut procedures: LocalProcMap = BTreeMap::new();
    let proc_body: Vec<Node> = vec![
        Node::Instruction(Instruction::PushU8(1)),
        Node::IfElse(
            [
                Node::Instruction(Instruction::PushU8(5)),
                Node::Instruction(Instruction::PushU8(1)),
            ]
            .to_vec(),
            vec![],
        ),
        Node::IfElse(
            [Node::Instruction(Instruction::PushU8(0)), Node::Instruction(Instruction::Sub)]
                .to_vec(),
            [Node::Instruction(Instruction::PushU8(1)), Node::Instruction(Instruction::Sub)]
                .to_vec(),
        ),
    ];
    procedures.insert(
        String::from("foo"),
        (
            0,
            ProcedureAst::new(String::from("foo").try_into().unwrap(), 0, proc_body, false, None),
        ),
    );
    ProgramAst::parse(source).expect_err("Program should contain body and no export");
    let module = ModuleAst::parse(source).unwrap();
    assert_eq!(module.local_procs.len(), procedures.len());
    for (i, proc) in module.local_procs.iter().enumerate() {
        assert_eq!(
            procedures
                .values()
                .find_map(|(idx, proc)| (*idx == i as u16).then_some(proc))
                .unwrap(),
            proc
        );
    }
}

// PROCEDURE IMPORTS
// ================================================================================================

#[test]
fn test_missing_import() {
    let source = "\
    begin
        exec.u64::add
    end";

    let result = ProgramAst::parse(source);
    match result {
        Ok(_) => assert!(false),
        Err(err) => assert!(err.to_string().contains("module 'u64' was not imported")),
    }
}

// INVALID BODY TESTS
// ================================================================================================

#[test]
fn test_use_in_proc_body() {
    let source = "\
    export.foo.1
        loc_load.0
        use
    end";

    let result = ModuleAst::parse(source);
    match result {
        Ok(_) => assert!(false),
        Err(err) => assert!(err.to_string().contains("import in procedure body")),
    }
}

#[test]
fn test_unterminated_proc() {
    let source = "proc.foo add mul begin push.1 end";

    let result = ModuleAst::parse(source);
    match result {
        Ok(_) => assert!(false),
        Err(err) => assert!(err.to_string().contains("procedure 'foo' has no matching end")),
    }
}

#[test]
fn test_unterminated_if() {
    let source = "proc.foo add mul if.true add.2 begin push.1 end";

    let result = ModuleAst::parse(source);
    match result {
        Ok(_) => assert!(false),
        Err(err) => assert!(err.to_string().contains("if without matching else/end")),
    }
}

// DOCUMENTATION PARSING TESTS
// ================================================================================================

#[test]
fn test_ast_parsing_simple_docs() {
    let source = "\
    #! proc doc
    export.foo.1
        loc_load.0
    end";

    let proc_body_foo: Vec<Node> = vec![Node::Instruction(Instruction::LocLoad(0))];
    let docs_foo = "proc doc".to_string();
    let procedure = ProcedureAst::new(
        String::from("foo").try_into().unwrap(),
        1,
        proc_body_foo,
        true,
        Some(docs_foo),
    );

    let module = ModuleAst::parse(source).unwrap();

    assert_eq!(module.local_procs.len(), 1);
    assert_eq!(procedure, module.local_procs[0]);
}

#[test]
fn test_ast_parsing_module_docs() {
    let source = "\
#! Test documenation for the whole module in parsing test. Lorem ipsum dolor sit amet,
#! consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
#! This comment is intentionally longer than 256 characters, since we need to be sure that the size
#! of the comments is correctly parsed. There was a bug here earlier.

#! Test documenation for export procedure foo in parsing test. Lorem ipsum dolor sit amet,
#! consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
#! This comment is intentionally longer than 256 characters, since we need to be sure that the size
#! of the comments is correctly parsed. There was a bug here earlier.
export.foo.1
    loc_load.0
end

#! Test documenation for internal procedure bar in parsing test. Lorem ipsum dolor sit amet,
#! consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna
#! aliqua.
proc.bar.2
    padw
end

#! Test documenation for export procedure baz in parsing test. Lorem ipsum dolor sit amet,
#! consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna
#! aliqua.
export.baz.3
    padw
    push.0
end";
    let mut procedures: LocalProcMap = BTreeMap::new();
    let proc_body_foo: Vec<Node> = vec![Node::Instruction(Instruction::LocLoad(0))];
    let docs_foo =
        "Test documenation for export procedure foo in parsing test. Lorem ipsum dolor sit amet,
consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
This comment is intentionally longer than 256 characters, since we need to be sure that the size
of the comments is correctly parsed. There was a bug here earlier."
            .to_string();
    procedures.insert(
        String::from("foo"),
        (
            0,
            ProcedureAst::new(
                String::from("foo").try_into().unwrap(),
                1,
                proc_body_foo,
                true,
                Some(docs_foo),
            ),
        ),
    );

    let proc_body_bar: Vec<Node> = vec![Node::Instruction(Instruction::PadW)];
    procedures.insert(
        String::from("bar"),
        (
            1,
            ProcedureAst::new(
                String::from("bar").try_into().unwrap(),
                2,
                proc_body_bar,
                false,
                None,
            ),
        ),
    );

    let proc_body_baz: Vec<Node> =
        vec![Node::Instruction(Instruction::PadW), Node::Instruction(Instruction::PushU8(0))];
    let docs_baz =
        "Test documenation for export procedure baz in parsing test. Lorem ipsum dolor sit amet,
consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna
aliqua."
            .to_string();
    procedures.insert(
        String::from("baz"),
        (
            2,
            ProcedureAst::new(
                String::from("baz").try_into().unwrap(),
                3,
                proc_body_baz,
                true,
                Some(docs_baz),
            ),
        ),
    );

    ProgramAst::parse(source).expect_err("Program should contain body and no export");
    let module = ModuleAst::parse(source).unwrap();

    let module_docs =
        "Test documenation for the whole module in parsing test. Lorem ipsum dolor sit amet,
consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
This comment is intentionally longer than 256 characters, since we need to be sure that the size
of the comments is correctly parsed. There was a bug here earlier."
            .to_string();
    assert_eq!(module.docs, Some(module_docs));
    assert_eq!(module.local_procs.len(), procedures.len());
    for (i, proc) in module.local_procs.iter().enumerate() {
        assert_eq!(
            procedures
                .values()
                .find_map(|(idx, proc)| (*idx == i as u16).then_some(proc))
                .unwrap(),
            proc
        );
    }
    let module_serialized = module.to_bytes();
    let module_deserialized = ModuleAst::from_bytes(module_serialized.as_slice()).unwrap();

    assert_eq!(module, module_deserialized);
}

#[test]
fn test_ast_parsing_module_docs_fail() {
    let source = "\
    #! module doc

    #! proc doc
    export.foo.1
        loc_load.0
    end

    #! malformed doc
    ";
    ModuleAst::parse(source)
        .expect_err("Procedure comment is not immediately followed by a procedure declaration.");

    let source = "\
    #! proc doc
    export.foo.1
        loc_load.0
    end

    #! malformed doc
    ";
    ModuleAst::parse(source)
        .expect_err("Procedure comment is not immediately followed by a procedure declaration.");

    let source = "\
    #! module doc

    #! malformed doc
    ";
    ModuleAst::parse(source)
        .expect_err("Procedure comment is not immediately followed by a procedure declaration.");

    let source = "\
    export.foo.1
        loc_load.0
    end

    #! malformed doc
    ";
    ModuleAst::parse(source)
        .expect_err("Procedure comment is not immediately followed by a procedure declaration.");

    let source = "\
    #! module doc

    export.foo.1
        loc_load.0
    end

    #! malformed doc
    ";
    ModuleAst::parse(source)
        .expect_err("Procedure comment is not immediately followed by a procedure declaration.");

    let source = "\
    #! proc doc
    export.foo.1
        #! malformed doc
        loc_load.0
    end
    ";
    ModuleAst::parse(source)
        .expect_err("Procedure comment is not immediately followed by a procedure declaration.");
}

// SERIALIZATION AND DESERIALIZATION TESTS
// ================================================================================================

#[test]
fn test_ast_program_serde_simple() {
    let source = "begin push.0xabc234 push.0 assertz end";
    let program = ProgramAst::parse(source).unwrap();
    let program_serialized = program.to_bytes();
    let program_deserialized = ProgramAst::from_bytes(program_serialized.as_slice()).unwrap();

    assert_eq!(program, program_deserialized);
}

#[test]
fn test_ast_program_serde_local_procs() {
    let source = "\
    proc.foo.1
        loc_load.0
    end
    proc.bar.2
        padw
    end
    begin
        exec.foo
        exec.bar
    end";
    let program = ProgramAst::parse(source).unwrap();
    let program_serialized = program.to_bytes();
    let program_deserialized = ProgramAst::from_bytes(program_serialized.as_slice()).unwrap();

    assert_eq!(program, program_deserialized);
}

#[test]
fn test_ast_program_serde_exported_procs() {
    let source = "\
    export.foo.1
        loc_load.0
    end
    export.bar.2
        padw
    end";
    let module = ModuleAst::parse(source).unwrap();
    let module_serialized = module.to_bytes();
    let module_deserialized = ModuleAst::from_bytes(module_serialized.as_slice()).unwrap();

    assert_eq!(module, module_deserialized);
}

#[test]
fn test_ast_program_serde_control_flow() {
    let source = "\
    begin
        repeat.3
            push.1
            push.0.1
        end

        if.true
            and
            loc_store.0
        else
            padw
        end

        while.true
            push.5.7
            u32checked_add
            loc_store.1
            push.0
        end

        repeat.3
            push.2
            u32overflowing_mul
        end

    end";

    let program = ProgramAst::parse(source).unwrap();
    let program_serialized = program.to_bytes();
    let program_deserialized = ProgramAst::from_bytes(program_serialized.as_slice()).unwrap();

    assert_eq!(program, program_deserialized);
}

#[test]
fn assert_parsing_line_unmatched_begin() {
    let source = format!("\n\nbegin\npush.1.2\n\nadd mul");
    let err = ProgramAst::parse(&source).err().unwrap();
    let location = SourceLocation::new(3, 1);
    assert_eq!(err, ParsingError::unmatched_begin(&Token::new("begin", location)));
}

#[test]
fn assert_parsing_line_extra_param() {
    let source = format!("begin add.1.2\nend");
    let err = ProgramAst::parse(&source).err().unwrap();
    let location = SourceLocation::new(1, 7);
    assert_eq!(err, ParsingError::extra_param(&Token::new("add.1.2", location)));
}

#[test]
fn assert_parsing_line_invalid_op() {
    let source = "\
    begin
        repeat.3
            push.1
            push.0.1
        end

        # some comments

        if.true
            and
            loc_store.0
        else
            padw
        end

        # more comments
        # to test if line is correct

        while.true
            push.5.7
            u32checked_add
            loc_store.1
            push.0
        end

        repeat.3
            push.2
            u32overflowing_mulx
        end

    end";
    let err = ProgramAst::parse(&source).err().unwrap();
    let location = SourceLocation::new(28, 13);
    assert_eq!(err, ParsingError::invalid_op(&Token::new("u32overflowing_mulx", location)));
}

#[test]
fn assert_parsing_line_unexpected_eof() {
    let source = format!("proc.foo\nadd\nend");
    let err = ProgramAst::parse(&source).err().unwrap();
    let location = SourceLocation::new(3, 1);
    assert_eq!(err, ParsingError::unexpected_eof(location));
}

#[test]
fn assert_parsing_line_unexpected_token() {
    let source = format!("proc.foo\nadd\nend\n\nmul");
    let err = ProgramAst::parse(&source).err().unwrap();
    let location = SourceLocation::new(5, 1);
    assert_eq!(err, ParsingError::unexpected_token(&Token::new("mul", location), "begin"));
}

fn assert_program_output(source: &str, procedures: LocalProcMap, body: Vec<Node>) {
    let program = ProgramAst::parse(source).unwrap();
    assert_eq!(program.body, body);
    assert_eq!(program.local_procs.len(), procedures.len());
    for (i, proc) in program.local_procs.iter().enumerate() {
        assert_eq!(
            procedures
                .values()
                .find_map(|(idx, proc)| (*idx == i as u16).then_some(proc))
                .unwrap(),
            proc
        );
    }
}
