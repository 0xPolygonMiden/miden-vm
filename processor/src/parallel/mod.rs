use alloc::{sync::Arc, vec::Vec};
use core::ops::ControlFlow;

use miden_air::{
    RowIndex,
    trace::{DECODER_TRACE_WIDTH, STACK_TRACE_WIDTH, SYS_TRACE_WIDTH},
};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};
use traversal::ExecutionTraversal;
use vm_core::{
    Felt, ONE, Operation, Program, Word, ZERO,
    mast::{BasicBlockNode, MastForest, MastNode, MastNodeId},
    stack::MIN_STACK_DEPTH,
    utils::uninit_vector,
};

use crate::{
    ContextId,
    decoder::BlockStack,
    resolve_external_node,
    stack::OverflowTable,
    system::{FMP_MIN, SYSCALL_FMP_MIN},
};

pub const MAIN_TRACE_WIDTH: usize = SYS_TRACE_WIDTH + DECODER_TRACE_WIDTH + STACK_TRACE_WIDTH;

mod call;
mod r#dyn;
mod join;
mod r#loop;
mod operations;

mod replay;
use replay::{AdviceReplay, HasherReplay, MemoryReplay};

mod split;
mod trace_builder;
mod traversal;

// TRACE ROW TYPE
// ================================================================================================

/// Enum to specify whether this is a start or end trace row for control block operations
/// (JOIN, SPLIT, LOOP, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceRowType {
    Start,
    End,
}

// NODE EXECUTION PHASE
// ================================================================================================

/// Enum to specify the execution phase when starting fragment generation.
///
/// This replaces the previous `Option<(usize, usize)>` parameter to provide clearer
/// semantics for different types of execution resumption scenarios.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeExecutionPhase {
    /// Resume execution within a basic block at a specific batch and operation index.
    /// This is used when continuing execution mid-way through a basic block.
    BasicBlock {
        /// Index of the operation batch within the basic block
        batch_index: usize,
        /// Index of the operation within the batch
        op_index_in_batch: usize,
    },
    /// Execute the START phase of a control flow node (JOIN, SPLIT, LOOP, etc.).
    /// This is used when beginning execution of a control flow construct.
    Start,
    /// Execute the END phase of a control flow node (JOIN, SPLIT, LOOP, etc.).
    /// This is used when completing execution of a control flow construct.
    End,
}

// STATE STRUCTS
// ================================================================================================

/// The `SystemState` represents all the information needed to build one row of the System trace.
///
/// This struct captures the complete state of the system at a specific clock cycle,
/// allowing for reconstruction of the system trace during concurrent execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemState {
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

/// The `DecoderState` represents all the information needed to build one row of the Decoder trace.
///
/// This struct captures the complete state of the decoder at a specific clock cycle,
/// allowing for reconstruction of the decoder trace during concurrent execution.
/// The decoder trace consists of 24 columns total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoderState {
    /// Block address/hasher table row address - unique block identifier
    pub addr: Felt,

    /// Operation being executed at this clock cycle
    pub operation: Operation,

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
}

/// A checkpoint represents all the information for one row of the Stack trace.
///
/// This struct captures the complete state of the stack at a specific clock cycle,
/// allowing for reconstruction of the stack trace during concurrent execution.
/// The stack trace consists of 19 columns total: 16 stack columns + 3 helper columns.
/// The helper columns (stack_depth, overflow_addr, and overflow_helper) are derived from the
/// OverflowTable.
#[derive(Debug)]
pub struct StackState {
    /// Top 16 stack slots (s0 to s15)
    /// These represent the top elements of the stack that are directly accessible
    pub stack_top: [Felt; MIN_STACK_DEPTH], // 16 columns

    /// Overflow table containing all stack elements beyond the top 16
    /// Used to derive the helper columns (b0, b1, h0) for the stack trace
    pub overflow: OverflowTable,
}

impl StackState {
    /// Creates a new StackState with the provided parameters
    pub fn new(stack_top: [Felt; MIN_STACK_DEPTH], overflow: OverflowTable) -> Self {
        Self { stack_top, overflow }
    }

