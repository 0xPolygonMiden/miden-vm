use smallvec::SmallVec;
use vm_core::{mast::MastNodeId, utils::Either};

use super::{Assembler, BasicBlockBuilder, Operation};
use crate::{
    assembler::{mast_forest_builder::MastForestBuilder, ProcedureContext},
    ast::{InvocationTarget, InvokeKind},
    AssemblyError, RpoDigest, SourceSpan, Spanned,
};

/// Procedure Invocation
impl Assembler {
    pub(super) fn invoke(
        &self,
        kind: InvokeKind,
        callee: &InvocationTarget,
        proc_ctx: &mut ProcedureContext,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<MastNodeId, AssemblyError> {
        let span = callee.span();
        let node_id_or_digest = self.resolve_target(kind, callee, proc_ctx, mast_forest_builder)?;
        match node_id_or_digest {
            Either::Left(node_id) => Ok(node_id),
            Either::Right(digest) => self.invoke_mast_root(kind, span, digest, mast_forest_builder),
        }
    }

    fn invoke_mast_root(
        &self,
        kind: InvokeKind,
        span: SourceSpan,
        mast_root: RpoDigest,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<MastNodeId, AssemblyError> {
        // Get the procedure from the assembler
        let current_source_file = self.source_manager.get(span.source_id()).ok();

        // TODO(plafer): bring back
        // If the procedure is cached and is a system call, ensure that the call is valid.
        // match mast_forest_builder.find_procedure(&mast_root) {
        //     Some(proc) if matches!(kind, InvokeKind::SysCall) => {
        //         // Verify if this is a syscall, that the callee is a kernel procedure
        //         //
        //         // NOTE: The assembler is expected to know the full set of all kernel
        //         // procedures at this point, so if we can't identify the callee as a
        //         // kernel procedure, it is a definite error.
        //         if !proc.visibility().is_syscall() {
        //             return Err(AssemblyError::InvalidSysCallTarget {
        //                 span,
        //                 source_file: current_source_file,
        //                 callee: proc.fully_qualified_name().clone(),
        //             });
        //         }
        //         let maybe_kernel_path = proc.path();
        //         self.module_graph
        //             .find_module(maybe_kernel_path)
        //             .ok_or_else(|| AssemblyError::InvalidSysCallTarget {
        //                 span,
        //                 source_file: current_source_file.clone(),
        //                 callee: proc.fully_qualified_name().clone(),
        //             })
        //             .and_then(|module| {
        //                 // Note: this module is guaranteed to be of AST variant, since we have
        // the                 // AST of a procedure contained in it (i.e. `proc`). Hence,
        // it must be that                 // the entire module is in AST representation as
        // well.                 if module.unwrap_ast().is_kernel() {
        //                     Ok(())
        //                 } else {
        //                     Err(AssemblyError::InvalidSysCallTarget {
        //                         span,
        //                         source_file: current_source_file.clone(),
        //                         callee: proc.fully_qualified_name().clone(),
        //                     })
        //                 }
        //             })?;
        //     },
        //     Some(_) | None => (),
        // }

        let mast_root_node_id = {
            // Note that here we rely on the fact that we topologically sorted the
            // procedures, such that when we assemble a procedure, all
            // procedures that it calls will have been assembled, and
            // hence be present in the `MastForest`.
            match kind {
                InvokeKind::ProcRef => {
                    match mast_forest_builder.find_procedure_node_id(mast_root) {
                        Some(root) => root,
                        None => {
                            // If the MAST root called isn't known to us, make it an external
                            // reference.
                            mast_forest_builder.ensure_external(mast_root)?
                        },
                    }
                },
                InvokeKind::Exec => {
                    match mast_forest_builder.find_procedure_node_id(mast_root) {
                        Some(root) => {
                            // We make sure to copy the root node so that the `exec` is associated
                            // with a different `MastNodeId` than the procedure it is referencing.
                            // Currently the only purpose of this is so that simple procedures that
                            // only have an `exec` have a different body node id than the procedure
                            // they're executing.
                            let root_node = mast_forest_builder.get_mast_node(root).unwrap();
                            mast_forest_builder.add_node(root_node.clone())?
                        },
                        None => {
                            // If the MAST root called isn't known to us, make it an external
                            // reference.
                            mast_forest_builder.ensure_external(mast_root)?
                        },
                    }
                },
                InvokeKind::Call => {
                    let callee_id = match mast_forest_builder.find_procedure_node_id(mast_root) {
                        Some(callee_id) => callee_id,
                        None => {
                            // If the MAST root called isn't known to us, make it an external
                            // reference.
                            mast_forest_builder.ensure_external(mast_root)?
                        },
                    };

                    mast_forest_builder.ensure_call(callee_id)?
                },
                InvokeKind::SysCall => {
                    let callee_id = match mast_forest_builder.find_procedure_node_id(mast_root) {
                        Some(callee_id) => callee_id,
                        None => {
                            // If the MAST root called isn't known to us, make it an external
                            // reference.
                            mast_forest_builder.ensure_external(mast_root)?
                        },
                    };

                    mast_forest_builder.ensure_syscall(callee_id)?
                },
            }
        };

        Ok(mast_root_node_id)
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
        mast_forest_builder: &MastForestBuilder,
    ) -> Result<(), AssemblyError> {
        let digest = match self.resolve_target(
            InvokeKind::ProcRef,
            callee,
            proc_ctx,
            mast_forest_builder,
        )? {
            Either::Left(node_id) => mast_forest_builder.get_mast_node(node_id).unwrap().digest(),
            Either::Right(digest) => digest,
        };
        self.procref_mast_root(digest, basic_block_builder)
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
