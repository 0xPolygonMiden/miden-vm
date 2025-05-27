use alloc::vec::Vec;
use core::ops::ControlFlow;

use miden_air::{
    RowIndex,
    trace::{DECODER_TRACE_WIDTH, STACK_TRACE_WIDTH, SYS_TRACE_WIDTH, decoder::NUM_OP_BITS},
};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};
use traversal::ExecutionTraversal;
use vm_core::{
    Felt, ONE, Operation, Program, Word, ZERO,
    mast::{BasicBlockNode, MastForest, MastNode, MastNodeId, OP_GROUP_SIZE, OpBatch},
    stack::MIN_STACK_DEPTH,
    utils::uninit_vector,
};

use crate::{
    ContextId,
    decoder::{
        SpanContext,
        block_stack::{BlockStack, BlockType, ExecutionContextInfo},
    },
    stack::OverflowTable,
    system::{FMP_MIN, SYSCALL_FMP_MIN},
};

pub const MAIN_TRACE_WIDTH: usize = SYS_TRACE_WIDTH + DECODER_TRACE_WIDTH + STACK_TRACE_WIDTH;

mod basic_block;
mod call;
mod r#dyn;
mod join;
mod r#loop;
mod operations;

mod replay;
use replay::{AdviceReplay, ExternalNodeReplay, HasherReplay, MemoryReplay};

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
        op_idx_in_batch: usize,
        /// Whether a RESPAN operation needs to be added before executing this batch. When true,
        /// `batch_index` refers to the batch to be executed *after* the RESPAN operation, and
        /// `op_index_in_batch` MUST be set to 0.
        needs_respan: bool,
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

    pub fn start_context(&mut self) -> (usize, Felt) {
        // Return the current stack depth and overflow address at the start of a new context
        let current_depth = self.stack_depth().as_int() as usize;
        let current_overflow_addr = self.overflow_addr();
        self.overflow.start_context();

        (current_depth, current_overflow_addr)
    }

    pub fn shift_left_and_start_context(&mut self) -> (usize, Felt) {
        // TODO(plafer): shift left

        // Return the current stack depth and overflow address at the start of a new context
        let current_depth = self.stack_depth().as_int() as usize;
        let current_overflow_addr = self.overflow_addr();
        self.overflow.start_context();

        (current_depth, current_overflow_addr)
    }
}

pub struct CoreTraceState {
    pub system: SystemState,
    pub stack: StackState,
    pub block_stack: BlockStack,
    pub traversal: ExecutionTraversal,
    pub memory: MemoryReplay,
    pub advice: AdviceReplay,
    pub hasher: HasherReplay,
    pub external_node_replay: ExternalNodeReplay,
}

// MAIN TRACE FRAGMENT AND MANAGER
// ================================================================================================

/// The columns of the main trace fragment. These consist of the system, decoder, and stack columns.
pub struct CoreTraceFragment {
    pub columns: [Vec<Felt>; MAIN_TRACE_WIDTH],
}

impl CoreTraceFragment {
    /// Creates a new CoreTraceFragment with *uninitialized* columns of length `num_rows`.
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

pub struct CoreTraceFragmentManager {
    /// Channel to receive execution state and phase information for fragment generation
    channel: Receiver<(CoreTraceState, NodeExecutionPhase)>,
    fragments: Vec<CoreTraceFragment>,
    program: Program,
}

impl CoreTraceFragmentManager {
    /// The number of rows per main trace fragment.
    pub const NUM_ROWS_PER_FRAGMENT: usize = 1024;

    /// Creates a new CoreTraceGenerator with the provided channel
    pub fn new(program: Program, channel: Receiver<(CoreTraceState, NodeExecutionPhase)>) -> Self {
        Self { program, channel, fragments: Vec::new() }
    }

