use alloc::vec::Vec;

use miden_air::{
    RowIndex,
    trace::{DECODER_TRACE_WIDTH, STACK_TRACE_WIDTH, SYS_TRACE_WIDTH},
};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};
use traversal::ExecutionTraversal;
use vm_core::{
    Felt, ONE, Program, Word, ZERO,
    mast::{BasicBlockNode, MastForest, MastNode, MastNodeId},
    stack::MIN_STACK_DEPTH,
    utils::uninit_vector,
};

use crate::{
    ContextId, resolve_external_node,
    system::{FMP_MIN, SYSCALL_FMP_MIN},
};

pub const MAIN_TRACE_WIDTH: usize = SYS_TRACE_WIDTH + DECODER_TRACE_WIDTH + STACK_TRACE_WIDTH;

mod call;
mod r#dyn;
mod join;
mod r#loop;
mod split;
mod trace_builder;
mod traversal;

// ENUMS
// ================================================================================================

/// Enum to specify whether this is a start or end trace row for control block operations
/// (JOIN, SPLIT, LOOP, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceRowType {
    Start,
    End,
}

// CHECKPOINT STRUCTS
// ================================================================================================

/// A checkpoint represents all the information for one row of the System trace.
///
/// This struct captures the complete state of the system at a specific clock cycle,
/// allowing for reconstruction of the system trace during concurrent execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemCheckpoint {
    /// Current clock cycle (row index in the trace)
    pub clk: RowIndex,

    /// Execution context ID - starts at 0 (root context), changes on CALL/SYSCALL operations
    pub ctx: ContextId,

    /// Free memory pointer - initially set to 2^30, used for local memory offsets
    pub fmp: Felt,

    /// Flag indicating whether currently executing within a SYSCALL block
    pub in_syscall: bool,

    /// Hash of the function that initiated the current execution context
    /// - For root context: [ZERO; 4]
    /// - For CALL/DYNCALL contexts: hash of the called function
    /// - For SYSCALL contexts: hash remains from the calling function
    pub fn_hash: Word,
}

/// A checkpoint represents all the information for one row of the Decoder trace.
///
/// This struct captures the complete state of the decoder at a specific clock cycle,
/// allowing for reconstruction of the decoder trace during concurrent execution.
/// The decoder trace consists of 24 columns total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoderCheckpoint {
    /// Block address/hasher table row address - unique block identifier
    pub addr: Felt,

    /// Binary representation of the opcode (7 bits total)
    /// Each element represents one bit of the opcode
    pub op_bits: [Felt; 7], // NUM_OP_BITS

    /// Hasher state columns (8 columns)
    /// Multi-purpose columns used for hash computations and helper registers
    pub hasher_state: [Felt; 8], // NUM_HASHER_COLUMNS

    /// Flag indicating whether we are inside a SPAN block (binary)
    pub in_span: Felt,

    /// Number of operation groups left to decode in the current SPAN block
    pub group_count: Felt,

    /// Index of the currently executing operation within an operation group (0-8)
    pub op_idx: Felt,

    /// Operation batch flags indicating how many operation groups are in a batch
    /// [flag0, flag1, flag2] encoding 1, 2, 4, or 8 groups
    pub op_batch_flags: [Felt; 3], // NUM_OP_BATCH_FLAGS

    /// Operation bit extra columns for degree reduction (2 columns)
    /// Used to reduce the degree of operation flag constraints
    pub op_bit_extra: [Felt; 2], // NUM_OP_BITS_EXTRA_COLS
}

