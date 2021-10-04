use super::{
    super::{hashing::hash_op, HACC_NUM_ROUNDS},
    Loop, OpCode, ProgramBlock, Span,
};
use winterfell::math::{fields::f128::BaseElement, FieldElement};

// PUBLIC FUNCTIONS
// ================================================================================================

pub fn traverse(
    blocks: &[ProgramBlock],
    stack: &mut Vec<BaseElement>,
    hash: &mut [BaseElement; 4],
    mut step: usize,
) -> usize {
    // execute first block in the sequence, which mast be a Span block
    step = match &blocks[0] {
        ProgramBlock::Span(block) => traverse_span(block, hash, true, step),
        _ => panic!("first block in a sequence must be a Span block"),
    };

    // execute all other blocks in the sequence one after another
    for block in blocks.iter().skip(1) {
        step = match block {
            ProgramBlock::Span(block) => traverse_span(block, hash, false, step),
            ProgramBlock::Group(block) => {
                step += 1; // BEGIN
                let mut state = [BaseElement::ZERO; 4];
                step = traverse(block.body(), stack, &mut state, step);
                step = close_block(&mut state, hash[0], BaseElement::ZERO, true, step);
                hash.copy_from_slice(&state);
                step
            }
            ProgramBlock::Switch(block) => {
                step += 1; // BEGIN
                let mut state = [BaseElement::ZERO; 4];
                let condition = stack.pop().unwrap();
                match condition {
                    BaseElement::ZERO => {
                        step = traverse(block.false_branch(), stack, &mut state, step);
                        step =
                            close_block(&mut state, hash[0], block.true_branch_hash(), false, step);
                        hash.copy_from_slice(&state);
                        step
                    }
                    BaseElement::ONE => {
                        step = traverse(block.true_branch(), stack, &mut state, step);
                        step =
                            close_block(&mut state, hash[0], block.false_branch_hash(), true, step);
                        hash.copy_from_slice(&state);
                        step
                    }
                    _ => panic!(
                        "cannot select a branch based on a non-binary condition {}",
                        condition
                    ),
                }
            }
            ProgramBlock::Loop(block) => {
                let condition = stack.pop().unwrap();
                match condition {
                    BaseElement::ZERO => {
                        step += 1; // BEGIN
                        let mut state = [BaseElement::ZERO; 4];
                        step = traverse(block.skip(), stack, &mut state, step);
                        step = close_block(&mut state, hash[0], block.body_hash(), false, step);
                        hash.copy_from_slice(&state);
                        step
                    }
                    BaseElement::ONE => traverse_loop(block, hash, stack, step),
                    _ => panic!(
                        "cannot enter loop based on a non-binary condition {}",
                        condition
                    ),
                }
            }
        };
    }

    return step;
}

// HELPER FUNCTIONS
// ================================================================================================

fn traverse_span(
    block: &Span,
    hash: &mut [BaseElement; 4],
    is_first: bool,
    mut step: usize,
) -> usize {
    if !is_first {
        hash_op(hash, OpCode::Noop as u8, BaseElement::ZERO, step);
        step += 1;
    }

    for i in 0..block.length() {
        let (op_code, op_hint) = block.get_op(i);
        hash_op(hash, op_code as u8, op_hint.value(), step);
        step += 1;
    }

    return step;
}

pub fn close_block(
    hash: &mut [BaseElement; 4],
    parent_hash: BaseElement,
    sibling_hash: BaseElement,
    is_true_branch: bool,
    mut step: usize,
) -> usize {
    hash_op(hash, OpCode::Noop as u8, BaseElement::ZERO, step);
    step += 1;

    step += 1; // TEND

    if is_true_branch {
        hash[1] = hash[0];
        hash[0] = parent_hash;
        hash[2] = sibling_hash;
        hash[3] = BaseElement::ZERO;
    } else {
        hash[2] = hash[0];
        hash[0] = parent_hash;
        hash[1] = sibling_hash;
        hash[3] = BaseElement::ZERO;
    }

    for _ in 0..HACC_NUM_ROUNDS {
        hash_op(hash, OpCode::Noop as u8, BaseElement::ZERO, step);
        step += 1;
    }

    return step;
}

fn traverse_loop(
    block: &Loop,
    hash: &mut [BaseElement; 4],
    stack: &mut Vec<BaseElement>,
    mut step: usize,
) -> usize {
    step += 1; // LOOP
    let mut state = [BaseElement::ZERO; 4];

    loop {
        step = traverse(block.body(), stack, &mut state, step);

        let condition = stack.pop().unwrap();
        match condition {
            BaseElement::ZERO => {
                assert!(
                    state[0] == block.image(),
                    "loop image didn't match loop body hash"
                );
                step += 1; // BREAK
                break;
            }
            BaseElement::ONE => {
                assert!(
                    state[0] == block.image(),
                    "loop image didn't match loop body hash"
                );
                state = [BaseElement::ZERO; 4];
                step += 1; // WRAP
            }
            _ => panic!(
                "cannot exit loop based on a non-binary condition {}",
                condition
            ),
        };
    }

    step = match &block.skip()[0] {
        ProgramBlock::Span(block) => traverse_span(block, &mut state, true, step),
        _ => panic!("invalid skip block content: content must be a Span block"),
    };

    step = close_block(&mut state, hash[0], block.skip_hash(), true, step);
    hash.copy_from_slice(&state);
    return step;
}
