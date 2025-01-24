#[cfg(any(test, feature = "testing"))]
use alloc::vec::Vec;
use core::ops::Range;

use vm_core::{
    utils::{range, uninit_vector},
    Felt, Word, ONE, ZERO,
};

use super::{
    chiplets::{
        hasher::{DIGEST_LEN, HASH_CYCLE_LEN, STATE_WIDTH},
        BITWISE_A_COL_IDX, BITWISE_B_COL_IDX, BITWISE_OUTPUT_COL_IDX, HASHER_NODE_INDEX_COL_IDX,
        HASHER_STATE_COL_RANGE, MEMORY_CLK_COL_IDX, MEMORY_CTX_COL_IDX, MEMORY_IDX0_COL_IDX,
        MEMORY_IDX1_COL_IDX, MEMORY_V_COL_RANGE, MEMORY_WORD_COL_IDX,
    },
    decoder::{
        GROUP_COUNT_COL_IDX, HASHER_STATE_OFFSET, IN_SPAN_COL_IDX, IS_CALL_FLAG_COL_IDX,
        IS_LOOP_BODY_FLAG_COL_IDX, IS_LOOP_FLAG_COL_IDX, IS_SYSCALL_FLAG_COL_IDX,
        NUM_HASHER_COLUMNS, NUM_OP_BATCH_FLAGS, OP_BATCH_FLAGS_OFFSET, OP_BITS_EXTRA_COLS_OFFSET,
        USER_OP_HELPERS_OFFSET,
    },
    stack::{B0_COL_IDX, B1_COL_IDX, H0_COL_IDX},
    CHIPLETS_OFFSET, CLK_COL_IDX, CTX_COL_IDX, DECODER_TRACE_OFFSET, FMP_COL_IDX, FN_HASH_OFFSET,
    STACK_TRACE_OFFSET, TRACE_WIDTH,
};
use crate::RowIndex;

// CONSTANTS
// ================================================================================================

const DECODER_HASHER_RANGE: Range<usize> =
    range(DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS);

// HELPER STRUCT AND METHODS
// ================================================================================================

pub struct MainTrace {
    rows_buffer: Vec<Felt>,
    last_program_row: RowIndex,
}

impl MainTrace {
    pub fn new(main_trace: Vec<Felt>, last_program_row: RowIndex) -> Self {
        Self {
            rows_buffer: main_trace,
            last_program_row,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.rows_buffer.len() / TRACE_WIDTH
    }

    pub fn get_row(&self, row_idx: RowIndex) -> &[Felt] {
        let start = row_idx.as_usize() * TRACE_WIDTH;
        &self.rows_buffer[range(start, TRACE_WIDTH)]
    }

    pub fn last_program_row(&self) -> RowIndex {
        self.last_program_row
    }

    // TODO(plafer): Turn this in an iterator instead
    pub fn get_column(&self, col_idx: usize) -> Vec<Felt> {
        let mut column = unsafe { uninit_vector(self.num_rows()) };
        for row_idx in 0..self.num_rows() {
            column[row_idx] = self.rows_buffer[row_idx * TRACE_WIDTH + col_idx];
        }
        column
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn get_column_range(&self, range: Range<usize>) -> Vec<Vec<Felt>> {
        range.fold(vec![], |mut acc, col_idx| {
            acc.push(self.get_column(col_idx));
            acc
        })
    }

    // SYSTEM COLUMNS
    // --------------------------------------------------------------------------------------------

    /// Returns the value of the clk column at the given row.
    pub fn clk(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CLK_COL_IDX]
    }

    /// Returns the value of the fmp column at the given row.
    pub fn fmp(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + FMP_COL_IDX]
    }

