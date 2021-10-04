use super::{
    blocks::{Group, Loop, ProgramBlock, Span, Switch},
    BaseElement, FieldElement, OpCode, Program,
};

mod utils;
use utils::{close_block, traverse};

// TESTS
// ================================================================================================

#[test]
fn single_block() {
    let block = build_first_block(OpCode::Noop, 15);

    let program = Program::new(Group::new(vec![block]));
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(program.root().body(), &mut vec![], &mut program_hash, 0);
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );

    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(31, step);
}

#[test]
fn linear_blocks() {
    let block1 = build_first_block(OpCode::Noop, 15);

    let inner_block1 = Span::new_block(vec![OpCode::Add; 15]);
    let block2 = Group::new_block(vec![inner_block1]);

    let inner_block2 = Span::new_block(vec![OpCode::Mul; 15]);
    let block3 = Group::new_block(vec![inner_block2]);

    // sequence of blocks ending with group block
    let program = Program::new(Group::new(vec![
        block1.clone(),
        block2.clone(),
        block3.clone(),
    ]));
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(program.root().body(), &mut vec![], &mut program_hash, 0);
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );

    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(95, step);

    // sequence of blocks ending with span block
    let block4 = Span::new_block(vec![OpCode::Inv; 15]);

    let program = Program::new(Group::new(vec![block1, block2, block3, block4]));
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(program.root().body(), &mut vec![], &mut program_hash, 0);
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );

    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(111, step);
}

#[test]
fn nested_blocks() {
    let block1 = build_first_block(OpCode::Noop, 15);

    let inner_block1 = Span::new_block(vec![OpCode::Add; 15]);
    let block2 = Group::new_block(vec![inner_block1]);

    let inner_block2 = Span::new_block(vec![OpCode::Mul; 15]);
    let inner_inner_block1 = Span::new_block(vec![OpCode::Inv; 15]);
    let inner_block3 = Group::new_block(vec![inner_inner_block1]);
    let block3 = Group::new_block(vec![inner_block2, inner_block3]);

    // sequence of blocks ending with group block
    let program = Program::new(Group::new(vec![block1, block2, block3]));
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(program.root().body(), &mut vec![], &mut program_hash, 0);
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );

    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(127, step);
}

#[test]
fn conditional_program() {
    let block1 = build_first_block(OpCode::Noop, 15);

    let t_branch = vec![Span::new_block(vec![
        OpCode::Assert,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
    ])];
    let f_branch = vec![Span::new_block(vec![
        OpCode::Not,
        OpCode::Assert,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
        OpCode::Mul,
    ])];
    let block2 = Switch::new_block(t_branch, f_branch);

    let program = Program::new(Group::new(vec![block1, block2]));

    // true branch execution
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(
        program.root().body(),
        &mut vec![BaseElement::ONE],
        &mut program_hash,
        0,
    );
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );
    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(63, step);

    // false branch execution
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(
        program.root().body(),
        &mut vec![BaseElement::ZERO],
        &mut program_hash,
        0,
    );
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );
    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(63, step);
}

#[test]
fn simple_loop() {
    let block1 = build_first_block(OpCode::Noop, 15);

    let loop_body = vec![Span::new_block(vec![
        OpCode::Assert,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
        OpCode::Add,
    ])];
    let block2 = Loop::new_block(loop_body);

    let program = Program::new(Group::new(vec![block1, block2]));

    // loop not entered
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(
        program.root().body(),
        &mut vec![BaseElement::ZERO],
        &mut program_hash,
        0,
    );
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );
    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(63, step);

    // loop executed once
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(
        program.root().body(),
        &mut vec![BaseElement::ZERO, BaseElement::ONE],
        &mut program_hash,
        0,
    );
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );
    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(79, step);

    // loop executed 3 times
    let mut program_hash = [BaseElement::ZERO; 4];
    let step = traverse(
        program.root().body(),
        &mut vec![
            BaseElement::ZERO,
            BaseElement::ONE,
            BaseElement::ONE,
            BaseElement::ONE,
        ],
        &mut program_hash,
        0,
    );
    let step = close_block(
        &mut program_hash,
        BaseElement::ZERO,
        BaseElement::ZERO,
        true,
        step,
    );
    assert_eq!(*program.hash(), hash_to_bytes(&program_hash));
    assert_eq!(111, step);
}

// HELPER FUNCTIONS
// ================================================================================================
fn build_first_block(op_code: OpCode, length: usize) -> ProgramBlock {
    let mut instructions = vec![op_code; length];
    instructions[0] = OpCode::Begin;
    return Span::new_block(instructions);
}

fn hash_to_bytes(hash: &[BaseElement; 4]) -> [u8; 32] {
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(BaseElement::elements_as_bytes(&hash[..2]));
    return hash_bytes;
}
