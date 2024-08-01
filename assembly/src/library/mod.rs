use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};

use vm_core::crypto::hash::RpoDigest;
use vm_core::mast::{MastForest, MastNodeId};
use vm_core::utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};
use vm_core::Kernel;

use crate::ast::{AstSerdeOptions, ProcedureIndex, ProcedureName, QualifiedProcedureName};

mod error;
#[cfg(feature = "std")]
mod masl;
mod namespace;
mod path;
mod version;

pub use self::error::{CompiledLibraryError, LibraryError};
pub use self::namespace::{LibraryNamespace, LibraryNamespaceError};
pub use self::path::{LibraryPath, LibraryPathComponent, PathError};
pub use self::version::{Version, VersionError};

#[cfg(test)]
mod tests;

// COMPILED LIBRARY
// ================================================================================================

/// Represents a library where all modules were compiled into a [`MastForest`].
///
/// A library exports a set of one or more procedures. Currently, all exported procedures belong
/// to the same top-level namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledLibrary {
    /// The content hash of this library, formed by hashing the roots of all exports in
    /// lexicographical order (by digest, not procedure name)
    digest: RpoDigest,
    /// A map between procedure paths and the corresponding procedure toots in the MAST forest.
    /// Multiple paths can map to the same root, and also, some roots may not be associated with
    /// any paths.
    exports: BTreeMap<QualifiedProcedureName, Export>,
    /// The MAST forest underlying this library.
    mast_forest: MastForest,
}

impl AsRef<CompiledLibrary> for CompiledLibrary {
    #[inline(always)]
    fn as_ref(&self) -> &CompiledLibrary {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Export {
    /// The export is contained in the [MastForest] of this library
    Local(MastNodeId),
    /// The export is a re-export of an externally-defined procedure from another library
    External(RpoDigest),
}

/// Constructors
impl CompiledLibrary {
    /// Constructs a new [`CompiledLibrary`] from the provided MAST forest and a set of exports.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The set of exported procedures is empty.
    /// - Not all exported procedures are present in the MAST forest.
    pub fn new(
        mast_forest: MastForest,
        exports: BTreeMap<QualifiedProcedureName, RpoDigest>,
    ) -> Result<Self, CompiledLibraryError> {
        if exports.is_empty() {
            return Err(CompiledLibraryError::EmptyExports);
        }

        let mut fqn_to_export = BTreeMap::new();

        // convert fqn |-> mast_root map into fqn |-> mast_node_id map
        for (fqn, mast_root) in exports.into_iter() {
            match mast_forest.find_procedure_root(mast_root) {
                Some(node_id) => {
                    fqn_to_export.insert(fqn, Export::Local(node_id));
                }
                None => {
                    fqn_to_export.insert(fqn, Export::External(mast_root));
                }
            }
        }

        let digest = content_hash(&fqn_to_export, &mast_forest);

        Ok(Self {
            digest,
            exports: fqn_to_export,
            mast_forest,
        })
    }
}

/// Accessors
impl CompiledLibrary {
    /// Returns the [RpoDigest] representing the content hash of this library
    pub fn digest(&self) -> &RpoDigest {
        &self.digest
    }

    /// Returns the fully qualified name of all procedures exported by the library.
    pub fn exports(&self) -> impl Iterator<Item = &QualifiedProcedureName> {
        self.exports.keys()
    }

    /// Returns the inner [`MastForest`].
    pub fn mast_forest(&self) -> &MastForest {
        &self.mast_forest
    }
}

/// Conversions
impl CompiledLibrary {
    /// Returns an iterator over the module infos of the library.
    pub fn module_infos(&self) -> impl Iterator<Item = ModuleInfo> {
        let mut modules_by_path: BTreeMap<LibraryPath, ModuleInfo> = BTreeMap::new();

        for (proc_name, export) in self.exports.iter() {
            modules_by_path
                .entry(proc_name.module.clone())
                .and_modify(|compiled_module| {
                    let proc_digest = export.digest(&self.mast_forest);

                    compiled_module.add_procedure_info(ProcedureInfo {
                        name: proc_name.name.clone(),
                        digest: proc_digest,
                    })
                })
                .or_insert_with(|| {
                    let proc_digest = export.digest(&self.mast_forest);
                    let proc = ProcedureInfo {
                        name: proc_name.name.clone(),
                        digest: proc_digest,
                    };

                    ModuleInfo::new(proc_name.module.clone(), vec![proc])
                });
        }

        modules_by_path.into_values()
    }
}

/// Serialization
impl CompiledLibrary {
    /// Serialize to `target` using `options`
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        let Self {
            digest: _,
            exports,
            mast_forest,
        } = self;

        options.write_into(target);
        mast_forest.write_into(target);

        target.write_usize(exports.len());
        for (proc_name, export) in exports {
            proc_name.write_into_with_options(target, options);
            export.write_into(target);
        }
    }
}

impl Serializable for CompiledLibrary {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.write_into_with_options(target, AstSerdeOptions::default())
    }
}