    /// Derives the stack depth (b0 helper column) from the overflow table
    pub fn stack_depth(&self) -> Felt {
        Felt::new((MIN_STACK_DEPTH + self.overflow.total_num_elements()) as u64)
    }

    /// Derives the overflow address (b1 helper column) from the overflow table
    pub fn overflow_addr(&self) -> Felt {
        self.overflow.last_update_clk_in_current_ctx()
    }

    /// Derives the overflow helper (h0 helper column) from the current stack depth
    pub fn overflow_helper(&self) -> Felt {
        let stack_depth = self.stack_depth();
        let depth_value = stack_depth.as_int() as usize;

        if depth_value > MIN_STACK_DEPTH {
            // Note: In the actual trace, this gets inverted later via batch inversion
            Felt::new((depth_value - MIN_STACK_DEPTH) as u64)
        } else {
            ZERO
        }
    }
}

pub struct MainTraceState {
    pub system: SystemState,
    pub decoder: DecoderState,
    pub stack: StackState,
    pub block_stack: BlockStack,
    pub traversal: ExecutionTraversal,
    pub memory: MemoryReplay,
    pub advice: AdviceReplay,
    pub hasher: HasherReplay,
}

// MAIN TRACE FRAGMENT AND MANAGER
// ================================================================================================

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
    /// Channel to receive execution state and phase information for fragment generation
    channel: Receiver<(MainTraceState, NodeExecutionPhase)>,
    fragments: Vec<MainTraceFragment>,
    program: Program,
}

impl MainTraceFragmentManager {
    /// The number of rows per main trace fragment.
    pub const NUM_ROWS_PER_FRAGMENT: usize = 1024;

    /// Creates a new MainTraceGenerator with the provided channel
    pub fn new(program: Program, channel: Receiver<(MainTraceState, NodeExecutionPhase)>) -> Self {
        Self { program, channel, fragments: Vec::new() }
    }

