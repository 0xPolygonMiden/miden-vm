use vm_core::{
    hasher, op_sponge,
    opcodes::{self, OpHint, UserOps as OpCode},
    program::blocks::{Loop, ProgramBlock, Span},
    BASE_CYCLE_LENGTH, HACC_NUM_ROUNDS, MAX_CONTEXT_DEPTH, MAX_LOOP_DEPTH, MAX_STACK_DEPTH,
    MIN_STACK_DEPTH, MIN_TRACE_LENGTH, NUM_CF_OP_BITS, NUM_HD_OP_BITS, NUM_LD_OP_BITS,
    PUSH_OP_ALIGNMENT,
};

mod decoder;
use decoder::Decoder;

mod stack;
use stack::Stack;

pub mod v1;

// EXPORTS
// ================================================================================================

pub use vm_core::{
    program::{Program, ProgramInputs},
    BaseElement, FieldElement, StarkField,
};
pub use winterfell::ExecutionTrace;

// PUBLIC FUNCTIONS
// ================================================================================================

/// Returns register traces resulting from executing the `program` against the specified inputs.
pub fn execute(program: &Program, inputs: &ProgramInputs) -> ExecutionTrace<BaseElement> {
    // initialize decoder and stack components
    let mut decoder = Decoder::new(MIN_TRACE_LENGTH);
    let mut stack = Stack::new(inputs, MIN_TRACE_LENGTH);

    // execute body of the program
    execute_blocks(program.root().body(), &mut decoder, &mut stack);
    close_block(&mut decoder, &mut stack, BaseElement::ZERO, true);

    // fill in remaining steps to make sure the length of the trace is a power of 2
    decoder.finalize_trace();
    stack.finalize_trace();

    // build execution trace metadata as a vector of bytes
    let op_counter = decoder.max_op_counter_value();
    let context_depth = decoder.max_ctx_stack_depth();
    let loop_depth = decoder.max_loop_stack_depth();
    let mut meta = op_counter.to_le_bytes().to_vec();
    meta.push(context_depth as u8);
    meta.push(loop_depth as u8);

    // merge decoder and stack register traces into a single vector
    let mut register_traces = decoder.into_register_traces();
    register_traces.append(&mut stack.into_register_traces());

    let mut trace = ExecutionTrace::init(register_traces);
    trace.set_meta(meta);

    trace
}

// HELPER FUNCTIONS
// ================================================================================================
fn execute_blocks(blocks: &[ProgramBlock], decoder: &mut Decoder, stack: &mut Stack) {
    // execute first block in the sequence, which mast be a Span block
    match &blocks[0] {
        ProgramBlock::Span(block) => execute_span(block, decoder, stack, true),
        _ => panic!("first block in a sequence must be a Span block"),
    }

    // execute all other blocks in the sequence one after another
    for block in blocks.iter().skip(1) {
        match block {
            ProgramBlock::Span(block) => execute_span(block, decoder, stack, false),
            ProgramBlock::Group(block) => {
                start_block(decoder, stack);
                execute_blocks(block.body(), decoder, stack);
                close_block(decoder, stack, BaseElement::ZERO, true);
            }
            ProgramBlock::Switch(block) => {
                start_block(decoder, stack);
                let condition = stack.get_stack_top();
                match condition {
                    BaseElement::ZERO => {
                        execute_blocks(block.false_branch(), decoder, stack);
                        close_block(decoder, stack, block.true_branch_hash(), false);
                    }
                    BaseElement::ONE => {
                        execute_blocks(block.true_branch(), decoder, stack);
                        close_block(decoder, stack, block.false_branch_hash(), true);
                    }
                    _ => panic!(
                        "cannot select a branch based on a non-binary condition {}",
                        condition
                    ),
                };
            }
            ProgramBlock::Loop(block) => {
                let condition = stack.get_stack_top();
                match condition {
                    BaseElement::ZERO => {
                        start_block(decoder, stack);
                        execute_blocks(block.skip(), decoder, stack);
                        close_block(decoder, stack, block.body_hash(), false);
                    }
                    BaseElement::ONE => execute_loop(block, decoder, stack),
                    _ => panic!(
                        "cannot enter loop based on a non-binary condition {}",
                        condition
                    ),
                }
            }
        }
    }
}

/// Executes all instructions in a Span block.
fn execute_span(block: &Span, decoder: &mut Decoder, stack: &mut Stack, is_first: bool) {
    // if this is the first Span block in a sequence of blocks, it needs to be
    // pre-padded with a NOOP to make sure the first instruction in the block
    // starts executing on a step which is a multiple of 16
    if !is_first {
        decoder.decode_op(OpCode::Noop, BaseElement::ZERO);
        stack.execute(OpCode::Noop, OpHint::None);
    }

    // execute all other instructions in the block
    for i in 0..block.length() {
        let (op_code, op_hint) = block.get_op(i);
        decoder.decode_op(op_code, op_hint.value());
        stack.execute(op_code, op_hint);
    }
}

/// Starts executing a new program block.
fn start_block(decoder: &mut Decoder, stack: &mut Stack) {
    decoder.start_block();
    stack.execute(OpCode::Noop, OpHint::None);
}

/// Closes the currently executing program block.
fn close_block(
    decoder: &mut Decoder,
    stack: &mut Stack,
    sibling_hash: BaseElement,
    is_true_branch: bool,
) {
    // a sequence of blocks always ends on a step which is one less than a multiple of 16;
    // all sequences end one operation short of multiple of 16 - so, we need to pad them
    // with a single NOOP ensure proper alignment
    decoder.decode_op(OpCode::Noop, BaseElement::ZERO);
    stack.execute(OpCode::Noop, OpHint::None);

    // end the block, this prepares decoder registers for merging block hash into
    // program hash
    decoder.end_block(sibling_hash, is_true_branch);
    stack.execute(OpCode::Noop, OpHint::None);

    // execute NOOPs to merge block hash into the program hash
    for _ in 0..HACC_NUM_ROUNDS {
        decoder.decode_op(OpCode::Noop, BaseElement::ZERO);
        stack.execute(OpCode::Noop, OpHint::None);
    }
}

/// Executes the specified loop.
fn execute_loop(block: &Loop, decoder: &mut Decoder, stack: &mut Stack) {
    // mark the beginning of the loop block
    decoder.start_loop(block.image());
    stack.execute(OpCode::Noop, OpHint::None);

    // execute blocks in loop body until top of the stack becomes 0
    loop {
        execute_blocks(block.body(), decoder, stack);

        let condition = stack.get_stack_top();
        match condition {
            BaseElement::ZERO => {
                decoder.break_loop();
                stack.execute(OpCode::Noop, OpHint::None);
                break;
            }
            BaseElement::ONE => {
                decoder.wrap_loop();
                stack.execute(OpCode::Noop, OpHint::None);
            }
            _ => panic!(
                "cannot exit loop based on a non-binary condition {}",
                condition
            ),
        };
    }

    // execute the contents of the skip block to make sure the loop was exited correctly
    match &block.skip()[0] {
        ProgramBlock::Span(block) => execute_span(block, decoder, stack, true),
        _ => panic!("invalid skip block content: content must be a Span block"),
    }

    // close block
    close_block(decoder, stack, block.skip_hash(), true);
}
