use super::{ColMatrix, Felt};
use core::ops::Range;
use miden_air::trace::{
    chiplets::{
        hasher::STATE_WIDTH, BITWISE_A_COL_IDX, BITWISE_B_COL_IDX, BITWISE_OUTPUT_COL_IDX,
        HASHER_NODE_INDEX_COL_IDX, HASHER_STATE_COL_RANGE, MEMORY_ADDR_COL_IDX, MEMORY_CLK_COL_IDX,
        MEMORY_CTX_COL_IDX, MEMORY_V_COL_RANGE,
    },
    decoder::{HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS, USER_OP_HELPERS_OFFSET},
    CHIPLETS_OFFSET, CLK_COL_IDX, CTX_COL_IDX, DECODER_TRACE_OFFSET, STACK_TRACE_OFFSET,
};
use vm_core::{utils::range, ONE, ZERO};

// CONSTANTS
// ================================================================================================

const DECODER_HASHER_RANGE: Range<usize> =
    range(DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET, NUM_HASHER_COLUMNS);

// HELPER STRUCT AND METHODS
// ================================================================================================

pub struct MainTrace<'a> {
    columns: &'a ColMatrix<Felt>,
}

impl<'a> MainTrace<'a> {
    pub fn new(main_trace: &'a ColMatrix<Felt>) -> Self {
        Self {
            columns: main_trace,
        }
    }

    // SYSTEM COLUMNS
    // --------------------------------------------------------------------------------------------

    pub fn clk(&self, i: usize) -> Felt {
        self.columns.get_column(CLK_COL_IDX)[i]
    }

    pub fn ctx(&self, i: usize) -> Felt {
        self.columns.get_column(CTX_COL_IDX)[i]
    }

    // DECODER COLUMNS
    // --------------------------------------------------------------------------------------------

