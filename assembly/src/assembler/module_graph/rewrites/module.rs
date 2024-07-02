use alloc::{collections::BTreeSet, sync::Arc};
use core::ops::ControlFlow;

use crate::{
    assembler::{
        module_graph::{CallerInfo, NameResolver, PhantomCall},
        ModuleIndex, ResolvedTarget,
    },
    ast::{
        visit::{self, VisitMut},
        InvocationTarget, Invoke, InvokeKind, Module, Procedure,
    },
    diagnostics::SourceFile,
    AssemblyError, Span, Spanned,
};

/// A [ModuleRewriter] handles applying all of the module-wide rewrites to a [Module] that is being
/// added to a [ModuleGraph]. These rewrites include:
///
/// * Resolving, at least partially, all of the invocation targets in procedures of the module, and
///   rewriting those targets as concretely as possible OR as phantom calls representing procedures
///   referenced by MAST root for which we have no definition.
pub struct ModuleRewriter<'a, 'b: 'a> {
    resolver: &'a NameResolver<'b>,
    module_id: ModuleIndex,
    invoked: BTreeSet<Invoke>,
    phantoms: BTreeSet<PhantomCall>,
    source_file: Option<Arc<SourceFile>>,
}

impl<'a, 'b: 'a> ModuleRewriter<'a, 'b> {
    /// Create a new [ModuleRewriter] with the given [NameResolver]
    pub fn new(resolver: &'a NameResolver<'b>) -> Self {
        Self {
            resolver,
            module_id: ModuleIndex::new(u16::MAX as usize),
            invoked: Default::default(),
            phantoms: Default::default(),
            source_file: None,
        }
    }

    /// Apply all rewrites to `module`
    pub fn apply(
        &mut self,
        module_id: ModuleIndex,
        module: &mut Module,
    ) -> Result<(), AssemblyError> {
        self.module_id = module_id;
        self.source_file = module.source_file();

        if let ControlFlow::Break(err) = self.visit_mut_module(module) {
            return Err(err);
        }

        Ok(())
    }

    /// Take the set of accumulated phantom calls out of this rewriter
    pub fn phantoms(&mut self) -> BTreeSet<PhantomCall> {
        core::mem::take(&mut self.phantoms)
    }

    fn rewrite_target(
        &mut self,
        kind: InvokeKind,
        target: &mut InvocationTarget,
    ) -> ControlFlow<AssemblyError> {
        let caller = CallerInfo {
            span: target.span(),
            source_file: self.source_file.clone(),
            module: self.module_id,
            kind,
        };
        match self.resolver.resolve_target(&caller, target) {
            Err(err) => return ControlFlow::Break(err),
            Ok(ResolvedTarget::Cached { digest, .. }) => {
                *target = InvocationTarget::MastRoot(Span::new(target.span(), digest));
                self.invoked.insert(Invoke {
                    kind,
                    target: target.clone(),
                });
            }
            Ok(ResolvedTarget::Phantom(callee)) => {
                let call = PhantomCall {
                    span: target.span(),
                    source_file: self.source_file.clone(),
                    callee,
                };
                self.phantoms.insert(call);
            }
            Ok(ResolvedTarget::Exact { .. }) => {
                self.invoked.insert(Invoke {
                    kind,
                    target: target.clone(),
                });
            }
            Ok(ResolvedTarget::Resolved {
                target: new_target, ..
            }) => {
                *target = new_target;
                self.invoked.insert(Invoke {
                    kind,
                    target: target.clone(),
                });
            }
        }

        ControlFlow::Continue(())
    }
}

impl<'a, 'b: 'a> VisitMut<AssemblyError> for ModuleRewriter<'a, 'b> {
    fn visit_mut_procedure(&mut self, procedure: &mut Procedure) -> ControlFlow<AssemblyError> {
        self.invoked.clear();
        self.invoked.extend(procedure.invoked().cloned());
        visit::visit_mut_procedure(self, procedure)?;
        procedure.extend_invoked(core::mem::take(&mut self.invoked));
        ControlFlow::Continue(())
    }
    fn visit_mut_syscall(&mut self, target: &mut InvocationTarget) -> ControlFlow<AssemblyError> {
        self.rewrite_target(InvokeKind::SysCall, target)
    }
    fn visit_mut_call(&mut self, target: &mut InvocationTarget) -> ControlFlow<AssemblyError> {
        self.rewrite_target(InvokeKind::Call, target)
    }
    fn visit_mut_invoke_target(
        &mut self,
        target: &mut InvocationTarget,
    ) -> ControlFlow<AssemblyError> {
        self.rewrite_target(InvokeKind::Exec, target)
    }
}