impl Deserializable for CompiledLibrary {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let options = AstSerdeOptions::read_from(source)?;
        let mast_forest = MastForest::read_from(source)?;

        let num_exports = source.read_usize()?;
        let mut exports = BTreeMap::new();
        for _ in 0..num_exports {
            let proc_name = QualifiedProcedureName::read_from_with_options(source, options)?;
            let export = Export::read_with_forest(source, &mast_forest)?;

            exports.insert(proc_name, export);
        }

        let digest = content_hash(&exports, &mast_forest);

        Ok(Self {
            digest,
            exports,
            mast_forest,
        })
    }
}

fn content_hash(
    exports: &BTreeMap<QualifiedProcedureName, Export>,
    mast_forest: &MastForest,
) -> RpoDigest {
    let digests = BTreeSet::from_iter(exports.values().map(|export| export.digest(mast_forest)));
    digests
        .into_iter()
        .reduce(|a, b| vm_core::crypto::hash::Rpo256::merge(&[a, b]))
        .unwrap()
}

#[cfg(feature = "std")]
mod use_std_library {
    use super::*;
    use crate::{
        ast::{self, ModuleKind},
        diagnostics::{IntoDiagnostic, SourceManager},
        Assembler,
    };
    use masl::{LibraryEntry, WalkLibrary};
    use miette::{Context, Report};
    use std::{collections::btree_map::Entry, fs, io, path::Path, sync::Arc};
    use vm_core::utils::ReadAdapter;

    impl CompiledLibrary {
        /// File extension for the Assembly Library.
        pub const LIBRARY_EXTENSION: &'static str = "masl";

        /// File extension for the Assembly Module.
        pub const MODULE_EXTENSION: &'static str = "masm";

        /// Name of the root module.
        pub const MOD: &'static str = "mod";

        /// Write the library to a target file
        ///
        /// NOTE: It is up to the caller to use the correct file extension, but there is no
        /// specific requirement that the extension be set, or the same as
        /// [`Self::LIBRARY_EXTENSION`].
        pub fn write_to_file(
            &self,
            path: impl AsRef<Path>,
            options: AstSerdeOptions,
        ) -> io::Result<()> {
            let path = path.as_ref();

            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }

            // NOTE: We catch panics due to i/o errors here due to the fact
            // that the ByteWriter trait does not provide fallible APIs, so
            // WriteAdapter will panic if the underlying writes fail. This
            // needs to be addressed in winterfall at some point
            std::panic::catch_unwind(|| {
                let mut file = fs::File::create(path)?;
                self.write_into_with_options(&mut file, options);
                Ok(())
            })
            .map_err(|p| {
                match p.downcast::<io::Error>() {
                    // SAFETY: It is guaranteed safe to read Box<std::io::Error>
                    Ok(err) => unsafe { core::ptr::read(&*err) },
                    Err(err) => std::panic::resume_unwind(err),
                }
            })?
        }