    pub fn addr(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET)[i]
    }

    // Helper method to detect change of address.
    pub fn is_addr_change(&self, i: usize) -> bool {
        self.addr(i) != self.addr(i + 1)
    }

    pub fn helper_0(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET)[i]
    }

    pub fn helper_1(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + 1)[i]
    }

    pub fn helper_2(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + USER_OP_HELPERS_OFFSET + 2)[i]
    }

    pub fn decoder_hasher_state(&self, i: usize) -> [Felt; NUM_HASHER_COLUMNS] {
        let mut state = [ZERO; NUM_HASHER_COLUMNS];
        for (idx, col_idx) in DECODER_HASHER_RANGE.enumerate() {
            let column = self.columns.get_column(col_idx);
            state[idx] = column[i];
        }
        state
    }

    pub fn get_op_code(&self, i: usize) -> Felt {
        let col_b0 = self.columns.get_column(DECODER_TRACE_OFFSET + 1);
        let col_b1 = self.columns.get_column(DECODER_TRACE_OFFSET + 2);
        let col_b2 = self.columns.get_column(DECODER_TRACE_OFFSET + 3);
        let col_b3 = self.columns.get_column(DECODER_TRACE_OFFSET + 4);
        let col_b4 = self.columns.get_column(DECODER_TRACE_OFFSET + 5);
        let col_b5 = self.columns.get_column(DECODER_TRACE_OFFSET + 6);
        let col_b6 = self.columns.get_column(DECODER_TRACE_OFFSET + 7);
        let [b0, b1, b2, b3, b4, b5, b6] =
            [col_b0[i], col_b1[i], col_b2[i], col_b3[i], col_b4[i], col_b5[i], col_b6[i]];
        b0 + b1.mul_small(2)
            + b2.mul_small(4)
            + b3.mul_small(8)
            + b4.mul_small(16)
            + b5.mul_small(32)
            + b6.mul_small(64)
    }

    // STACK COLUMNS
    // --------------------------------------------------------------------------------------------

    pub fn s0(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET)[i]
    }

    pub fn s1(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 1)[i]
    }

    pub fn s2(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 2)[i]
    }

    pub fn s3(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 3)[i]
    }

    pub fn s4(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 4)[i]
    }

    pub fn s5(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 5)[i]
    }

    pub fn s6(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 6)[i]
    }

    pub fn s7(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 7)[i]
    }

    pub fn s8(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 8)[i]
    }

    pub fn s9(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 9)[i]
    }

    pub fn s10(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 10)[i]
    }

    pub fn s11(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 11)[i]
    }

    pub fn s12(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 12)[i]
    }

    pub fn s13(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + 13)[i]
    }

    // CHIPLETS COLUMNS
    // --------------------------------------------------------------------------------------------

    pub fn chiplet_selector_0(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET)[i]
    }

    pub fn chiplet_selector_1(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 1)[i]
    }

    pub fn chiplet_selector_2(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 2)[i]
    }

    pub fn chiplet_selector_3(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 3)[i]
    }

    pub fn chiplet_selector_4(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 4)[i]
    }

    pub fn chiplet_hasher_state(&self, i: usize) -> [Felt; STATE_WIDTH] {
        let mut state = [ZERO; STATE_WIDTH];
        for (idx, col_idx) in HASHER_STATE_COL_RANGE.enumerate() {
            let column = self.columns.get_column(col_idx);
            state[idx] = column[i];
        }
        state
    }

    pub fn chiplet_node_index(&self, i: usize) -> Felt {
        self.columns.get(HASHER_NODE_INDEX_COL_IDX, i)
    }

    pub fn chiplet_bitwise_a(&self, i: usize) -> Felt {
        self.columns.get_column(BITWISE_A_COL_IDX)[i]
    }

    pub fn chiplet_bitwise_b(&self, i: usize) -> Felt {
        self.columns.get_column(BITWISE_B_COL_IDX)[i]
    }

    pub fn chiplet_bitwise_z(&self, i: usize) -> Felt {
        self.columns.get_column(BITWISE_OUTPUT_COL_IDX)[i]
    }

    pub fn chiplet_memory_ctx(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_CTX_COL_IDX)[i]
    }

    pub fn chiplet_memory_addr(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_ADDR_COL_IDX)[i]
    }

    pub fn chiplet_memory_clk(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_CLK_COL_IDX)[i]
    }

    pub fn chiplet_memory_value_0(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_V_COL_RANGE.start)[i]
    }

    pub fn chiplet_memory_value_1(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_V_COL_RANGE.start + 1)[i]
    }

    pub fn chiplet_memory_value_2(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_V_COL_RANGE.start + 2)[i]
    }

    pub fn chiplet_memory_value_3(&self, i: usize) -> Felt {
        self.columns.get_column(MEMORY_V_COL_RANGE.start + 3)[i]
    }

    pub fn chiplet_kernel_root_0(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 6)[i]
    }

    pub fn chiplet_kernel_root_1(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 7)[i]
    }

    pub fn chiplet_kernel_root_2(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 8)[i]
    }

    pub fn chiplet_kernel_root_3(&self, i: usize) -> Felt {
        self.columns.get_column(CHIPLETS_OFFSET + 9)[i]
    }

    //  MERKLE PATH HASHING SELECTORS
    // --------------------------------------------------------------------------------------------

    pub fn f_mv(&self, i: usize) -> bool {
        (i % 8 == 0)
            && self.chiplet_selector_0(i) == ZERO
            && self.chiplet_selector_1(i) == ONE
            && self.chiplet_selector_2(i) == ONE
            && self.chiplet_selector_3(i) == ZERO
    }

    pub fn f_mva(&self, i: usize) -> bool {
        (i % 8 == 7)
            && self.chiplet_selector_0(i) == ZERO
            && self.chiplet_selector_1(i) == ONE
            && self.chiplet_selector_2(i) == ONE
            && self.chiplet_selector_3(i) == ZERO
    }

    pub fn f_mu(&self, i: usize) -> bool {
        (i % 8 == 0)
            && self.chiplet_selector_0(i) == ZERO
            && self.chiplet_selector_1(i) == ONE
            && self.chiplet_selector_2(i) == ONE
            && self.chiplet_selector_3(i) == ONE
    }

    pub fn f_mua(&self, i: usize) -> bool {
        (i % 8 == 7)
            && self.chiplet_selector_0(i) == ZERO
            && self.chiplet_selector_1(i) == ONE
            && self.chiplet_selector_2(i) == ONE
            && self.chiplet_selector_3(i) == ONE
    }

    // Detects if a row is part of the kernel chiplet.
    pub fn is_kernel_row(&self, i: usize) -> bool {
        self.chiplet_selector_0(i) == ONE
            && self.chiplet_selector_1(i) == ONE
            && self.chiplet_selector_2(i) == ONE
            && self.chiplet_selector_3(i) == ZERO
    }
}
