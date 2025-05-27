use core::ops::ControlFlow;

use vm_core::{Word, mast::DynNode};

use super::{MainTraceFragmentGenerator, TraceRowType, trace_builder::OperationTraceConfig};

impl MainTraceFragmentGenerator {
    /// Adds a trace row for a DYN/DYNCALL operation (start or end) to the main trace fragment.
    ///
    /// This method populates the system, decoder, and stack columns for a single trace row
    /// corresponding to either the start or end of a DYN/DYNCALL block execution. It uses the
    /// shared control flow trace building infrastructure.
    pub fn add_dyn_trace_row(
        &mut self,
        dyn_node: &DynNode,
        trace_type: TraceRowType,
    ) -> ControlFlow<()> {
        // For DYN/DYNCALL operations, the hasher state is set to zeros since the dynamic target
        // is resolved at runtime and cannot be known at compile time during parallel execution
        let zero_hash: Word = [vm_core::ZERO; 4];

        let config = OperationTraceConfig {
            start_opcode: if dyn_node.is_dyncall() {
                vm_core::Operation::Dyncall.op_code()
            } else {
                vm_core::Operation::Dyn.op_code()
            },
            start_hasher_state: (zero_hash, zero_hash),
            end_node_digest: dyn_node.digest().into(),
        };

        self.add_control_flow_trace_row(config, trace_type)
    }

    /// Adds a trace row for the start of a DYN/DYNCALL operation.
    ///
    /// This is a convenience method that calls `add_dyn_trace_row` with `TraceRowType::Start`.
    pub fn add_dyn_start_trace_row(&mut self, dyn_node: &DynNode) -> ControlFlow<()> {
        self.add_dyn_trace_row(dyn_node, TraceRowType::Start)
    }

    /// Adds a trace row for the end of a DYN/DYNCALL operation.
    ///
    /// This is a convenience method that calls `add_dyn_trace_row` with `TraceRowType::End`.
    pub fn add_dyn_end_trace_row(&mut self, dyn_node: &DynNode) -> ControlFlow<()> {
        self.add_dyn_trace_row(dyn_node, TraceRowType::End)
    }
}