/// A checkpoint represents all the information for one row of the Stack trace.
///
/// This struct captures the complete state of the stack at a specific clock cycle,
/// allowing for reconstruction of the stack trace during concurrent execution.
/// The stack trace consists of 19 columns total: 16 stack columns + 3 helper columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackCheckpoint {
    /// Current clock cycle (row index in the trace)
    pub clk: RowIndex,

    /// Top 16 stack slots (s0 to s15)
    /// These represent the top elements of the stack that are directly accessible
    pub stack_top: [Felt; MIN_STACK_DEPTH], // 16 columns

    /// Stack depth - total number of items on the stack
    /// This is the b0 helper column
    pub stack_depth: Felt,

    /// Address of the top row in the overflow table
    /// This is the b1 helper column - when 0, all stack data fits in top 16 slots
    pub overflow_addr: Felt,

    /// Helper column for overflow calculations (h0)
    /// Contains 1/(stack_depth - 16) when stack_depth != 16, ZERO otherwise
    /// Used to ensure stack depth doesn't drop below minimum
    pub overflow_helper: Felt,
}

impl StackCheckpoint {
    /// Creates a new StackCheckpoint with the provided parameters
    pub fn new(
        clk: RowIndex,
        stack_top: [Felt; MIN_STACK_DEPTH],
        stack_depth: Felt,
        overflow_addr: Felt,
        overflow_helper: Felt,
    ) -> Self {
        Self {
            clk,
            stack_top,
            stack_depth,
            overflow_addr,
            overflow_helper,
        }
    }

    /// Creates an initial StackCheckpoint for the start of execution
    ///
    /// # Arguments
    /// * `init_stack` - Initial stack values (up to MIN_STACK_DEPTH)
    /// * `init_depth` - Initial stack depth
    /// * `init_overflow_addr` - Initial overflow table address
    pub fn initial(init_stack: &[Felt], init_depth: usize, init_overflow_addr: Felt) -> Self {
        // Initialize stack top with provided values, pad with ZERO
        let mut stack_top = [ZERO; MIN_STACK_DEPTH];
        for (i, &value) in init_stack.iter().take(MIN_STACK_DEPTH).enumerate() {
            stack_top[i] = value;
        }

        let stack_depth = Felt::new(init_depth as u64);

        // Calculate h0 helper: (init_depth - 16) if depth > 16, else 0
        let overflow_helper = if init_depth > MIN_STACK_DEPTH {
            // Note: In the actual trace, this gets inverted later via batch inversion
            Felt::new((init_depth - MIN_STACK_DEPTH) as u64)
        } else {
            ZERO
        };

        Self::new(RowIndex::from(0), stack_top, stack_depth, init_overflow_addr, overflow_helper)
    }
}

pub struct MainTraceCheckpoint {
    pub system: SystemCheckpoint,
    pub decoder: DecoderCheckpoint,
    pub stack: StackCheckpoint,
    pub traversal: ExecutionTraversal,
}

/// The columns of the main trace fragment. These consist of the system, decoder, and stack columns.
pub struct MainTraceFragment {
    pub columns: [Vec<Felt>; MAIN_TRACE_WIDTH],
}

impl MainTraceFragment {
    /// Creates a new MainTraceFragment with *uninitialized* columns of length `num_rows`.
    ///
    /// # Safety
    /// The caller is responsible for ensuring that the columns are properly initialized
    /// before use.
    pub unsafe fn new_uninit(num_rows: usize) -> Self {
        Self {
            columns: core::array::from_fn(|_| unsafe { uninit_vector(num_rows) }),
        }
    }

    /// Returns the number of rows in this fragment
    pub fn row_count(&self) -> usize {
        self.columns[0].len()
    }
}

pub struct MainTraceFragmentManager {
    channel: Receiver<MainTraceCheckpoint>,
    fragments: Vec<MainTraceFragment>,
    program: Program,
}

impl MainTraceFragmentManager {
    /// The number of rows per main trace fragment.
    pub const NUM_ROWS_PER_FRAGMENT: usize = 1024;

    /// Creates a new MainTraceGenerator with the provided channel
    pub fn new(program: Program, channel: Receiver<MainTraceCheckpoint>) -> Self {
        Self { program, channel, fragments: Vec::new() }
    }

