use alloc::{string::String, vec::Vec};
use vm_core::mast::MastForest;

use crate::{ast::{FullyQualifiedProcedureName, ProcedureName}, LibraryPath, Version};

// TODOP: Refactor `FullyQualifiedProcedureName` instead, and use `Span<FQDN>` where needed?
pub struct CompiledFullyQualifiedProcedureName {
    /// The module path for this procedure.
    pub module: LibraryPath,
    /// The name of the procedure.
    pub name: ProcedureName,
}

impl From<FullyQualifiedProcedureName> for CompiledFullyQualifiedProcedureName {
    fn from(fqdn: FullyQualifiedProcedureName) -> Self {
        Self {
            module: fqdn.module,
            name: fqdn.name,
        }
    }
}

// TODOP: Move into `miden-core` along with `LibraryPath`
pub struct CompiledLibrary {
    mast_forest: MastForest,
    // a path for every `root` in the associated [MastForest]
    exports: Vec<CompiledFullyQualifiedProcedureName>,
    metadata: CompiledLibraryMetadata,
}

/// Constructors
impl CompiledLibrary {
    pub fn new(
        mast_forest: MastForest,
        exports: Vec<CompiledFullyQualifiedProcedureName>,
        metadata: CompiledLibraryMetadata,
    ) -> Self {
        Self {
            mast_forest,
            exports,
            metadata,
        }
    }
}

impl CompiledLibrary {
    pub fn mast_forest(&self) -> &MastForest {
        &self.mast_forest
    }

    pub fn exports(&self) -> &[CompiledFullyQualifiedProcedureName] {
        &self.exports
    }

    pub fn metadata(&self) -> &CompiledLibraryMetadata {
        &self.metadata
    }
}

pub struct CompiledLibraryMetadata {
    pub name: String,
    pub version: Version,
}
