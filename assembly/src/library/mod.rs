use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::{String, ToString},
    vec::Vec,
};

use vm_core::{
    crypto::hash::RpoDigest,
    mast::{MastForest, MastNodeId},
    utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable},
    Kernel,
};

use crate::ast::{ProcedureName, QualifiedProcedureName};

mod error;
#[cfg(feature = "std")]
mod masl;
mod module;
mod namespace;
mod path;
mod version;

pub use module::{ModuleInfo, ProcedureInfo};

pub use self::{
    error::LibraryError,
    namespace::{LibraryNamespace, LibraryNamespaceError},
    path::{LibraryPath, LibraryPathComponent, PathError},
    version::{Version, VersionError},
};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// Maximum number of modules in a library.
#[cfg(feature = "std")]
const MAX_MODULES: usize = u16::MAX as usize;

// LIBRARY
// ================================================================================================

/// Represents a library where all modules were compiled into a [`MastForest`].
///
/// A library exports a set of one or more procedures. Currently, all exported procedures belong
/// to the same top-level namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Library {
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

impl AsRef<Library> for Library {
    #[inline(always)]
    fn as_ref(&self) -> &Library {
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
impl Library {
    /// Constructs a new [`Library`] from the provided MAST forest and a set of exports.
    pub fn new(
        mast_forest: MastForest,
        exports: BTreeMap<QualifiedProcedureName, RpoDigest>,
    ) -> Self {
        let mut fqn_to_export = BTreeMap::new();

        // convert fqn |-> mast_root map into fqn |-> mast_node_id map
        for (fqn, mast_root) in exports.into_iter() {
            match mast_forest.find_procedure_root(mast_root) {
                Some(node_id) => {
                    fqn_to_export.insert(fqn, Export::Local(node_id));
                },
                None => {
                    fqn_to_export.insert(fqn, Export::External(mast_root));
                },
            }
        }

        let digest = content_hash(&fqn_to_export, &mast_forest);

        Self {
            digest,
            exports: fqn_to_export,
            mast_forest,
        }
    }
}

/// Accessors
impl Library {
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
impl Library {
    /// Returns an iterator over the module infos of the library.
    pub fn module_infos(&self) -> impl Iterator<Item = ModuleInfo> {
        let mut modules_by_path: BTreeMap<LibraryPath, ModuleInfo> = BTreeMap::new();

        for (proc_name, export) in self.exports.iter() {
            modules_by_path
                .entry(proc_name.module.clone())
                .and_modify(|compiled_module| {
                    let proc_digest = export.digest(&self.mast_forest);
                    compiled_module.add_procedure(proc_name.name.clone(), proc_digest);
                })
                .or_insert_with(|| {
                    let mut module_info = ModuleInfo::new(proc_name.module.clone());

                    let proc_digest = export.digest(&self.mast_forest);
                    module_info.add_procedure(proc_name.name.clone(), proc_digest);

                    module_info
                });
        }

        modules_by_path.into_values()
    }
}

impl From<Library> for MastForest {
    fn from(value: Library) -> Self {
        value.mast_forest
    }
}

impl Serializable for Library {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { digest: _, exports, mast_forest } = self;

        mast_forest.write_into(target);

        target.write_usize(exports.len());
        for (proc_name, export) in exports {
            proc_name.module.write_into(target);
            proc_name.name.as_str().write_into(target);
            export.write_into(target);
        }
    }
}

impl Deserializable for Library {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let mast_forest = MastForest::read_from(source)?;

        let num_exports = source.read_usize()?;
        let mut exports = BTreeMap::new();
        for _ in 0..num_exports {
            let proc_module = source.read()?;
            let proc_name: String = source.read()?;
            let proc_name = ProcedureName::new(proc_name)
                .map_err(|err| DeserializationError::InvalidValue(err.to_string()))?;
            let proc_name = QualifiedProcedureName::new(proc_module, proc_name);
            let export = Export::read_with_forest(source, &mast_forest)?;

            exports.insert(proc_name, export);
        }

        let digest = content_hash(&exports, &mast_forest);

        Ok(Self { digest, exports, mast_forest })
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
    use std::{boxed::Box, collections::btree_map::Entry, fs, io, path::Path, sync::Arc};

    use masl::{LibraryEntry, WalkLibrary};
    use miette::{Context, Report};
    use vm_core::utils::ReadAdapter;

    use super::*;
    use crate::{
        ast::{self, ModuleKind},
        diagnostics::IntoDiagnostic,
        Assembler, SourceManager,
    };

    impl Library {
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
        pub fn write_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
            let path = path.as_ref();

            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }

            // NOTE: We catch panics due to i/o errors here due to the fact that the ByteWriter
            // trait does not provide fallible APIs, so WriteAdapter will panic if the underlying
            // writes fail. This needs to be addressed in winterfell at some point
            std::panic::catch_unwind(|| {
                let mut file = fs::File::create(path)?;
                self.write_into(&mut file);
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

        /// Create a [Library] from a standard Miden Assembly project layout.
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
        /// Library::from_dir(
        ///     "~/masm/std",
        ///     LibraryNamespace::new("std").unwrap()
        ///     Arc::new(crate::DefaultSourceManager::default()),
        /// );
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
            assembler: Assembler,
        ) -> Result<Self, Report> {
            let path = path.as_ref();
            let modules = Self::read_modules_from_dir(namespace, path, assembler.source_manager())?;
            assembler.assemble_library(modules)
        }

        /// Read the contents (modules) of this library from `dir`, returning any errors that occur
        /// while traversing the file system.
        ///
        /// Errors may also be returned if traversal discovers issues with the library, such as
        /// invalid names, etc.
        ///
        /// Returns an iterator over all parsed modules.
        pub(super) fn read_modules_from_dir(
            namespace: LibraryNamespace,
            dir: &Path,
            source_manager: Arc<dyn SourceManager>,
        ) -> Result<impl Iterator<Item = Box<ast::Module>>, Report> {
            if !dir.is_dir() {
                return Err(Report::msg(format!(
                    "the provided path '{}' is not a valid directory",
                    dir.display()
                )));
            }

            // mod.masm is not allowed in the root directory
            if dir.join("mod.masm").exists() {
                return Err(Report::msg("mod.masm is not allowed in the root directory"));
            }

            let mut modules = BTreeMap::default();

            let walker = WalkLibrary::new(namespace.clone(), dir)
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to load library from '{}'", dir.display()))?;
            for entry in walker {
                let LibraryEntry { mut name, source_path } = entry?;
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
                    },
                    Entry::Vacant(entry) => {
                        entry.insert(ast);
                    },
                }
            }

            if modules.len() > MAX_MODULES {
                return Err(LibraryError::TooManyModulesInLibrary {
                    name: namespace.clone(),
                    count: modules.len(),
                    max: MAX_MODULES,
                }
                .into());
            }

            Ok(modules.into_values())
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
            },
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
/// This differs from the regular [Library] as follows:
/// - All exported procedures must be exported directly from the kernel namespace (i.e., `#sys`).
/// - There must be at least one exported procedure.
/// - The number of exported procedures cannot exceed [Kernel::MAX_NUM_PROCEDURES] (i.e., 256).
#[derive(Clone)]
pub struct KernelLibrary {
    kernel: Kernel,
    kernel_info: ModuleInfo,
    library: Library,
}

impl AsRef<Library> for KernelLibrary {
    #[inline(always)]
    fn as_ref(&self) -> &Library {
        &self.library
    }
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

impl TryFrom<Library> for KernelLibrary {
    type Error = LibraryError;

    fn try_from(library: Library) -> Result<Self, Self::Error> {
        if library.exports.is_empty() {
            return Err(LibraryError::EmptyKernel);
        }

        let kernel_path = LibraryPath::from(LibraryNamespace::Kernel);
        let mut proc_digests = Vec::with_capacity(library.exports.len());

        let mut kernel_module = ModuleInfo::new(kernel_path.clone());

        for (proc_path, export) in library.exports.iter() {
            // make sure all procedures are exported only from the kernel root
            if proc_path.module != kernel_path {
                return Err(LibraryError::InvalidKernelExport {
                    procedure_path: proc_path.clone(),
                });
            }

            let proc_digest = export.digest(&library.mast_forest);
            proc_digests.push(proc_digest);
            kernel_module.add_procedure(proc_path.name.clone(), proc_digest);
        }

        let kernel = Kernel::new(&proc_digests)?;

        Ok(Self {
            kernel,
            kernel_info: kernel_module,
            library,
        })
    }
}

impl From<KernelLibrary> for MastForest {
    fn from(value: KernelLibrary) -> Self {
        value.library.mast_forest
    }
}

impl Serializable for KernelLibrary {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { kernel: _, kernel_info: _, library } = self;

        library.write_into(target);
    }
}

impl Deserializable for KernelLibrary {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let library = Library::read_from(source)?;

        Self::try_from(library).map_err(|err| {
            DeserializationError::InvalidValue(format!(
                "Failed to deserialize kernel library: {err}"
            ))
        })
    }
}

#[cfg(feature = "std")]
mod use_std_kernel {
    use std::{io, path::Path};