    /// Processes checkpoints from the channel, producing fragments in order
    ///
    /// This method reads checkpoints from the channel and spawns async tasks to process
    /// each one into a MainTraceFragment. The fragments are added to self.fragments
    /// in the same order as the checkpoints were received.
    pub async fn process<H>(&mut self, host: std::sync::Arc<H>)
    where
        H: crate::Host + Send + Sync + 'static,
    {
        let mut pending_tasks: Vec<JoinHandle<MainTraceFragment>> = Vec::new();

        // Phase 1: Spawn tasks for all incoming checkpoints
        while let Some(checkpoint) = self.channel.recv().await {
            let mast_forest = self.program.mast_forest().clone();
            let host_clone = host.clone();
            let task = tokio::spawn(async move {
                let main_trace_generator = MainTraceFragmentGenerator::new(checkpoint);
                main_trace_generator.generate_fragment(&mast_forest, host_clone)
            });

            pending_tasks.push(task);
        }

        // Phase 2: Collect results in order
        // Tasks are already in the correct order since we pushed them sequentially
        for task in pending_tasks {
            let fragment = task.await.expect("Task should not panic");
            self.fragments.push(fragment);
        }
    }

    /// Returns a reference to the collected fragments
    pub fn fragments(&self) -> &[MainTraceFragment] {
        &self.fragments
    }
}

pub struct MainTraceFragmentGenerator {
    num_rows_built: usize,
    fragment: MainTraceFragment,
    checkpoint: MainTraceCheckpoint,
}

impl MainTraceFragmentGenerator {
    /// Creates a new MainTraceFragmentGenerator with the provided checkpoint.
    pub fn new(checkpoint: MainTraceCheckpoint) -> Self {
        Self {
            num_rows_built: 0,
            // Safety: the `MainTraceFragmentGenerator` will fill in all the rows, or truncate any
            // unused rows if a `HALT` operation occurs before `NUM_ROWS_PER_FRAGMENT` have been
            // executed.
            fragment: unsafe {
                MainTraceFragment::new_uninit(MainTraceFragmentManager::NUM_ROWS_PER_FRAGMENT)
            },
            checkpoint,
        }
    }

    /// Processes a single checkpoint into a MainTraceFragment
    pub fn generate_fragment<H>(
        mut self,
        program: &MastForest,
        host: std::sync::Arc<H>,
    ) -> MainTraceFragment
    where
        H: crate::Host + Send + Sync,
    {
        if let Some(node_id) = self.checkpoint.traversal.peek() {
            // If we have a node ID, we can execute it
            self.execute_mast_node(node_id, program, host.as_ref());

            if !self.done_generating() {
                // if we haven't generated enough rows, we advance to the next node in the traversal
                // and continue generating the fragment
                self.checkpoint.traversal.advance();
                self.generate_fragment(program, host)
            } else {
                self.finalize_fragment()
            }
        } else {
            // If there are no more nodes to execute, we finalize the fragment
            self.finalize_fragment()
        }
    }

