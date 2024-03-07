use super::{Assembler, AssemblyContext, CodeBlock, Operation, SpanBuilder};
use crate::{
    ast::{InvocationTarget, InvokeKind},
    AssemblyError, RpoDigest, SourceSpan, Span, Spanned,
};

use smallvec::SmallVec;

/// Procedure Invocation
impl Assembler {
    pub(super) fn invoke(
        &self,
        kind: InvokeKind,
        callee: &InvocationTarget,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        let span = callee.span();
        let digest = self.resolve_target(kind, callee, context)?;
        self.invoke_mast_root(kind, span, digest, context)
    }

    fn invoke_mast_root(
        &self,
        kind: InvokeKind,
        span: SourceSpan,
        mast_root: RpoDigest,
        context: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
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
                context.register_external_call(&proc, false)?;
            }
            Some(proc) => context.register_external_call(&proc, false)?,
            None if matches!(kind, InvokeKind::SysCall) => {
                return Err(AssemblyError::UnknownSysCallTarget {
                    span,
                    source_file: current_source_file,
                    callee: mast_root,
                });
            }
            None => context.register_phantom_call(Span::new(span, mast_root))?,
        }

        let block = match kind {
            // For `exec`, we use a PROXY block to reflect that the root is
            // conceptually inlined at this location
            InvokeKind::Exec => CodeBlock::new_proxy(mast_root),
            // For `call`, we just use the corresponding CALL block
            InvokeKind::Call => CodeBlock::new_call(mast_root),
            // For `syscall`, we just use the corresponding SYSCALL block
            InvokeKind::SysCall => CodeBlock::new_syscall(mast_root),
        };
        Ok(Some(block))
    }

    pub(super) fn dynexec(&self) -> Result<Option<CodeBlock>, AssemblyError> {
        // create a new DYN block for the dynamic code execution and return
        Ok(Some(CodeBlock::new_dyn()))
    }

    pub(super) fn dyncall(&self) -> Result<Option<CodeBlock>, AssemblyError> {
        // create a new CALL block whose target is DYN
        Ok(Some(CodeBlock::new_dyncall()))
    }

    pub(super) fn procref(
        &self,
        callee: &InvocationTarget,
        context: &mut AssemblyContext,
        span_builder: &mut SpanBuilder,
    ) -> Result<(), AssemblyError> {
        let span = callee.span();
        let digest = self.resolve_target(InvokeKind::Exec, callee, context)?;
        self.procref_mast_root(span, digest, context, span_builder)
    }

    fn procref_mast_root(
        &self,
        span: SourceSpan,
        mast_root: RpoDigest,
        context: &mut AssemblyContext,
        span_builder: &mut SpanBuilder,
    ) -> Result<(), AssemblyError> {
        // Add the root to the callset to be able to use dynamic instructions
        // with the referenced procedure later
        let cache = &self.procedure_cache;
        match cache.get_by_mast_root(&mast_root) {
            Some(proc) => context.register_external_call(&proc, false)?,
            None => context.register_phantom_call(Span::new(span, mast_root))?,
        }

        // Create an array with `Push` operations containing root elements
        let ops = mast_root
            .iter()
            .map(|elem| Operation::Push(*elem))
            .collect::<SmallVec<[_; 4]>>();
        span_builder.add_ops(ops);
        Ok(())
    }
}