    use super::*;
    use crate::{diagnostics::Report, Assembler};

    impl KernelLibrary {
        /// Write the library to a target file
        pub fn write_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
            self.library.write_to_file(path)
        }

        /// Create a [KernelLibrary] from a standard Miden Assembly kernel project layout.
        ///
        /// The kernel library will export procedures defined by the module at `sys_module_path`.
        /// If the optional `lib_dir` is provided, all modules under this directory will be
        /// available from the kernel module under the `kernel` namespace. For example, if
        /// `lib_dir` is set to "~/masm/lib", the files will be accessible in the kernel module as
        /// follows:
        ///
        /// - ~/masm/lib/foo.masm        -> Can be imported as "kernel::foo"
        /// - ~/masm/lib/bar/baz.masm    -> Can be imported as "kernel::bar::baz"
        ///
        /// Note: this is a temporary structure which will likely change once
        /// <https://github.com/0xPolygonMiden/miden-vm/issues/1436> is implemented.
        pub fn from_dir(
            sys_module_path: impl AsRef<Path>,
            lib_dir: Option<impl AsRef<Path>>,
            mut assembler: Assembler,
        ) -> Result<Self, Report> {
            // if library directory is provided, add modules from this directory to the assembler
            if let Some(lib_dir) = lib_dir {
                let lib_dir = lib_dir.as_ref();
                let namespace = LibraryNamespace::new("kernel").expect("invalid namespace");
                let modules =
                    Library::read_modules_from_dir(namespace, lib_dir, assembler.source_manager())?;

                for module in modules {
                    assembler.add_module(module)?;
                }
            }

            assembler.assemble_kernel(sys_module_path.as_ref())
        }
    }
}
