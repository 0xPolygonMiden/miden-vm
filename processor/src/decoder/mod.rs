use super::{
    ExecutionError, Felt, FieldElement, Join, Loop, OpBatch, Operation, Process, Span, Split,
    StarkField, Word, MIN_TRACE_LEN, OP_BATCH_SIZE,
};

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

    /// Starts decoding of a JOIN block.
    pub(super) fn start_join_block(&mut self, block: &Join) -> Result<(), ExecutionError> {
        // use the hasher to compute the hash of the JOIN block; the row address returned by the
        // hasher is used as the ID of the block; the result of the hash is expected to be in
        // row addr + 7.
        let child1_hash = block.first().hash().into();
        let child2_hash = block.second().hash().into();
        let (addr, _result) = self.hasher.hash_2to1(child1_hash, child2_hash);

        // make sure the result computed by the hasher is the same as the expected block hash
        debug_assert_eq!(block.hash(), _result.into());

        // start decoding the JOIN block; this appends a row with JOIN operation to the decoder
        // trace. when JOIN operation is executed, the rest of the VM state does not change
        self.decoder.start_join(child1_hash, child2_hash, addr);
        self.execute_op(Operation::Noop)
    }

    ///  Ends decoding of a JOIN block.
    pub(super) fn end_join_block(&mut self, block: &Join) -> Result<(), ExecutionError> {
        // this appends a row with END operation to the decoder trace. when END operation is
        // executed the rest of the VM state does not change
        self.decoder.end_control_block(block.hash().into());
        self.execute_op(Operation::Noop)
    }

    // SPLIT BLOCK
    // --------------------------------------------------------------------------------------------

    /// Starts decoding a SPLIT block. This also pops the value from the top of the stack and
    /// returns it.
    pub(super) fn start_split_block(&mut self, block: &Split) -> Result<Felt, ExecutionError> {
        let condition = self.stack.peek();

        // use the hasher to compute the hash of the SPLIT block; the row address returned by the
        // hasher is used as the ID of the block; the result of the hash is expected to be in
        // row addr + 7.
        let child1_hash = block.on_true().hash().into();
        let child2_hash = block.on_false().hash().into();
        let (addr, _result) = self.hasher.hash_2to1(child1_hash, child2_hash);

        // make sure the result computed by the hasher is the same as the expected block hash
        debug_assert_eq!(block.hash(), _result.into());

        // start decoding the SPLIT block. this appends a row with SPLIT operation to the decoder
        // trace. we also pop the value off the top of the stack and return it.
        self.decoder.start_split(child1_hash, child2_hash, addr);
        self.execute_op(Operation::Drop)?;
        Ok(condition)
    }

    /// Ends decoding of a SPLIT block.
    pub(super) fn end_split_block(&mut self, block: &Split) -> Result<(), ExecutionError> {
        // this appends a row with END operation to the decoder trace. when END operation is
        // executed the rest of the VM state does not change
        self.decoder.end_control_block(block.hash().into());
        self.execute_op(Operation::Noop)
    }

    // LOOP BLOCK
    // --------------------------------------------------------------------------------------------

    /// Starts decoding a LOOP block. This also pops the value from the top of the stack and
    /// returns it.
    pub(super) fn start_loop_block(&mut self, block: &Loop) -> Result<Felt, ExecutionError> {
        let condition = self.stack.peek();

        // use the hasher to compute the hash of the LOOP block; for LOOP block there is no
        // second child so we set the second hash to ZEROs; the row address returned by the
        // hasher is used as the ID of the block; the result of the hash is expected to be in
        // row addr + 7.
        let body_hash = block.body().hash().into();
        let (addr, _result) = self.hasher.hash_2to1(body_hash, [Felt::ZERO; 4]);

        // make sure the result computed by the hasher is the same as the expected block hash
        debug_assert_eq!(block.hash(), _result.into());

        // start decoding the LOOP block; this appends a row with LOOP operation to the decoder
        // trace, but if the value on the top of the stack is not ONE, the block is not marked
        // as the loop block, and the hash of the body will not be added to the block hash table.
        // basically, if the top of the stack is ZERO, a LOOP operation should be immediately
        // followed by an END operation.
        self.decoder.start_loop(body_hash, addr, condition);
        self.execute_op(Operation::Drop)?;
        Ok(condition)
    }

    /// Ends decoding of a LOOP block. If pop_stack is set to true, this also removes the
    /// value at the top of the stack.
    pub(super) fn end_loop_block(
        &mut self,
        block: &Loop,
        pop_stack: bool,
    ) -> Result<(), ExecutionError> {
        // this appends a row with END operation to the decoder trace.
        self.decoder.end_control_block(block.hash().into());

        // if we are exiting a loop, we also need to pop the top value off the stack (and this
        // value must be ZERO - otherwise, we should have stayed in the loop). but, if we never
        // entered the loop in the first place, the stack would have been popped when the LOOP
        // operation was executed.
        if pop_stack {
            #[cfg(debug_assertions)]
            let condition = self.stack.peek();
            debug_assert_eq!(Felt::ZERO, condition);

            self.execute_op(Operation::Drop)
        } else {
            self.execute_op(Operation::Noop)
        }
    }

    // SPAN BLOCK
    // --------------------------------------------------------------------------------------------

    /// Starts decoding a SPAN block.
    pub(super) fn start_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        // use the hasher to compute the hash of the SPAN block; the row address returned by the
        // hasher is used as the ID of the block; hash of a SPAN block is computed by sequentially
        // hashing operation batches. Thus, the result of the hash is expected to be in row
        // addr + (num_batches * 8) - 1.
        let op_batches = block.op_batches();
        let (addr, _result) = self.hasher.hash_span_block(op_batches);

        // make sure the result computed by the hasher is the same as the expected block hash
        debug_assert_eq!(block.hash(), _result.into());

        // start decoding the first operation batch; this also appends a row with SPAN operation
        // to th decoder trace. we also need the total number of operation groups so that we can
        // set the value of the group_count register at the beginning of the SPAN.
        let num_op_groups = Felt::new((op_batches.len() * OP_BATCH_SIZE) as u64);
        self.decoder.start_span(&op_batches[0], num_op_groups, addr);
        self.execute_op(Operation::Noop)
    }

    /// Ends decoding a SPAN block.
    pub(super) fn end_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        // this appends a row with END operation to the decoder trace. when END operation is
        // executed the rest of the VM state does not change
        self.decoder.end_span(block.hash().into());
        self.execute_op(Operation::Noop)
    }
}

