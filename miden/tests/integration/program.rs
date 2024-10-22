use alloc::vec::Vec;

use assembly::Assembler;
use miden_vm::Program;
use test_utils::serde::{Deserializable, Serializable};

// PROGRAM SERIALIZATION AND DESERIALIZATION TESTS
// =================================================================

#[test]
fn test_program_serde_simple() {
    let source = "
    begin
        push.1.2
        add
        drop
    end
    ";

    let assembler = Assembler::default();
    let original_program = assembler.assemble_program(source).unwrap();

    let mut target = Vec::new();
    original_program.write_into(&mut target);
    let deserialized_program = Program::read_from_bytes(&target).unwrap();

    assert_eq!(original_program, deserialized_program);
}

#[test]
fn test_program_serde_with_decorators() {
    let source = "
    const.DEFAULT_CONST=100

    proc.foo
        push.1.2 add
        debug.stack.8
    end

    begin
        emit.DEFAULT_CONST

        exec.foo

        debug.stack.4

        drop

        trace.DEFAULT_CONST
    end
    ";

    let assembler = Assembler::default().with_debug_mode(true);
    let original_program = assembler.assemble_program(source).unwrap();

    let mut target = Vec::new();
    original_program.write_into(&mut target);
    let deserialized_program = Program::read_from_bytes(&target).unwrap();

    assert_eq!(original_program, deserialized_program);
}
