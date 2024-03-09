use alloc::{boxed::Box, sync::Arc};

use super::{procedure::CallSet, ArtifactKind, GlobalProcedureIndex, Procedure};
use crate::{
    ast::{FullyQualifiedProcedureName, Visibility},
    diagnostics::SourceFile,
    AssemblyError, LibraryPath, RpoDigest, SourceSpan, Span, Spanned,
};
use vm_core::code_blocks::CodeBlock;

/// An [AssemblyContext] is used to store configuration and state
/// pertaining to the current compilation of a module/procedure
/// by an [crate::Assembler].
///
/// The context specifies context-specific configuration, the type
/// of artifact being compiled, the current module being compiled,
/// and the current procedure being compiled.
///
/// To provide a custom context, you must compile by invoking the
/// [crate::Assembler::compile_in_context] API, which will use
/// the provided context in place of the default one generated internally
/// by the other `compile`-like APIs.
#[derive(Default)]
pub struct AssemblyContext {
    /// What kind of artifact are we assembling
    kind: ArtifactKind,
    /// When true, this permits calls to refer to procedures which are
    /// not locally available, as long as they are referenced by MAST root,
    /// and not by name. As long as the MAST for those roots is present
    /// when the code is executed, this works fine. However, if the
    /// VM tries to execute a program with such calls, and the MAST is not
    /// available, the program will trap.
    allow_phantom_calls: bool,
    /// The current procedure being compiled
    current_procedure: Option<ProcedureContext>,
    /// The fully-qualified module path which should be compiled.
    ///
    /// If unset, it defaults to the module which
    /// represents the specified `kind`, i.e. if
    /// the kind is executable, we compile the
    /// executable module, and so on.
    ///
    /// When set, the module graph is traversed from the given
    /// module only, so any code unreachable from this module
    /// is not considered for compilation.
    root: Option<LibraryPath>,
}

pub(super) struct ProcedureContext {
    span: SourceSpan,
    source_file: Option<Arc<SourceFile>>,
    gid: GlobalProcedureIndex,
    name: FullyQualifiedProcedureName,
    visibility: Visibility,
    num_locals: u16,
    callset: CallSet,
}

