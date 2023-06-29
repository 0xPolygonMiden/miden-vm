use super::{
    crypto::hash::RpoDigest, tokens::SourceLocation, LibraryNamespace, ProcedureId, String,
    ToString, Token, Vec,
};
use core::fmt;
use vm_core::utils::write_hex_bytes;

// ASSEMBLY ERROR
// ================================================================================================

/// An error which can be generated while compiling a Miden assembly program into a MAST.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AssemblyError {
    CallInKernel(String),
    CallerOutOKernel,
    CallSetProcedureNotFound(ProcedureId),
    CircularModuleDependency(Vec<String>),
    DivisionByZero,
    DuplicateProcName(String, String),
    DuplicateProcId(ProcedureId),
    ExportedProcInProgram(String),
    ProcMastRootNotFound(RpoDigest),
    ImportedProcModuleNotFound(ProcedureId),
    ImportedProcNotFoundInModule(ProcedureId, String),
    InvalidProgramAssemblyContext,
    InvalidCacheLock,
    KernelProcNotFound(ProcedureId),
    LocalProcNotFound(u16, String),
    ParsingError(String),
    ParamOutOfBounds(u64, u64, u64),
    ProcedureNameError(String),
    SysCallInKernel(String),
    LibraryError(String),
    Io(String),
}

impl AssemblyError {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn call_in_kernel(kernel_proc_name: &str) -> Self {
        Self::CallInKernel(kernel_proc_name.to_string())
    }

    pub fn caller_out_of_kernel() -> Self {
        Self::CallerOutOKernel
    }

    pub fn circular_module_dependency(dep_chain: &[String]) -> Self {
        Self::CircularModuleDependency(dep_chain.to_vec())
    }

    pub fn division_by_zero() -> Self {
        Self::DivisionByZero
    }

    pub fn duplicate_proc_name(proc_name: &str, module_path: &str) -> Self {
        Self::DuplicateProcName(proc_name.to_string(), module_path.to_string())
    }

    pub fn duplicate_proc_id(proc_id: &ProcedureId) -> Self {
        Self::DuplicateProcId(*proc_id)
    }

    pub fn exported_proc_in_program(proc_name: &str) -> Self {
        Self::ExportedProcInProgram(proc_name.to_string())
    }

    pub fn proc_mast_root_not_found(root: &RpoDigest) -> Self {
        Self::ProcMastRootNotFound(*root)
    }

    pub fn imported_proc_module_not_found(proc_id: &ProcedureId) -> Self {
        Self::ImportedProcModuleNotFound(*proc_id)
    }

    pub fn imported_proc_not_found_in_module(proc_id: &ProcedureId, module_path: &str) -> Self {
        Self::ImportedProcNotFoundInModule(*proc_id, module_path.to_string())
    }

    pub fn kernel_proc_not_found(kernel_proc_id: &ProcedureId) -> Self {
        Self::KernelProcNotFound(*kernel_proc_id)
    }

    pub fn local_proc_not_found(proc_idx: u16, module_path: &str) -> Self {
        Self::LocalProcNotFound(proc_idx, module_path.to_string())
    }

    pub fn param_out_of_bounds(value: u64, min: u64, max: u64) -> Self {
        Self::ParamOutOfBounds(value, min, max)
    }

    pub fn syscall_in_kernel(kernel_proc_name: &str) -> Self {
        Self::SysCallInKernel(kernel_proc_name.to_string())
    }

    pub fn invalid_cache_lock() -> Self {
        Self::InvalidCacheLock
    }
}

impl From<ParsingError> for AssemblyError {
    fn from(err: ParsingError) -> Self {
        Self::ParsingError(err.message)
    }
}

impl From<LibraryError> for AssemblyError {
    fn from(err: LibraryError) -> Self {
        Self::LibraryError(err.to_string())
    }
}

