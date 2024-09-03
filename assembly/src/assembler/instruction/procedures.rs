use smallvec::SmallVec;
use vm_core::mast::MastNodeId;

use super::{Assembler, BasicBlockBuilder, Operation};
use crate::{
    assembler::{mast_forest_builder::MastForestBuilder, ProcedureContext},
    ast::{InvocationTarget, InvokeKind},
    AssemblyError, RpoDigest,
};

/// Procedure Invocation
impl Assembler {
    /// Returns the [`MastNodeId`] of the invoked procedure specified by `callee`.
    ///
    /// For example, given `exec.f`, this method would return the procedure body id of `f`.
    pub(super) fn invoke(
        &self,
        kind: InvokeKind,
        callee: &InvocationTarget,
        proc_ctx: &ProcedureContext,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<MastNodeId, AssemblyError> {
        let invoked_proc_node_id =
            self.resolve_target(kind, callee, proc_ctx, mast_forest_builder)?;

        match kind {
            InvokeKind::ProcRef | InvokeKind::Exec => Ok(invoked_proc_node_id),
            InvokeKind::Call => mast_forest_builder.ensure_call(invoked_proc_node_id),
            InvokeKind::SysCall => mast_forest_builder.ensure_syscall(invoked_proc_node_id),
        }
    }

    /// Creates a new DYN block for the dynamic code execution and return.
    pub(super) fn dynexec(
        &self,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        let dyn_node_id = mast_forest_builder.ensure_dyn()?;

        Ok(Some(dyn_node_id))
    }

    /// Creates a new CALL block whose target is DYN.
    pub(super) fn dyncall(
        &self,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        let dyn_call_node_id = {
            let dyn_node_id = mast_forest_builder.ensure_dyn()?;
            mast_forest_builder.ensure_call(dyn_node_id)?
        };

        Ok(Some(dyn_call_node_id))
    }

    pub(super) fn procref(
        &self,
        callee: &InvocationTarget,
        proc_ctx: &mut ProcedureContext,
        basic_block_builder: &mut BasicBlockBuilder,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<(), AssemblyError> {
        let mast_root = {
            let proc_body_id =
                self.resolve_target(InvokeKind::ProcRef, callee, proc_ctx, mast_forest_builder)?;
            mast_forest_builder.get_mast_node(proc_body_id).unwrap().digest()
        };

        self.procref_mast_root(mast_root, basic_block_builder)
    }

    fn procref_mast_root(
        &self,
        mast_root: RpoDigest,
        basic_block_builder: &mut BasicBlockBuilder,
    ) -> Result<(), AssemblyError> {
        // Create an array with `Push` operations containing root elements
        let ops = mast_root
            .iter()
            .map(|elem| Operation::Push(*elem))
            .collect::<SmallVec<[_; 4]>>();
        basic_block_builder.push_ops(ops);
        Ok(())
    }
}