    /// Processes checkpoints from the channel, producing fragments in order
    ///
    /// This method reads checkpoints from the channel and spawns async tasks to process
    /// each one into a MainTraceFragment. The fragments are added to self.fragments
    /// in the same order as the checkpoints were received.
    pub async fn process<H>(&mut self, host: Arc<H>)
    where
        H: crate::Host + Send + Sync + 'static,
    {
        let mut pending_tasks: Vec<JoinHandle<MainTraceFragment>> = Vec::new();

        // Phase 1: Spawn tasks for all incoming checkpoints
        while let Some((main_trace_state, execution_phase)) = self.channel.recv().await {
            let mast_forest = self.program.mast_forest().clone();
            let host_clone = host.clone();
            let task = tokio::task::spawn_blocking(move || {
                let main_trace_generator = MainTraceFragmentGenerator::new(main_trace_state);
                main_trace_generator.generate_fragment(&mast_forest, host_clone, execution_phase)
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
    fragment_start_clk: RowIndex,
    fragment: MainTraceFragment,
    state: MainTraceState,
}

impl MainTraceFragmentGenerator {
    /// Creates a new MainTraceFragmentGenerator with the provided checkpoint.
    pub fn new(state: MainTraceState) -> Self {
        Self {
            fragment_start_clk: state.system.clk,
            // Safety: the `MainTraceFragmentGenerator` will fill in all the rows, or truncate any
            // unused rows if a `HALT` operation occurs before `NUM_ROWS_PER_FRAGMENT` have been
            // executed.
            fragment: unsafe {
                MainTraceFragment::new_uninit(MainTraceFragmentManager::NUM_ROWS_PER_FRAGMENT)
            },
            state,
        }
    }

    /// Processes a single checkpoint into a MainTraceFragment
    pub fn generate_fragment<H>(
        mut self,
        program: &MastForest,
        host: Arc<H>,
        execution_phase: NodeExecutionPhase,
    ) -> MainTraceFragment
    where
        H: crate::Host + Send + Sync,
    {
        // Execute fragment generation and always finalize at the end
        let _ = self.execute_fragment_generation(program, host.as_ref(), execution_phase);
        self.finalize_fragment()
    }

    /// Internal method that performs fragment generation with automatic early returns
    fn execute_fragment_generation<H>(
        &mut self,
        program: &MastForest,
        host: &H,
        execution_phase: NodeExecutionPhase,
    ) -> ControlFlow<()>
    where
        H: crate::Host,
    {
        match execution_phase {
            NodeExecutionPhase::BasicBlock { batch_index, op_index_in_batch } => {
                // Finish running the current basic block from the specified batch and operation
                // index
                if let Some(node_id) = self.state.traversal.peek() {
                    let mast_node = program.get_node_by_id(node_id).expect("node should exist");
                    if let MastNode::Block(basic_block_node) = mast_node {
                        let basic_block_node = basic_block_node.clone();
                        let op_batches = basic_block_node.op_batches();

                        // Resume execution from the specified batch
                        if batch_index < op_batches.len() {
                            let mut batch_offset_in_block = 0;

                            // Calculate the batch offset by summing operations in previous batches
                            for i in 0..batch_index {
                                batch_offset_in_block += op_batches[i].ops().len();
                            }

                            // Execute remaining operations in the specified batch
                            let current_batch = &op_batches[batch_index];
                            if op_index_in_batch < current_batch.ops().len() {
                                self.execute_op_batch(
                                    current_batch,
                                    batch_offset_in_block,
                                    &basic_block_node,
                                    host,
                                    Some(op_index_in_batch),
                                )?;
                                batch_offset_in_block += current_batch.ops().len();
                            }

                            // Execute remaining batches
                            for op_batch in op_batches.iter().skip(batch_index + 1) {
                                self.add_respan_trace_row(op_batch)?;
                                self.execute_op_batch(
                                    op_batch,
                                    batch_offset_in_block,
                                    &basic_block_node,
                                    host,
                                    None,
                                )?;
                                batch_offset_in_block += op_batch.ops().len();
                            }

                            // Add END trace row to complete the basic block
                            self.add_span_end_trace_row(&basic_block_node)?;
                        }
                    }

                    // Advance the traversal past the current basic block
                    self.state.traversal.advance();
                }
            },
            NodeExecutionPhase::Start => {
                // do nothing
            },
            NodeExecutionPhase::End => {
                // Handle END phase of control flow nodes
                // Add the appropriate end trace row based on the current node type
                if let Some(node_id) = self.state.traversal.peek() {
                    let mast_node = program.get_node_by_id(node_id).expect("node should exist");

                    match mast_node {
                        MastNode::Block(basic_block_node) => {
                            self.add_span_end_trace_row(basic_block_node)?;
                        },
                        MastNode::Join(join_node) => {
                            self.add_join_end_trace_row(join_node, program)?;
                        },
                        MastNode::Split(split_node) => {
                            self.add_split_end_trace_row(split_node, program)?;
                        },
                        MastNode::Loop(loop_node) => {
                            self.add_loop_end_trace_row(loop_node, program)?;
                        },
                        MastNode::Call(call_node) => {
                            self.add_call_end_trace_row(call_node, program)?;
                        },
                        MastNode::Dyn(dyn_node) => {
                            self.add_dyn_end_trace_row(dyn_node)?;
                        },
                        MastNode::External(_external_node) => {
                            // External nodes don't have their own end trace rows
                            // as they resolve to other nodes
                            // TODO(plafer): Should this be unreachable?
                        },
                    }

                    // Advance the traversal past the current node
                    self.state.traversal.advance();
                }
            },
        }

        while let Some(node_id) = self.state.traversal.peek() {
            // Execute the node - this will return early if fragment is complete
            self.execute_mast_node(node_id, program, host)?;

            // If we reach here, the node completed without filling the fragment
            self.state.traversal.advance();
        }

        // All nodes completed without filling the fragment
        ControlFlow::Continue(())
    }

    fn execute_mast_node<H>(
        &mut self,
        node_id: MastNodeId,
        program: &MastForest,
        host: &H,
    ) -> ControlFlow<()>
    where
        H: crate::Host,
    {
        let mast_node = program.get_node_by_id(node_id).expect("node should exist");

        // Set the address of the new block
        self.state.decoder.addr = self.state.hasher.replay_block_address();

        match mast_node {
            MastNode::Block(basic_block_node) => {
                // Clone the basic_block_node to avoid borrowing issues
                let basic_block_node = basic_block_node.clone();

                // 1. Add SPAN start trace row (analogous to FastProcessor SPAN start)
                self.add_span_start_trace_row(&basic_block_node)?;

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
                        None,
                    )?;
                    batch_offset_in_block += first_op_batch.ops().len();
                }

                // Execute the rest of the op batches
                for op_batch in op_batches.iter().skip(1) {
                    // 3. Add RESPAN trace row between batches (analogous to FastProcessor RESPAN)
                    self.add_respan_trace_row(op_batch)?;

                    self.execute_op_batch(
                        op_batch,
                        batch_offset_in_block,
                        &basic_block_node,
                        host,
                        None,
                    )?;
                    batch_offset_in_block += op_batch.ops().len();
                }

                // 4. Add END trace row (analogous to FastProcessor END)
                self.add_span_end_trace_row(&basic_block_node)?;

                // 5. Handle decorators after block completion (analogous to FastProcessor)
                self.execute_decorators_after_span(&basic_block_node, program, host);

                ControlFlow::Continue(())
            },
            MastNode::Join(join_node) => {
                // Clone the join_node to avoid borrowing issues
                let join_node = join_node.clone();

                // 1. Add "start JOIN" row
                self.add_join_start_trace_row(&join_node, program)?;

                // 2. Execute first child
                self.execute_mast_node(join_node.first(), program, host)?;

                // 3. Execute second child
                self.execute_mast_node(join_node.second(), program, host)?;

                // 4. Add "end JOIN" row
                self.add_join_end_trace_row(&join_node, program)
            },
            MastNode::Split(split_node) => {
                // Clone the split_node to avoid borrowing issues
                let split_node = split_node.clone();

                // 1. Add "start SPLIT" row
                self.add_split_start_trace_row(&split_node, program)?;

                // 2. Execute the appropriate branch based on the stack top value
                let condition = self.state.stack.stack_top[0];
                if condition == ONE {
                    self.execute_mast_node(split_node.on_true(), program, host)?;
                } else {
                    self.execute_mast_node(split_node.on_false(), program, host)?;
                }

                // 3. Add "end SPLIT" row
                self.add_split_end_trace_row(&split_node, program)
            },
            MastNode::Loop(loop_node) => {
                // Clone the loop_node to avoid borrowing issues
                let loop_node = loop_node.clone();

                // 1. Add "start LOOP" row
                self.add_loop_start_trace_row(&loop_node, program)?;

                // 2. In parallel execution, we simulate executing the loop body once
                // based on the current stack top value
                let condition = self.state.stack.stack_top[0];
                if condition == vm_core::ONE {
                    // Note: In the refactored version, we don't directly modify stack depth
                    // as it's derived from the overflow table. The stack manipulation would
                    // be handled by proper stack operations in a full implementation.

                    // Execute loop body (in real execution this would be in a while loop)
                    self.execute_mast_node(loop_node.body(), program, host)?;

                    // Note: In a real implementation, the loop would continue until the condition
                    // becomes false. For parallel analysis, we simulate one iteration.
                }

                // 3. Add "end LOOP" row
                self.add_loop_end_trace_row(&loop_node, program)
            },
            MastNode::Call(call_node) => {
                // Clone the call_node to avoid borrowing issues
                let call_node = call_node.clone();

                // 1. Add "start CALL/SYSCALL" row
                self.add_call_start_trace_row(&call_node, program)?;

                // Save current context state if needed
                let saved_ctx = self.state.system.ctx;
                let saved_fmp = self.state.system.fmp;
                let saved_in_syscall = self.state.system.in_syscall;

                // Set up new context for the call
                if call_node.is_syscall() {
                    self.state.system.ctx = ContextId::root(); // Root context for syscalls
                    self.state.system.fmp = Felt::new(SYSCALL_FMP_MIN as u64);
                    self.state.system.in_syscall = true;
                } else {
                    self.state.system.ctx = ContextId::from(self.state.system.clk); // New context ID
                    self.state.system.fmp = Felt::new(FMP_MIN);
                }

                // Execute the callee
                self.execute_mast_node(call_node.callee(), program, host)?;

                // Restore context state
                self.state.system.ctx = saved_ctx;
                self.state.system.fmp = saved_fmp;
                self.state.system.in_syscall = saved_in_syscall;

                // 2. Add "end CALL/SYSCALL" row
                self.add_call_end_trace_row(&call_node, program)
            },
            MastNode::Dyn(dyn_node) => {
                // Clone the dyn_node to avoid borrowing issues
                let dyn_node = dyn_node.clone();

                // 1. Add "start DYN/DYNCALL" row
                self.add_dyn_start_trace_row(&dyn_node)?;

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
                self.add_dyn_end_trace_row(&dyn_node)
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
    fn add_span_start_trace_row(&mut self, _basic_block_node: &BasicBlockNode) -> ControlFlow<()> {
        // TODO: Add actual trace row generation for SPAN start
        // This would populate system, decoder, and stack columns similar to control flow operations
        // but with SPAN-specific decoder state (op groups, batch flags, etc.)

        // For now, we leave this as a placeholder since the task is to focus on the overall
        // structure without implementing the detailed "writing to the rows" logic

        self.increment_clk()
    }

    /// Adds a trace row for RESPAN operation to the main trace fragment.
    ///
    /// This method creates a trace row that corresponds to the RESPAN operation that starts
    /// processing of a new operation batch within the same basic block.
    fn add_respan_trace_row(&mut self, _op_batch: &vm_core::mast::OpBatch) -> ControlFlow<()> {
        // TODO: Add actual trace row generation for RESPAN
        // This would populate decoder columns with the new operation batch data
        // and update hasher state, op group count, etc.

        // For now, we leave this as a placeholder

        // Check if we're done generating and return early if so
        self.increment_clk()
    }

    /// Adds a trace row for SPAN end operation to the main trace fragment.
    ///
    /// This method creates a trace row that corresponds to the END operation that completes
    /// a basic block execution.
    fn add_span_end_trace_row(&mut self, _basic_block_node: &BasicBlockNode) -> ControlFlow<()> {
        // TODO: Add actual trace row generation for SPAN end
        // This would populate decoder columns with END opcode and the block hash

        self.increment_clk()
    }

    /// Executes operations within an operation batch, analogous to FastProcessor::execute_op_batch.
    ///
    /// This method processes operations within a single operation batch, handling decorators
    /// and operation execution following the same pattern as the FastProcessor.
    ///
    /// If `start_op_idx` is provided, execution begins from that operation index within the batch.
    fn execute_op_batch<H>(
        &mut self,
        op_batch: &vm_core::mast::OpBatch,
        batch_offset_in_block: usize,
        basic_block_node: &BasicBlockNode,
        host: &H,
        start_op_idx: Option<usize>,
    ) -> ControlFlow<()>
    where
        H: crate::Host,
    {
        let op_counts = op_batch.op_counts();
        let mut op_idx_in_group = 0;
        let mut group_idx = 0;
        let mut next_group_idx = 1;
        let start_op_idx = start_op_idx.unwrap_or(0);

        // Find which group and position within group corresponds to start_op_idx
        if start_op_idx > 0 {
            let mut ops_processed = 0;
            for (idx, &count) in op_counts.iter().enumerate() {
                if ops_processed + count > start_op_idx {
                    group_idx = idx;
                    op_idx_in_group = start_op_idx - ops_processed;
                    break;
                }
                ops_processed += count;
                if idx < op_counts.len() - 1 {
                    next_group_idx = idx + 2; // Account for immediate values taking up group slots
                }
            }
        }

        // Round up the number of groups to be processed to the next power of two
        let num_batch_groups = op_batch.num_groups().next_power_of_two();

        // Execute operations in the batch starting from start_op_idx
        for (op_idx_in_batch, op) in op_batch.ops().iter().enumerate().skip(start_op_idx) {
            // Handle decorators before this operation
            self.execute_decorators_at_position(
                basic_block_node,
                batch_offset_in_block + op_idx_in_batch,
                host,
            );

            // Execute the operation and check if we're done generating
            self.execute_op(op, batch_offset_in_block + op_idx_in_batch)?;

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
                    self.increment_clk()?;
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
        self.state.system.clk += (num_batch_groups - group_idx) as u32;

        // Check if we're done generating after padding NOOPs
        if self.done_generating() {
            // If we have reached the maximum, we are done generating
            ControlFlow::Break(())
        } else {
            // Otherwise, we continue generating
            ControlFlow::Continue(())
        }
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

    /// Executes a single operation, similar to Process::execute_op.
    ///
    /// This implementation executes the operation by updating the state and recording
    /// any memory or advice provider operations for parallel trace generation.
    fn execute_op(&mut self, op: &Operation, _op_idx: usize) -> ControlFlow<()> {
        // Execute the operation by dispatching to appropriate operation methods
        self.dispatch_operation(op);

        // Increment clock for the executed operation
        self.increment_clk()
    }

    /// Dispatches the operation to the appropriate execution method
    fn dispatch_operation(&mut self, op: &Operation) {
        use vm_core::Operation;

        match op {
            // ----- system operations ------------------------------------------------------------
            Operation::Noop => {},
            Operation::Assert(_err_code) => self.op_assert(),
            Operation::FmpAdd => self.fmpadd(),
            Operation::FmpUpdate => self.fmpupdate(),
            Operation::SDepth => self.sdepth(),
            Operation::Caller => self.caller(),
            Operation::Clk => self.clk(),
            Operation::Emit(event_id) => self.emit(*event_id),

            // ----- flow control operations ------------------------------------------------------
            // control flow operations are never executed directly
            Operation::Join => unreachable!("control flow operation"),
            Operation::Split => unreachable!("control flow operation"),
            Operation::Loop => unreachable!("control flow operation"),
            Operation::Call => unreachable!("control flow operation"),
            Operation::SysCall => unreachable!("control flow operation"),
            Operation::Dyn => unreachable!("control flow operation"),
            Operation::Dyncall => unreachable!("control flow operation"),
            Operation::Span => unreachable!("control flow operation"),
            Operation::Repeat => unreachable!("control flow operation"),
            Operation::Respan => unreachable!("control flow operation"),
            Operation::End => unreachable!("control flow operation"),
            Operation::Halt => unreachable!("control flow operation"),

            // ----- field operations -------------------------------------------------------------
            Operation::Add => self.op_add(),
            Operation::Neg => self.op_neg(),
            Operation::Mul => self.op_mul(),
            Operation::Inv => self.op_inv(),
            Operation::Incr => self.op_incr(),
            Operation::And => self.op_and(),
            Operation::Or => self.op_or(),
            Operation::Not => self.op_not(),
            Operation::Eq => self.op_eq(),
            Operation::Eqz => self.op_eqz(),
            Operation::Expacc => self.op_expacc(),

            // ----- ext2 operations --------------------------------------------------------------
            Operation::Ext2Mul => self.ext2mul(),

            // ----- u32 operations ---------------------------------------------------------------
            Operation::U32split => self.u32split(),
            Operation::U32add => self.u32add(),
            Operation::U32add3 => self.u32add3(),
            Operation::U32sub => self.u32sub(),
            Operation::U32mul => self.u32mul(),
            Operation::U32madd => self.u32madd(),
            Operation::U32div => self.u32div(),
            Operation::U32and => self.u32and(),
            Operation::U32xor => self.u32xor(),
            Operation::U32assert2(_err_code) => self.u32assert2(),

            // ----- stack manipulation -----------------------------------------------------------
            Operation::Pad => self.pad(),
            Operation::Drop => self.drop(),
            Operation::Dup0 => self.dup(0),
            Operation::Dup1 => self.dup(1),
            Operation::Dup2 => self.dup(2),
            Operation::Dup3 => self.dup(3),
            Operation::Dup4 => self.dup(4),
            Operation::Dup5 => self.dup(5),
            Operation::Dup6 => self.dup(6),
            Operation::Dup7 => self.dup(7),
            Operation::Dup9 => self.dup(9),
            Operation::Dup11 => self.dup(11),
            Operation::Dup13 => self.dup(13),
            Operation::Dup15 => self.dup(15),
            Operation::Swap => self.swap(),
            Operation::SwapW => self.swapw(),
            Operation::SwapW2 => self.swapw2(),
            Operation::SwapW3 => self.swapw3(),
            Operation::SwapDW => self.swapdw(),
            Operation::MovUp2 => self.movup(2),
            Operation::MovUp3 => self.movup(3),
            Operation::MovUp4 => self.movup(4),
            Operation::MovUp5 => self.movup(5),
            Operation::MovUp6 => self.movup(6),
            Operation::MovUp7 => self.movup(7),
            Operation::MovUp8 => self.movup(8),
            Operation::MovDn2 => self.movdn(2),
            Operation::MovDn3 => self.movdn(3),
            Operation::MovDn4 => self.movdn(4),
            Operation::MovDn5 => self.movdn(5),
            Operation::MovDn6 => self.movdn(6),
            Operation::MovDn7 => self.movdn(7),
            Operation::MovDn8 => self.movdn(8),
            Operation::CSwap => self.cswap(),
            Operation::CSwapW => self.cswapw(),

            // ----- input / output ---------------------------------------------------------------
            Operation::Push(value) => self.push(*value),
            Operation::AdvPop => self.advpop(),
            Operation::AdvPopW => self.advpopw(),
            Operation::MLoadW => self.mloadw(),
            Operation::MStoreW => self.mstorew(),
            Operation::MLoad => self.mload(),
            Operation::MStore => self.mstore(),
            Operation::MStream => self.mstream(),
            Operation::Pipe => self.pipe(),

            // ----- cryptographic operations -----------------------------------------------------
            Operation::HPerm => self.hperm(),
            Operation::MpVerify(_err_code) => self.mpverify(),
            Operation::MrUpdate => self.mrupdate(),
            Operation::FriE2F4 => self.fri_ext2fold4(),
            Operation::HornerBase => self.horner_eval_base(),
            Operation::HornerExt => self.horner_eval_ext(),
            Operation::ArithmeticCircuitEval => self.arithmetic_circuit_eval(),
        }
    }

    fn finalize_fragment(mut self) -> MainTraceFragment {
        // If we have not built enough rows, we need to truncate the fragment. Similarly, in the
        // rare case where we built too many rows, we truncate to the correct number of rows (i.e.
        // NUM_ROWS_PER_FRAGMENT).
        {
            let num_rows = core::cmp::min(
                self.num_rows_built(),
                MainTraceFragmentManager::NUM_ROWS_PER_FRAGMENT,
            );
            for column in &mut self.fragment.columns {
                column.truncate(num_rows);
            }
        }

        self.fragment
    }

    fn done_generating(&mut self) -> bool {
        // If we have built all the rows in the fragment, we are done
        self.num_rows_built() >= MainTraceFragmentManager::NUM_ROWS_PER_FRAGMENT
    }

    fn num_rows_built(&self) -> usize {
        // Returns the number of rows built so far in the fragment
        (self.state.system.clk - self.fragment_start_clk) as usize
    }

    fn increment_clk(&mut self) -> ControlFlow<()> {
        self.state.system.clk += 1u32;

        // Check if we have reached the maximum number of rows in the fragment
        if self.done_generating() {
            // If we have reached the maximum, we are done generating
            ControlFlow::Break(())
        } else {
            // Otherwise, we continue generating
            ControlFlow::Continue(())
        }
    }
}
