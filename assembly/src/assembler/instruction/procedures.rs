use super::{Assembler, AssemblyContext, BasicBlockBuilder, Operation};
use crate::{
    ast::{InvocationTarget, InvokeKind},
    AssemblyError, RpoDigest, SourceSpan, Span, Spanned,
};

use smallvec::SmallVec;
use vm_core::mast::{MastForest, MastNode, MastNodeId};

/// Procedure Invocation
impl Assembler {
    pub(super) fn invoke(
        &self,
        kind: InvokeKind,
        callee: &InvocationTarget,
        context: &mut AssemblyContext,
        mast_forest: &mut MastForest,
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        let span = callee.span();
        let digest = self.resolve_target(kind, callee, context, mast_forest)?;
        self.invoke_mast_root(kind, span, digest, context, mast_forest)
    }

    fn invoke_mast_root(
        &self,
        kind: InvokeKind,
        span: SourceSpan,
        mast_root: RpoDigest,
        context: &mut AssemblyContext,
        mast_forest: &mut MastForest,
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        // Get the procedure from the assembler
        let cache = &self.procedure_cache;
        let current_source_file = context.unwrap_current_procedure().source_file();

        // If the procedure is cached, register the call to ensure the callset
        // is updated correctly. Otherwise, register a phantom call.
        match cache.get_by_mast_root(&mast_root) {
            Some(proc) if matches!(kind, InvokeKind::SysCall) => {
                // Verify if this is a syscall, that the callee is a kernel procedure
                //
                // NOTE: The assembler is expected to know the full set of all kernel
                // procedures at this point, so if we can't identify the callee as a
                // kernel procedure, it is a definite error.
                if !proc.visibility().is_syscall() {
                    return Err(AssemblyError::InvalidSysCallTarget {
                        span,
                        source_file: current_source_file,
                        callee: proc.fully_qualified_name().clone(),
                    });
                }
                let maybe_kernel_path = proc.path();
                self.module_graph
                    .find_module(maybe_kernel_path)
                    .ok_or_else(|| AssemblyError::InvalidSysCallTarget {
                        span,
                        source_file: current_source_file.clone(),
                        callee: proc.fully_qualified_name().clone(),
                    })
                    .and_then(|module| {
                        if module.is_kernel() {
                            Ok(())
                        } else {
                            Err(AssemblyError::InvalidSysCallTarget {
                                span,
                                source_file: current_source_file.clone(),
                                callee: proc.fully_qualified_name().clone(),
                            })
                        }
                    })?;
                context.register_external_call(&proc, false, mast_forest)?;
            }
            Some(proc) => context.register_external_call(&proc, false, mast_forest)?,
            None if matches!(kind, InvokeKind::SysCall) => {
                return Err(AssemblyError::UnknownSysCallTarget {
                    span,
                    source_file: current_source_file,
                    callee: mast_root,
                });
            }
            None => context.register_phantom_call(Span::new(span, mast_root))?,
        }

        let mast_root_node_id = match kind {
            // For `exec`, we use a PROXY block to reflect that the root is
            // conceptually inlined at this location
            InvokeKind::Exec => {
                let node = MastNode::new_external(mast_root);
                mast_forest.add_node(node)
            }
            // For `call`, we just use the corresponding CALL block
            InvokeKind::Call => {
                let callee_id = mast_forest
                    .get_node_id_by_digest(mast_root)
                    .unwrap_or_else(|| panic!("MAST root {} not in MAST forest", mast_root));
                let node = MastNode::new_call(callee_id, mast_forest);
                mast_forest.add_node(node)
            }
            // For `syscall`, we just use the corresponding SYSCALL block
            InvokeKind::SysCall => {
                let callee_id = mast_forest
                    .get_node_id_by_digest(mast_root)
                    .unwrap_or_else(|| panic!("MAST root {} not in MAST forest", mast_root));
                let node = MastNode::new_syscall(callee_id, mast_forest);
                mast_forest.add_node(node)
            }
        };

        Ok(Some(mast_root_node_id))
    }

    /// Creates a new DYN block for the dynamic code execution and return.
    pub(super) fn dynexec(
        &self,
        mast_forest: &mut MastForest,
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        let dyn_node_id = mast_forest.add_node(MastNode::Dyn);

        Ok(Some(dyn_node_id))
    }

    /// Creates a new CALL block whose target is DYN.
    pub(super) fn dyncall(
        &self,
        mast_forest: &mut MastForest,
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        let dyn_call_node_id = {
            let dyn_node_id = mast_forest.add_node(MastNode::Dyn);
            let dyn_call_node = MastNode::new_call(dyn_node_id, mast_forest);

            mast_forest.add_node(dyn_call_node)
        };

        Ok(Some(dyn_call_node_id))
    }

    pub(super) fn procref(
        &self,
        callee: &InvocationTarget,
        context: &mut AssemblyContext,
        span_builder: &mut BasicBlockBuilder,
        mast_forest: &MastForest,
    ) -> Result<(), AssemblyError> {
        let span = callee.span();
        let digest = self.resolve_target(InvokeKind::Exec, callee, context, mast_forest)?;
        self.procref_mast_root(span, digest, context, span_builder, mast_forest)
    }

    fn procref_mast_root(
        &self,
        span: SourceSpan,
        mast_root: RpoDigest,
        context: &mut AssemblyContext,
        span_builder: &mut BasicBlockBuilder,
        mast_forest: &MastForest,
    ) -> Result<(), AssemblyError> {
        // Add the root to the callset to be able to use dynamic instructions
        // with the referenced procedure later
        let cache = &self.procedure_cache;
        match cache.get_by_mast_root(&mast_root) {
            Some(proc) => context.register_external_call(&proc, false, mast_forest)?,
            None => context.register_phantom_call(Span::new(span, mast_root))?,
        }

        // Create an array with `Push` operations containing root elements
        let ops = mast_root
            .iter()
            .map(|elem| Operation::Push(*elem))
            .collect::<SmallVec<[_; 4]>>();
        span_builder.push_ops(ops);
        Ok(())
    }
}
