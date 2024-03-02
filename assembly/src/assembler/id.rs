use crate::ast::ProcedureIndex;

/// Uniquely identifies a procedure in a set of [crate::ast::Module]
///
/// This is intended for use when we are doing global inter-procedural
/// analysis on a (possibly growable) set of modules. It is expected
/// that the index of a module in the set, as well as the index of a
/// procedure in a module, are stable once allocated. The set of modules
/// and functions can grow, as long as growing the set only allocates
/// unused identifiers.
///
/// NOTE: This struct is the same size as a u32
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalProcedureIndex {
    /// The index of the containing module in the global set of modules
    pub module: ModuleIndex,
    /// The local index of the procedure in the module
    pub index: ProcedureIndex,
}

/// A strongly-typed index into a set of [Module]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ModuleIndex(u16);
impl ModuleIndex {
    pub fn new(index: usize) -> Self {
        Self(index.try_into().expect("invalid module index: too many modules"))
    }

    #[inline(always)]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }
}