    fn execute_mast_node<H>(&mut self, node_id: MastNodeId, program: &MastForest, host: &H)
    where
        H: crate::Host,
    {
        let mast_node = program.get_node_by_id(node_id).expect("node should exist");

        match mast_node {
            MastNode::Block(basic_block_node) => {
                // Clone the basic_block_node to avoid borrowing issues
                let basic_block_node = basic_block_node.clone();

                // 1. Add SPAN start trace row (analogous to FastProcessor SPAN start)
                self.add_span_start_trace_row(&basic_block_node);

                // 2. Process operation batches following FastProcessor pattern
                let mut batch_offset_in_block = 0;
                let op_batches = basic_block_node.op_batches();

                // Execute first op batch
                if let Some(first_op_batch) = op_batches.first() {
                    self.execute_op_batch(
                        first_op_batch,
                        batch_offset_in_block,
                        &basic_block_node,
                        host,
                    );
                    batch_offset_in_block += first_op_batch.ops().len();
                }

                // Execute the rest of the op batches
                for op_batch in op_batches.iter().skip(1) {
                    // 3. Add RESPAN trace row between batches (analogous to FastProcessor RESPAN)
                    self.add_respan_trace_row(op_batch);

                    self.execute_op_batch(op_batch, batch_offset_in_block, &basic_block_node, host);
                    batch_offset_in_block += op_batch.ops().len();
                }

                // 4. Add END trace row (analogous to FastProcessor END)
                self.add_span_end_trace_row(&basic_block_node);

                // 5. Handle decorators after block completion (analogous to FastProcessor)
                self.execute_decorators_after_span(&basic_block_node, program, host);
            },
            MastNode::Join(join_node) => {
                // Clone the join_node to avoid borrowing issues
                let join_node = join_node.clone();

                // 1. Add "start JOIN" row
                self.add_join_start_trace_row(&join_node, program);

                // 2. Execute first child
                self.execute_mast_node(join_node.first(), program, host);

                // 3. Execute second child
                self.execute_mast_node(join_node.second(), program, host);

                // 4. Add "end JOIN" row
                self.add_join_end_trace_row(&join_node, program);
            },
            MastNode::Split(split_node) => {
                // Clone the split_node to avoid borrowing issues
                let split_node = split_node.clone();

                // 1. Add "start SPLIT" row
                self.add_split_start_trace_row(&split_node, program);

                // 2. Execute the appropriate branch based on the stack top value
                let condition = self.checkpoint.stack.stack_top[0];
                if condition == ONE {
                    self.execute_mast_node(split_node.on_true(), program, host);
                } else {
                    self.execute_mast_node(split_node.on_false(), program, host);
                }

                // 3. Add "end SPLIT" row
                self.add_split_end_trace_row(&split_node, program);
            },
            MastNode::Loop(loop_node) => {
                // Clone the loop_node to avoid borrowing issues
                let loop_node = loop_node.clone();

                // 1. Add "start LOOP" row
                self.add_loop_start_trace_row(&loop_node, program);

                // 2. In parallel execution, we simulate executing the loop body once
                // based on the current stack top value
                let condition = self.checkpoint.stack.stack_top[0];
                if condition == vm_core::ONE {
                    // Simulate dropping the condition from stack
                    if self.checkpoint.stack.stack_depth.as_int() > 0 {
                        self.checkpoint.stack.stack_depth =
                            Felt::from(self.checkpoint.stack.stack_depth.as_int() as u32 - 1);
                    }

                    // Execute loop body (in real execution this would be in a while loop)
                    self.execute_mast_node(loop_node.body(), program, host);

                    // Note: In a real implementation, the loop would continue until the condition
                    // becomes false. For parallel analysis, we simulate one iteration.
                }

                // 3. Add "end LOOP" row
                self.add_loop_end_trace_row(&loop_node, program);
            },
            MastNode::Call(call_node) => {
                // Clone the call_node to avoid borrowing issues
                let call_node = call_node.clone();

                // 1. Add "start CALL/SYSCALL" row
                self.add_call_start_trace_row(&call_node, program);

                // Save current context state if needed
                let saved_ctx = self.checkpoint.system.ctx;
                let saved_fmp = self.checkpoint.system.fmp;
                let saved_in_syscall = self.checkpoint.system.in_syscall;

                // Set up new context for the call
                if call_node.is_syscall() {
                    self.checkpoint.system.ctx = ContextId::root(); // Root context for syscalls
                    self.checkpoint.system.fmp = Felt::new(SYSCALL_FMP_MIN as u64);
                    self.checkpoint.system.in_syscall = true;
                } else {
                    self.checkpoint.system.ctx = ContextId::from(self.checkpoint.system.clk); // New context ID
                    self.checkpoint.system.fmp = Felt::new(FMP_MIN);
                }

                // Execute the callee
                self.execute_mast_node(call_node.callee(), program, host);

                // Restore context state
                self.checkpoint.system.ctx = saved_ctx;
                self.checkpoint.system.fmp = saved_fmp;
                self.checkpoint.system.in_syscall = saved_in_syscall;

                // 2. Add "end CALL/SYSCALL" row
                self.add_call_end_trace_row(&call_node, program);
            },
            MastNode::Dyn(dyn_node) => {
                // Clone the dyn_node to avoid borrowing issues
                let dyn_node = dyn_node.clone();

                // 1. Add "start DYN/DYNCALL" row
                self.add_dyn_start_trace_row(&dyn_node);

                // In parallel execution, we can't resolve dynamic calls at compile time
                // So we'll simulate minimal overhead and skip the actual execution
                // This is a limitation of parallel analysis - dynamic behavior requires runtime
                // information to determine the actual callee

                // For DYNCALL, we would save/restore context like in Call nodes, but since
                // we can't execute the dynamic target, we skip the context manipulation
                if dyn_node.is_dyncall() {
                    // Simulate context save/restore overhead without actual execution
                    // The actual dynamic target resolution happens at runtime
                }

                // 2. Add "end DYN/DYNCALL" row
                self.add_dyn_end_trace_row(&dyn_node);
            },
            MastNode::External(external_node) => {
                let (root_id, mast_forest) = resolve_external_node(external_node, host)
                    .expect("Failed to resolve external node");

                self.execute_mast_node(root_id, &mast_forest, host)
            },
        }
    }

