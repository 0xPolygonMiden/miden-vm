use core::ops::Range;

// RE-EXPORTS
// ================================================================================================

pub use winterfell::{
    math::{fields::f128::BaseElement, FieldElement, StarkField},
    ExecutionTrace,
};

mod programs;
use programs::blocks::{Loop, ProgramBlock, Span};
pub use programs::{blocks, Program, ProgramInputs};

mod decoder;
use decoder::Decoder;

mod stack;
use stack::Stack;

pub mod opcodes;
pub use opcodes::{OpHint, UserOps as OpCode};

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

// GLOBAL CONSTANTS
// ================================================================================================

pub const MAX_CONTEXT_DEPTH: usize = 16;
pub const MAX_LOOP_DEPTH: usize = 8;
const MIN_TRACE_LENGTH: usize = 16;
pub const BASE_CYCLE_LENGTH: usize = 16;

const MIN_STACK_DEPTH: usize = 8;
const MIN_CONTEXT_DEPTH: usize = 1;
const MIN_LOOP_DEPTH: usize = 1;

// PUSH OPERATION
// ------------------------------------------------------------------------------------------------
const PUSH_OP_ALIGNMENT: usize = 8;

// HASH OPERATION
// ------------------------------------------------------------------------------------------------
const HASH_STATE_RATE: usize = 4;
const HASH_STATE_CAPACITY: usize = 2;
const HASH_STATE_WIDTH: usize = HASH_STATE_RATE + HASH_STATE_CAPACITY;
const HASH_NUM_ROUNDS: usize = 10;
const HASH_DIGEST_SIZE: usize = 2;

// OPERATION SPONGE
// ------------------------------------------------------------------------------------------------
const SPONGE_WIDTH: usize = 4;
const PROGRAM_DIGEST_SIZE: usize = 2;
const HACC_NUM_ROUNDS: usize = 14;

// DECODER LAYOUT
// ------------------------------------------------------------------------------------------------
//
//  ctr ╒═════ sponge ══════╕╒═══ cf_ops ══╕╒═══════ ld_ops ═══════╕╒═ hd_ops ╕╒═ ctx ══╕╒═ loop ═╕
//   0    1    2    3    4    5    6    7    8    9    10   11   12   13   14   15   ..   ..   ..
// ├────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┤

const NUM_CF_OP_BITS: usize = 3;
const NUM_LD_OP_BITS: usize = 5;
const NUM_HD_OP_BITS: usize = 2;

const NUM_CF_OPS: usize = 8;
const NUM_LD_OPS: usize = 32;
const NUM_HD_OPS: usize = 4;

const OP_COUNTER_IDX: usize = 0;
const OP_SPONGE_RANGE: Range<usize> = Range { start: 1, end: 5 };
const CF_OP_BITS_RANGE: Range<usize> = Range { start: 5, end: 8 };
const LD_OP_BITS_RANGE: Range<usize> = Range { start: 8, end: 13 };
const HD_OP_BITS_RANGE: Range<usize> = Range { start: 13, end: 15 };

// STACK LAYOUT
// ------------------------------------------------------------------------------------------------
//
// ╒═══════════════════ user registers ════════════════════════╕
//    0      1    2    .................................    31
// ├─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┤

pub const MAX_PUBLIC_INPUTS: usize = 8;
pub const MAX_OUTPUTS: usize = MAX_PUBLIC_INPUTS;
pub const MAX_STACK_DEPTH: usize = 32;

// TESTS
// ================================================================================================