// DECODER
// ================================================================================================
/// TODO: add docs
///
///  addr  b0  b1  b2  b3  b4  b5  b6 in_span  h0  h1  h2  h3  h4  h5  h6  h7 g_count op_idx
/// ├────┴───┴───┴───┴───┴───┴───┴───┴───────┴───┴───┴───┴───┴───┴───┴───┴───┴───────┴───────┤
///
pub struct Decoder {
    block_stack: BlockStack,
    span_context: Option<SpanContext>,
    trace: DecoderTrace,
}

impl Decoder {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns an empty instance of [Decoder].
    pub fn new() -> Self {
        Self {
            block_stack: BlockStack::new(),
            span_context: None,
            trace: DecoderTrace::new(),
        }
    }

    // CONTROL BLOCKS
    // --------------------------------------------------------------------------------------------

    /// Starts decoding of a JOIN block.
    ///
    /// This pushes a block with ID=addr onto the block stack and appending execution of a JOIN
    /// operation to the trace.
    pub fn start_join(&mut self, left_child_hash: Word, right_child_hash: Word, addr: Felt) {
        let parent_addr = self.block_stack.push(addr, Felt::ZERO);
        self.trace.append_block_start(
            parent_addr,
            Operation::Join,
            left_child_hash,
            right_child_hash,
        );
    }

