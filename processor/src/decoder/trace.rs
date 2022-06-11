use super::{Felt, Operation, Word, MIN_TRACE_LEN, NUM_HASHER_COLUMNS, NUM_OP_BITS};
use std::ops::Range;
use vm_core::{program::blocks::OP_BATCH_SIZE, utils::new_array_vec, FieldElement, StarkField};

// CONSTANTS
// ================================================================================================

const ZERO: Felt = Felt::ZERO;
const ONE: Felt = Felt::ONE;

/// The range of columns in the decoder's `hasher_trace` which is available for use as helper
/// registers during user operations.
pub const USER_OP_HELPERS: Range<usize> = Range {
    start: 2,
    end: NUM_HASHER_COLUMNS,
};

// DECODER TRACE
// ================================================================================================

/// Execution trace of the decoder.
///
/// The trace currently consists of 19 columns grouped logically as follows:
/// - 1 column for code block ID / related hasher table row address.
/// - 7 columns for the binary representation of an opcode.
/// - 1 column for the flag indicating whether we are in a SPAN block or not.
/// - 8 columns used for providing inputs to, and reading results from the hasher, but also used
///   for other purposes when inside a SPAN block.
/// - 1 column to keep track of the number of operation groups left to decode in the current
///   SPAN block.
/// - 1 column to keep track of the index of a currently executing operation within an operation
///   group.
pub struct DecoderTrace {
    addr_trace: Vec<Felt>,
    op_bits_trace: [Vec<Felt>; NUM_OP_BITS],
    in_span_trace: Vec<Felt>,
    hasher_trace: [Vec<Felt>; NUM_HASHER_COLUMNS],
    group_count_trace: Vec<Felt>,
    op_idx_trace: Vec<Felt>,
}