    /// Returns the value of the ctx column at the given row.
    pub fn ctx(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CTX_COL_IDX]
    }

    // DECODER COLUMNS
    // --------------------------------------------------------------------------------------------

    /// Returns the value in the block address column at the row i.
    pub fn addr(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET]
    }

    /// Helper method to detect change of address.
    pub fn is_addr_change(&self, row_idx: RowIndex) -> bool {
        self.addr(row_idx) != self.addr(row_idx + 1)
    }

    /// The i-th decoder helper register at `row_idx`.
    pub fn helper_register(&self, i: usize, row_idx: RowIndex) -> Felt {
        self.rows_buffer
            [row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + i]
    }

    /// Returns the hasher state at the given row.
    pub fn decoder_hasher_state(&self, row_idx: RowIndex) -> [Felt; NUM_HASHER_COLUMNS] {
        let row_start = &self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH..];

        row_start[DECODER_HASHER_RANGE].try_into().unwrap()
    }

    /// Returns the first half of the hasher state at the given row.
    pub fn decoder_hasher_state_first_half(&self, row_idx: RowIndex) -> Word {
        let row_start = &self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH..];

        let mut state = [ZERO; DIGEST_LEN];
        state.copy_from_slice(&row_start[range(DECODER_HASHER_RANGE.start, DIGEST_LEN)]);

        state
    }

    /// Returns the second half of the hasher state at the given row.
    pub fn decoder_hasher_state_second_half(&self, row_idx: RowIndex) -> Word {
        const SECOND_WORD_OFFSET: usize = 4;
        let row_start = &self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH..];

        let mut state = [ZERO; DIGEST_LEN];
        state.copy_from_slice(
            &row_start[range(DECODER_HASHER_RANGE.start + SECOND_WORD_OFFSET, DIGEST_LEN)],
        );

        state
    }

    /// Returns a specific element from the hasher state at the given row.
    pub fn decoder_hasher_state_element(&self, element: usize, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + DECODER_HASHER_RANGE.start + element]
    }

    /// Returns the current function hash (i.e., root) at the given row.
    pub fn fn_hash(&self, row_idx: RowIndex) -> [Felt; DIGEST_LEN] {
        let row_start = &self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH..];

        let mut state = [ZERO; DIGEST_LEN];
        state.copy_from_slice(&row_start[range(FN_HASH_OFFSET, DIGEST_LEN)]);
        state
    }

    /// Returns the `is_loop_body` flag at the given row.
    pub fn is_loop_body_flag(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer
            [row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET + IS_LOOP_BODY_FLAG_COL_IDX]
    }

    /// Returns the `is_loop` flag at the given row.
    pub fn is_loop_flag(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer
            [row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET + IS_LOOP_FLAG_COL_IDX]
    }

    /// Returns the `is_call` flag at the given row.
    pub fn is_call_flag(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer
            [row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET + IS_CALL_FLAG_COL_IDX]
    }

    /// Returns the `is_syscall` flag at the given row.
    pub fn is_syscall_flag(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer
            [row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET + IS_SYSCALL_FLAG_COL_IDX]
    }

    /// Returns the operation batch flags at the given row. This indicates the number of op groups
    /// in the current batch that is being processed.
    pub fn op_batch_flag(&self, row_idx: RowIndex) -> [Felt; NUM_OP_BATCH_FLAGS] {
        let base_idx =
            row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET + OP_BATCH_FLAGS_OFFSET;
        [
            self.rows_buffer[base_idx],
            self.rows_buffer[base_idx + 1],
            self.rows_buffer[base_idx + 2],
        ]
    }

    /// Returns the operation group count. This indicates the number of operation that remain
    /// to be executed in the current span block.
    pub fn group_count(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer
            [row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET + GROUP_COUNT_COL_IDX]
    }

    /// Returns the delta between the current and next group counts.
    pub fn delta_group_count(&self, row_idx: RowIndex) -> Felt {
        self.group_count(row_idx) - self.group_count(row_idx + 1)
    }

    /// Returns the `in_span` flag at the given row.
    pub fn is_in_span(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET + IN_SPAN_COL_IDX]
    }

    /// Constructs the i-th op code value from its individual bits.
    pub fn get_op_code(&self, row_idx: RowIndex) -> Felt {
        let base_idx = row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET;
        let b0 = self.rows_buffer[base_idx + 1];
        let b1 = self.rows_buffer[base_idx + 2];
        let b2 = self.rows_buffer[base_idx + 3];
        let b3 = self.rows_buffer[base_idx + 4];
        let b4 = self.rows_buffer[base_idx + 5];
        let b5 = self.rows_buffer[base_idx + 6];
        let b6 = self.rows_buffer[base_idx + 7];
        b0 + b1.mul_small(2)
            + b2.mul_small(4)
            + b3.mul_small(8)
            + b4.mul_small(16)
            + b5.mul_small(32)
            + b6.mul_small(64)
    }

    /// Returns an iterator of [`RowIndex`] values over the row indices of this trace.
    pub fn row_iter(&self) -> impl Iterator<Item = RowIndex> {
        (0..self.num_rows()).map(RowIndex::from)
    }

    /// Returns a flag indicating whether the current operation induces a left shift of the operand
    /// stack.
    pub fn is_left_shift(&self, row_idx: RowIndex) -> bool {
        let base_idx = row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET;
        let b0 = self.rows_buffer[base_idx + 1];
        let b1 = self.rows_buffer[base_idx + 2];
        let b2 = self.rows_buffer[base_idx + 3];
        let b3 = self.rows_buffer[base_idx + 4];
        let b4 = self.rows_buffer[base_idx + 5];
        let b5 = self.rows_buffer[base_idx + 6];
        let b6 = self.rows_buffer[base_idx + 7];
        let e0 = self.rows_buffer[base_idx + OP_BITS_EXTRA_COLS_OFFSET];
        let h5 = self.rows_buffer[base_idx + IS_LOOP_FLAG_COL_IDX];

        // group with left shift effect grouped by a common prefix
        ([b6, b5, b4] == [ZERO, ONE, ZERO])||
        // U32ADD3 or U32MADD
        ([b6, b5, b4, b3, b2] == [ONE, ZERO, ZERO, ONE, ONE]) ||
        // SPLIT or LOOP block
        ([e0, b3, b2, b1] == [ONE, ZERO, ONE, ZERO]) ||
        // REPEAT
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ONE, ONE, ZERO, ONE, ZERO, ZERO]) ||
        // DYN
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ZERO, ONE, ONE, ZERO, ZERO, ZERO]) ||
        // END of a loop
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ONE, ONE, ZERO, ZERO, ZERO, ZERO] && h5 == ONE)
    }

    /// Returns a flag indicating whether the current operation induces a right shift of the operand
    /// stack.
    pub fn is_right_shift(&self, row_idx: RowIndex) -> bool {
        let base_idx = row_idx.as_usize() * TRACE_WIDTH + DECODER_TRACE_OFFSET;
        let b0 = self.rows_buffer[base_idx + 1];
        let b1 = self.rows_buffer[base_idx + 2];
        let b2 = self.rows_buffer[base_idx + 3];
        let b3 = self.rows_buffer[base_idx + 4];
        let b4 = self.rows_buffer[base_idx + 5];
        let b5 = self.rows_buffer[base_idx + 6];
        let b6 = self.rows_buffer[base_idx + 7];

        // group with right shift effect grouped by a common prefix
        [b6, b5, b4] == [ZERO, ONE, ONE]||
        // u32SPLIT 100_1000
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ZERO, ZERO, ONE, ZERO, ZERO, ZERO]) ||
        // PUSH i.e., 101_1011
        ([b6, b5, b4, b3, b2, b1, b0] == [ONE, ZERO, ONE, ONE, ZERO, ONE, ONE])
    }

    // STACK COLUMNS
    // --------------------------------------------------------------------------------------------

    /// Returns the value of the stack depth column at the given row.
    pub fn stack_depth(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + STACK_TRACE_OFFSET + B0_COL_IDX]
    }

    /// Returns the element at the given row in a given stack trace column.
    pub fn stack_element(&self, column: usize, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + STACK_TRACE_OFFSET + column]
    }

    /// Returns the address of the top element in the stack overflow table at the given row.
    pub fn parent_overflow_address(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + STACK_TRACE_OFFSET + B1_COL_IDX]
    }

    /// Returns a flag indicating whether the overflow stack is non-empty.
    pub fn is_non_empty_overflow(&self, row_idx: RowIndex) -> bool {
        let b0 =
            self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + STACK_TRACE_OFFSET + B0_COL_IDX];
        let h0 =
            self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + STACK_TRACE_OFFSET + H0_COL_IDX];
        (b0 - Felt::new(16)) * h0 == ONE
    }

    // CHIPLETS COLUMNS
    // --------------------------------------------------------------------------------------------

    /// Returns chiplet column number 0 at the given row.
    pub fn chiplet_selector_0(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET]
    }

    /// Returns chiplet column number 1 at the given row.
    pub fn chiplet_selector_1(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 1]
    }

    /// Returns chiplet column number 2 at the given row.
    pub fn chiplet_selector_2(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 2]
    }

    /// Returns chiplet column number 3 at the given row.
    pub fn chiplet_selector_3(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 3]
    }

    /// Returns chiplet column number 4 at the given row.
    pub fn chiplet_selector_4(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 4]
    }

    /// Returns `true` if a row is part of the hash chiplet.
    pub fn is_hash_row(&self, i: RowIndex) -> bool {
        self.chiplet_selector_0(i) == ZERO
    }

    /// Returns the (full) state of the hasher chiplet at the given row.
    pub fn chiplet_hasher_state(&self, row_idx: RowIndex) -> [Felt; STATE_WIDTH] {
        let row_start = &self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH..];

        let mut state = [ZERO; STATE_WIDTH];
        state.copy_from_slice(&row_start[HASHER_STATE_COL_RANGE]);
        state
    }

    /// Returns the hasher's node index column at the given row
    pub fn chiplet_node_index(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + HASHER_NODE_INDEX_COL_IDX]
    }

    /// Returns `true` if a row is part of the bitwise chiplet.
    pub fn is_bitwise_row(&self, i: RowIndex) -> bool {
        self.chiplet_selector_0(i) == ONE && self.chiplet_selector_1(i) == ZERO
    }

    /// Returns the bitwise column holding the aggregated value of input `a` at the given row.
    pub fn chiplet_bitwise_a(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + BITWISE_A_COL_IDX]
    }

    /// Returns the bitwise column holding the aggregated value of input `b` at the given row.
    pub fn chiplet_bitwise_b(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + BITWISE_B_COL_IDX]
    }

    /// Returns the bitwise column holding the aggregated value of the output at the given row.
    pub fn chiplet_bitwise_z(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + BITWISE_OUTPUT_COL_IDX]
    }

    /// Returns `true` if a row is part of the memory chiplet.
    pub fn is_memory_row(&self, i: RowIndex) -> bool {
        self.chiplet_selector_0(i) == ONE
            && self.chiplet_selector_1(i) == ONE
            && self.chiplet_selector_2(i) == ZERO
    }

    /// Returns the i-th row of the chiplet column containing memory context.
    pub fn chiplet_memory_ctx(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_CTX_COL_IDX]
    }

    /// Returns the i-th row of the chiplet column containing memory address.
    pub fn chiplet_memory_word(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_WORD_COL_IDX]
    }

    /// Returns the i-th row of the chiplet column containing 0th bit of the word index.
    pub fn chiplet_memory_idx0(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_IDX0_COL_IDX]
    }

    /// Returns the i-th row of the chiplet column containing 1st bit of the word index.
    pub fn chiplet_memory_idx1(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_IDX1_COL_IDX]
    }

    /// Returns the i-th row of the chiplet column containing clock cycle.
    pub fn chiplet_memory_clk(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_CLK_COL_IDX]
    }

    /// Returns the i-th row of the chiplet column containing the zeroth memory value element.
    pub fn chiplet_memory_value_0(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_V_COL_RANGE.start]
    }

    /// Returns the i-th row of the chiplet column containing the first memory value element.
    pub fn chiplet_memory_value_1(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_V_COL_RANGE.start + 1]
    }

    /// Returns the i-th row of the chiplet column containing the second memory value element.
    pub fn chiplet_memory_value_2(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_V_COL_RANGE.start + 2]
    }

    /// Returns the i-th row of the chiplet column containing the third memory value element.
    pub fn chiplet_memory_value_3(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + MEMORY_V_COL_RANGE.start + 3]
    }

    /// Returns `true` if a row is part of the kernel chiplet.
    pub fn is_kernel_row(&self, i: RowIndex) -> bool {
        self.chiplet_selector_0(i) == ONE
            && self.chiplet_selector_1(i) == ONE
            && self.chiplet_selector_2(i) == ONE
            && self.chiplet_selector_3(i) == ZERO
    }

    /// Returns the i-th row of the kernel chiplet `addr` column.
    pub fn chiplet_kernel_addr(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 5]
    }

    /// Returns the i-th row of the chiplet column containing the zeroth element of the kernel
    /// procedure root.
    pub fn chiplet_kernel_root_0(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 6]
    }

    /// Returns the i-th row of the chiplet column containing the first element of the kernel
    /// procedure root.
    pub fn chiplet_kernel_root_1(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 7]
    }

    /// Returns the i-th row of the chiplet column containing the second element of the kernel
    /// procedure root.
    pub fn chiplet_kernel_root_2(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 8]
    }

    /// Returns the i-th row of the chiplet column containing the third element of the kernel
    /// procedure root.
    pub fn chiplet_kernel_root_3(&self, row_idx: RowIndex) -> Felt {
        self.rows_buffer[row_idx.as_usize() * TRACE_WIDTH + CHIPLETS_OFFSET + 9]
    }

    //  MERKLE PATH HASHING SELECTORS
    // --------------------------------------------------------------------------------------------

    /// Returns `true` if the hasher chiplet flags indicate the initialization of verifying
    /// a Merkle path to an old node during Merkle root update procedure (MRUPDATE).
    pub fn f_mv(&self, row_idx: RowIndex) -> bool {
        (row_idx.as_usize() % HASH_CYCLE_LEN == 0)
            && self.chiplet_selector_0(row_idx) == ZERO
            && self.chiplet_selector_1(row_idx) == ONE
            && self.chiplet_selector_2(row_idx) == ONE
            && self.chiplet_selector_3(row_idx) == ZERO
    }

    /// Returns `true` if the hasher chiplet flags indicate the continuation of verifying
    /// a Merkle path to an old node during Merkle root update procedure (MRUPDATE).
    pub fn f_mva(&self, row_idx: RowIndex) -> bool {
        (row_idx.as_usize() % HASH_CYCLE_LEN == HASH_CYCLE_LEN - 1)
            && self.chiplet_selector_0(row_idx) == ZERO
            && self.chiplet_selector_1(row_idx) == ONE
            && self.chiplet_selector_2(row_idx) == ONE
            && self.chiplet_selector_3(row_idx) == ZERO
    }

    /// Returns `true` if the hasher chiplet flags indicate the initialization of verifying
    /// a Merkle path to a new node during Merkle root update procedure (MRUPDATE).
    pub fn f_mu(&self, row_idx: RowIndex) -> bool {
        (row_idx.as_usize() % HASH_CYCLE_LEN == 0)
            && self.chiplet_selector_0(row_idx) == ZERO
            && self.chiplet_selector_1(row_idx) == ONE
            && self.chiplet_selector_2(row_idx) == ONE
            && self.chiplet_selector_3(row_idx) == ONE
    }

    /// Returns `true` if the hasher chiplet flags indicate the continuation of verifying
    /// a Merkle path to a new node during Merkle root update procedure (MRUPDATE).
    pub fn f_mua(&self, row_idx: RowIndex) -> bool {
        (row_idx.as_usize() % HASH_CYCLE_LEN == HASH_CYCLE_LEN - 1)
            && self.chiplet_selector_0(row_idx) == ZERO
            && self.chiplet_selector_1(row_idx) == ONE
            && self.chiplet_selector_2(row_idx) == ONE
            && self.chiplet_selector_3(row_idx) == ONE
    }
}