        /// Create a [CompiledLibrary] from a standard Miden Assembly project layout.
        ///
        /// The standard layout dictates that a given path is the root of a namespace, and the
        /// directory hierarchy corresponds to the namespace hierarchy. A `.masm` file found in a
        /// given subdirectory of the root, will be parsed with its [LibraryPath] set based on
        /// where it resides in the directory structure.
        ///
        /// This function recursively parses the entire directory structure under `path`, ignoring
        /// any files which do not have the `.masm` extension.
        ///
        /// For example, let's say I call this function like so:
        ///
        /// ```rust
        /// CompiledLibrary::from_dir("~/masm/std", LibraryNamespace::new("std").unwrap()):
        /// ```
        ///
        /// Here's how we would handle various files under this path:
        ///
        /// - ~/masm/std/sys.masm            -> Parsed as "std::sys"
        /// - ~/masm/std/crypto/hash.masm    -> Parsed as "std::crypto::hash"
        /// - ~/masm/std/math/u32.masm       -> Parsed as "std::math::u32"
        /// - ~/masm/std/math/u64.masm       -> Parsed as "std::math::u64"
        /// - ~/masm/std/math/README.md      -> Ignored
        pub fn from_dir(
            path: impl AsRef<Path>,
            namespace: LibraryNamespace,
            source_manager: Arc<dyn SourceManager>,
        ) -> Result<Self, Report> {
            let path = path.as_ref();
            if !path.is_dir() {
                return Err(Report::msg(format!(
                    "the provided path '{}' is not a valid directory",
                    path.display()
                )));
            }

            // mod.masm is not allowed in the root directory
            if path.join("mod.masm").exists() {
                return Err(Report::msg("mod.masm is not allowed in the root directory"));
            }

            Self::compile_modules_from_dir(namespace, path, source_manager)
        }

        /// Read the contents (modules) of this library from `dir`, returning any errors that occur
        /// while traversing the file system.
        ///
        /// Errors may also be returned if traversal discovers issues with the library, such as
        /// invalid names, etc.
        ///
        /// Returns a library built from the set of modules that were compiled.
        fn compile_modules_from_dir(
            namespace: LibraryNamespace,
            dir: &Path,
            source_manager: Arc<dyn SourceManager>,
        ) -> Result<Self, Report> {
            let mut modules = BTreeMap::default();

            let walker = WalkLibrary::new(namespace.clone(), dir)
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to load library from '{}'", dir.display()))?;
            for entry in walker {
                let LibraryEntry {
                    mut name,
                    source_path,
                } = entry?;
                if name.last() == Self::MOD {
                    name.pop();
                }
                // Parse module at the given path
                let mut parser = ast::Module::parser(ModuleKind::Library);
                let ast = parser.parse_file(name.clone(), &source_path, &source_manager)?;
                match modules.entry(name) {
                    Entry::Occupied(ref entry) => {
                        return Err(LibraryError::DuplicateModulePath(entry.key().clone()))
                            .into_diagnostic();
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(ast);
                    }
                }
            }

            if modules.is_empty() {
                return Err(LibraryError::Empty(namespace.clone()).into());
            }
            if modules.len() > MAX_MODULES {
                return Err(LibraryError::TooManyModulesInLibrary {
                    name: namespace.clone(),
                    count: modules.len(),
                    max: MAX_MODULES,
                }
                .into());
            }

            Assembler::new(source_manager)
                .with_debug_mode(true)
                .assemble_library(modules.into_values())
        }

        pub fn deserialize_from_file(path: impl AsRef<Path>) -> Result<Self, DeserializationError> {
            let path = path.as_ref();
            let mut file = fs::File::open(path).map_err(|err| {
                DeserializationError::InvalidValue(format!(
                    "failed to open file at {}: {err}",
                    path.to_string_lossy()
                ))
            })?;
            let mut adapter = ReadAdapter::new(&mut file);

            Self::read_from(&mut adapter)
        }
    }
}