/// Builders
impl AssemblyContext {
    pub fn new(kind: ArtifactKind) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }

    /// Returns a new [AssemblyContext] for a non-executable kernel modules.
    pub fn for_kernel(path: &LibraryPath) -> Self {
        Self::new(ArtifactKind::Kernel).with_root(path.clone())
    }

    /// Returns a new [AssemblyContext] for library modules.
    pub fn for_library(path: &LibraryPath) -> Self {
        Self::new(ArtifactKind::Library).with_root(path.clone())
    }

    /// Returns a new [AssemblyContext] for a executable module.
    pub fn for_program(path: &LibraryPath) -> Self {
        Self::new(ArtifactKind::Executable).with_root(path.clone())
    }

    fn with_root(mut self, path: LibraryPath) -> Self {
        self.root = Some(path);
        self
    }

    #[inline]
    pub(super) fn set_current_procedure(&mut self, context: ProcedureContext) {
        self.current_procedure = Some(context);
    }

    #[inline]
    pub(super) fn take_current_procedure(&mut self) -> Option<ProcedureContext> {
        self.current_procedure.take()
    }

    #[inline]
    pub(super) fn unwrap_current_procedure(&self) -> &ProcedureContext {
        self.current_procedure.as_ref().expect("missing current procedure context")
    }

    #[inline]
    pub(super) fn unwrap_current_procedure_mut(&mut self) -> &mut ProcedureContext {
        self.current_procedure.as_mut().expect("missing current procedure context")
    }

    /// Enables phantom calls when compiling with this context.
    ///
    /// # Panics
    ///
    /// This function will panic if you attempt to enable phantom
    /// calls for a kernel-mode context, as non-local procedure calls
    /// are not allowed in kernel modules.
    pub fn with_phantom_calls(mut self, allow_phantom_calls: bool) -> Self {
        assert!(
            !self.is_kernel() || !allow_phantom_calls,
            "kernel modules cannot have phantom calls enabled"
        );
        self.allow_phantom_calls = allow_phantom_calls;
        self
    }

    /// Returns true if this context is used for compiling a kernel.
    pub fn is_kernel(&self) -> bool {
        matches!(self.kind, ArtifactKind::Kernel)
    }

    /// Returns true if this context is used for compiling an executable.
    pub fn is_executable(&self) -> bool {
        matches!(self.kind, ArtifactKind::Executable)
    }

    /// Returns the type of artifact to produce with this context
    pub fn kind(&self) -> ArtifactKind {
        self.kind
    }

    /// Registers a "phantom" call to the procedure with the specified MAST root.
    ///
    /// A phantom call indicates that code for the procedure is not available. Executing a phantom
    /// call will result in a runtime error. However, the VM may be able to execute a program with
    /// phantom calls as long as the branches containing them are not taken.
    ///
    /// # Errors
    /// Returns an error if phantom calls are not allowed in this assembly context.
    pub fn register_phantom_call(
        &mut self,
        mast_root: Span<RpoDigest>,
    ) -> Result<(), AssemblyError> {
        if !self.allow_phantom_calls {
            let source_file = self.unwrap_current_procedure().source_file().clone();
            let (span, digest) = mast_root.into_parts();
            Err(AssemblyError::PhantomCallsNotAllowed {
                span,
                source_file,
                digest,
            })
        } else {
            Ok(())
        }
    }

    /// Registers a call to an externally-defined procedure which we have previously compiled.
    ///
    /// The call set of the callee is added to the call set of the procedure we are currently
    /// compiling, to reflect that all of the code reachable from the callee is by extension
    /// reachable by the caller.
    pub fn register_external_call(
        &mut self,
        callee: &Procedure,
        inlined: bool,
    ) -> Result<(), AssemblyError> {
        let context = self.unwrap_current_procedure_mut();

        // If we call the callee, it's callset is by extension part of our callset
        context.extend_callset(callee.callset().iter().cloned());

        // If the callee is not being inlined, add it to our callset
        if !inlined {
            context.insert_callee(callee.mast_root());
        }

        Ok(())
    }
}

impl ProcedureContext {
    pub(super) fn new(
        gid: GlobalProcedureIndex,
        name: FullyQualifiedProcedureName,
        visibility: Visibility,
    ) -> Self {
        Self {
            span: name.span(),
            source_file: None,
            gid,
            name,
            visibility,
            num_locals: 0,
            callset: Default::default(),
        }
    }

    pub(super) fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = span;
        self
    }

    pub(super) fn with_source_file(mut self, source_file: Option<Arc<SourceFile>>) -> Self {
        self.source_file = source_file;
        self
    }

    pub(super) fn with_num_locals(mut self, num_locals: u16) -> Self {
        self.num_locals = num_locals;
        self
    }

    pub(crate) fn insert_callee(&mut self, callee: RpoDigest) {
        self.callset.insert(callee);
    }

    pub(crate) fn extend_callset<I>(&mut self, callees: I)
    where
        I: IntoIterator<Item = RpoDigest>,
    {
        self.callset.extend(callees);
    }

    pub fn num_locals(&self) -> u16 {
        self.num_locals
    }

    pub fn id(&self) -> GlobalProcedureIndex {
        self.gid
    }

    pub fn name(&self) -> &FullyQualifiedProcedureName {
        &self.name
    }

    #[allow(unused)]
    pub fn module(&self) -> &LibraryPath {
        &self.name.module
    }

    pub fn source_file(&self) -> Option<Arc<SourceFile>> {
        self.source_file.clone()
    }

    pub fn is_kernel(&self) -> bool {
        self.visibility.is_syscall()
    }

    pub fn into_procedure(self, code: CodeBlock) -> Box<Procedure> {
        let procedure = Procedure::new(self.name, self.visibility, self.num_locals as u32, code)
            .with_span(self.span)
            .with_source_file(self.source_file)
            .with_callset(self.callset);
        Box::new(procedure)
    }
}
impl Spanned for ProcedureContext {
    fn span(&self) -> SourceSpan {
        self.span
    }
}