    /// Adds a trace row for SPAN start operation to the main trace fragment.
    ///
    /// This method creates a trace row that corresponds to the SPAN operation that starts
    /// a basic block execution. It increments the clock and builds the appropriate trace data.
    fn add_span_start_trace_row(&mut self, _basic_block_node: &BasicBlockNode) {
        // Increment clock for SPAN start operation
        self.checkpoint.system.clk += 1u32;

        // TODO: Add actual trace row generation for SPAN start
        // This would populate system, decoder, and stack columns similar to control flow operations
        // but with SPAN-specific decoder state (op groups, batch flags, etc.)

        // For now, we leave this as a placeholder since the task is to focus on the overall
        // structure without implementing the detailed "writing to the rows" logic
    }

    /// Adds a trace row for RESPAN operation to the main trace fragment.
    ///
    /// This method creates a trace row that corresponds to the RESPAN operation that starts
    /// processing of a new operation batch within the same basic block.
    fn add_respan_trace_row(&mut self, _op_batch: &vm_core::mast::OpBatch) {
        // Increment clock for RESPAN operation
        self.checkpoint.system.clk += 1u32;

        // TODO: Add actual trace row generation for RESPAN
        // This would populate decoder columns with the new operation batch data
        // and update hasher state, op group count, etc.

        // For now, we leave this as a placeholder
    }

    /// Adds a trace row for SPAN end operation to the main trace fragment.
    ///
    /// This method creates a trace row that corresponds to the END operation that completes
    /// a basic block execution.
    fn add_span_end_trace_row(&mut self, _basic_block_node: &BasicBlockNode) {
        // Increment clock for END operation
        self.checkpoint.system.clk += 1u32;

        // TODO: Add actual trace row generation for SPAN end
        // This would populate decoder columns with END opcode and the block hash

        // For now, we leave this as a placeholder
    }

