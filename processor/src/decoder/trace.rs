use alloc::vec::Vec;
use core::ops::Range;

#[cfg(test)]
use miden_air::trace::decoder::NUM_USER_OP_HELPERS;
use miden_air::trace::{
    decoder::{
        ADDR_COL_IDX, GROUP_COUNT_COL_IDX, HASHER_STATE_OFFSET, IN_SPAN_COL_IDX,
        OP_BATCH_FLAGS_OFFSET, OP_BITS_EXTRA_COLS_OFFSET, OP_BITS_OFFSET, OP_INDEX_COL_IDX,
    },
    DECODER_TRACE_WIDTH,
};
use vm_core::{utils::uninit_vector, OPCODE_HALT};

use super::{
    get_num_groups_in_next_batch, Felt, Operation, Word, DIGEST_LEN, MIN_TRACE_LEN,
    NUM_HASHER_COLUMNS, NUM_OP_BATCH_FLAGS, NUM_OP_BITS, NUM_OP_BITS_EXTRA_COLS, ONE,
    OP_BATCH_1_GROUPS, OP_BATCH_2_GROUPS, OP_BATCH_4_GROUPS, OP_BATCH_8_GROUPS, OP_BATCH_SIZE,
    ZERO,
};

// CONSTANTS
// ================================================================================================

/// The range of columns in the decoder's `hasher_trace` which is available for use as helper
/// registers during user operations.
pub const USER_OP_HELPERS: Range<usize> = Range { start: 2, end: NUM_HASHER_COLUMNS };

// DECODER TRACE
// ================================================================================================

struct DecoderTraceRow {
    addr: Felt,
    op_bits: [Felt; NUM_OP_BITS],
    op_bit_extra: [Felt; NUM_OP_BITS_EXTRA_COLS],
    hasher: [Felt; NUM_HASHER_COLUMNS],
    in_span: Felt,
    group_count: Felt,
    op_idx: Felt,
    op_batch_flags: [Felt; NUM_OP_BATCH_FLAGS],
}

/// Execution trace of the decoder.
///
/// The trace currently consists of 24 columns grouped logically as follows:
/// - 1 column for code block ID / related hasher table row address.
/// - 7 columns for the binary representation of an opcode.
/// - 8 columns used for providing inputs to, and reading results from the hasher, but also used for
///   other purposes when inside a SPAN block.
/// - 1 column for the flag indicating whether we are in a SPAN block or not.
/// - 1 column to keep track of the number of operation groups left to decode in the current SPAN
///   block.
/// - 1 column to keep track of the index of a currently executing operation within an operation
///   group.
/// - 3 columns for keeping track of operation batch flags.
/// - 2 columns used for op flag degree reduction (to support degree 4 and 5 operations).
pub struct DecoderTrace {
    rows: Vec<DecoderTraceRow>,
}

