use super::{AbsolutePath, BTreeSet, CodeBlock, Felt, LibraryError, String, MODULE_PATH_DELIM};
use core::{
    fmt,
    ops::{self, Deref},
};
use crypto::{hashers::Blake3_256, Digest, Hasher};

// PROCEDURE
// ================================================================================================

/// Contains metadata and MAST of a procedure.
#[derive(Clone, Debug)]
pub struct Procedure {
    id: ProcedureId,
    label: ProcedureName,
    is_export: bool,
    num_locals: u32,
    code_root: CodeBlock,
    callset: CallSet,
}

impl Procedure {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Procedure] instantiated with the specified properties.
    pub fn new(
        id: ProcedureId,
        label: ProcedureName,
        is_export: bool,
        num_locals: u32,
        code_root: CodeBlock,
        callset: CallSet,
    ) -> Self {
        Procedure {
            id,
            label,
            is_export,
            num_locals,
            code_root,
            callset,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns ID of this procedure.
    pub fn id(&self) -> &ProcedureId {
        &self.id
    }

    /// Returns a label of this procedure.
    pub fn label(&self) -> &ProcedureName {
        &self.label
    }

    /// Returns `true` if this is an exported procedure.
    pub fn is_export(&self) -> bool {
        self.is_export
    }

    /// Returns the number of memory locals reserved by the procedure.
    #[allow(dead_code)]
    pub fn num_locals(&self) -> u32 {
        self.num_locals
    }

    /// Returns a root of this procedure's MAST.
    pub fn code_root(&self) -> &CodeBlock {
        &self.code_root
    }

    /// Returns a reference to a set of all procedures (identified by their IDs) which may be
    /// called during the execution of this procedure.
    pub fn callset(&self) -> &CallSet {
        &self.callset
    }
}

// PROCEDURE NAME
// ================================================================================================

/// Procedure name.
///
/// # Type-safety
///
/// It is achieved as any instance of this type can be created only via the checked
/// [`Self::try_from`]. A valid procedure name cannot be empty or contain a [`MODULE_PATH_DELIM`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcedureName {
    name: String,
}

impl ProcedureName {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    /// Reserved name for a main procedure.
    pub const MAIN_PROC_NAME: &str = "#main";

    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Create a new procedure name with the reserved label for `main`.
    pub fn main() -> Self {
        Self {
            name: Self::MAIN_PROC_NAME.into(),
        }
    }

    // TYPE-SAFE TRANSFORMATION
    // --------------------------------------------------------------------------------------------

    /// Append the procedure name to a module path.
    pub fn to_absolute(&self, module: &AbsolutePath) -> AbsolutePath {
        AbsolutePath::new_unchecked(format!(
            "{}{MODULE_PATH_DELIM}{}",
            module.as_str(),
            &self.name
        ))
    }

    // HELPERS
    // --------------------------------------------------------------------------------------------

    /// Check if the procedure label is the reserved name for `main`.
    pub fn is_main(&self) -> bool {
        self.name == Self::MAIN_PROC_NAME
    }
}

impl TryFrom<String> for ProcedureName {
    type Error = LibraryError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            return Err(LibraryError::empty_procedure_name());
        } else if name.contains(MODULE_PATH_DELIM) {
            return Err(LibraryError::procedure_name_with_delimiter(&name));
        }
        Ok(Self { name })
    }
}

impl Deref for ProcedureName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl AsRef<str> for ProcedureName {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

// PROCEDURE ID
// ================================================================================================

/// A procedure identifier computed as a digest truncated to [`Self::LEN`] bytes, product of the
/// label of a procedure
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcedureId(pub [u8; Self::SIZE]);

impl From<[u8; ProcedureId::SIZE]> for ProcedureId {
    fn from(value: [u8; ProcedureId::SIZE]) -> Self {
        Self(value)
    }
}

impl ops::Deref for ProcedureId {
    type Target = [u8; Self::SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ProcedureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.iter() {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl ProcedureId {
    /// Truncated length of the id
    pub const SIZE: usize = 24;

    /// Creates a new procedure id from its path, composed by module path + name identifier.
    ///
    /// No validation is performed regarding the consistency of the path format.
    pub fn new<L>(path: L) -> Self
    where
        L: AsRef<str>,
    {
        let mut digest = [0u8; Self::SIZE];
        let hash = Blake3_256::<Felt>::hash(path.as_ref().as_bytes());
        digest.copy_from_slice(&hash.as_bytes()[..Self::SIZE]);
        Self(digest)
    }

    /// Computes the full path, given a module path and the procedure name
    pub fn path<N, M>(name: N, module_path: M) -> String
    where
        N: AsRef<str>,
        M: AsRef<str>,
    {
        format!(
            "{}{MODULE_PATH_DELIM}{}",
            module_path.as_ref(),
            name.as_ref()
        )
    }

    /// Creates a new procedure ID from a name to be resolved in the kernel.
    pub fn from_kernel_name(name: &str) -> Self {
        let path = format!("{}{MODULE_PATH_DELIM}{name}", AbsolutePath::KERNEL_PATH);
        Self::new(path)
    }

    /// Creates a new procedure ID from its name and module path.
    ///
    /// No validation is performed regarding the consistency of the module path or procedure name
    /// format.
    pub fn from_name(name: &str, module_path: &str) -> Self {
        let path = Self::path(name, module_path);
        Self::new(path)
    }

    /// Creates a new procedure ID from its local index and module path.
    ///
    /// No validation is performed regarding the consistency of the module path format.
    pub fn from_index(index: u16, module_path: &str) -> Self {
        let path = format!("{module_path}{MODULE_PATH_DELIM}{index}");
        Self::new(path)
    }
}

// CALLSET
// ================================================================================================

/// Contains a list of all procedures which may be invoked from a procedure via call or syscall
/// instructions.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CallSet(BTreeSet<ProcedureId>);

impl CallSet {
    pub fn contains(&self, proc_id: &ProcedureId) -> bool {
        self.0.contains(proc_id)
    }

    pub fn insert(&mut self, proc_id: ProcedureId) {
        self.0.insert(proc_id);
    }

    pub fn append(&mut self, other: &CallSet) {
        for &item in other.0.iter() {
            self.0.insert(item);
        }
    }
}

impl ops::Deref for CallSet {
    type Target = BTreeSet<ProcedureId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
