use alloc::{collections::BTreeMap, vec::Vec};
use vm_core::{
    crypto::hash::RpoDigest,
    mast::{MastForest, MerkleTreeNode},
};

use crate::{
    ast::{FullyQualifiedProcedureName, ProcedureIndex, ProcedureName, ResolvedProcedure},
    CompiledLibraryError, LibraryPath, Version,
};

/// A procedure's name, along with its module path.
///
/// The only difference between this type and [`FullyQualifiedProcedureName`] is that
/// [`CompiledFullyQualifiedProcedureName`] doesn't have a [`crate::SourceSpan`].
pub struct CompiledFullyQualifiedProcedureName {
    /// The module path for this procedure.
    pub module_path: LibraryPath,
    /// The name of the procedure.
    pub name: ProcedureName,
}

impl CompiledFullyQualifiedProcedureName {
    pub fn new(module_path: LibraryPath, name: ProcedureName) -> Self {
        Self { module_path, name }
    }
}

impl From<FullyQualifiedProcedureName> for CompiledFullyQualifiedProcedureName {
    fn from(fqdn: FullyQualifiedProcedureName) -> Self {
        Self {
            module_path: fqdn.module,
            name: fqdn.name,
        }
    }
}

/// Stores the name and digest of a procedure.
#[derive(Debug, Clone)]
pub struct ProcedureInfo {
    pub name: ProcedureName,
    pub digest: RpoDigest,
}

/// Represents a library where all modules modules were compiled into a [`MastForest`].
pub struct CompiledLibrary {
    mast_forest: MastForest,
    // a path for every `root` in the associated MAST forest
    exports: Vec<CompiledFullyQualifiedProcedureName>,
    metadata: CompiledLibraryMetadata,
}

/// Constructors
impl CompiledLibrary {
    /// Constructs a new [`CompiledLibrary`].
    pub fn new(
        mast_forest: MastForest,
        exports: Vec<CompiledFullyQualifiedProcedureName>,
        metadata: CompiledLibraryMetadata,
    ) -> Result<Self, CompiledLibraryError> {
        if mast_forest.procedure_roots().len() != exports.len() {
            return Err(CompiledLibraryError::InvalidExports {
                exports_len: exports.len(),
                roots_len: mast_forest.procedure_roots().len(),
            });
        }

        Ok(Self {
            mast_forest,
            exports,
            metadata,
        })
    }
}

impl CompiledLibrary {
    /// Returns the inner [`MastForest`].
    pub fn mast_forest(&self) -> &MastForest {
        &self.mast_forest
    }

    /// Returns the fully qualified name of all procedures exported by the library.
    pub fn exports(&self) -> &[CompiledFullyQualifiedProcedureName] {
        &self.exports
    }

    /// Returns the library metadata.
    pub fn metadata(&self) -> &CompiledLibraryMetadata {
        &self.metadata
    }

    /// Returns an iterator over the module infos of the library.
    pub fn into_module_infos(self) -> impl Iterator<Item = ModuleInfo> {
        let mut modules_by_path: BTreeMap<LibraryPath, ModuleInfo> = BTreeMap::new();

        for (proc_index, proc_name) in self.exports.into_iter().enumerate() {
            modules_by_path
                .entry(proc_name.module_path.clone())
                .and_modify(|compiled_module| {
                    let proc_node_id = self.mast_forest.procedure_roots()[proc_index];
                    let proc_digest = self.mast_forest[proc_node_id].digest();

                    compiled_module.add_procedure(ProcedureInfo {
                        name: proc_name.name.clone(),
                        digest: proc_digest,
                    })
                })
                .or_insert_with(|| {
                    let proc_node_id = self.mast_forest.procedure_roots()[proc_index];
                    let proc_digest = self.mast_forest[proc_node_id].digest();
                    let proc = ProcedureInfo {
                        name: proc_name.name,
                        digest: proc_digest,
                    };

                    ModuleInfo::new(proc_name.module_path, core::iter::once(proc))
                });
        }

        modules_by_path.into_values()
    }
}

pub struct CompiledLibraryMetadata {
    pub path: LibraryPath,
    pub version: Version,
}

/// Stores a module's path, as well as information about all exported procedures.
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    path: LibraryPath,
    procedures: Vec<(ProcedureIndex, ProcedureInfo)>,
}

impl ModuleInfo {
    pub fn new(path: LibraryPath, procedures: impl Iterator<Item = ProcedureInfo>) -> Self {
        Self {
            path,
            procedures: procedures
                .enumerate()
                .map(|(idx, proc)| (ProcedureIndex::new(idx), proc))
                .collect(),
        }
    }

    pub fn add_procedure(&mut self, procedure: ProcedureInfo) {
        let index = ProcedureIndex::new(self.procedures.len());
        self.procedures.push((index, procedure));
    }

    pub fn path(&self) -> &LibraryPath {
        &self.path
    }

    // TODOP: Store as `CompiledProcedure`, and add a method `iter()` that iterates with
    // `ProcedureIndex`
    pub fn procedures(&self) -> &[(ProcedureIndex, ProcedureInfo)] {
        &self.procedures
    }

    pub fn resolve(&self, name: &ProcedureName) -> Option<ResolvedProcedure> {
        self.procedures.iter().find_map(|(_, proc)| {
            if &proc.name == name {
                Some(ResolvedProcedure::MastRoot(proc.digest))
            } else {
                None
            }
        })
    }
}