impl Export {
    pub fn digest(&self, mast_forest: &MastForest) -> RpoDigest {
        match self {
            Self::Local(node_id) => mast_forest[*node_id].digest(),
            Self::External(digest) => *digest,
        }
    }

    fn tag(&self) -> u8 {
        // SAFETY: This is safe because we have given this enum a primitive representation with
        // #[repr(u8)], with the first field of the underlying union-of-structs the discriminant.
        //
        // See the section on "accessing the numeric value of the discriminant"
        // here: https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Serializable for Export {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8(self.tag());
        match self {
            Self::Local(node_id) => target.write_u32(node_id.into()),
            Self::External(digest) => digest.write_into(target),
        }
    }
}

impl Export {
    pub fn read_with_forest<R: ByteReader>(
        source: &mut R,
        mast_forest: &MastForest,
    ) -> Result<Self, DeserializationError> {
        match source.read_u8()? {
            0 => {
                let node_id = MastNodeId::from_u32_safe(source.read_u32()?, mast_forest)?;
                if !mast_forest.is_procedure_root(node_id) {
                    return Err(DeserializationError::InvalidValue(format!(
                        "node with id {node_id} is not a procedure root"
                    )));
                }
                Ok(Self::Local(node_id))
            }
            1 => RpoDigest::read_from(source).map(Self::External),
            n => Err(DeserializationError::InvalidValue(format!(
                "{} is not a valid compiled library export entry",
                n
            ))),
        }
    }
}

// KERNEL LIBRARY
// ================================================================================================

/// Represents a library containing a Miden VM kernel.
///
/// This differs from the regular [CompiledLibrary] as follows:
/// - All exported procedures must be exported directly from the kernel namespace (i.e., `#sys`).
/// - The number of exported procedures cannot exceed [Kernel::MAX_NUM_PROCEDURES] (i.e., 256).
pub struct KernelLibrary {
    kernel: Kernel,
    kernel_info: ModuleInfo,
    library: CompiledLibrary,
}

impl KernelLibrary {
    /// Returns the inner [`MastForest`].
    pub fn mast_forest(&self) -> &MastForest {
        self.library.mast_forest()
    }

    /// Destructures this kernel library into individual parts.
    pub fn into_parts(self) -> (Kernel, ModuleInfo, MastForest) {
        (self.kernel, self.kernel_info, self.library.mast_forest)
    }
}

impl TryFrom<CompiledLibrary> for KernelLibrary {
    type Error = CompiledLibraryError;

    fn try_from(library: CompiledLibrary) -> Result<Self, Self::Error> {
        let kernel_path = LibraryPath::from(LibraryNamespace::Kernel);
        let mut kernel_procs = Vec::with_capacity(library.exports.len());
        let mut proc_digests = Vec::with_capacity(library.exports.len());

        for (proc_path, export) in library.exports.iter() {
            // make sure all procedures are exported only from the kernel root
            if proc_path.module != kernel_path {
                return Err(CompiledLibraryError::InvalidKernelExport {
                    procedure_path: proc_path.clone(),
                });
            }

            let proc_digest = export.digest(&library.mast_forest);
            proc_digests.push(proc_digest);
            kernel_procs.push(ProcedureInfo {
                name: proc_path.name.clone(),
                digest: proc_digest,
            });
        }

        let kernel = Kernel::new(&proc_digests)?;
        let module_info = ModuleInfo::new(kernel_path, kernel_procs);

        Ok(Self {
            kernel,
            kernel_info: module_info,
            library,
        })
    }
}

/// Serialization
impl KernelLibrary {
    /// Serialize to `target` using `options`
    pub fn write_into_with_options<W: ByteWriter>(&self, target: &mut W, options: AstSerdeOptions) {
        let Self {
            kernel: _,
            kernel_info: _,
            library,
        } = self;

        library.write_into_with_options(target, options);
    }
}