    /// Starts decoding of a SPLIT block.
    ///
    /// This pushes a block with ID=addr onto the block stack and appending execution of a SPLIT
    /// operation to the trace.
    pub fn start_split(&mut self, left_child_hash: Word, right_child_hash: Word, addr: Felt) {
        let parent_addr = self.block_stack.push(addr, Felt::ZERO);
        self.trace.append_block_start(
            parent_addr,
            Operation::Split,
            left_child_hash,
            right_child_hash,
        );
    }

    /// Starts decoding of a LOOP block.
    ///
    /// This pushes a block with ID=addr onto the block stack and appending execution of a LOOP
    /// operation to the trace. A block is marked as a loop block only if is_loop = ONE.
    pub fn start_loop(&mut self, loop_body_hash: Word, addr: Felt, is_loop: Felt) {
        let parent_addr = self.block_stack.push(addr, is_loop);
        self.trace.append_block_start(
            parent_addr,
            Operation::Loop,
            loop_body_hash,
            [Felt::ZERO; 4],
        );
    }

    /// Starts decoding another iteration of a loop.
    ///
    /// This appending execution of a REPEAT operation to the trace.
    pub fn repeat(&mut self) {
        let block_info = self.block_stack.peek();
        debug_assert_eq!(Felt::ONE, block_info.is_loop);
        self.trace.append_loop_repeat(block_info.addr);
    }

    /// Ends decoding of a control block (i.e., a non-SPAN block).
    ///
    /// This appending execution of an END operation to the trace. The top block on the block
    /// stack is also popped.
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

    /// Starts decoding of a SPAN block defined by the specified operation batches.
    pub fn start_span(&mut self, first_op_batch: &OpBatch, num_op_groups: Felt, addr: Felt) {
        debug_assert!(self.span_context.is_none(), "already in span");
        let parent_addr = self.block_stack.push(addr, Felt::ZERO);

        // add a SPAN row to the trace
        self.trace
            .append_span_start(parent_addr, first_op_batch.groups(), num_op_groups);

        // after SPAN operation is executed, we decrement the number of remaining groups by the
        // number of unused groups in the batch + 1. We add one because executing SPAN consumes
        // the first group of the batch.
        let num_unused_groups = get_num_unused_groups(first_op_batch);
        let consumed_op_groups = Felt::from(num_unused_groups + 1);

        self.span_context = Some(SpanContext {
            num_groups_left: num_op_groups - consumed_op_groups,
            group_ops_left: first_op_batch.groups()[0],
        });
    }

    /// Starts decoding of the next operation batch in the current SPAN.
    pub fn respan(&mut self, op_batch: &OpBatch) {
        // add RESPAN row to the trace
        self.trace.append_respan(op_batch.groups());

        // we also need to increment block address by 8 because hashing every additional operation
        // batch requires 8 rows of the hasher trace.
        let block_info = self.block_stack.peek_mut();
        block_info.addr += HASHER_CYCLE_LEN;

        // after RESPAN operation is executed, we decrement the number of remaining groups by the
        // number of unused groups in the batch + 1. We add one because executing RESPAN consumes
        // the first group of the batch.
        let num_unused_groups = get_num_unused_groups(op_batch);
        let consumed_op_groups = Felt::from(num_unused_groups + 1);

        let ctx = self.span_context.as_mut().expect("not in span");
        ctx.num_groups_left -= consumed_op_groups;
        ctx.group_ops_left = op_batch.groups()[0];
    }

    /// Starts decoding a new operation group.
    pub fn start_op_group(&mut self, op_group: Felt) {
        let ctx = self.span_context.as_mut().expect("not in span");
        ctx.group_ops_left = op_group;
        ctx.num_groups_left -= Felt::ONE;
    }

