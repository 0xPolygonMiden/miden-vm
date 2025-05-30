use vm_core::{Word, mast::LoopNode};

use super::{MainTraceFragmentGenerator, TraceRowType, trace_builder::OperationTraceConfig};

impl MainTraceFragmentGenerator {
    /// Adds a trace row for a LOOP operation (start or end) to the main trace fragment.
    ///
    /// This method populates the system, decoder, and stack columns for a single trace row
    /// corresponding to either the start or end of a LOOP block execution. It uses the
    /// shared control flow trace building infrastructure.
    pub fn add_loop_trace_row(
        &mut self,
        loop_node: &LoopNode,
        program: &vm_core::mast::MastForest,
        trace_type: TraceRowType,
    ) {
        // For LOOP operations, the hasher state in start operations contains the loop body hash
        // in the first half, and zeros in the second half (since LOOP only has one child)
        let body_hash: Word = program
            .get_node_by_id(loop_node.body())
            .expect("loop body should exist")
            .digest()
            .into();
        let zero_hash: Word = [vm_core::ZERO; 4];

        let config = OperationTraceConfig {
            start_opcode: vm_core::Operation::Loop.op_code(),
            start_hasher_state: (body_hash, zero_hash),
            end_node_digest: loop_node.digest().into(),
        };

        self.add_control_flow_trace_row(config, trace_type);
    }

    /// Adds a trace row for the start of a LOOP operation.
    ///
    /// This is a convenience method that calls `add_loop_trace_row` with `TraceRowType::Start`.
    pub fn add_loop_start_trace_row(
        &mut self,
        loop_node: &LoopNode,
        program: &vm_core::mast::MastForest,
    ) {
        self.add_loop_trace_row(loop_node, program, TraceRowType::Start);
    }

    /// Adds a trace row for the end of a LOOP operation.
    ///
    /// This is a convenience method that calls `add_loop_trace_row` with `TraceRowType::End`.
    pub fn add_loop_end_trace_row(
        &mut self,
        loop_node: &LoopNode,
        program: &vm_core::mast::MastForest,
    ) {
        self.add_loop_trace_row(loop_node, program, TraceRowType::End);
    }
}
