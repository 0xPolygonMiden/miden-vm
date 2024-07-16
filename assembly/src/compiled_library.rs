use alloc::{string::String, vec::Vec};
use vm_core::mast::MastForest;

use crate::{LibraryPath, Version};

// TODOP: Move into `miden-core` along with `LibraryPath`
pub struct CompiledLibrary {
    mast_forest: MastForest,
    // a path for every `root` in the associated [MastForest]
    exports: Vec<LibraryPath>,
    metadata: CompiledLibraryMetadata,
}

/// Constructors
impl CompiledLibrary {
    pub fn new(
        mast_forest: MastForest,
        exports: Vec<LibraryPath>,
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

    pub fn exports(&self) -> &[LibraryPath] {
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
