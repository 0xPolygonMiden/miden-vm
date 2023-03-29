use super::{ProcedureId, String, ToString, Token, Vec};
use core::fmt;

// ASSEMBLY ERROR
// ================================================================================================

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AssemblyError {
    CallInKernel(String),
    CallerOutOKernel,
    CircularModuleDependency(Vec<String>),
    DivisionByZero,
    DuplicateProcName(String, String),
    ExportedProcInProgram(String),
    ImportedProcModuleNotFound(ProcedureId),
    ImportedProcNotFoundInModule(ProcedureId, String),
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

    pub fn exported_proc_in_program(proc_name: &str) -> Self {
        Self::ExportedProcInProgram(proc_name.to_string())
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
            CircularModuleDependency(dep_chain) => write!(f, "circular module dependency in the following chain: {dep_chain:?}"),
            DivisionByZero => write!(f, "division by zero"),
            DuplicateProcName(proc_name, module_path) => write!(f, "duplicate proc name '{proc_name}' in module {module_path}"),
            ExportedProcInProgram(proc_name) => write!(f, "exported procedure '{proc_name}' in executable program"),
            ImportedProcModuleNotFound(proc_id) => write!(f, "module for imported procedure {proc_id} not found"),
            ImportedProcNotFoundInModule(proc_id, module_path) => write!(f, "imported procedure {proc_id} not found in module {module_path}"),
            InvalidCacheLock => write!(f, "an attempt was made to lock a borrowed procedures cache"),
            KernelProcNotFound(proc_id) => write!(f, "procedure {proc_id} not found in kernel"),
            LocalProcNotFound(proc_idx, module_path) => write!(f, "procedure at index {proc_idx} not found in module {module_path}"),
            ParamOutOfBounds(value, min, max) => write!(f, "parameter value must be greater than or equal to {min} and less than or equal to {max}, but was {value}"),
            SysCallInKernel(proc_name) => write!(f, "syscall instruction used in kernel procedure '{proc_name}'"),
            LibraryError(err) | ParsingError(err) | ProcedureNameError(err) => write!(f, "{err}"),
            Io(description) => write!(f, "I/O error: {description}"),
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

#[derive(Clone, Eq, PartialEq)]
pub struct ParsingError {
    message: String,
    step: usize,
    op: String,
}

impl ParsingError {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn empty_source() -> Self {
        ParsingError {
            message: "source code cannot be an empty string".to_string(),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn unexpected_eof(step: usize) -> Self {
        ParsingError {
            message: "unexpected EOF".to_string(),
            step,
            op: "".to_string(),
        }
    }

    pub fn unexpected_token(token: &Token, expected: &str) -> Self {
        ParsingError {
            message: format!("unexpected token: expected '{expected}' but was '{token}'"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // CONSTANTS DECLARATION
    // --------------------------------------------------------------------------------------------
    pub fn duplicate_const_name(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("duplicate constant name: '{label}'"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_const_name(token: &Token, err: LabelError) -> Self {
        ParsingError {
            message: format!("invalid constant name: {err}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_const_value(token: &Token, value: &str, reason: &str) -> Self {
        ParsingError {
            message: format!(
                "malformed constant `{token}` - invalid value: `{value}` - reason: {reason}"
            ),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn const_invalid_scope(token: &Token) -> Self {
        ParsingError {
            message: format!("invalid constant declaration: `{token}` - constants can only be defined below imports and above procedure / program bodies"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn const_not_found(token: &Token) -> Self {
        ParsingError {
            message: format!("constant used in operation `{token}` not found"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // INVALID / MALFORMED INSTRUCTIONS
    // --------------------------------------------------------------------------------------------

    pub fn invalid_op(token: &Token) -> Self {
        ParsingError {
            message: format!("instruction '{token}' is invalid"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn missing_param(token: &Token) -> Self {
        ParsingError {
            message: format!("malformed instruction '{token}': missing required parameter"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn extra_param(token: &Token) -> Self {
        ParsingError {
            message: format!("malformed instruction '{token}': too many parameters provided"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_param(token: &Token, part_idx: usize) -> Self {
        ParsingError {
            message: format!(
                "malformed instruction `{token}`: parameter '{}' is invalid",
                token.parts()[part_idx]
            ),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_param_with_reason(token: &Token, part_idx: usize, reason: &str) -> Self {
        ParsingError {
            message: format!(
                "malformed instruction '{token}', parameter {} is invalid: {reason}",
                token.parts()[part_idx],
            ),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // MALFORMED CODE BLOCKS
    // --------------------------------------------------------------------------------------------

    pub fn dangling_else(token: &Token) -> Self {
        ParsingError {
            message: "else without matching if".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_if(token: &Token) -> Self {
        ParsingError {
            message: "if without matching else/end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_while(token: &Token) -> Self {
        ParsingError {
            message: "while without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_repeat(token: &Token) -> Self {
        ParsingError {
            message: "repeat without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_else(token: &Token) -> Self {
        ParsingError {
            message: "else without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_begin(token: &Token) -> Self {
        ParsingError {
            message: "begin without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn dangling_ops_after_program(token: &Token) -> Self {
        ParsingError {
            message: "dangling instructions after program end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn dangling_ops_after_module(token: &Token) -> Self {
        ParsingError {
            message: "dangling instructions after module end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn dangling_procedure_comment(step: usize) -> Self {
        ParsingError {
            message: "Procedure comment is not immediately followed by a procedure declaration."
                .to_string(),
            step,
            op: "".to_string(),
        }
    }

    pub fn not_a_library_module(token: &Token) -> Self {
        ParsingError {
            message: "not a module: `begin` instruction found".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // PROCEDURES DECLARATION
    // --------------------------------------------------------------------------------------------

    pub fn duplicate_proc_name(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("duplicate procedure name: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_proc_name(token: &Token, err: LabelError) -> Self {
        ParsingError {
            message: format!("invalid procedure name: {err}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn proc_name_too_long(token: &Token, label: &str, max_len: u8) -> Self {
        ParsingError {
            message: format!(
                "procedure name cannot be longer than {max_len} characters, but was {}",
                label.len()
            ),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_proc_locals(token: &Token, locals: &str) -> Self {
        ParsingError {
            message: format!("invalid procedure locals: {locals}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn too_many_proc_locals(token: &Token, num_locals: u64, max_locals: u64) -> Self {
        ParsingError {
            message: format!("number of procedure locals cannot be greater than {max_locals} characters, but was {num_locals}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_proc(token: &Token, proc_name: &str) -> Self {
        ParsingError {
            message: format!("procedure '{proc_name}' has no matching end"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn proc_export_not_allowed(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("exported procedures not allowed in this context: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // PROCEDURE INVOCATION
    // --------------------------------------------------------------------------------------------

    pub fn invalid_proc_invocation(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("invalid procedure invocation: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn syscall_with_module_name(token: &Token) -> Self {
        ParsingError {
            message: "invalid syscall: cannot invoke a syscall on a named module".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn undefined_local_proc(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("undefined local procedure: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn procedure_module_not_imported(token: &Token, module_name: &str) -> Self {
        ParsingError {
            message: format!("module '{module_name}' was not imported"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // IMPORTS AND MODULES
    // --------------------------------------------------------------------------------------------

    pub fn duplicate_module_import(token: &Token, module: &str) -> Self {
        ParsingError {
            message: format!("duplicate module import found: {module}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_module_path(token: &Token, module_path: &str) -> Self {
        ParsingError {
            message: format!("invalid module import path: {module_path}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn import_inside_body(token: &Token) -> Self {
        ParsingError {
            message: "import in procedure body".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_library_path(token: &Token, error: LibraryError) -> Self {
        ParsingError {
            message: format!("invalid path resolution: {error}"),
            step: token.pos(),
            op: token.to_string(),
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

    pub fn step(&self) -> usize {
        self.step
    }
}

impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parsing error at {}: {}", self.step, self.message)
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parsing error at {}: {}", self.step, self.message)
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
    InvalidFirstLetter(String),
    InvalidChars(String),
    LabelTooLong(String, usize),
    Uppercase(String),
}

impl LabelError {
    pub const fn empty_label() -> Self {
        Self::EmptyLabel
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

// SERIALIZATION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum SerializationError {
    EndOfReader,
    InvalidBoolValue,
    InvalidFieldElement,
    InvalidModulePath,
    InvalidNamespace,
    InvalidNumber,
    InvalidNumOfPushValues,
    InvalidOpCode,
    InvalidPathNoDelimiter,
    InvalidProcedureName,
    InvalidUtf8,
    LengthTooLong,
    UnexpectedEndOfStream,
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SerializationError::*;
        match self {
            EndOfReader => write!(f, "unexpected reader EOF"),
            InvalidBoolValue => write!(f, "invalid boolean value"),
            InvalidFieldElement => write!(f, "could not read a valid field element"),
            InvalidModulePath => write!(f, "could not read a valid module path definition"),
            InvalidNamespace => write!(f, "could not read a valid namespace definition"),
            InvalidNumber => write!(f, "could not read a valid number"),
            InvalidNumOfPushValues => write!(f, "invalid push values argument"),
            InvalidOpCode => write!(f, "could not read a valid opcode"),
            InvalidPathNoDelimiter => write!(f, "a path must contain a delimiter"),
            InvalidProcedureName => write!(f, "invalid procedure name"),
            InvalidUtf8 => write!(f, "could not read a well-formed utf-8 string"),
            LengthTooLong => write!(f, "the provided length is too long and is not supported"),
            UnexpectedEndOfStream => write!(f, "the stream of tokens reached an unexpected end"),
        }
    }
}

#[cfg(feature = "std")]
impl From<SerializationError> for std::io::Error {
    fn from(e: SerializationError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SerializationError {}

impl From<LabelError> for SerializationError {
    fn from(_err: LabelError) -> Self {
        Self::InvalidProcedureName
    }
}

// LIBRARY ERROR
// ================================================================================================

#[derive(Clone, Debug)]
pub enum LibraryError {
    ModuleNotFound(String),
    DeserializationFailed(String, String),
    DuplicateModulePath(String),
    DuplicateNamespace(String),
    EmptyProcedureName,
    FileIO(String, String),
    ProcedureNameWithDelimiter(String),
    ModulePathStartsWithDelimiter(String),
    ModulePathEndsWithDelimiter(String),
    LibraryNameWithDelimiter(String),
    NamespaceViolation { expected: String, found: String },
}

impl LibraryError {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn duplicate_module_path(path: &str) -> Self {
        Self::DuplicateModulePath(path.to_string())
    }

    pub fn duplicate_namespace(namespace: &str) -> Self {
        Self::DuplicateNamespace(namespace.to_string())
    }

    pub fn file_error(path: &str, message: &str) -> Self {
        Self::FileIO(path.to_string(), message.to_string())
    }

    pub fn deserialization_error(path: &str, message: &str) -> Self {
        Self::DeserializationFailed(path.to_string(), message.to_string())
    }

    pub fn procedure_name_with_delimiter(name: &str) -> Self {
        Self::ProcedureNameWithDelimiter(name.to_string())
    }

    pub fn module_path_starts_with_delimiter(path: &str) -> Self {
        Self::ModulePathStartsWithDelimiter(path.to_string())
    }

    pub fn module_path_ends_with_delimiter(path: &str) -> Self {
        Self::ModulePathEndsWithDelimiter(path.to_string())
    }

    pub fn library_name_with_delimiter(name: &str) -> Self {
        Self::LibraryNameWithDelimiter(name.to_string())
    }

    pub fn namespace_violation(expected: &str, found: &str) -> Self {
        Self::NamespaceViolation {
            expected: expected.into(),
            found: found.into(),
        }
    }
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LibraryError::*;
        match self {
            ModuleNotFound(path) => write!(f, "module '{path}' not found"),
            DeserializationFailed(path, message) => {
                write!(f, "library deserialization failed - '{path}': {message}")
            }
            DuplicateModulePath(path) => write!(f, "duplciate module path '{path}'"),
            DuplicateNamespace(namespace) => write!(f, "duplicate namespace '{namespace}'"),
            EmptyProcedureName => write!(f, "the procedure name cannot be empty"),
            FileIO(path, message) => {
                write!(f, "file error - '{path}': {message}")
            }
            ProcedureNameWithDelimiter(name) => {
                write!(f, "'{name}' cannot contain a module delimiter")
            }
            ModulePathStartsWithDelimiter(path) => {
                write!(f, "'{path}' cannot start with a module delimiter")
            }
            ModulePathEndsWithDelimiter(path) => {
                write!(f, "'{path}' cannot end with a module delimiter")
            }
            LibraryNameWithDelimiter(name) => {
                write!(f, "'{name}' cannot contain a module delimiter")
            }
            NamespaceViolation { expected, found } => {
                write!(f, "invalid namespace! expected '{expected}', found '{found}'")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LibraryError {}
