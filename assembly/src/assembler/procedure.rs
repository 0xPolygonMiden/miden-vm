use alloc::{collections::BTreeSet, sync::Arc};

use vm_core::mast::MastNodeId;

use super::GlobalProcedureIndex;
use crate::{
    ast::{ProcedureName, QualifiedProcedureName, Visibility},
    diagnostics::{SourceManager, SourceSpan, Spanned},
    AssemblyError, LibraryPath, RpoDigest,
};

pub type CallSet = BTreeSet<RpoDigest>;

// PROCEDURE CONTEXT
// ================================================================================================

/// Information about a procedure currently being compiled.
pub struct ProcedureContext {
    source_manager: Arc<dyn SourceManager>,
    gid: GlobalProcedureIndex,
    span: SourceSpan,
    name: QualifiedProcedureName,
    visibility: Visibility,
    is_kernel: bool,
    num_locals: u16,
    callset: CallSet,
}

// ------------------------------------------------------------------------------------------------
/// Constructors
impl ProcedureContext {
    pub fn new(
        gid: GlobalProcedureIndex,
        name: QualifiedProcedureName,
        visibility: Visibility,
        is_kernel: bool,
        source_manager: Arc<dyn SourceManager>,
    ) -> Self {
        Self {
            source_manager,
            gid,
            span: name.span(),
            name,
            visibility,
            is_kernel,
            num_locals: 0,
            callset: Default::default(),
        }
    }

    pub fn with_num_locals(mut self, num_locals: u16) -> Self {
        self.num_locals = num_locals;
        self
    }

    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = span;
        self
    }
}

// ------------------------------------------------------------------------------------------------
/// Public accessors
impl ProcedureContext {
    pub fn id(&self) -> GlobalProcedureIndex {
        self.gid
    }

    pub fn name(&self) -> &QualifiedProcedureName {
        &self.name
    }

    pub fn num_locals(&self) -> u16 {
        self.num_locals
    }

    #[allow(unused)]
    pub fn module(&self) -> &LibraryPath {
        &self.name.module
    }

    /// Returns true if the procedure is being assembled for a kernel.
    pub fn is_kernel(&self) -> bool {
        self.is_kernel
    }

    #[inline(always)]
    pub fn source_manager(&self) -> &dyn SourceManager {
        self.source_manager.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
/// State mutators
impl ProcedureContext {
    pub fn insert_callee(&mut self, callee: RpoDigest) {
        self.callset.insert(callee);
    }

    pub fn extend_callset<I>(&mut self, callees: I)
    where
        I: IntoIterator<Item = RpoDigest>,
    {
        self.callset.extend(callees);
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
        // If we call the callee, it's callset is by extension part of our callset
        self.extend_callset(callee.callset().iter().cloned());

        // If the callee is not being inlined, add it to our callset
        if !inlined {
            self.insert_callee(callee.mast_root());
        }

        Ok(())
    }

    /// Transforms this procedure context into a [Procedure].
    ///
    /// The passed-in `mast_root` defines the MAST root of the procedure's body while
    /// `mast_node_id` specifies the ID of the procedure's body node in the MAST forest in
    /// which the procedure is defined. Note that if the procedure is re-exported (i.e., the body
    /// of the procedure is defined in some other MAST forest) `mast_node_id` will point to a
    /// single `External` node.
    ///
    /// <div class="warning">
    /// `mast_root` and `mast_node_id` must be consistent. That is, the node located in the MAST
    /// forest under `mast_node_id` must have the digest equal to the `mast_root`.
    /// </div>
    pub fn into_procedure(self, mast_root: RpoDigest, mast_node_id: MastNodeId) -> Procedure {
        Procedure::new(self.name, self.visibility, self.num_locals as u32, mast_root, mast_node_id)
            .with_span(self.span)
            .with_callset(self.callset)
    }
}

impl Spanned for ProcedureContext {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

// PROCEDURE
// ================================================================================================

/// A compiled Miden Assembly procedure, consisting of MAST info and basic metadata.
///
/// Procedure metadata includes:
///
/// - Fully-qualified path of the procedure in Miden Assembly (if known).
/// - Number of procedure locals to allocate.
/// - The visibility of the procedure (e.g. public/private/syscall)
/// - The set of MAST roots invoked by this procedure.
/// - The original source span and file of the procedure (if available).
#[derive(Clone, Debug)]
pub struct Procedure {
    span: SourceSpan,
    path: QualifiedProcedureName,
    visibility: Visibility,
    num_locals: u32,
    /// The MAST root of the procedure.
    mast_root: RpoDigest,
    /// The MAST node id which resolves to the above MAST root.
    body_node_id: MastNodeId,
    /// The set of MAST roots called by this procedure
    callset: CallSet,
}

// ------------------------------------------------------------------------------------------------
/// Constructors
impl Procedure {
    fn new(
        path: QualifiedProcedureName,
        visibility: Visibility,
        num_locals: u32,
        mast_root: RpoDigest,
        body_node_id: MastNodeId,
    ) -> Self {
        Self {
            span: SourceSpan::default(),
            path,
            visibility,
            num_locals,
            mast_root,
            body_node_id,
            callset: Default::default(),
        }
    }

    pub(crate) fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = span;
        self
    }

    pub(crate) fn with_callset(mut self, callset: CallSet) -> Self {
        self.callset = callset;
        self
    }
}

// ------------------------------------------------------------------------------------------------
/// Public accessors
impl Procedure {
    /// Returns a reference to the name of this procedure
    #[allow(unused)]
    pub fn name(&self) -> &ProcedureName {
        &self.path.name
    }

    /// Returns a reference to the fully-qualified name of this procedure
    pub fn fully_qualified_name(&self) -> &QualifiedProcedureName {
        &self.path
    }

    /// Returns the visibility of this procedure as expressed in the original source code
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Returns a reference to the fully-qualified module path of this procedure
    pub fn path(&self) -> &LibraryPath {
        &self.path.module
    }

    /// Returns the number of memory locals reserved by the procedure.
    pub fn num_locals(&self) -> u32 {
        self.num_locals
    }

    /// Returns the root of this procedure's MAST.
    pub fn mast_root(&self) -> RpoDigest {
        self.mast_root
    }

    /// Returns a reference to the MAST node ID of this procedure.
    pub fn body_node_id(&self) -> MastNodeId {
        self.body_node_id
    }

    /// Returns a reference to a set of all procedures (identified by their MAST roots) which may
    /// be called during the execution of this procedure.
    pub fn callset(&self) -> &CallSet {
        &self.callset
    }
}

impl Spanned for Procedure {
    fn span(&self) -> SourceSpan {
        self.span
    }
}
