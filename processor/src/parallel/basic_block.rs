use core::ops::ControlFlow;

use miden_air::trace::{
    DECODER_TRACE_OFFSET,
    decoder::{
        ADDR_COL_IDX, GROUP_COUNT_COL_IDX, HASHER_STATE_OFFSET, IN_SPAN_COL_IDX,
        NUM_OP_BATCH_FLAGS, NUM_OP_BITS, NUM_OP_BITS_EXTRA_COLS, NUM_USER_OP_HELPERS,
        OP_BATCH_FLAGS_OFFSET, OP_BITS_EXTRA_COLS_OFFSET, OP_BITS_OFFSET, OP_INDEX_COL_IDX,
    },
};
use vm_core::{
    Felt, ONE, Operation, Word, ZERO,
    mast::{BasicBlockNode, OpBatch},
};

use super::{CoreTraceFragmentGenerator, TraceRowType, trace_builder::OperationTraceConfig};

const HASH_CYCLE_LEN: Felt = Felt::new(miden_air::trace::chiplets::hasher::HASH_CYCLE_LEN as u64);

impl CoreTraceFragmentGenerator {
    /// Adds a trace row for SPAN start operation to the main trace fragment.
    ///
    /// This method creates a trace row that corresponds to the SPAN operation that starts
    /// a basic block execution. It follows the same pattern as `DecoderTrace::append_span_start()`.
    pub fn add_span_start_trace_row(
        &mut self,
        first_op_batch: &OpBatch,
        num_groups: Felt,
        parent_addr: Felt,
    ) -> ControlFlow<()> {
        let row_idx = self.num_rows_built();

        // Populate system trace columns
        self.populate_system_trace_columns(row_idx);

        // Populate decoder trace columns
        // Set the address to the parent address
        self.fragment.columns[DECODER_TRACE_OFFSET + ADDR_COL_IDX][row_idx] = parent_addr;

        self.append_opcode(Operation::Span.op_code(), row_idx);

        // Set the hasher state to the groups of the first op batch
        for (i, &group) in first_op_batch.groups().iter().enumerate() {
            self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + i][row_idx] = group;
        }

        // Set in_span to ZERO (we are starting a span, not inside one yet)
        self.fragment.columns[DECODER_TRACE_OFFSET + IN_SPAN_COL_IDX][row_idx] = ZERO;

        // Set group_count to the total number of operation groups in the basic block
        self.fragment.columns[DECODER_TRACE_OFFSET + GROUP_COUNT_COL_IDX][row_idx] = num_groups;

        // Set operation index to ZERO
        self.fragment.columns[DECODER_TRACE_OFFSET + OP_INDEX_COL_IDX][row_idx] = ZERO;

        // Set the op_batch_flags based on the number of operation groups
        {
            let op_batch_flags = get_op_batch_flags(num_groups);
            for i in 0..NUM_OP_BATCH_FLAGS {
                self.fragment.columns[DECODER_TRACE_OFFSET + OP_BATCH_FLAGS_OFFSET + i][row_idx] =
                    op_batch_flags[i];
            }
        }

        // Set extra bit columns for degree reduction
        {
            let bit6 = self.fragment.columns
                [DECODER_TRACE_OFFSET + OP_BITS_OFFSET + NUM_OP_BITS - 1][row_idx];
            let bit5 = self.fragment.columns
                [DECODER_TRACE_OFFSET + OP_BITS_OFFSET + NUM_OP_BITS - 2][row_idx];
            let bit4 = self.fragment.columns
                [DECODER_TRACE_OFFSET + OP_BITS_OFFSET + NUM_OP_BITS - 3][row_idx];

            // set the columns
            self.fragment.columns[DECODER_TRACE_OFFSET + OP_BITS_EXTRA_COLS_OFFSET][row_idx] =
                bit6 * (ONE - bit5) * bit4;
            self.fragment.columns[DECODER_TRACE_OFFSET + OP_BITS_EXTRA_COLS_OFFSET + 1][row_idx] =
                bit6 * bit5;
        }

        // Populate stack trace columns
        self.populate_stack_trace_columns(row_idx);

