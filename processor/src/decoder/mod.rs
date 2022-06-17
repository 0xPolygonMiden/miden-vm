use super::{
    ExecutionError, Felt, FieldElement, Join, Loop, OpBatch, Operation, Process, Span, Split,
    StarkField, Vec, Word, MIN_TRACE_LEN, OP_BATCH_SIZE,
};
use vm_core::decoder::NUM_HASHER_COLUMNS;

mod trace;
use trace::DecoderTrace;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const NUM_OP_BITS: usize = Operation::OP_BITS;
const HASH_CYCLE_LEN: Felt = Felt::new(vm_core::hasher::HASH_CYCLE_LEN as u64);

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
        let (addr, _result) = self.hasher.merge(child1_hash, child2_hash);

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
        let (addr, _result) = self.hasher.merge(child1_hash, child2_hash);

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
        let (addr, _result) = self.hasher.merge(body_hash, [Felt::ZERO; 4]);

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
            {
                let condition = self.stack.peek();
                debug_assert_eq!(Felt::ZERO, condition);
            }

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
        // to the decoder trace. we also need the total number of operation groups so that we can
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

/// Program decoder for the VM.
///
/// This component is responsible for decoding operations executed on the VM, computing the hash
/// of the executed program, as well as building an execution trace for these computations.
///
/// ## Execution trace
/// Decoder execution trace currently consists of 19 columns as illustrated below (this will
/// be increased to 24 columns in the future):
///
///  addr  b0  b1  b2  b3  b4  b5  b6 in_span  h0  h1  h2  h3  h4  h5  h6  h7 g_count op_idx
/// ├────┴───┴───┴───┴───┴───┴───┴───┴───────┴───┴───┴───┴───┴───┴───┴───┴───┴───────┴───────┤
///
/// In the above, the meaning of the columns is as follows:
/// * addr column contains address of the hasher for the current block (row index from the
///   auxiliary hashing table). It also serves the role of unique block identifiers. This is
///   convenient, because hasher addresses are guaranteed to be unique.
/// * op_bits columns b0 through b6 are used to encode an operation to be executed by the VM.
///   Each of these columns contains a single binary value, which together form a single opcode.
/// * in_span column is a binary flag set to ONE when we are inside a SPAN block, and to ZERO
///   otherwise.
/// * Hasher state columns h0 through h7. These are multi purpose columns used as follows:
///   - When starting decoding of a new code block (e.g., via JOIN, SPLIT, LOOP, SPAN operations)
///    these columns are used for providing inputs for the current block's hash computations.
///   - When finishing decoding of a code block (i.e., via END operation), these columns are
///     used to record the result of the hash computation.
///   - Inside a SPAN block, the first two columns are used to keep track of un-executed
///     operations in the current operation group, as well as the address of the parent code
///     block. The remaining 6 columns are unused by the decoder and, thus, can be used by the
///     VM as helper columns.
/// * operation group count column is used to keep track of the number of un-executed operation
///   groups in the current SPAN block.
/// * operation index column is used to keep track of the indexes of the currently executing
///   operations within an operation group. Values in this column could be between 0 and 8
///   (both inclusive) as there could be at most 9 operations in an operation group.
///
/// Also keeps track of operations executed when run in debug mode.
pub struct Decoder {
    block_stack: BlockStack,
    span_context: Option<SpanContext>,
    trace: DecoderTrace,
    operations: Vec<Operation>,
    in_debug_mode: bool,
}

impl Decoder {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns an empty instance of [Decoder].
    pub fn new(in_debug_mode: bool) -> Self {
        Self {
            block_stack: BlockStack::new(),
            span_context: None,
            trace: DecoderTrace::new(),
            operations: Vec::<Operation>::new(),
            in_debug_mode,
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

        self.append_operation(Operation::Join);
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

        self.append_operation(Operation::Split);
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

        self.append_operation(Operation::Loop);
    }

    /// Starts decoding another iteration of a loop.
    ///
    /// This appends an execution of a REPEAT operation to the trace.
    pub fn repeat(&mut self) {
        let block_info = self.block_stack.peek();
        debug_assert_eq!(Felt::ONE, block_info.is_loop);
        self.trace.append_loop_repeat(block_info.addr);

        self.append_operation(Operation::Repeat);
    }

    /// Ends decoding of a control block (i.e., a non-SPAN block).
    ///
    /// This appends an execution of an END operation to the trace. The top block on the block
    /// stack is also popped.
    pub fn end_control_block(&mut self, block_hash: Word) {
        let block_info = self.block_stack.pop();
        self.trace.append_block_end(
            block_info.addr,
            block_hash,
            block_info.is_loop_body,
            block_info.is_loop,
        );

        self.append_operation(Operation::End);
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

        self.append_operation(Operation::Span);
    }

    /// Starts decoding of the next operation batch in the current SPAN.
    pub fn respan(&mut self, op_batch: &OpBatch) {
        // add RESPAN row to the trace
        self.trace.append_respan(op_batch.groups());

        // we also need to increment block address by 8 because hashing every additional operation
        // batch requires 8 rows of the hasher trace.
        let block_info = self.block_stack.peek_mut();
        block_info.addr += HASH_CYCLE_LEN;

        // after RESPAN operation is executed, we decrement the number of remaining groups by the
        // number of unused groups in the batch + 1. We add one because executing RESPAN consumes
        // the first group of the batch.
        let num_unused_groups = get_num_unused_groups(op_batch);
        let consumed_op_groups = Felt::from(num_unused_groups + 1);

        let ctx = self.span_context.as_mut().expect("not in span");
        ctx.num_groups_left -= consumed_op_groups;
        ctx.group_ops_left = op_batch.groups()[0];

        self.append_operation(Operation::Respan);
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

        self.append_operation(op);
    }

    /// Sets the helper registers in the trace to the user-provided helper values. This is expected
    /// to be called during the execution of a user operation.
    ///
    /// TODO: it might be better to get the operation information from the decoder trace, rather
    /// than passing it in as a parameter.
    pub fn set_user_op_helpers(&mut self, op: Operation, values: &[Felt]) {
        debug_assert!(!op.is_control_op(), "op is a control operation");
        self.trace.set_user_op_helpers(values);
    }

    /// Ends decoding of a SPAN block.
    pub fn end_span(&mut self, block_hash: Word) {
        let is_loop_body = self.block_stack.pop().is_loop_body;
        self.trace.append_span_end(block_hash, is_loop_body);
        self.span_context = None;

        self.append_operation(Operation::End);
    }

    /// Get operation at a particular clock cycle. Only applicable in debug mode.
    pub fn get_operation_at(&self, clk: usize) -> Operation {
        self.operations[clk]
    }

    // TRACE GENERATIONS
    // --------------------------------------------------------------------------------------------

    /// Returns an array of columns containing an execution trace of this decoder.
    ///
    /// The columns are extended to match the specified trace length.
    pub fn into_trace(self, trace_len: usize, num_rand_rows: usize) -> super::DecoderTrace {
        self.trace
            .into_vec(trace_len, num_rand_rows)
            .try_into()
            .expect("failed to convert vector to array")
    }

    // TRACE GENERATIONS
    // --------------------------------------------------------------------------------------------

    /// Adds an operation to the operations vector in debug mode.
    #[inline(always)]
    fn append_operation(&mut self, op: Operation) {
        if self.in_debug_mode {
            self.operations.push(op);
        }
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    /// Adds a row of zeros to the decoder trace for testing purposes.
    #[cfg(test)]
    pub fn add_dummy_trace_row(&mut self) {
        self.trace.add_dummy_row();
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new(false)
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