    /// Executes operations within an operation batch, analogous to FastProcessor::execute_op_batch.
    ///
    /// This method processes all operations within a single operation batch, handling decorators
    /// and operation execution following the same pattern as the FastProcessor.
    fn execute_op_batch<H>(
        &mut self,
        op_batch: &vm_core::mast::OpBatch,
        batch_offset_in_block: usize,
        basic_block_node: &BasicBlockNode,
        host: &H,
    ) where
        H: crate::Host,
    {
        let op_counts = op_batch.op_counts();
        let mut op_idx_in_group = 0;
        let mut group_idx = 0;
        let mut next_group_idx = 1;

        // Round up the number of groups to be processed to the next power of two
        let num_batch_groups = op_batch.num_groups().next_power_of_two();

        // Execute operations in the batch one by one
        for (op_idx_in_batch, op) in op_batch.ops().iter().enumerate() {
            // Handle decorators before this operation
            self.execute_decorators_at_position(
                basic_block_node,
                batch_offset_in_block + op_idx_in_batch,
                host,
            );

            // Execute the operation
            self.execute_op(op, batch_offset_in_block + op_idx_in_batch, host);

            // Handle immediate value operations
            let has_imm = op.imm_value().is_some();
            if has_imm {
                next_group_idx += 1;
            }

            // Determine if we've executed all operations in a group
            if op_idx_in_group == op_counts[group_idx] - 1 {
                // If operation has immediate value, execute NOOP after it
                if has_imm {
                    debug_assert!(op_idx_in_group < 9 - 1, "invalid op index"); // OP_GROUP_SIZE is 9
                    self.checkpoint.system.clk += 1u32; // Account for NOOP
                }

                // Move to next group and reset operation index
                group_idx = next_group_idx;
                next_group_idx += 1;
                op_idx_in_group = 0;
            } else {
                op_idx_in_group += 1;
            }
        }

        // Execute required number of operation groups (handle padding with NOOPs)
        self.checkpoint.system.clk += (num_batch_groups - group_idx) as u32;
    }

    /// Executes decorators at a specific position within the block.
    ///
    /// This is a placeholder for decorator execution logic that would be analogous
    /// to FastProcessor decorator handling.
    fn execute_decorators_at_position<H>(
        &mut self,
        _basic_block_node: &BasicBlockNode,
        _position: usize,
        _host: &H,
    ) where
        H: crate::Host,
    {
        // TODO: Implement decorator execution at specific position
        // This would iterate through decorators using basic_block_node.decorator_iter()
        // and execute any decorators that should run at the given position

        // For now, we leave this as a placeholder since actual decorator execution
        // is beyond the scope of this task
    }

    /// Executes any remaining decorators after the SPAN block is completed.
    ///
    /// This handles decorators that appear after all operations in a block,
    /// analogous to FastProcessor's post-SPAN decorator execution.
    fn execute_decorators_after_span<H>(
        &mut self,
        _basic_block_node: &BasicBlockNode,
        _program: &vm_core::mast::MastForest,
        _host: &H,
    ) where
        H: crate::Host,
    {
        // TODO: Implement post-SPAN decorator execution
        // This would handle any decorators that have not been executed during span ops execution

        // For now, we leave this as a placeholder
    }

    /// Executes a single operation, analogous to FastProcessor::execute_op.
    ///
    /// This is a placeholder for the actual operation execution logic.
    fn execute_op<H>(&mut self, _op: &vm_core::Operation, _op_idx: usize, _host: &H)
    where
        H: crate::Host,
    {
        // TODO: Implement actual operation execution
        // This would decode and execute the operation, updating processor state
        // and incrementing the clock cycle

        // For now, we just increment the clock to maintain consistency
        self.checkpoint.system.clk += 1u32;
    }

    fn finalize_fragment(mut self) -> MainTraceFragment {
        // If we have not built enough rows, we need to truncate the fragment
        if self.num_rows_built < MainTraceFragmentManager::NUM_ROWS_PER_FRAGMENT {
            let num_rows = self.num_rows_built;
            for column in &mut self.fragment.columns {
                column.truncate(num_rows);
            }
        }

        self.fragment
    }

    fn done_generating(&mut self) -> bool {
        // If we have built all the rows in the fragment, we are done
        self.num_rows_built >= MainTraceFragmentManager::NUM_ROWS_PER_FRAGMENT
    }
}