impl From<LabelError> for AssemblyError {
    fn from(err: LabelError) -> Self {
        Self::ProcedureNameError(format!("invalid procedure name: {err}"))
    }
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AssemblyError::*;
        match self {
            CallInKernel(proc_name) => write!(f, "call instruction used kernel procedure '{proc_name}'"),
            CallerOutOKernel => write!(f, "caller instruction used outside of kernel"),
            CallSetProcedureNotFound(proc_id) => write!(f, "callset procedure not found in assembler cache for procedure  '{proc_id}'"),
            CircularModuleDependency(dep_chain) => write!(f, "circular module dependency in the following chain: {dep_chain:?}"),
            DivisionByZero => write!(f, "division by zero"),
            DuplicateProcName(proc_name, module_path) => write!(f, "duplicate proc name '{proc_name}' in module {module_path}"),
            DuplicateProcId(proc_id) => write!(f, "duplicate proc id {proc_id}"),
            ExportedProcInProgram(proc_name) => write!(f, "exported procedure '{proc_name}' in executable program"),
            ImportedProcModuleNotFound(proc_id) => write!(f, "module for imported procedure {proc_id} not found"),
            ImportedProcNotFoundInModule(proc_id, module_path) => write!(f, "imported procedure {proc_id} not found in module {module_path}"),
            InvalidProgramAssemblyContext => write!(f, "assembly context improperly initialized for program compilation"),
            InvalidCacheLock => write!(f, "an attempt was made to lock a borrowed procedures cache"),
            Io(description) => write!(f, "I/O error: {description}"),
            KernelProcNotFound(proc_id) => write!(f, "procedure {proc_id} not found in kernel"),
            LibraryError(err) | ParsingError(err) | ProcedureNameError(err) => write!(f, "{err}"),
            LocalProcNotFound(proc_idx, module_path) => write!(f, "procedure at index {proc_idx} not found in module {module_path}"),
            ParamOutOfBounds(value, min, max) => write!(f, "parameter value must be greater than or equal to {min} and less than or equal to {max}, but was {value}"),
            ProcMastRootNotFound(digest) => {
                write!(f, "procedure mast root not found for digest - ")?;
                write_hex_bytes(f, &digest.as_bytes())
            },
            SysCallInKernel(proc_name) => write!(f, "syscall instruction used in kernel procedure '{proc_name}'"),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for AssemblyError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AssemblyError {}

// PARSING ERROR
// ================================================================================================

/// An error which can be generated while parsing a Miden assembly source code into an AST.
#[derive(Clone, Eq, PartialEq)]
pub struct ParsingError {
    message: String,
    location: SourceLocation,
    op: String,
}

impl ParsingError {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn empty_source() -> Self {
        ParsingError {
            message: "source code cannot be an empty string".to_string(),
            location: SourceLocation::default(),
            op: "".to_string(),
        }
    }

    pub fn unexpected_eof(location: SourceLocation) -> Self {
        ParsingError {
            message: "unexpected EOF".to_string(),
            location,
            op: "".to_string(),
        }
    }

    pub fn unexpected_token(token: &Token, expected: &str) -> Self {
        ParsingError {
            message: format!("unexpected token: expected '{expected}' but was '{token}'"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    // CONSTANTS DECLARATION
    // --------------------------------------------------------------------------------------------
    pub fn duplicate_const_name(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("duplicate constant name: '{label}'"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_const_name(token: &Token, err: LabelError) -> Self {
        ParsingError {
            message: format!("invalid constant name: {err}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_const_value(token: &Token, value: &str, reason: &str) -> Self {
        ParsingError {
            message: format!(
                "malformed constant `{token}` - invalid value: `{value}` - reason: {reason}"
            ),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn const_invalid_scope(token: &Token) -> Self {
        ParsingError {
            message: format!("invalid constant declaration: `{token}` - constants can only be defined below imports and above procedure / program bodies"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn const_not_found(token: &Token) -> Self {
        ParsingError {
            message: format!("constant used in operation `{token}` not found"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn const_conversion_failed(token: &Token, type_name: &str) -> Self {
        ParsingError {
            message: format!(
                "failed to convert u64 constant used in `{token}` to required type {type_name}"
            ),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    // INVALID / MALFORMED INSTRUCTIONS
    // --------------------------------------------------------------------------------------------

    pub fn invalid_op(token: &Token) -> Self {
        ParsingError {
            message: format!("instruction '{token}' is invalid"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn missing_param(token: &Token) -> Self {
        ParsingError {
            message: format!("malformed instruction '{token}': missing required parameter"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn extra_param(token: &Token) -> Self {
        ParsingError {
            message: format!("malformed instruction '{token}': too many parameters provided"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_param(token: &Token, part_idx: usize) -> Self {
        ParsingError {
            message: format!(
                "malformed instruction `{token}`: parameter '{}' is invalid",
                token.parts()[part_idx]
            ),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_param_with_reason(token: &Token, part_idx: usize, reason: &str) -> Self {
        ParsingError {
            message: format!(
                "malformed instruction '{token}', parameter {} is invalid: {reason}",
                token.parts()[part_idx],
            ),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    // MALFORMED CODE BLOCKS
    // --------------------------------------------------------------------------------------------

    pub fn dangling_else(token: &Token) -> Self {
        ParsingError {
            message: "else without matching if".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_if(token: &Token) -> Self {
        ParsingError {
            message: "if without matching else/end".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_while(token: &Token) -> Self {
        ParsingError {
            message: "while without matching end".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_repeat(token: &Token) -> Self {
        ParsingError {
            message: "repeat without matching end".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_else(token: &Token) -> Self {
        ParsingError {
            message: "else without matching end".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_begin(token: &Token) -> Self {
        ParsingError {
            message: "begin without matching end".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn dangling_ops_after_program(token: &Token) -> Self {
        ParsingError {
            message: "dangling instructions after program end".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn dangling_ops_after_module(token: &Token) -> Self {
        ParsingError {
            message: "dangling instructions after module end".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn dangling_procedure_comment(location: SourceLocation) -> Self {
        ParsingError {
            message: "Procedure comment is not immediately followed by a procedure declaration."
                .to_string(),
            location,
            op: "".to_string(),
        }
    }

    pub fn not_a_library_module(token: &Token) -> Self {
        ParsingError {
            message: "not a module: `begin` instruction found".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn too_many_module_procs(num_procs: usize, max_procs: usize) -> Self {
        ParsingError {
            message: format!(
                "a module cannot contain more than {max_procs} procedures, but had {num_procs}"
            ),
            location: SourceLocation::default(),
            op: "".to_string(),
        }
    }

    pub fn module_docs_too_long(doc_len: usize, max_len: usize) -> Self {
        ParsingError {
            message: format!(
                "module doc comments cannot exceed {max_len} bytes, but was {doc_len}"
            ),
            location: SourceLocation::default(),
            op: "".to_string(),
        }
    }

    pub fn body_too_long(token: &Token, body_size: usize, max_body_size: usize) -> Self {
        ParsingError {
            message: format!("body block size cannot contain more than {max_body_size} instructions, but had {body_size}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    // PROCEDURES DECLARATION
    // --------------------------------------------------------------------------------------------

    pub fn duplicate_proc_name(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("duplicate procedure name: {label}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_proc_name(token: &Token, err: LabelError) -> Self {
        ParsingError {
            message: format!("invalid procedure name: {err}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_reexported_procedure(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("invalid re-exported procedure: {label}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn proc_name_too_long(token: &Token, label: &str, max_len: u8) -> Self {
        ParsingError {
            message: format!(
                "procedure name cannot be longer than {max_len} characters, but was {}",
                label.len()
            ),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_proc_locals(token: &Token, locals: &str) -> Self {
        ParsingError {
            message: format!("invalid procedure locals: {locals}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn too_many_proc_locals(token: &Token, num_locals: u64, max_locals: u64) -> Self {
        ParsingError {
            message: format!("number of procedure locals cannot be greater than {max_locals} characters, but was {num_locals}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_proc(token: &Token, proc_name: &str) -> Self {
        ParsingError {
            message: format!("procedure '{proc_name}' has no matching end"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn proc_export_not_allowed(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("exported procedures not allowed in this context: {label}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn proc_docs_too_long(token: &Token, doc_len: usize, max_len: usize) -> Self {
        ParsingError {
            message: format!(
                "procedure doc comments cannot exceed {max_len} bytes, but was {doc_len}"
            ),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    // PROCEDURE INVOCATION
    // --------------------------------------------------------------------------------------------

    pub fn invalid_proc_root_invocation(token: &Token, label: &str, err: LabelError) -> Self {
        ParsingError {
            message: format!("invalid procedure root invocation: {label} - {err}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_proc_invocation(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("invalid procedure invocation: {label}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn exec_with_mast_root(token: &Token) -> Self {
        ParsingError {
            message: "invalid exec: cannot invoke a procedure on a mast root".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn syscall_with_module_name(token: &Token) -> Self {
        ParsingError {
            message: "invalid syscall: cannot invoke a syscall on a named module".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn syscall_with_mast_root(token: &Token) -> Self {
        ParsingError {
            message: "invalid syscall: cannot invoke a syscall on a mast root".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn undefined_local_proc(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("undefined local procedure: {label}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn procedure_module_not_imported(token: &Token, module_name: &str) -> Self {
        ParsingError {
            message: format!("module '{module_name}' was not imported"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn too_many_imported_procs_invoked(
        token: &Token,
        num_procs: usize,
        max_procs: usize,
    ) -> Self {
        ParsingError {
            message: format!(
                "a module cannot invoke more than {max_procs} imported procedures, but had {num_procs}"
            ),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    // IMPORTS AND MODULES
    // --------------------------------------------------------------------------------------------

    pub fn duplicate_module_import(token: &Token, module: &str) -> Self {
        ParsingError {
            message: format!("duplicate module import found: {module}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_module_path(token: &Token, module_path: &str) -> Self {
        ParsingError {
            message: format!("invalid module import path: {module_path}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn import_inside_body(token: &Token) -> Self {
        ParsingError {
            message: "import in procedure body".to_string(),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn invalid_library_path(token: &Token, error: LibraryError) -> Self {
        ParsingError {
            message: format!("invalid path resolution: {error}"),
            location: *token.location(),
            op: token.to_string(),
        }
    }

    pub fn too_many_imports(num_imports: usize, max_imports: usize) -> Self {
        ParsingError {
            message: format!(
                "a module cannot contain more than {max_imports} imports, but had {num_imports}"
            ),
            location: SourceLocation::default(),
            op: "".to_string(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn operation(&self) -> &String {
        &self.op
    }

    pub const fn location(&self) -> &SourceLocation {
        &self.location
    }
}

impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parsing error at {}: {}", self.location, self.message)
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parsing error at {}: {}", self.location, self.message)
    }
}

#[cfg(feature = "std")]
impl From<ParsingError> for std::io::Error {
    fn from(e: ParsingError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParsingError {}

// NAME ERROR
// ================================================================================================

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LabelError {
    EmptyLabel,
    RpoDigestHexLabelIncorrectLength(usize),
    InvalidHexCharacters(String),
    InvalidHexRpoDigestLabel(String),
    InvalidFirstLetter(String),
    InvalidChars(String),
    LabelTooLong(String, usize),
    Uppercase(String),
}

impl LabelError {
    pub const fn empty_label() -> Self {
        Self::EmptyLabel
    }

    pub fn rpo_digest_hex_label_incorrect_length(len: usize) -> Self {
        Self::RpoDigestHexLabelIncorrectLength(len)
    }

    pub fn invalid_label(label: &str) -> Self {
        Self::InvalidChars(label.to_string())
    }

    pub fn invalid_fist_letter(label: &str) -> Self {
        Self::InvalidFirstLetter(label.to_string())
    }

    pub fn label_too_long(label: &str, max_len: usize) -> Self {
        Self::LabelTooLong(label.to_string(), max_len)
    }

    pub fn must_be_uppercase(label: &str) -> Self {
        Self::Uppercase(label.to_string())
    }
}

impl fmt::Display for LabelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LabelError::*;
        match self {
            EmptyLabel => write!(f, "label cannot be empty"),
            RpoDigestHexLabelIncorrectLength(len) => {
                write!(f, "rpo digest hex label must have 66 characters, but was {}", len)
            }
            InvalidHexCharacters(label) => {
                write!(f, "'{label}' contains invalid hex characters")
            }
            InvalidHexRpoDigestLabel(label) => {
                write!(f, "'{label}' is not a valid Rpo Digest hex label")
            }
            InvalidFirstLetter(label) => {
                write!(f, "'{label}' does not start with a letter")
            }
            InvalidChars(label) => {
                write!(f, "'{label}' contains invalid characters")
            }
            LabelTooLong(label, max_len) => {
                write!(f, "'{label}' is over {max_len} characters long")
            }
            Uppercase(label) => write!(f, "'{label}' cannot contain lower-case characters"),
        }
    }
}

// LIBRARY ERROR
// ================================================================================================

#[derive(Clone, Debug)]
pub enum LibraryError {
    DeserializationFailed(String, String),
    DuplicateModulePath(String),
    DuplicateNamespace(String),
    FileIO(String, String),
    InconsistentNamespace {
        expected: String,
        actual: String,
    },
    InvalidNamespace(LabelError),
    InvalidPath(PathError),
    InvalidVersionNumber {
        version: String,
        err_msg: String,
    },
    MissingVersionComponent {
        version: String,
        component: String,
    },
    ModuleNotFound(String),
    NoModulesInLibrary {
        name: LibraryNamespace,
    },
    TooManyDependenciesInLibrary {
        name: LibraryNamespace,
        num_dependencies: usize,
        max_dependencies: usize,
    },
    TooManyModulesInLibrary {
        name: LibraryNamespace,
        num_modules: usize,
        max_modules: usize,
    },
    TooManyVersionComponents {
        version: String,
    },
}

impl LibraryError {
    pub fn deserialization_error(path: &str, message: &str) -> Self {
        Self::DeserializationFailed(path.into(), message.into())
    }

    pub fn duplicate_module_path(path: &str) -> Self {
        Self::DuplicateModulePath(path.into())
    }

    pub fn duplicate_namespace(namespace: &str) -> Self {
        Self::DuplicateNamespace(namespace.into())
    }

    pub fn file_error(path: &str, message: &str) -> Self {
        Self::FileIO(path.into(), message.into())
    }

    pub fn inconsistent_namespace(expected: &str, actual: &str) -> Self {
        Self::InconsistentNamespace {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub fn invalid_namespace(err: LabelError) -> Self {
        Self::InvalidNamespace(err)
    }

    pub fn invalid_version_number(version: &str, err_msg: String) -> Self {
        Self::InvalidVersionNumber {
            version: version.into(),
            err_msg,
        }
    }

    pub fn missing_version_component(version: &str, component: &str) -> Self {
        Self::MissingVersionComponent {
            version: version.into(),
            component: component.into(),
        }
    }

    pub fn no_modules_in_library(name: LibraryNamespace) -> Self {
        Self::NoModulesInLibrary { name }
    }

    pub fn too_many_modules_in_library(
        name: LibraryNamespace,
        num_modules: usize,
        max_modules: usize,
    ) -> Self {
        Self::TooManyModulesInLibrary {
            name,
            num_modules,
            max_modules,
        }
    }

    pub fn too_many_dependencies_in_library(
        name: LibraryNamespace,
        num_dependencies: usize,
        max_dependencies: usize,
    ) -> Self {
        Self::TooManyDependenciesInLibrary {
            name,
            num_dependencies,
            max_dependencies,
        }
    }

    pub fn too_many_version_components(version: &str) -> Self {
        Self::TooManyVersionComponents {
            version: version.into(),
        }
    }
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LibraryError::*;
        match self {
            DeserializationFailed(path, message) => {
                write!(f, "library deserialization failed - '{path}': {message}")
            }
            DuplicateModulePath(path) => write!(f, "duplciate module path '{path}'"),
            DuplicateNamespace(namespace) => write!(f, "duplicate namespace '{namespace}'"),
            FileIO(path, message) => {
                write!(f, "file error - '{path}': {message}")
            }
            InconsistentNamespace { expected, actual } => {
                write!(f, "inconsistent module namespace: expected '{expected}', but was {actual}")
            }
            InvalidNamespace(err) => {
                write!(f, "invalid namespace: {err}")
            }
            InvalidPath(err) => {
                write!(f, "invalid path: {err}")
            }
            InvalidVersionNumber { version, err_msg } => {
                write!(f, "version '{version}' is invalid: {err_msg}")
            }
            MissingVersionComponent { version, component } => {
                write!(f, "version '{version}' is invalid: missing {component} version component")
            }
            ModuleNotFound(path) => write!(f, "module '{path}' not found"),
            NoModulesInLibrary { name } => {
                write!(f, "library '{}' does not contain any modules", name.as_str())
            }
            TooManyDependenciesInLibrary {
                name,
                num_dependencies,
                max_dependencies,
            } => {
                write!(
                    f,
                    "library '{}' contains {num_dependencies} dependencies, but max is {max_dependencies}",
                    name.as_str()
                )
            }
            TooManyModulesInLibrary {
                name,
                num_modules,
                max_modules,
            } => {
                write!(
                    f,
                    "library '{}' contains {num_modules} modules, but max is {max_modules}",
                    name.as_str()
                )
            }
            TooManyVersionComponents { version } => {
                write!(f, "version '{version}' contains too many components")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LibraryError {}

#[cfg(feature = "std")]
impl From<LibraryError> for std::io::Error {
    fn from(err: LibraryError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, err)
    }
}

impl From<PathError> for LibraryError {
    fn from(value: PathError) -> Self {
        LibraryError::InvalidPath(value)
    }
}

// PATH ERROR
// ================================================================================================

#[derive(Clone, Debug)]
pub enum PathError {
    ComponentInvalidChar { component: String },
    ComponentInvalidFirstChar { component: String },
    ComponentTooLong { component: String, max_len: usize },
    EmptyComponent,
    EmptyPath,
    PathTooLong { path: String, max_len: usize },
    TooFewComponents { path: String, min_components: usize },
}

impl PathError {
    pub fn component_invalid_char(component: &str) -> Self {
        Self::ComponentInvalidChar {
            component: component.into(),
        }
    }

    pub fn component_invalid_first_char(component: &str) -> Self {
        Self::ComponentInvalidFirstChar {
            component: component.into(),
        }
    }

    pub fn component_too_long(component: &str, max_len: usize) -> Self {
        Self::ComponentTooLong {
            component: component.into(),
            max_len,
        }
    }

    pub fn path_too_long(path: &str, max_len: usize) -> Self {
        Self::PathTooLong {
            path: path.into(),
            max_len,
        }
    }

    pub fn too_few_components(path: &str, min_components: usize) -> Self {
        Self::TooFewComponents {
            path: path.into(),
            min_components,
        }
    }
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PathError::*;
        match self {
            ComponentInvalidChar { component } => {
                write!(f, "path component '{component}' contains invalid characters")
            }
            ComponentInvalidFirstChar { component } => {
                write!(f, "path component '{component}' does not start with a letter")
            }
            ComponentTooLong { component, max_len } => {
                write!(f, "path component '{component}' contains over {max_len} characters")
            }
            EmptyComponent => {
                write!(f, "path component cannot be an empty string")
            }
            EmptyPath => {
                write!(f, "path cannot be an empty string")
            }
            PathTooLong { path, max_len } => {
                write!(f, "path  `{path}` contains over {max_len} characters")
            }
            TooFewComponents {
                path,
                min_components,
            } => {
                write!(f, "path  `{path}` does not consist of at least {min_components} components")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PathError {}
