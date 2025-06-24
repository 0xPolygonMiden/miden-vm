use smallvec::SmallVec;
use vm_core::mast::MastNodeId;

use super::{Assembler, BasicBlockBuilder, Operation};
use crate::{
    Word,
    assembler::{ProcedureContext, mast_forest_builder::MastForestBuilder},
    ast::{InvocationTarget, InvokeKind},
    diagnostics::Report,
};

/// Procedure Invocation
impl Assembler {
    /// Returns the [`MastNodeId`] of the invoked procedure specified by `callee`.
    ///
    /// For example, given `exec.f`, this method would return the procedure body id of `f`. If the
    /// only representation of `f` that we have is its MAST root, then this method will also insert
    /// a [`core::mast::ExternalNode`] that wraps `f`'s MAST root and return the corresponding id.
    pub(super) fn invoke(
        &self,
        kind: InvokeKind,
        callee: &InvocationTarget,
        proc_ctx: &ProcedureContext,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<MastNodeId, Report> {
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
    ) -> Result<Option<MastNodeId>, Report> {
        let dyn_node_id = mast_forest_builder.ensure_dyn()?;

        Ok(Some(dyn_node_id))
    }

    /// Creates a new DYNCALL block for the dynamic function call and return.
    pub(super) fn dyncall(
        &self,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Option<MastNodeId>, Report> {
        let dyn_call_node_id = mast_forest_builder.ensure_dyncall()?;

        Ok(Some(dyn_call_node_id))
    }

    pub(super) fn procref(
        &self,
        callee: &InvocationTarget,
        proc_ctx: &mut ProcedureContext,
        block_builder: &mut BasicBlockBuilder,
    ) -> Result<(), Report> {
        let mast_root = {
            let proc_body_id = self.resolve_target(
                InvokeKind::ProcRef,
                callee,
                proc_ctx,
                block_builder.mast_forest_builder_mut(),
            )?;
            // Note: it's ok to `unwrap()` here since `proc_body_id` was returned from
            // `mast_forest_builder`
            block_builder
                .mast_forest_builder()
                .get_mast_node(proc_body_id)
                .unwrap()
                .digest()
        };

        self.procref_mast_root(mast_root, block_builder)
    }

    fn procref_mast_root(
        &self,
        mast_root: Word,
        block_builder: &mut BasicBlockBuilder,
    ) -> Result<(), Report> {
        // Create an array with `Push` operations containing root elements
        let ops = mast_root
            .iter()
            .map(|elem| Operation::Push(*elem))
            .collect::<SmallVec<[_; 4]>>();
        block_builder.push_ops(ops);
        Ok(())
    }
}