impl DecoderTrace {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Initializes a blank [DecoderTrace].
    pub fn new() -> Self {
        Self {
            addr_trace: Vec::with_capacity(MIN_TRACE_LEN),
            op_bits_trace: new_array_vec(MIN_TRACE_LEN),
            in_span_trace: Vec::with_capacity(MIN_TRACE_LEN),
            hasher_trace: new_array_vec(MIN_TRACE_LEN),
            group_count_trace: Vec::with_capacity(MIN_TRACE_LEN),
            op_idx_trace: Vec::with_capacity(MIN_TRACE_LEN),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the current length of columns in this trace.
    pub fn trace_len(&self) -> usize {
        self.addr_trace.len()
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Appends a trace row marking the start of a flow control block (JOIN, SPLIT, LOOP).
    ///
    /// When a control block is starting, we do the following:
    /// - Set the address to the address of the parent block. This is not necessarily equal to the
    ///   address from the previous row because in a SPLIT block, the second child follows the
    ///   first child, rather than the parent.
    /// - Set op_bits to opcode of the specified block (e.g., JOIN, SPLIT, LOOP).
    /// - Set is_span to ZERO.
    /// - Set the first half of the hasher state to the h1 parameter. For JOIN and SPLIT blocks
    ///   this will contain the hash of the left child; for LOOP block this will contain hash of
    ///   the loop's body.
    /// - Set the second half of the hasher state to the h2 parameter. For JOIN and SPLIT blocks
    ///   this will contain hash of the right child.
    /// - Set op group count register to the ZERO.
    /// - Set operation index register to ZERO.
    pub fn append_block_start(&mut self, parent_addr: Felt, op: Operation, h1: Word, h2: Word) {
        self.addr_trace.push(parent_addr);
        self.append_opcode(op);
        self.in_span_trace.push(ZERO);

        self.hasher_trace[0].push(h1[0]);
        self.hasher_trace[1].push(h1[1]);
        self.hasher_trace[2].push(h1[2]);
        self.hasher_trace[3].push(h1[3]);

        self.hasher_trace[4].push(h2[0]);
        self.hasher_trace[5].push(h2[1]);
        self.hasher_trace[6].push(h2[2]);
        self.hasher_trace[7].push(h2[3]);

        self.group_count_trace.push(ZERO);
        self.op_idx_trace.push(ZERO);
    }

    /// Appends a trace row marking the end of a flow control block (JOIN, SPLIT, LOOP).
    ///
    /// When a control block is ending, we do the following:
    /// - Set the block address to the specified address.
    /// - Set op_bits to END opcode.
    /// - Set in_span to ZERO.
    /// - Put the provided block hash into the first 4 elements of the hasher state.
    /// - Set the remaining 4 elements of the hasher state to [is_loop_body, is_loop, 0, 0].
    /// - Copy over op group count from the previous row. This group count must be ZERO.
    /// - Set operation index register to ZERO.
    pub fn append_block_end(
        &mut self,
        block_addr: Felt,
        block_hash: Word,
        is_loop_body: Felt,
        is_loop: Felt,
    ) {
        debug_assert!(is_loop_body.as_int() <= 1, "invalid loop body");
        debug_assert!(is_loop.as_int() <= 1, "invalid is loop");

        self.addr_trace.push(block_addr);
        self.append_opcode(Operation::End);
        self.in_span_trace.push(ZERO);

        self.hasher_trace[0].push(block_hash[0]);
        self.hasher_trace[1].push(block_hash[1]);
        self.hasher_trace[2].push(block_hash[2]);
        self.hasher_trace[3].push(block_hash[3]);

        self.hasher_trace[4].push(is_loop_body);
        self.hasher_trace[5].push(is_loop);
        self.hasher_trace[6].push(ZERO);
        self.hasher_trace[7].push(ZERO);

        let last_group_count = self.last_group_count();
        debug_assert!(last_group_count == ZERO, "group count not zero");
        self.group_count_trace.push(last_group_count);

        self.op_idx_trace.push(ZERO);
    }

    /// Appends a trace row marking the beginning of a new loop iteration.
    ///
    /// When we start a new loop iteration, we do the following:
    /// - Set the block address to the address of the loop block.
    /// - Set op_bits to REPEAT opcode.
    /// - Set in_span to ZERO.
    /// - Copy over the hasher state from the previous row. Technically, we need to copy over
    ///   only the first 5 elements of the hasher state, but it is easier to copy over the whole
    ///   row.
    /// - Set op group count register to the ZERO.
    /// - Set operation index register to ZERO.
    pub fn append_loop_repeat(&mut self, loop_addr: Felt) {
        self.addr_trace.push(loop_addr);
        self.append_opcode(Operation::Repeat);
        self.in_span_trace.push(ZERO);

        let last_row = self.hasher_trace[0].len() - 1;
        for column in self.hasher_trace.iter_mut() {
            column.push(column[last_row]);
        }

        self.group_count_trace.push(ZERO);
        self.op_idx_trace.push(ZERO);
    }

    /// Appends a trace row marking the start of a SPAN block.
    ///
    /// When a SPAN block is starting, we do the following:
    /// - Set the address to the address of the parent block. This is not necessarily equal to the
    ///   address from the previous row because in a SPLIT block, the second child follows the
    ///   first child, rather than the parent.
    /// - Set op_bits to SPAN opcode.
    /// - Set is_span to ZERO. is_span will be set to one in the following row.
    /// - Set hasher state to op groups of the first op batch of the SPAN.
    /// - Set op group count to the total number of op groups in the SPAN.
    /// - Set operation index register to ZERO.
    pub fn append_span_start(
        &mut self,
        parent_addr: Felt,
        first_op_batch: &[Felt; OP_BATCH_SIZE],
        num_op_groups: Felt,
    ) {
        self.addr_trace.push(parent_addr);
        self.append_opcode(Operation::Span);
        self.in_span_trace.push(ZERO);
        for (i, &op_group) in first_op_batch.iter().enumerate() {
            self.hasher_trace[i].push(op_group);
        }
        self.group_count_trace.push(num_op_groups);
        self.op_idx_trace.push(ZERO);
    }

    /// Appends a trace row marking a RESPAN operation.
    ///
    /// When a RESPAN operation is executed, we do the following:
    /// - Copy over the block address from the previous row. The SPAN address will be updated in
    ///   the following row.
    /// - Set op_bits to RESPAN opcode.
    /// - Set in_span to ONE.
    /// - Set hasher state to op groups of the next op batch of the SPAN.
    /// - Copy over op group count from the previous row.
    /// - Set operation index register to ZERO.
    pub fn append_respan(&mut self, op_batch: &[Felt; OP_BATCH_SIZE]) {
        self.addr_trace.push(self.last_addr());
        self.append_opcode(Operation::Respan);
        self.in_span_trace.push(ONE);
        for (i, &op_group) in op_batch.iter().enumerate() {
            self.hasher_trace[i].push(op_group);
        }
        self.group_count_trace.push(self.last_group_count());
        self.op_idx_trace.push(ZERO);
    }

    /// Appends a trace row for a user operation.
    ///
    /// When we execute a user operation in a SPAN block, we do the following:
    /// - Set the address of the row to the address of the span block.
    /// - Set op_bits to the opcode of the executed operation.
    /// - Set is_span to ONE.
    /// - Set the first hasher state register to the aggregation of remaining operations to be
    ///   executed in the current operation group.
    /// - Set the second hasher state register to the address of the SPAN's parent block.
    /// - Set the remaining hasher state registers to ZEROs.
    /// - Set the number of groups remaining to be processed. This number of groups changes if
    ///   in the previous row an operation with an immediate value was executed or if this
    ///   operation is a start of a new operation group.
    /// - Set the operation's index within the current operation group.
    pub fn append_user_op(
        &mut self,
        op: Operation,
        span_addr: Felt,
        parent_addr: Felt,
        num_groups_left: Felt,
        group_ops_left: Felt,
        op_idx: Felt,
    ) {
        self.addr_trace.push(span_addr);
        self.append_opcode(op);
        self.in_span_trace.push(ONE);

        self.hasher_trace[0].push(group_ops_left);
        self.hasher_trace[1].push(parent_addr);

        for idx in USER_OP_HELPERS {
            self.hasher_trace[idx].push(ZERO);
        }

        self.group_count_trace.push(num_groups_left);
        self.op_idx_trace.push(op_idx);
    }

    /// Appends a trace row marking the end of a SPAN block.
    ///
    /// When the SPAN block is ending, we do the following:
    /// - Copy over the block address from the previous row.
    /// - Set op_bits to END opcode.
    /// - Set in_span to ZERO to indicate that the span block is completed.
    /// - Put the hash of the span block into the first 4 registers of the hasher state.
    /// - Put a flag indicating whether the SPAN block was a body of a loop into the 5th
    ///   register of the hasher state.
    /// - Copy over op group count from the previous row. This group count must be ZERO.
    /// - Set operation index register to ZERO.
    pub fn append_span_end(&mut self, span_hash: Word, is_loop_body: Felt) {
        debug_assert!(is_loop_body.as_int() <= 1, "invalid loop body");

        self.addr_trace.push(self.last_addr());
        self.append_opcode(Operation::End);
        self.in_span_trace.push(ZERO);

        self.hasher_trace[0].push(span_hash[0]);
        self.hasher_trace[1].push(span_hash[1]);
        self.hasher_trace[2].push(span_hash[2]);
        self.hasher_trace[3].push(span_hash[3]);

        // we don't need to set is_loop here because we know we are not in a loop block
        self.hasher_trace[4].push(is_loop_body);
        self.hasher_trace[5].push(ZERO);
        self.hasher_trace[6].push(ZERO);
        self.hasher_trace[7].push(ZERO);

        let last_group_count = self.last_group_count();
        debug_assert!(last_group_count == ZERO, "group count not zero");
        self.group_count_trace.push(last_group_count);

        self.op_idx_trace.push(ZERO);
    }

    // TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Returns vector of columns of this execution trace.
    ///
    /// Each column in the trace is extended to the specified trace length. The extension is done
    /// by inserting ZEROs into the unfilled rows of most columns. The only exceptions are:
    /// - The op_bits columns, where the unfilled rows are filled with the opcode of the HALT
    ///   operation.
    /// - The first 4 columns of the hasher state, where the unfilled rows are filled with the
    ///   values from the last filled row. This is done so that the hash of the program is
    ///   propagated to the last row.
    pub fn into_vec(mut self, trace_len: usize, num_rand_rows: usize) -> Vec<Vec<Felt>> {
        let own_len = self.trace_len();
        // make sure that only the duplicate rows will be overwritten with random values
        assert!(
            own_len + num_rand_rows <= trace_len,
            "target trace length too small"
        );

        let mut trace = Vec::new();

        // put ZEROs into the unfilled rows of block address column
        self.addr_trace.resize(trace_len, ZERO);
        trace.push(self.addr_trace);

        // insert HALT opcode into the unfilled rows of op_bits columns
        let halt_opcode = Operation::Halt.op_code().expect("missing opcode");
        for (i, mut column) in self.op_bits_trace.into_iter().enumerate() {
            debug_assert_eq!(own_len, column.len());
            let value = Felt::from((halt_opcode >> i) & 1);
            column.resize(trace_len, value);
            trace.push(column);
        }

        // put ZEROs into the unfilled rows of in_span column
        debug_assert_eq!(own_len, self.in_span_trace.len());
        self.in_span_trace.resize(trace_len, ZERO);
        trace.push(self.in_span_trace);

        // for unfilled rows of hasher state columns, copy over values from the last row for the
        // first 4 columns, and pad the other 4 columns with ZEROs
        for (i, mut column) in self.hasher_trace.into_iter().enumerate() {
            debug_assert_eq!(own_len, column.len());
            if i < 4 {
                let last_value = *column.last().expect("no last hasher trace value");
                column.resize(trace_len, last_value);
            } else {
                column.resize(trace_len, ZERO);
            }
            trace.push(column);
        }

        // put ZEROs into the unfilled rows of operation group count column
        debug_assert_eq!(own_len, self.group_count_trace.len());
        self.group_count_trace.resize(trace_len, ZERO);
        trace.push(self.group_count_trace);

        // put ZEROs into the unfilled rows of operation index column
        debug_assert_eq!(own_len, self.op_idx_trace.len());
        self.op_idx_trace.resize(trace_len, ZERO);
        trace.push(self.op_idx_trace);

        trace
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    /// Returns the last block address.
    fn last_addr(&self) -> Felt {
        *self.addr_trace.last().expect("no last addr")
    }

    /// Returns the last value of the operation group count.
    fn last_group_count(&self) -> Felt {
        *self.group_count_trace.last().expect("no group count")
    }

    /// Returns a reference to the last value in the helper register at the specified index.
    fn last_helper_mut(&mut self, idx: usize) -> &mut Felt {
        debug_assert!(idx < USER_OP_HELPERS.len(), "invalid helper register index");

        self.hasher_trace[USER_OP_HELPERS.start + idx]
            .last_mut()
            .expect("no last helper value")
    }

    /// Populates op_bits registers for the next row with the opcode of the provided operation.
    fn append_opcode(&mut self, op: Operation) {
        let op_code = op.op_code().expect("missing opcode");
        for i in 0..NUM_OP_BITS {
            let bit = Felt::from((op_code >> i) & 1);
            self.op_bits_trace[i].push(bit);
        }
    }

    /// Add all provided values to the helper registers in the order provided, starting from the
    /// first hasher register that is available for user operation helper values.
    ///
    /// The specified USER_OP_HELPERS in the `hasher_trace` are used as helper registers, since they
    /// are not required for hashing during execution of user operations.
    pub fn set_user_op_helpers(&mut self, values: &[Felt]) {
        assert!(
            values.len() <= USER_OP_HELPERS.len(),
            "too many values for helper columns"
        );

        for (idx, value) in values.iter().enumerate() {
            *self.last_helper_mut(idx) = *value;
        }
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    /// Adds a new decoder trace row of zeros for testing purposes.
    #[cfg(test)]
    pub fn add_dummy_row(&mut self) {
        self.addr_trace.push(ZERO);
        for column in self.op_bits_trace.iter_mut() {
            column.push(ZERO);
        }
        self.in_span_trace.push(ZERO);
        for column in self.hasher_trace.iter_mut() {
            column.push(ZERO);
        }
        self.group_count_trace.push(ZERO);
        self.op_idx_trace.push(ZERO);
    }
}