        self.increment_clk()
    }

    /// Adds a trace row for SPAN end operation to the main trace fragment.
    ///
    /// This method creates a trace row that corresponds to the END operation that completes
    /// a basic block execution.
    pub fn add_span_end_trace_row(&mut self, basic_block_node: &BasicBlockNode) -> ControlFlow<()> {
        let block_info = self.state.block_stack.pop();

        // Get the span hash from the basic block node
        let span_hash: Word = basic_block_node.digest();

        // For SPAN blocks, the hasher state is not used in the same way as control flow blocks,
        // but we need to provide it for the config. We can use zero hashes as placeholders
        // since the END operation for SPAN blocks has a different structure.
        let zero_hash = Word::default();

        let config = OperationTraceConfig {
            start_opcode: Operation::Span.op_code(), /* Not used for END operations, but required
                                                      * by config */
            start_hasher_state: (zero_hash, zero_hash), // Not used for END operations
            end_node_digest: span_hash,
            addr: block_info.addr,
            block_info: Some(block_info),
        };

        // Reset the span context after completing the basic block
        self.span_context = None;

        self.add_control_flow_trace_row(config, TraceRowType::End)
    }

    // RESPAN
    // -------------------------------------------------------------------------------------------

    pub fn respan(&mut self, op_batch: &OpBatch) -> ControlFlow<()> {
        let addr = self.state.block_stack.peek().addr;
        self.add_respan_trace_row(op_batch, addr)?;

        // Update block address for the upcoming block
        self.state.block_stack.peek_mut().addr += HASH_CYCLE_LEN;

        // Update span context
        let span_context = self
            .span_context
            .as_mut()
            .expect("Span context should be initialized for RESPAN");
        span_context.num_groups_left -= ONE;
        span_context.group_ops_left = op_batch.groups()[0];

        ControlFlow::Continue(())
    }

    /// Adds a trace row for RESPAN operation to the main trace fragment.
    ///
    /// This method creates a trace row that corresponds to the RESPAN operation that starts
    /// processing of a new operation batch within the same basic block.
    /// It follows the same pattern as `DecoderTrace::append_respan()`.
    fn add_respan_trace_row(&mut self, op_batch: &OpBatch, addr: Felt) -> ControlFlow<()> {
        use miden_air::trace::{
            DECODER_TRACE_OFFSET,
            decoder::{
                ADDR_COL_IDX, GROUP_COUNT_COL_IDX, HASHER_STATE_OFFSET, IN_SPAN_COL_IDX,
                NUM_OP_BATCH_FLAGS, OP_BATCH_FLAGS_OFFSET, OP_INDEX_COL_IDX,
            },
        };
        let group_count = self
            .span_context
            .as_ref()
            .expect("Span context should be initialized for RESPAN")
            .num_groups_left;

        let row_idx = self.num_rows_built();

        // Populate system trace columns
        self.populate_system_trace_columns(row_idx);

        // populate decoder trace columns
        self.fragment.columns[DECODER_TRACE_OFFSET + ADDR_COL_IDX][row_idx] = addr;

        self.append_opcode(Operation::Respan.op_code(), row_idx);

        // Set hasher state to op groups of the next op batch
        for (i, &group) in op_batch.groups().iter().enumerate() {
            self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + i][row_idx] = group;
        }

        self.fragment.columns[DECODER_TRACE_OFFSET + IN_SPAN_COL_IDX][row_idx] = ZERO;
        self.fragment.columns[DECODER_TRACE_OFFSET + GROUP_COUNT_COL_IDX][row_idx] = group_count;
        self.fragment.columns[DECODER_TRACE_OFFSET + OP_INDEX_COL_IDX][row_idx] = ZERO;

        // Set the op_batch_flags based on the current operation group count
        let op_batch_flags = get_op_batch_flags(group_count);
        for i in 0..NUM_OP_BATCH_FLAGS {
            self.fragment.columns[DECODER_TRACE_OFFSET + OP_BATCH_FLAGS_OFFSET + i][row_idx] =
                op_batch_flags[i];
        }

        // Populate stack trace columns
        self.populate_stack_trace_columns(row_idx);

        self.increment_clk()
    }

    /// Writes a trace row for an operation within a basic block.
    ///
    /// This must be called *after* the operation has been executed and the
    /// stack has been updated.
    pub fn add_operation_trace_row(
        &mut self,
        operation: Operation,
        op_idx_in_group: usize,
        user_op_helpers: Option<[Felt; NUM_USER_OP_HELPERS]>,
    ) -> ControlFlow<()> {
        let row_idx = self.num_rows_built();

        // Populate system trace columns
        self.populate_system_trace_columns(row_idx);

        let block = self.state.block_stack.peek();
        let ctx = self.span_context.as_mut().expect("not in span");

        // update operations left to be executed in the group
        ctx.group_ops_left = remove_opcode_from_group(ctx.group_ops_left, operation);

        self.fragment.columns[DECODER_TRACE_OFFSET + ADDR_COL_IDX][row_idx] = block.addr;

        // TODO(plafer): copy/pasted from trace_builder.rs; use `append_opcode` method
        {
            let opcode = operation.op_code();
            for i in 0..NUM_OP_BITS {
                let bit = Felt::from((opcode >> i) & 1);
                self.fragment.columns[DECODER_TRACE_OFFSET + OP_BITS_OFFSET + i][row_idx] = bit;
            }
        }

        // hasher trace: group_ops_left and parent address
        self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET][row_idx] =
            ctx.group_ops_left;
        self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + 1][row_idx] =
            block.parent_addr;

        // hasher trace: user op helpers
        {
            let user_op_helpers = user_op_helpers.unwrap_or([ZERO; NUM_USER_OP_HELPERS]);
            self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + 2][row_idx] =
                user_op_helpers[0];
            self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + 3][row_idx] =
                user_op_helpers[1];
            self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + 4][row_idx] =
                user_op_helpers[2];
            self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + 5][row_idx] =
                user_op_helpers[3];
            self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + 6][row_idx] =
                user_op_helpers[4];
            self.fragment.columns[DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + 7][row_idx] =
                user_op_helpers[5];
        }

        self.fragment.columns[DECODER_TRACE_OFFSET + IN_SPAN_COL_IDX][row_idx] = ONE;
        self.fragment.columns[DECODER_TRACE_OFFSET + GROUP_COUNT_COL_IDX][row_idx] =
            ctx.num_groups_left;
        self.fragment.columns[DECODER_TRACE_OFFSET + OP_INDEX_COL_IDX][row_idx] =
            Felt::from(op_idx_in_group as u32);

        // Batch flag columns - all 0 for control flow operations
        for i in 0..NUM_OP_BATCH_FLAGS {
            self.fragment.columns[DECODER_TRACE_OFFSET + OP_BATCH_FLAGS_OFFSET + i][row_idx] = ZERO;
        }

        // Extra bit columns - all 0 for control flow operations
        for i in 0..NUM_OP_BITS_EXTRA_COLS {
            self.fragment.columns[DECODER_TRACE_OFFSET + OP_BITS_EXTRA_COLS_OFFSET + i][row_idx] =
                ZERO;
        }

        if operation.imm_value().is_some() {
            ctx.num_groups_left -= ONE;
        }

        // Populate stack trace columns
        self.populate_stack_trace_columns(row_idx);

        self.increment_clk()
    }
}

// HELPERS
// ===============================================================================================

/// Returns op batch flags for the specified group count, following the same logic as
/// `DecoderTrace::get_op_batch_flags()`.
fn get_op_batch_flags(num_groups_left: Felt) -> [Felt; 3] {
    use miden_air::trace::decoder::{
        OP_BATCH_1_GROUPS, OP_BATCH_2_GROUPS, OP_BATCH_4_GROUPS, OP_BATCH_8_GROUPS,
    };
    use vm_core::mast::OP_BATCH_SIZE;

    let num_groups = core::cmp::min(num_groups_left.as_int() as usize, OP_BATCH_SIZE);
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

/// Removes the specified operation from the op group and returns the resulting op group.
fn remove_opcode_from_group(op_group: Felt, op: Operation) -> Felt {
    let opcode = op.op_code() as u64;
    let result = Felt::new((op_group.as_int() - opcode) >> NUM_OP_BITS);
    debug_assert!(op_group.as_int() >= result.as_int(), "op group underflow");
    result
}