impl DecoderTrace {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Initializes a blank [DecoderTrace].
    pub fn new() -> Self {
        Self { rows: Vec::with_capacity(MIN_TRACE_LEN) }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the current length of columns in this trace.
    pub fn trace_len(&self) -> usize {
        self.rows.len()
    }

    /// Returns the contents of the first 4 registers of the hasher state at the last row.
    pub fn program_hash(&self) -> [Felt; DIGEST_LEN] {
        self.last_hasher()[0..DIGEST_LEN].try_into().unwrap()
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Appends a trace row marking the start of a flow control block (JOIN, SPLIT, LOOP, CALL,
    /// SYSCALL, DYN, DYNCALL).
    ///
    /// When a control block is starting, we do the following:
    /// - Set the address to the address of the parent block. This is not necessarily equal to the
    ///   address from the previous row because in a SPLIT block, the second child follows the first
    ///   child, rather than the parent.
    /// - Set op_bits to opcode of the specified block (e.g., JOIN, SPLIT, LOOP, CALL, SYSCALL, DYN,
    ///   DYNCALL).
    /// - Set the first half of the hasher state to the h1 parameter. For JOIN and SPLIT blocks this
    ///   will contain the hash of the left child; for LOOP block this will contain hash of the
    ///   loop's body, for CALL, SYSCALL, DYN and DYNCALL blocks this will contain hash of the
    ///   called function.
    /// - Set the second half of the hasher state to the h2 parameter. For JOIN and SPLIT blocks
    ///   this will contain hash of the right child.
    /// - Set is_span to ZERO.
    /// - Set op group count register to the ZERO.
    /// - Set operation index register to ZERO.
    /// - Set op_batch_flags to ZEROs.
    pub fn append_block_start(&mut self, parent_addr: Felt, op: Operation, h1: Word, h2: Word) {
        let (op_bits, op_bit_extra) = get_op_bits(op.op_code());
        let hasher = [h1[0], h1[1], h1[2], h1[3], h2[0], h2[1], h2[2], h2[3]];

        let row = DecoderTraceRow {
            addr: parent_addr,
            op_bits,
            op_bit_extra,
            hasher,
            in_span: ZERO,
            group_count: ZERO,
            op_idx: ZERO,
            op_batch_flags: [ZERO, ZERO, ZERO],
        };

        // TODO(plafer): Don't push on every row (applies everywhere).
        // Can we also write directly to the buffer instead of needing this intermediary `rows`?
        self.rows.push(row);
    }

    /// Appends a trace row marking the end of a flow control block (JOIN, SPLIT, LOOP, CALL,
    /// SYSCALL).
    ///
    /// When a control block is ending, we do the following:
    /// - Set the block address to the specified address.
    /// - Set op_bits to END opcode.
    /// - Put the provided block hash into the first 4 elements of the hasher state.
    /// - Set the remaining 4 elements of the hasher state to [is_loop_body, is_loop, is_call,
    ///   is_syscall].
    /// - Set in_span to ZERO.
    /// - Copy over op group count from the previous row. This group count must be ZERO.
    /// - Set operation index register to ZERO.
    /// - Set op_batch_flags to ZEROs.
    pub fn append_block_end(
        &mut self,
        block_addr: Felt,
        block_hash: Word,
        is_loop_body: Felt,
        is_loop: Felt,
        is_call: Felt,
        is_syscall: Felt,
    ) {
        debug_assert!(is_loop_body.as_int() <= 1, "invalid is_loop_body");
        debug_assert!(is_loop.as_int() <= 1, "invalid is_loop");
        debug_assert!(is_call.as_int() <= 1, "invalid is_call");
        debug_assert!(is_syscall.as_int() <= 1, "invalid is_syscall");

        let (op_bits, op_bit_extra) = get_op_bits(Operation::End.op_code());
        let hasher = [
            block_hash[0],
            block_hash[1],
            block_hash[2],
            block_hash[3],
            is_loop_body,
            is_loop,
            is_call,
            is_syscall,
        ];
        let last_group_count = self.last_group_count();
        debug_assert!(last_group_count == ZERO, "group count not zero");

        let row = DecoderTraceRow {
            addr: block_addr,
            op_bits,
            op_bit_extra,
            hasher,
            in_span: ZERO,
            group_count: last_group_count,
            op_idx: ZERO,
            op_batch_flags: [ZERO, ZERO, ZERO],
        };

        self.rows.push(row);
    }

    /// Appends a trace row marking the beginning of a new loop iteration.
    ///
    /// When we start a new loop iteration, we do the following:
    /// - Set the block address to the address of the loop block.
    /// - Set op_bits to REPEAT opcode.
    /// - Copy over the hasher state from the previous row. Technically, we need to copy over only
    ///   the first 5 elements of the hasher state, but it is easier to copy over the whole row.
    /// - Set in_span to ZERO.
    /// - Set op group count register to the ZERO.
    /// - Set operation index register to ZERO.
    /// - Set op_batch_flags to ZEROs.
    pub fn append_loop_repeat(&mut self, loop_addr: Felt) {
        let (op_bits, op_bit_extra) = get_op_bits(Operation::Repeat.op_code());

        let row = DecoderTraceRow {
            addr: loop_addr,
            op_bits,
            op_bit_extra,
            hasher: self.last_hasher(),
            in_span: ZERO,
            group_count: ZERO,
            op_idx: ZERO,
            op_batch_flags: [ZERO, ZERO, ZERO],
        };

        self.rows.push(row);
    }

    /// Appends a trace row marking the start of a SPAN block.
    ///
    /// When a SPAN block is starting, we do the following:
    /// - Set the address to the address of the parent block. This is not necessarily equal to the
    ///   address from the previous row because in a SPLIT block, the second child follows the first
    ///   child, rather than the parent.
    /// - Set op_bits to SPAN opcode.
    /// - Set hasher state to op groups of the first op batch of the SPAN.
    /// - Set is_span to ZERO. is_span will be set to one in the following row.
    /// - Set op group count to the total number of op groups in the SPAN.
    /// - Set operation index register to ZERO.
    /// - Set the op_batch_flags based on the specified number of operation groups.
    pub fn append_span_start(
        &mut self,
        parent_addr: Felt,
        first_op_batch: &[Felt; OP_BATCH_SIZE],
        num_op_groups: Felt,
    ) {
        let (op_bits, op_bit_extra) = get_op_bits(Operation::Span.op_code());

        let row = DecoderTraceRow {
            addr: parent_addr,
            op_bits,
            op_bit_extra,
            hasher: *first_op_batch,
            in_span: ZERO,
            group_count: num_op_groups,
            op_idx: ZERO,
            op_batch_flags: get_op_batch_flags(num_op_groups),
        };

        self.rows.push(row);
    }

    /// Appends a trace row marking a RESPAN operation.
    ///
    /// When a RESPAN operation is executed, we do the following:
    /// - Copy over the block address from the previous row. The SPAN address will be updated in the
    ///   following row.
    /// - Set op_bits to RESPAN opcode.
    /// - Set hasher state to op groups of the next op batch of the SPAN.
    /// - Set in_span to ZERO.
    /// - Copy over op group count from the previous row.
    /// - Set operation index register to ZERO.
    /// - Set the op_batch_flags based on the current operation group count.
    pub fn append_respan(&mut self, op_batch: &[Felt; OP_BATCH_SIZE]) {
        let (op_bits, op_bit_extra) = get_op_bits(Operation::Respan.op_code());
        let group_count = self.last_group_count();

        let row = DecoderTraceRow {
            addr: self.last_addr(),
            op_bits,
            op_bit_extra,
            hasher: *op_batch,
            in_span: ZERO,
            group_count,
            op_idx: ZERO,
            op_batch_flags: get_op_batch_flags(group_count),
        };

        self.rows.push(row);
    }

    /// Appends a trace row for a user operation.
    ///
    /// When we execute a user operation in a SPAN block, we do the following:
    /// - Set the address of the row to the address of the span block.
    /// - Set op_bits to the opcode of the executed operation.
    /// - Set the first hasher state register to the aggregation of remaining operations to be
    ///   executed in the current operation group.
    /// - Set the second hasher state register to the address of the SPAN's parent block.
    /// - Set the remaining hasher state registers to ZEROs.
    /// - Set is_span to ONE.
    /// - Set the number of groups remaining to be processed. This number of groups changes if in
    ///   the previous row an operation with an immediate value was executed or if this operation is
    ///   a start of a new operation group.
    /// - Set the operation's index within the current operation group.
    /// - Set op_batch_flags to ZEROs.
    pub fn append_user_op(
        &mut self,
        op: Operation,
        basic_block_addr: Felt,
        parent_addr: Felt,
        num_groups_left: Felt,
        group_ops_left: Felt,
        op_idx: Felt,
    ) {
        let (op_bits, op_bit_extra) = get_op_bits(op.op_code());
        // Note: use `Decoder::set_user_op_helpers()` when processing an instruction to set any of
        // these values to something other than 0
        let hasher = [group_ops_left, parent_addr, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO];

        let row = DecoderTraceRow {
            addr: basic_block_addr,
            op_bits,
            op_bit_extra,
            hasher,
            in_span: ONE,
            group_count: num_groups_left,
            op_idx,
            op_batch_flags: [ZERO, ZERO, ZERO],
        };

        self.rows.push(row);
    }

    /// Appends a trace row marking the end of a SPAN block.
    ///
    /// When the SPAN block is ending, we do the following:
    /// - Copy over the block address from the previous row.
    /// - Set op_bits to END opcode.
    /// - Put the hash of the span block into the first 4 registers of the hasher state.
    /// - Put a flag indicating whether the SPAN block was a body of a loop into the 5th register of
    ///   the hasher state.
    /// - Set in_span to ZERO to indicate that the span block is completed.
    /// - Copy over op group count from the previous row. This group count must be ZERO.
    /// - Set operation index register to ZERO.
    /// - Set op_batch_flags to ZEROs.
    pub fn append_span_end(&mut self, span_hash: Word, is_loop_body: Felt) {
        debug_assert!(is_loop_body.as_int() <= 1, "invalid loop body");
        let (op_bits, op_bit_extra) = get_op_bits(Operation::End.op_code());
        let hasher = [
            span_hash[0],
            span_hash[1],
            span_hash[2],
            span_hash[3],
            is_loop_body,
            ZERO,
            ZERO,
            ZERO,
        ];
        let last_group_count = self.last_group_count();
        debug_assert!(last_group_count == ZERO, "group count not zero");

        let row = DecoderTraceRow {
            addr: self.last_addr(),
            op_bits,
            op_bit_extra,
            hasher,
            in_span: ZERO,
            group_count: last_group_count,
            op_idx: ZERO,
            op_batch_flags: [ZERO, ZERO, ZERO],
        };

        self.rows.push(row);
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
    pub fn into_vec(self, trace_len: usize, num_rand_rows: usize) -> Vec<Vec<Felt>> {
        let own_len = self.trace_len();
        // make sure that only the duplicate rows will be overwritten with random values
        assert!(own_len + num_rand_rows <= trace_len, "target trace length too small");

        let mut trace_columns = unsafe { vec![uninit_vector(trace_len); DECODER_TRACE_WIDTH] };

        for (i, row) in self.rows.into_iter().enumerate() {
            trace_columns[ADDR_COL_IDX][i] = row.addr;
            for j in 0..NUM_OP_BITS {
                trace_columns[OP_BITS_OFFSET + j][i] = row.op_bits[j];
            }
            for j in 0..NUM_HASHER_COLUMNS {
                trace_columns[HASHER_STATE_OFFSET + j][i] = row.hasher[j];
            }
            trace_columns[IN_SPAN_COL_IDX][i] = row.in_span;
            trace_columns[GROUP_COUNT_COL_IDX][i] = row.group_count;
            trace_columns[OP_INDEX_COL_IDX][i] = row.op_idx;
            for j in 0..NUM_OP_BATCH_FLAGS {
                trace_columns[OP_BATCH_FLAGS_OFFSET + j][i] = row.op_batch_flags[j];
            }

            for j in 0..NUM_OP_BITS_EXTRA_COLS {
                trace_columns[OP_BITS_EXTRA_COLS_OFFSET + j][i] = row.op_bit_extra[j];
            }
        }

        // padding rows

        // put ZEROs into the unfilled rows of block address column
        trace_columns[ADDR_COL_IDX][own_len..trace_len].fill(ZERO);

        // insert HALT opcode into the unfilled rows of op_bits columns
        let halt_opcode = Operation::Halt.op_code();
        for j in 0..NUM_OP_BITS {
            trace_columns[OP_BITS_OFFSET + j][own_len..trace_len]
                .fill(Felt::from((halt_opcode >> j) & 1));
        }

        // for unfilled rows of hasher state columns, copy over values from the last row for the
        // first 4 columns, and pad the other 4 columns with ZEROs
        for j in 0..NUM_HASHER_COLUMNS / 2 {
            let last_value = trace_columns[HASHER_STATE_OFFSET + j][own_len - 1];
            trace_columns[HASHER_STATE_OFFSET + j][own_len..trace_len].fill(last_value);
        }
        for j in NUM_HASHER_COLUMNS / 2..NUM_HASHER_COLUMNS {
            trace_columns[HASHER_STATE_OFFSET + j][own_len..trace_len].fill(ZERO);
        }

        // put ZEROs into the unfilled rows of in_span column
        trace_columns[IN_SPAN_COL_IDX][own_len..trace_len].fill(ZERO);

        // put ZEROs into the unfilled rows of operation group count column
        trace_columns[GROUP_COUNT_COL_IDX][own_len..trace_len].fill(ZERO);

        // put ZEROs into the unfilled rows of operation index column
        trace_columns[OP_INDEX_COL_IDX][own_len..trace_len].fill(ZERO);

        // put ZEROs into the unfilled rows of op_batch_flags columns
        for j in 0..NUM_OP_BATCH_FLAGS {
            trace_columns[OP_BATCH_FLAGS_OFFSET + j][own_len..trace_len].fill(ZERO);
        }

        // put ZEROs into the unfilled rows of the first op bit extra column, because the HALT
        // operation does not use this column.
        trace_columns[OP_BITS_EXTRA_COLS_OFFSET][own_len..trace_len].fill(ZERO);

        // put ONEs into the unfilled rows of the second op bit extra column. we put ONE because the
        // two most significant bits of the HALT operation are ONE and this column is computed as
        // the product of the two most significant op bits.
        debug_assert_eq!(1, (halt_opcode >> 6) & 1);
        debug_assert_eq!(1, (halt_opcode >> 5) & 1);
        debug_assert_eq!(2, NUM_OP_BITS_EXTRA_COLS);
        trace_columns[OP_BITS_EXTRA_COLS_OFFSET + 1][own_len..trace_len].fill(ONE);

        trace_columns
    }

    pub fn write_row(&self, row_idx: usize, row_out: &mut [Felt]) {
        if row_idx < self.rows.len() {
            row_out[ADDR_COL_IDX] = self.rows[row_idx].addr;
            for j in 0..NUM_OP_BITS {
                row_out[OP_BITS_OFFSET + j] = self.rows[row_idx].op_bits[j];
            }

            for j in 0..NUM_HASHER_COLUMNS {
                row_out[HASHER_STATE_OFFSET + j] = self.rows[row_idx].hasher[j];
            }

            row_out[IN_SPAN_COL_IDX] = self.rows[row_idx].in_span;
            row_out[GROUP_COUNT_COL_IDX] = self.rows[row_idx].group_count;
            row_out[OP_INDEX_COL_IDX] = self.rows[row_idx].op_idx;

            for j in 0..NUM_OP_BATCH_FLAGS {
                row_out[OP_BATCH_FLAGS_OFFSET + j] = self.rows[row_idx].op_batch_flags[j];
            }

            for j in 0..NUM_OP_BITS_EXTRA_COLS {
                row_out[OP_BITS_EXTRA_COLS_OFFSET + j] = self.rows[row_idx].op_bit_extra[j];
            }
        // padding rows
        } else {
            // put ZEROs into the unfilled rows of block address column
            row_out[ADDR_COL_IDX] = ZERO;

            // insert HALT opcode into the unfilled rows of op_bits columns
            for j in 0..NUM_OP_BITS {
                row_out[OP_BITS_OFFSET + j] = Felt::from((OPCODE_HALT >> j) & 1);
            }

            // for unfilled rows of hasher state columns, copy over values from the last row for the
            // first 4 columns, and pad the other 4 columns with ZEROs
            let last_row_hasher = self.last_hasher();
            for j in 0..NUM_HASHER_COLUMNS / 2 {
                row_out[HASHER_STATE_OFFSET + j] = last_row_hasher[j];
            }
            for j in NUM_HASHER_COLUMNS / 2..NUM_HASHER_COLUMNS {
                row_out[HASHER_STATE_OFFSET + j] = ZERO;
            }

            // put ZERO into the unfilled rows of in_span column
            row_out[IN_SPAN_COL_IDX] = ZERO;
            // put ZERO into the unfilled rows of operation group count column
            row_out[GROUP_COUNT_COL_IDX] = ZERO;
            // put ZERO into the unfilled rows of operation index column
            row_out[OP_INDEX_COL_IDX] = ZERO;

            // put ZEROs into the unfilled rows of op_batch_flags columns
            for j in 0..NUM_OP_BATCH_FLAGS {
                row_out[OP_BATCH_FLAGS_OFFSET + j] = ZERO;
            }

            // put ZEROs into the unfilled rows of the first op bit extra column, because the HALT
            // operation does not use this column.
            row_out[OP_BITS_EXTRA_COLS_OFFSET] = ZERO;

            // put ONEs into the unfilled rows of the second op bit extra column. we put ONE because
            // the two most significant bits of the HALT operation are ONE and this
            // column is computed as the product of the two most significant op bits.
            debug_assert_eq!(1, (OPCODE_HALT >> 6) & 1);
            debug_assert_eq!(1, (OPCODE_HALT >> 5) & 1);
            debug_assert_eq!(2, NUM_OP_BITS_EXTRA_COLS);
            row_out[OP_BITS_EXTRA_COLS_OFFSET + 1] = ONE;
        }
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    /// Returns the last block address.
    fn last_addr(&self) -> Felt {
        self.rows.last().expect("no rows in decoder trace").addr
    }

    /// Returns the last value of the operation group count.
    fn last_group_count(&self) -> Felt {
        self.rows.last().expect("no rows in decoder trace").group_count
    }

    fn last_hasher(&self) -> [Felt; NUM_HASHER_COLUMNS] {
        self.rows.last().expect("no rows in decoder trace").hasher
    }

    /// Returns a reference to the last value in the helper register at the specified index.
    fn last_helper_mut(&mut self, idx: usize) -> &mut Felt {
        debug_assert!(idx < USER_OP_HELPERS.len(), "invalid helper register index");

        &mut self.rows.last_mut().expect("no last helper value").hasher[USER_OP_HELPERS.start + idx]
    }

    /// Add all provided values to the helper registers in the order provided, starting from the
    /// first hasher register that is available for user operation helper values.
    ///
    /// The specified USER_OP_HELPERS in the `hasher_trace` are used as helper registers, since they
    /// are not required for hashing during execution of user operations.
    pub fn set_user_op_helpers(&mut self, values: &[Felt]) {
        assert!(values.len() <= USER_OP_HELPERS.len(), "too many values for helper columns");

        for (idx, value) in values.iter().enumerate() {
            *self.last_helper_mut(idx) = *value;
        }
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    /// Adds a new decoder trace row of zeros for testing purposes.
    #[cfg(test)]
    pub fn add_dummy_row(&mut self) {
        let row = DecoderTraceRow {
            addr: ZERO,
            op_bits: [ZERO; NUM_OP_BITS],
            op_bit_extra: [ZERO; NUM_OP_BITS_EXTRA_COLS],
            hasher: [ZERO; NUM_HASHER_COLUMNS],
            in_span: ZERO,
            group_count: ZERO,
            op_idx: ZERO,
            op_batch_flags: [ZERO; NUM_OP_BATCH_FLAGS],
        };
        self.rows.push(row);
    }

    /// Fetches all the helper registers from the trace.
    #[cfg(test)]
    pub fn get_user_op_helpers(&self) -> [Felt; NUM_USER_OP_HELPERS] {
        self.rows.last().expect("no last helper value").hasher[USER_OP_HELPERS.start..]
            .try_into()
            .unwrap()
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Returns op batch flags for the specified group count. If the group count is greater than 8,
/// we assume that the operation batch is full - i.e., has 8 operation groups.
fn get_op_batch_flags(num_groups_left: Felt) -> [Felt; NUM_OP_BATCH_FLAGS] {
    let num_groups = get_num_groups_in_next_batch(num_groups_left);
    match num_groups {
        8 => OP_BATCH_8_GROUPS,
        4 => OP_BATCH_4_GROUPS,
        2 => OP_BATCH_2_GROUPS,
        1 => OP_BATCH_1_GROUPS,
        _ => panic!(
            "invalid number of groups in a batch: {num_groups}, group count: {num_groups_left}"
        ),
    }
}

/// Returns the bits of the specified operation code, along with the two values for the two extra
/// columns used for degree reduction.
fn get_op_bits(op_code: u8) -> ([Felt; NUM_OP_BITS], [Felt; NUM_OP_BITS_EXTRA_COLS]) {
    let op_bits = {
        let mut op_bits = [ZERO; NUM_OP_BITS];
        for (i, op_bit) in op_bits.iter_mut().enumerate().take(NUM_OP_BITS) {
            *op_bit = Felt::from((op_code >> i) & 1);
        }
        op_bits
    };

    let op_bit_extra = {
        let bit6 = op_bits[NUM_OP_BITS - 1];
        let bit5 = op_bits[NUM_OP_BITS - 2];
        let bit4 = op_bits[NUM_OP_BITS - 3];

        [bit6 * (ONE - bit5) * bit4, bit6 * bit5]
    };

    (op_bits, op_bit_extra)
}
