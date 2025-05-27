use vm_core::{Word, mast::SplitNode};

use super::{MainTraceFragmentGenerator, TraceRowType, trace_builder::OperationTraceConfig};

impl MainTraceFragmentGenerator {
    /// Adds a trace row for a SPLIT operation (start or end) to the main trace fragment.
    ///
    /// This method populates the system, decoder, and stack columns for a single trace row
    /// corresponding to either the start or end of a SPLIT block execution. It uses the
    /// shared control flow trace building infrastructure.
    pub fn add_split_trace_row(
        &mut self,
        split_node: &SplitNode,
        program: &vm_core::mast::MastForest,
        trace_type: TraceRowType,
    ) {
        // Get the child hashes for the hasher state
        let on_true_hash: Word = program
            .get_node_by_id(split_node.on_true())
            .expect("on_true child should exist")
            .digest()
            .into();
        let on_false_hash: Word = program
            .get_node_by_id(split_node.on_false())
            .expect("on_false child should exist")
            .digest()
            .into();

        let config = OperationTraceConfig {
            start_opcode: vm_core::Operation::Split.op_code(),
            start_hasher_state: (on_true_hash, on_false_hash),
            end_node_digest: split_node.digest().into(),
        };

        self.add_control_flow_trace_row(config, trace_type);
    }

    /// Adds a trace row for the start of a SPLIT operation.
    ///
    /// This is a convenience method that calls `add_split_trace_row` with `TraceRowType::Start`.
    pub fn add_split_start_trace_row(
        &mut self,
        split_node: &SplitNode,
        program: &vm_core::mast::MastForest,
    ) {
        self.add_split_trace_row(split_node, program, TraceRowType::Start);
    }

    /// Adds a trace row for the end of a SPLIT operation.
    ///
    /// This is a convenience method that calls `add_split_trace_row` with `TraceRowType::End`.
    pub fn add_split_end_trace_row(
        &mut self,
        split_node: &SplitNode,
        program: &vm_core::mast::MastForest,
    ) {
        self.add_split_trace_row(split_node, program, TraceRowType::End);
    }
}