/*

#[cfg(test)]
mod tests {

    use super::ProgramInputs;
    use crate::{
        air::{utils::ToElements, TraceMetadata, TraceState},
        programs::assembly,
    };
    use winterfell::{
        math::{fields::f128::BaseElement, FieldElement},
        ExecutionTrace, Serializable,
    };

    #[test]
    fn execute_span() {
        let program = assembly::compile("begin add push.5 mul push.7 end").unwrap();
        let inputs = ProgramInputs::from_public(&[BaseElement::new(1), BaseElement::new(2)]);

        let trace = super::execute(&program, &inputs);
        let trace_length = trace.length();
        let trace_width = trace.width();

        assert_eq!(64, trace_length);
        assert_eq!(17, trace_width);
        let state = get_trace_state(&trace, trace_length - 1);

        assert_eq!(BaseElement::new(46), state.op_counter());
        assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
        assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
        assert_eq!([1, 1].to_elements(), state.hd_op_bits());
        assert_eq!([0].to_elements(), state.ctx_stack());
        assert_eq!([7, 15, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
    }

    #[test]
    fn execute_block() {
        let program = assembly::compile("begin add block push.5 mul push.7 end end").unwrap();
        let inputs = ProgramInputs::from_public(&[BaseElement::new(1), BaseElement::new(2)]);

        let trace = super::execute(&program, &inputs);
        let trace_length = trace.length();
        let trace_width = trace.width();

        assert_eq!(64, trace_length);
        assert_eq!(18, trace_width);
        let state = get_trace_state(&trace, trace_length - 1);

        assert_eq!(BaseElement::new(60), state.op_counter());
        assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
        assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
        assert_eq!([1, 1].to_elements(), state.hd_op_bits());
        assert_eq!([0].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!([7, 15, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
    }

    #[test]
    fn execute_if_else() {
        let program =
            assembly::compile("begin read if.true add push.3 else push.7 add push.8 end mul end")
                .unwrap();

        // execute true branch
        let inputs = ProgramInputs::new(
            &[BaseElement::new(5), BaseElement::new(3)],
            &[BaseElement::new(1)],
            &[],
        );
        let trace = super::execute(&program, &inputs);
        let trace_length = trace.length();
        let trace_width = trace.width();

        assert_eq!(128, trace_length);
        assert_eq!(19, trace_width);
        let state = get_trace_state(&trace, trace_length - 1);

        assert_eq!(BaseElement::new(76), state.op_counter());
        assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
        assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
        assert_eq!([1, 1].to_elements(), state.hd_op_bits());
        assert_eq!([0].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!([24, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());

        // execute false branch
        let inputs = ProgramInputs::new(
            &[BaseElement::new(5), BaseElement::new(3)],
            &[BaseElement::new(0)],
            &[],
        );
        let trace = super::execute(&program, &inputs);
        let trace_length = trace.length();
        let trace_width = trace.width();

        assert_eq!(128, trace_length);
        assert_eq!(19, trace_width);
        let state = get_trace_state(&trace, trace_length - 1);

        assert_eq!(BaseElement::new(92), state.op_counter());
        assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
        assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
        assert_eq!([1, 1].to_elements(), state.hd_op_bits());
        assert_eq!([0].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!([96, 3, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
    }

    #[test]
    fn execute_loop() {
        let program = assembly::compile("begin mul read while.true dup mul read end end").unwrap();

        // don't enter the loop
        let inputs = ProgramInputs::new(
            &[BaseElement::new(5), BaseElement::new(3)],
            &[BaseElement::new(0)],
            &[],
        );
        let trace = super::execute(&program, &inputs);

        assert_eq!(64, trace.length());
        assert_eq!(18, trace.width());
        let state = get_trace_state(&trace, trace.length() - 1);

        assert_eq!(BaseElement::new(60), state.op_counter());
        assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
        assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
        assert_eq!([1, 1].to_elements(), state.hd_op_bits());
        assert_eq!([0].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!([15, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());

        // execute one iteration
        let inputs = ProgramInputs::new(
            &[BaseElement::new(5), BaseElement::new(3)],
            &[BaseElement::new(1), BaseElement::new(0)],
            &[],
        );
        let trace = super::execute(&program, &inputs);

        assert_eq!(128, trace.length());
        assert_eq!(19, trace.width());
        let state = get_trace_state(&trace, trace.length() - 1);

        assert_eq!(BaseElement::new(75), state.op_counter());
        assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
        assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
        assert_eq!([1, 1].to_elements(), state.hd_op_bits());
        assert_eq!([0].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!([225, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());

        // execute five iteration
        let inputs = ProgramInputs::new(
            &[BaseElement::new(5), BaseElement::new(3)],
            &[
                BaseElement::new(1),
                BaseElement::new(1),
                BaseElement::new(1),
                BaseElement::new(1),
                BaseElement::new(1),
                BaseElement::new(0),
            ],
            &[],
        );
        let trace = super::execute(&program, &inputs);

        assert_eq!(256, trace.length());
        assert_eq!(19, trace.width());
        let state = get_trace_state(&trace, trace.length() - 1);

        assert_eq!(BaseElement::new(135), state.op_counter());
        assert_eq!(program.hash().to_vec(), state.program_hash().to_bytes());
        assert_eq!([1, 1, 1].to_elements(), state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1].to_elements(), state.ld_op_bits());
        assert_eq!([1, 1].to_elements(), state.hd_op_bits());
        assert_eq!([0].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!(
            [43143988327398919500410556793212890625, 0, 0, 0, 0, 0, 0, 0].to_elements(),
            state.user_stack()
        );
    }

    fn get_trace_state(
        trace: &ExecutionTrace<BaseElement>,
        step: usize,
    ) -> TraceState<BaseElement> {
        let meta = TraceMetadata::from_trace_info(&trace.get_info());
        let mut row = vec![BaseElement::ZERO; trace.width()];
        trace.read_row_into(step, &mut row);
        TraceState::from_vec(meta.ctx_depth, meta.loop_depth, meta.stack_depth, &row)
    }
}

*/
