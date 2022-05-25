use super::{
    ExecutionError, Felt, Join, Loop, OpBatch, Operation, Process, Span, Split, StarkField,
    MIN_TRACE_LEN,
};
use vm_core::{FieldElement, Word};

mod trace;
use trace::DecoderTrace;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const NUM_OP_BITS: usize = Operation::OP_BITS;

// TODO: get from core
const HASHER_WIDTH: usize = 8;
const HASHER_CYCLE_LEN: Felt = Felt::new(8);

// DECODER PROCESS EXTENSION
// ================================================================================================

impl Process {
    // JOIN BLOCK
    // --------------------------------------------------------------------------------------------

    pub(super) fn start_join_block(&mut self, block: &Join) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;

        let hasher_state = [Felt::ZERO; 12];
        let (addr, _result) = self.hasher.hash(hasher_state);
        self.decoder.start_join(block, addr);

        Ok(())
    }

    pub(super) fn end_join_block(&mut self, block: &Join) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;
        self.decoder.end_control_block(block.hash().into());
        Ok(())
    }

    // SPLIT BLOCK
    // --------------------------------------------------------------------------------------------

    pub(super) fn start_split_block(&mut self, block: &Split) -> Result<Felt, ExecutionError> {
        let condition = self.stack.peek();
        self.execute_op(Operation::Drop)?;

        let hasher_state = [Felt::ZERO; 12];
        let (addr, _result) = self.hasher.hash(hasher_state);
        self.decoder.start_split(block, addr);

        Ok(condition)
    }

    pub(super) fn end_split_block(&mut self, block: &Split) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;
        self.decoder.end_control_block(block.hash().into());
        Ok(())
    }

    // LOOP BLOCK
    // --------------------------------------------------------------------------------------------

    pub(super) fn start_loop_block(&mut self, block: &Loop) -> Result<Felt, ExecutionError> {
        let condition = self.stack.peek();
        self.execute_op(Operation::Drop)?;

        let hasher_state = [Felt::ZERO; 12];
        let (addr, _result) = self.hasher.hash(hasher_state);
        self.decoder.start_loop(block, addr, condition);

        Ok(condition)
    }

    pub(super) fn end_loop_block(
        &mut self,
        block: &Loop,
        pop_stack: bool,
    ) -> Result<(), ExecutionError> {
        if pop_stack {
            self.execute_op(Operation::Drop)?;
        } else {
            self.execute_op(Operation::Noop)?;
        }

        self.decoder.end_control_block(block.hash().into());

        Ok(())
    }

    // SPAN BLOCK
    // --------------------------------------------------------------------------------------------

    pub(super) fn start_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;

        let first_batch = &block.op_batches()[0].groups();

        let hasher_state = [
            first_batch[0],
            first_batch[1],
            first_batch[2],
            first_batch[3],
            first_batch[4],
            first_batch[5],
            first_batch[6],
            first_batch[7],
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
        ];
        let (addr, _result) = self.hasher.hash(hasher_state);
        self.decoder.start_span(block, addr);

        Ok(())
    }

    pub(super) fn end_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;

        self.decoder.end_span(block);

        Ok(())
    }
}