    /// Decodes a user operation (i.e., not a control flow operation).
    pub fn execute_user_op(&mut self, op: Operation, op_idx: usize) {
        debug_assert!(!op.is_decorator(), "op is a decorator");
        let block = self.block_stack.peek();
        let ctx = self.span_context.as_mut().expect("not in span");

        // update operations left to be executed in the group
        ctx.group_ops_left = remove_opcode_from_group(ctx.group_ops_left, op);

        // append the row for the operation to the trace
        self.trace.append_user_op(
            op,
            block.addr,
            block.parent_addr,
            ctx.num_groups_left,
            ctx.group_ops_left,
            Felt::from(op_idx as u32),
        );

        // if the operation carries an immediate value, decrement the number of  operation
        // groups left to decode. this number will be inserted into the trace in the next row
        if op.imm_value().is_some() {
            ctx.num_groups_left -= Felt::ONE
        }
    }

    /// Ends decoding of a SPAN block.
    pub fn end_span(&mut self, block_hash: Word) {
        let is_loop_body = self.block_stack.pop().is_loop_body;
        self.trace.append_span_end(block_hash, is_loop_body);
        self.span_context = None;
    }

    // TRACE GENERATIONS
    // --------------------------------------------------------------------------------------------

    /// Returns an columns of the execution trace for this decoder.
    ///
    /// The columns are extended to match the specified trace length.
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

// BLOCK STACK
// ================================================================================================

/// Keeps track of code blocks which are currently being executed by the VM.
struct BlockStack {
    blocks: Vec<BlockInfo>,
}

impl BlockStack {
    /// Returns an empty [BlockStack].
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    /// Pushes a new code block onto the block stack and returns the address of the block's parent.
    ///
    /// The block is identified by its address, and we also need to know whether this block is a
    /// LOOP block. Other information (i.e., the block's parent and whether the block is a body of
    /// a loop) is determined from the information already on the stack.
    pub fn push(&mut self, addr: Felt, is_loop: Felt) -> Felt {
        let (parent_addr, is_loop_body) = if self.blocks.is_empty() {
            // if the stack is empty, the new block has no parent and cannot be a body of a LOOP
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

    /// Removes a block from the top of the stack and returns it.
    pub fn pop(&mut self) -> BlockInfo {
        self.blocks.pop().expect("block stack is empty")
    }

    /// Returns a reference to a block at the top of the stack.
    pub fn peek(&self) -> &BlockInfo {
        self.blocks.last().expect("block stack is empty")
    }

    /// Returns a mutable reference to a block at the top of the stack.
    pub fn peek_mut(&mut self) -> &mut BlockInfo {
        self.blocks.last_mut().expect("block stack is empty")
    }
}

/// Contains basic information about a code block.
#[derive(Debug, Clone, Copy)]
struct BlockInfo {
    addr: Felt,
    parent_addr: Felt,
    is_loop_body: Felt,
    is_loop: Felt,
}

// SPAN CONTEXT
// ================================================================================================

/// Keeps track of the info needed to decode a currently executing SPAN block. The info includes:
/// - Operations which still need to be executed in the current group. The operations are
///   encoded as opcodes (7 bits) appended one after another into a single field element, with the
///   next operation to be executed located at the least significant position.
/// - Number of operation groups left to be executed in the entire SPAN block.
struct SpanContext {
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

/// Returns the number of unused operation groups in the specified batch.
///
/// The number of unused groups is computed as follows:
/// - Number of op groups in the batch is rounded to the next power of two. This is done because
///   the processor pads these op groups with NOOPs.
/// - Next, the number of op groups is subtracted from max batch size.
///
/// Thus, for example, if a batch contains 3 op groups, the number of unused op groups will be 4.
fn get_num_unused_groups(op_batch: &OpBatch) -> u8 {
    (OP_BATCH_SIZE - op_batch.num_groups().next_power_of_two()) as u8
}

/// Removes the specified operation from the op group and returns the resulting op group.
fn remove_opcode_from_group(op_group: Felt, op: Operation) -> Felt {
    let opcode = op.op_code().expect("no opcode") as u64;
    let result = Felt::new((op_group.as_int() - opcode) >> NUM_OP_BITS);
    debug_assert!(op_group.as_int() >= result.as_int(), "op group underflow");
    result
}