    /// Processes checkpoints from the channel, producing fragments in order
    ///
    /// This method reads checkpoints from the channel and spawns async tasks to process
    /// each one into a CoreTraceFragment. The fragments are added to self.fragments
    /// in the same order as the checkpoints were received.
    pub async fn process(&mut self) {
        let mut pending_tasks: Vec<JoinHandle<CoreTraceFragment>> = Vec::new();

        // Phase 1: Spawn tasks for all incoming checkpoints
        while let Some((main_trace_state, execution_phase)) = self.channel.recv().await {
            let mast_forest = self.program.mast_forest().clone();
            let task = tokio::task::spawn_blocking(move || {
                let main_trace_generator = CoreTraceFragmentGenerator::new(main_trace_state);
                main_trace_generator.generate_fragment(&mast_forest, execution_phase)
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
    pub fn fragments(&self) -> &[CoreTraceFragment] {
        &self.fragments
    }
}

pub struct CoreTraceFragmentGenerator {
    fragment_start_clk: RowIndex,
    fragment: CoreTraceFragment,
    state: CoreTraceState,
    span_context: Option<SpanContext>,
}

impl CoreTraceFragmentGenerator {
    /// Creates a new CoreTraceFragmentGenerator with the provided checkpoint.
    pub fn new(state: CoreTraceState) -> Self {
        Self {
            fragment_start_clk: state.system.clk,
            // Safety: the `CoreTraceFragmentGenerator` will fill in all the rows, or truncate any
            // unused rows if a `HALT` operation occurs before `NUM_ROWS_PER_FRAGMENT` have been
            // executed.
            fragment: unsafe {
                CoreTraceFragment::new_uninit(CoreTraceFragmentManager::NUM_ROWS_PER_FRAGMENT)
            },
            state,
            span_context: None,
        }
    }

    /// Processes a single checkpoint into a CoreTraceFragment
    pub fn generate_fragment(
        mut self,
        program: &MastForest,
        execution_phase: NodeExecutionPhase,
    ) -> CoreTraceFragment {
        // Execute fragment generation and always finalize at the end
        let _ = self.execute_fragment_generation(program, execution_phase);
        self.finalize_fragment()
    }

    /// Internal method that performs fragment generation with automatic early returns
    fn execute_fragment_generation(
        &mut self,
        program: &MastForest,
        execution_phase: NodeExecutionPhase,
    ) -> ControlFlow<()> {
        match execution_phase {
            NodeExecutionPhase::BasicBlock {
                batch_index,
                op_idx_in_batch,
                needs_respan,
            } => {
                // Finish running the current basic block from the specified batch and operation
                // index
                let node_id = self.state.traversal.peek().expect("traversal should not be empty");
                let basic_block_node = {
                    let mast_node = program.get_node_by_id(node_id).expect("node should exist");
                    mast_node.get_basic_block().expect("Expected a basic block node")
                };

                let op_batches = basic_block_node.op_batches();
                assert!(
                    batch_index < op_batches.len(),
                    "Batch index out of bounds: {batch_index} >= {}",
                    op_batches.len()
                );

                // Initialize the span context for the current basic block
                self.span_context =
                    Some(initialize_span_context(basic_block_node, batch_index, op_idx_in_batch));

                // Insert RESPAN if needed
                if needs_respan {
                    assert_eq!(op_idx_in_batch, 0);
                    let current_batch = &op_batches[batch_index];
                    self.respan(current_batch)?;
                }

                // Execute remaining operations in the specified batch
                let current_batch = &op_batches[batch_index];
                if op_idx_in_batch < current_batch.ops().len() {
                    self.execute_op_batch(current_batch, Some(op_idx_in_batch))?;
                }

                // Execute remaining batches
                for op_batch in op_batches.iter().skip(batch_index + 1) {
                    self.respan(op_batch)?;

                    self.execute_op_batch(op_batch, None)?;
                }

                // Add END trace row to complete the basic block
                self.add_span_end_trace_row(basic_block_node)?;

                // Advance the traversal past the current basic block
                self.state.traversal.advance();
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
            self.execute_mast_node(node_id, program)?;

            // If we reach here, the node completed without filling the fragment
            self.state.traversal.advance();
        }

        // All nodes completed without filling the fragment
        ControlFlow::Continue(())
    }

    fn execute_mast_node(&mut self, node_id: MastNodeId, program: &MastForest) -> ControlFlow<()> {
        let mast_node = program.get_node_by_id(node_id).expect("node should exist");

        // Set the address of the new block
        let addr = self.state.hasher.replay_block_address();

        match mast_node {
            MastNode::Block(basic_block_node) => {
                // Clone the basic_block_node to avoid borrowing issues
                let basic_block_node = basic_block_node.clone();

                // Push block onto block stack and get parent address
                let parent_addr = self.state.block_stack.push(addr, BlockType::Span, None);
                let num_groups_left_in_block = Felt::from(basic_block_node.num_op_groups() as u32);
                let first_op_batch = basic_block_node
                    .op_batches()
                    .first()
                    .expect("Basic block should have at least one op batch");

                // 1. Add SPAN start trace row
                self.add_span_start_trace_row(
                    first_op_batch,
                    num_groups_left_in_block,
                    parent_addr,
                )?;

                // Initialize the span context for the current basic block. After SPAN operation is
                // executed, we decrement the number of remaining groups by 1 because executing
                // SPAN consumes the first group of the batch.
                // TODO(plafer): use `initialize_span_context` once the potential off-by-one issue
                // is resolved.
                self.span_context = Some(SpanContext {
                    group_ops_left: Felt::from(basic_block_node.num_op_groups() as u32 - 1_u32),
                    num_groups_left: first_op_batch.groups()[0],
                });

                // 2. Execute batches one by one
                let op_batches = basic_block_node.op_batches();

                // Execute first op batch
                {
                    let first_op_batch =
                        op_batches.first().expect("Basic block should have at least one op batch");
                    self.execute_op_batch(first_op_batch, None)?;
                }

                // Execute the rest of the op batches
                for op_batch in op_batches.iter().skip(1) {
                    // 3. Add RESPAN trace row between batches
                    self.respan(op_batch)?;

                    self.execute_op_batch(op_batch, None)?;
                }

                // 4. Add END trace row
                self.add_span_end_trace_row(&basic_block_node)?;

                ControlFlow::Continue(())
            },
            MastNode::Join(join_node) => {
                let parent_addr = self.state.block_stack.push(addr, BlockType::Join(false), None);

                // 1. Add "start JOIN" row
                self.add_join_start_trace_row(join_node, program, parent_addr)?;

                // 2. Execute first child
                self.execute_mast_node(join_node.first(), program)?;

                // 3. Execute second child
                self.execute_mast_node(join_node.second(), program)?;

                // 4. Add "end JOIN" row
                self.add_join_end_trace_row(join_node, program)
            },
            MastNode::Split(split_node) => {
                let parent_addr = self.state.block_stack.push(addr, BlockType::Split, None);

                // 1. Add "start SPLIT" row
                self.add_split_start_trace_row(split_node, program, parent_addr)?;

                // 2. Execute the appropriate branch based on the stack top value
                let condition = self.state.stack.stack_top[0];
                if condition == ONE {
                    self.execute_mast_node(split_node.on_true(), program)?;
                } else {
                    self.execute_mast_node(split_node.on_false(), program)?;
                }

                // 3. Add "end SPLIT" row
                self.add_split_end_trace_row(&split_node, program)
            },
            MastNode::Loop(loop_node) => {
                let parent_addr = {
                    let enter_loop = self.state.stack.stack_top[0] == ONE;
                    self.state.block_stack.push(addr, BlockType::Loop(enter_loop), None)
                };

                // 1. Add "start LOOP" row
                self.add_loop_start_trace_row(&loop_node, program, parent_addr)?;

                // 2. In parallel execution, we simulate executing the loop body once
                // based on the current stack top value
                let condition = self.state.stack.stack_top[0];
                if condition == vm_core::ONE {
                    // Note: In the refactored version, we don't directly modify stack depth
                    // as it's derived from the overflow table. The stack manipulation would
                    // be handled by proper stack operations in a full implementation.

                    // Execute loop body (in real execution this would be in a while loop)
                    self.execute_mast_node(loop_node.body(), program)?;

                    // Note: In a real implementation, the loop would continue until the condition
                    // becomes false. For parallel analysis, we simulate one iteration.
                }

                // 3. Add "end LOOP" row
                self.add_loop_end_trace_row(&loop_node, program)
            },
            MastNode::Call(call_node) => {
                let (stack_depth, next_overflow_addr) = self.state.stack.start_context();
                let ctx_info = ExecutionContextInfo::new(
                    self.state.system.ctx,
                    self.state.system.fn_hash,
                    self.state.system.fmp,
                    stack_depth as u32,
                    next_overflow_addr,
                );

                let parent_addr = if call_node.is_syscall() {
                    self.state.block_stack.push(addr, BlockType::SysCall, Some(ctx_info))
                } else {
                    self.state.block_stack.push(addr, BlockType::Call, Some(ctx_info))
                };

                // 1. Add "start CALL/SYSCALL" row
                self.add_call_start_trace_row(call_node, program, parent_addr)?;

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
                self.execute_mast_node(call_node.callee(), program)?;

                // Restore context state
                self.state.system.ctx = saved_ctx;
                self.state.system.fmp = saved_fmp;
                self.state.system.in_syscall = saved_in_syscall;

                // 2. Add "end CALL/SYSCALL" row
                self.add_call_end_trace_row(&call_node, program)
            },
            MastNode::Dyn(dyn_node) => {
                let parent_addr = if dyn_node.is_dyncall() {
                    let (stack_depth, next_overflow_addr) =
                        self.state.stack.shift_left_and_start_context();
                    // For DYNCALL, we need to save the current context state
                    // and prepare for dynamic execution
                    let ctx_info = ExecutionContextInfo::new(
                        self.state.system.ctx,
                        self.state.system.fn_hash,
                        self.state.system.fmp,
                        stack_depth as u32,
                        next_overflow_addr,
                    );
                    self.state.block_stack.push(addr, BlockType::Dyncall, Some(ctx_info))
                } else {
                    // For DYN, we just push the block stack without context info
                    self.state.block_stack.push(addr, BlockType::Dyn, None)
                };

                // 1. Add "start DYN/DYNCALL" row
                self.add_dyn_start_trace_row(dyn_node, parent_addr)?;

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
                self.add_dyn_end_trace_row(dyn_node)
            },
            MastNode::External(_external_node) => {
                // Use the ExternalNodeReplay to get the resolved node ID
                let resolved_node_id = self.state.external_node_replay.replay_resolution();

                // Execute the resolved node within the current program context
                self.execute_mast_node(resolved_node_id, program)
            },
        }
    }

    /// Executes operations within an operation batch, analogous to FastProcessor::execute_op_batch.
    ///
    /// If `start_op_idx` is provided, execution begins from that operation index within the batch.
    fn execute_op_batch(
        &mut self,
        batch: &OpBatch,
        start_op_idx: Option<usize>,
    ) -> ControlFlow<()> {
        let op_counts = batch.op_counts();
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
        let num_batch_groups = batch.num_groups().next_power_of_two();

        // Execute operations in the batch starting from start_op_idx
        for &op in batch.ops().iter().skip(start_op_idx) {
            // Execute the operation and check if we're done generating
            self.execute_op(op, op_idx_in_group)?;

            // Handle immediate value operations
            let has_imm = op.imm_value().is_some();
            if has_imm {
                next_group_idx += 1;
            }

            // Determine if we've executed all operations in a group
            if op_idx_in_group == op_counts[group_idx] - 1 {
                // If operation has immediate value, execute NOOP after it
                if has_imm {
                    debug_assert!(op_idx_in_group < OP_GROUP_SIZE - 1, "invalid op index");
                    self.increment_clk()?;
                    self.execute_op(Operation::Noop, op_idx_in_group + 1)?;
                }

                // Move to next group and reset operation index
                group_idx = next_group_idx;
                next_group_idx += 1;
                op_idx_in_group = 0;

                // if we haven't reached the end of the batch yet, set up the decoder for
                // decoding the next operation group
                if group_idx < num_batch_groups - 1 {
                    self.start_op_group(batch.groups()[group_idx]);
                }
            } else {
                op_idx_in_group += 1;
            }
        }

        // Execute required number of operation groups (handle padding with NOOPs)
        for group_idx in group_idx..num_batch_groups {
            self.execute_op(Operation::Noop, 0)?;

            // if we are not at the last group yet, set up the decoder for decoding the next
            // operation groups. the groups were are processing are just NOOPs - so, the op group
            // value is ZERO
            if group_idx < num_batch_groups - 1 {
                self.start_op_group(ZERO);
            }
        }

        ControlFlow::Continue(())
    }

    /// Starts decoding a new operation group.
    pub fn start_op_group(&mut self, op_group: Felt) {
        let ctx = self.span_context.as_mut().expect("not in span");

        // reset the current group value and decrement the number of left groups by ONE
        debug_assert_eq!(ZERO, ctx.group_ops_left, "not all ops executed in current group");
        ctx.group_ops_left = op_group;
        ctx.num_groups_left -= ONE;
    }

    /// Executes a single operation, similar to Process::execute_op.
    ///
    /// This implementation executes the operation by updating the state and recording
    /// any memory or advice provider operations for parallel trace generation.
    fn execute_op(&mut self, op: Operation, op_idx_in_group: usize) -> ControlFlow<()> {
        // Execute the operation by dispatching to appropriate operation methods
        self.dispatch_operation(&op);

        // write the operation to the trace
        self.add_operation_trace_row(op, op_idx_in_group)
    }

    /// Dispatches the operation to the appropriate execution method.
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

    fn finalize_fragment(mut self) -> CoreTraceFragment {
        // If we have not built enough rows, we need to truncate the fragment. Similarly, in the
        // rare case where we built too many rows, we truncate to the correct number of rows (i.e.
        // NUM_ROWS_PER_FRAGMENT).
        {
            let num_rows = core::cmp::min(
                self.num_rows_built(),
                CoreTraceFragmentManager::NUM_ROWS_PER_FRAGMENT,
            );
            for column in &mut self.fragment.columns {
                column.truncate(num_rows);
            }
        }

        self.fragment
    }

    // HELPERS
    // -------------------------------------------------------------------------------------------

    fn append_opcode(&mut self, opcode: u8, row_idx: usize) {
        use miden_air::trace::{
            DECODER_TRACE_OFFSET,
            decoder::{NUM_OP_BITS, OP_BITS_OFFSET},
        };

        // Append the opcode bits to the trace row
        for i in 0..NUM_OP_BITS {
            let bit = Felt::from((opcode >> i) & 1);
            self.fragment.columns[DECODER_TRACE_OFFSET + OP_BITS_OFFSET + i][row_idx] = bit;
        }
    }

    fn done_generating(&mut self) -> bool {
        // If we have built all the rows in the fragment, we are done
        self.num_rows_built() >= CoreTraceFragmentManager::NUM_ROWS_PER_FRAGMENT
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

// HELPERS
// ===============================================================================================

fn initialize_span_context(
    basic_block_node: &BasicBlockNode,
    batch_index: usize,
    op_idx_in_batch: usize,
) -> SpanContext {
    let op_batches = basic_block_node.op_batches();
    let current_op_group_idx = get_current_op_group_idx(&op_batches[batch_index], op_idx_in_batch);

    let group_ops_left = {
        let current_op_group = op_batches[batch_index].groups()[current_op_group_idx];

        // shift out all operations that are already executed in this group
        Felt::new(current_op_group.as_int() >> (NUM_OP_BITS * op_idx_in_batch))
    };

    // TODO(plafer): double check that this isn't off-by-one (how after the first SPAN, we decrement
    // by 1)
    let num_groups_left = {
        let total_groups = basic_block_node.num_op_groups();

        // Count groups consumed by completed batches (all batches before current one)
        let mut groups_consumed = 0;
        for i in 0..batch_index {
            groups_consumed += op_batches[i].num_groups();
        }

        // Count groups consumed within the current batch up to op_idx_in_batch
        let current_batch = &op_batches[batch_index];

        // Add the number of complete groups before the current group in this batch
        groups_consumed += current_op_group_idx;

        // Count immediate values consumed by executed operations
        for op in current_batch.ops().iter().take(op_idx_in_batch) {
            if op.imm_value().is_some() {
                groups_consumed += 1; // immediate values consume an additional group slot
            }
        }

        Felt::from((total_groups - groups_consumed) as u32)
    };

    SpanContext { group_ops_left, num_groups_left }
}

/// Returns the index of the current operation group in the batch based on the operation index
/// within the batch.
fn get_current_op_group_idx(op_batch: &OpBatch, op_idx_in_batch: usize) -> usize {
    // Find the group index for the given operation index
    let mut ops_processed = 0;
    for (group_idx, &num_ops_in_group) in op_batch.op_counts().iter().enumerate() {
        if ops_processed + num_ops_in_group > op_idx_in_batch {
            return group_idx;
        }
        ops_processed += num_ops_in_group;
    }

    panic!("operation index {op_idx_in_batch} exceeds batch size");
}