impl Serializable for KernelLibrary {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.write_into_with_options(target, AstSerdeOptions::default())
    }
}

impl Deserializable for KernelLibrary {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let library = CompiledLibrary::read_from(source)?;

        Self::try_from(library).map_err(|err| {
            DeserializationError::InvalidValue(format!(
                "Failed to deserialize kernel library: {err}"
            ))
        })
    }
}

#[cfg(feature = "std")]
mod use_std_kernel {
    use std::{io, path::Path, sync::Arc};

    use super::*;
    use crate::diagnostics::{Report, SourceManager};

    impl KernelLibrary {
        /// Write the library to a target file
        pub fn write_to_file(
            &self,
            path: impl AsRef<Path>,
            options: AstSerdeOptions,
        ) -> io::Result<()> {
            self.library.write_to_file(path, options)
        }
        /// Read a directory and recursively create modules from its `masm` files.
        ///
        /// For every directory, concatenate the module path with the dir name and proceed.
        ///
        /// For every file, pick and compile the ones with `masm` extension; skip otherwise.
        pub fn from_dir(
            path: impl AsRef<Path>,
            source_manager: Arc<dyn SourceManager>,
        ) -> Result<Self, Report> {
            let library =
                CompiledLibrary::from_dir(path, LibraryNamespace::Kernel, source_manager)?;

            Ok(Self::try_from(library)?)
        }
    }
}

// MODULE INFO
// ================================================================================================

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    path: LibraryPath,
    procedure_infos: Vec<ProcedureInfo>,
}

impl ModuleInfo {
    /// Returns a new [`ModuleInfo`] instantiated from the provided procedures.
    ///
    /// Note: this constructor assumes that the fully-qualified names of the provided procedures
    /// are consistent with the provided module path, but this is not checked.
    fn new(path: LibraryPath, procedures: Vec<ProcedureInfo>) -> Self {
        Self {
            path,
            procedure_infos: procedures,
        }
    }

    /// Adds a [`ProcedureInfo`] to the module.
    pub fn add_procedure_info(&mut self, procedure: ProcedureInfo) {
        self.procedure_infos.push(procedure);
    }

    /// Returns the module's library path.
    pub fn path(&self) -> &LibraryPath {
        &self.path
    }

    /// Returns the number of procedures in the module.
    pub fn num_procedures(&self) -> usize {
        self.procedure_infos.len()
    }

    /// Returns an iterator over the procedure infos in the module with their corresponding
    /// procedure index in the module.
    pub fn procedure_infos(&self) -> impl Iterator<Item = (ProcedureIndex, &ProcedureInfo)> {
        self.procedure_infos
            .iter()
            .enumerate()
            .map(|(idx, proc)| (ProcedureIndex::new(idx), proc))
    }

    /// Returns an iterator over the MAST roots of procedures defined in this module.
    pub fn procedure_digests(&self) -> impl Iterator<Item = RpoDigest> + '_ {
        self.procedure_infos.iter().map(|p| p.digest)
    }

    /// Returns the [`ProcedureInfo`] of the procedure at the provided index, if any.
    pub fn get_proc_info_by_index(&self, index: ProcedureIndex) -> Option<&ProcedureInfo> {
        self.procedure_infos.get(index.as_usize())
    }

    /// Returns the digest of the procedure with the provided name, if any.
    pub fn get_proc_digest_by_name(&self, name: &ProcedureName) -> Option<RpoDigest> {
        self.procedure_infos.iter().find_map(|proc_info| {
            if &proc_info.name == name {
                Some(proc_info.digest)
            } else {
                None
            }
        })
    }
}

/// Stores the name and digest of a procedure.
#[derive(Debug, Clone)]
pub struct ProcedureInfo {
    pub name: ProcedureName,
    pub digest: RpoDigest,
}

/// Maximum number of modules in a library.
#[cfg(feature = "std")]
const MAX_MODULES: usize = u16::MAX as usize;
