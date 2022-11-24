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
    KernelProcNotFound(ProcedureId),
    LocalProcNotFound(u16, String),
    ParsingError(String),
    ParamOutOfBounds(u64, u64, u64),
    SysCallInKernel(String),
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
}

impl From<ParsingError> for AssemblyError {
    fn from(err: ParsingError) -> Self {
        Self::ParsingError(err.message)
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
            KernelProcNotFound(proc_id) => write!(f, "procedure {proc_id} not found in kernel"),
            LocalProcNotFound(proc_idx, module_path) => write!(f, "procedure at index {proc_idx} not found in module {module_path}"),
            ParsingError(err) => write!(f, "{err}"),
            ParamOutOfBounds(value, min, max) => write!(f, "parameter value must be greater than or equal to {min} and less than or equal to {max}, but was {value}"),
            SysCallInKernel(proc_name) => write!(f, "syscall instruction used in kernel procedure '{proc_name}'"),
        }
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

    pub fn unexpected_body_end(token: &Token) -> Self {
        ParsingError {
            message: format!("unexpected body termination: invalid token '{token}'"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn empty_block(token: &Token) -> Self {
        ParsingError {
            message: "a code block must contain at least one instruction".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_op(token: &Token) -> Self {
        ParsingError {
            message: format!("instruction '{token}' is invalid"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    /// TODO: currently unused
    pub fn invalid_op_with_reason(token: &Token, reason: &str) -> Self {
        ParsingError {
            message: format!("instruction '{token}' is invalid: {reason}"),
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

    pub fn malformed_doc_comment(step: usize) -> Self {
        ParsingError {
            message: "doc comments separated by line break".to_string(),
            step,
            op: "".to_string(),
        }
    }

    // PROGRAM
    // --------------------------------------------------------------------------------------------

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

    // PROCEDURES
    // --------------------------------------------------------------------------------------------

    pub fn duplicate_proc_label(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("duplicate procedure label: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_proc_label(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("invalid procedure label: {label}"),
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

    pub fn unmatched_proc(token: &Token) -> Self {
        ParsingError {
            message: "proc without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn undefined_proc(token: &Token, label: &str) -> Self {
        ParsingError {
            message: format!("undefined procedure: {label}"),
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

    /// TODO: currently unused
    pub fn syscall_in_kernel(token: &Token) -> Self {
        ParsingError {
            message: "syscall inside kernel".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    /// TODO: currently unused
    pub fn call_in_kernel(token: &Token) -> Self {
        ParsingError {
            message: "call inside kernel".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    /// TODO: currently unused
    pub fn caller_out_of_kernel(token: &Token) -> Self {
        ParsingError {
            message: "caller instruction executed outside of kernel context".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // IMPORTS AND MODULES
    // --------------------------------------------------------------------------------------------

    /// TODO: currently unused
    pub fn dangling_ops_after_module(token: &Token, module_path: &str) -> Self {
        ParsingError {
            message: format!("dangling instructions after module end at {module_path}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

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
impl std::error::Error for ParsingError {}

// SERIALIZATION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum SerializationError {
    InvalidBoolValue,
    StringTooLong,
    EndOfReader,
    InvalidOpCode,
    InvalidFieldElement,
}

// LIBRARY ERROR
// ================================================================================================

#[derive(Clone, Debug)]
pub enum LibraryError {
    ModuleNotFound(String),
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LibraryError::*;
        match self {
            ModuleNotFound(path) => write!(f, "module '{path}' not found"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LibraryError {}