// DECODER
// ================================================================================================
/// TODO: add docs
pub struct Decoder {
    block_stack: BlockStack,
    span_context: Option<SpanContext>,
    trace: DecoderTrace,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            block_stack: BlockStack::new(),
            span_context: None,
            trace: DecoderTrace::new(),
        }
    }

    // CONTROL BLOCKS
    // --------------------------------------------------------------------------------------------

    pub fn start_join(&mut self, block: &Join, addr: Felt) {
        let parent_addr = self.block_stack.push(addr, Felt::ZERO);
        let left_child_hash: Word = block.first().hash().into();
        let right_child_hash: Word = block.second().hash().into();
        self.trace.append_block_start(
            parent_addr,
            Operation::Join,
            left_child_hash,
            right_child_hash,
        );
    }

    pub fn start_split(&mut self, block: &Split, addr: Felt) {
        let parent_addr = self.block_stack.push(addr, Felt::ZERO);
        let left_child_hash: Word = block.on_true().hash().into();
        let right_child_hash: Word = block.on_false().hash().into();
        self.trace.append_block_start(
            parent_addr,
            Operation::Split,
            left_child_hash,
            right_child_hash,
        );
    }

    pub fn start_loop(&mut self, block: &Loop, addr: Felt, is_loop: Felt) {
        let parent_addr = self.block_stack.push(addr, is_loop);
        let loop_body_hash: Word = block.body().hash().into();
        self.trace.append_block_start(
            parent_addr,
            Operation::Loop,
            loop_body_hash,
            [Felt::ZERO; 4],
        );
    }

    pub fn repeat(&mut self, _block: &Loop) {
        let loop_addr = self.block_stack.peek().addr;
        self.trace.append_loop_repeat(loop_addr);
    }

    pub fn end_control_block(&mut self, block_hash: Word) {
        let block_info = self.block_stack.pop();
        self.trace.append_block_end(
            block_info.addr,
            block_hash,
            block_info.is_loop_body,
            block_info.is_loop,
        );
    }

    // SPAN BLOCK
    // --------------------------------------------------------------------------------------------

    pub fn start_span(&mut self, block: &Span, addr: Felt) {
        debug_assert!(self.span_context.is_none(), "already in span");
        let parent_addr = self.block_stack.push(addr, Felt::ZERO);
        let first_op_batch = &block.op_batches()[0].groups();
        let num_op_groups = get_num_op_groups_in_span(block);
        self.trace
            .append_span_start(parent_addr, first_op_batch, num_op_groups);

        self.span_context = Some(SpanContext {
            num_groups_left: num_op_groups - Felt::ONE,
            group_ops_left: first_op_batch[0],
        });
    }

    pub fn respan(&mut self, op_batch: &OpBatch) {
        self.trace.append_respan(op_batch.groups());

        let block = self.block_stack.pop();
        self.block_stack
            .push(block.addr + HASHER_CYCLE_LEN, block.is_loop);

        let ctx = self.span_context.as_mut().expect("not in span");
        ctx.num_groups_left -= Felt::ONE;
        ctx.group_ops_left = op_batch.groups()[0];
    }

    pub fn consume_imm_value(&mut self) {
        let ctx = self.span_context.as_mut().expect("not a span");
        ctx.num_groups_left -= Felt::ONE
    }

    pub fn start_op_group(&mut self, op_group: Felt) {
        let ctx = self.span_context.as_mut().expect("not a span");
        ctx.group_ops_left = op_group;
        ctx.num_groups_left -= Felt::ONE;
    }

    pub fn execute_user_op(&mut self, op: Operation, op_idx: usize) {
        debug_assert!(!op.is_decorator(), "op is a decorator");
        let block = self.block_stack.peek();
        let ctx = self.span_context.as_mut().expect("not in span");

        ctx.group_ops_left = remove_opcode_from_group(ctx.group_ops_left, op);

        self.trace.append_user_op(
            op,
            block.addr,
            block.parent_addr,
            ctx.num_groups_left,
            ctx.group_ops_left,
            Felt::from(op_idx as u32),
        );
    }

    pub fn end_span(&mut self, block: &Span) {
        let block_info = self.block_stack.pop();
        let block_hash: Word = block.hash().into();
        self.trace
            .append_span_end(block_hash, block_info.is_loop_body);
        self.span_context = None;
    }

    // TRACE GENERATIONS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn into_trace(self, trace_len: usize, num_rand_rows: usize) -> super::DecoderTrace {
        self.trace
            .into_vec(trace_len, num_rand_rows)
            .try_into()
            .expect("failed to convert vector to array")
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

// BLOCK INFO
// ================================================================================================

pub struct BlockStack {
    blocks: Vec<BlockInfo>,
}

impl BlockStack {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn push(&mut self, addr: Felt, is_loop: Felt) -> Felt {
        let (parent_addr, is_loop_body) = if self.blocks.is_empty() {
            (Felt::ZERO, Felt::ZERO)
        } else {
            let parent = &self.blocks[self.blocks.len() - 1];
            (parent.addr, parent.is_loop)
        };

        self.blocks.push(BlockInfo {
            addr,
            parent_addr,
            is_loop_body,
            is_loop,
        });
        parent_addr
    }

    pub fn pop(&mut self) -> BlockInfo {
        self.blocks.pop().expect("block stack is empty")
    }

    pub fn peek(&self) -> &BlockInfo {
        self.blocks.last().expect("block stack is empty")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BlockInfo {
    addr: Felt,
    parent_addr: Felt,
    is_loop_body: Felt,
    is_loop: Felt,
}

// SPAN CONTEXT
// ================================================================================================

pub struct SpanContext {
    group_ops_left: Felt,
    num_groups_left: Felt,
}

impl Default for SpanContext {
    fn default() -> Self {
        Self {
            group_ops_left: Felt::ZERO,
            num_groups_left: Felt::ZERO,
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn get_num_op_groups_in_span(block: &Span) -> Felt {
    let result = block.op_batches().iter().fold(0usize, |acc, batch| {
        acc + batch.num_groups().next_power_of_two()
    });
    Felt::new(result as u64)
}

fn remove_opcode_from_group(op_group: Felt, op: Operation) -> Felt {
    let opcode = op.op_code().expect("no opcode") as u64;
    Felt::new((op_group.as_int() - opcode) >> NUM_OP_BITS)
}
