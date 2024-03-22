//! Abstract syntax tree (AST) components of Miden programs, modules, and procedures.

pub use tracing::{event, info_span, instrument, Level};

mod block;
mod constants;
mod form;
mod ident;
mod immediate;
mod imports;
mod instruction;
mod invocation_target;
mod module;
mod op;
mod procedure;
mod serde;
#[cfg(test)]
mod tests;
pub mod visit;

pub use self::block::Block;
pub use self::constants::{Constant, ConstantExpr, ConstantOp};
pub use self::form::Form;
pub use self::ident::{CaseKindError, Ident, IdentError};
pub use self::immediate::{ErrorCode, ImmFelt, ImmU16, ImmU32, ImmU8, Immediate};
pub use self::imports::Import;
pub use self::instruction::{
    advice::SignatureKind, AdviceInjectorNode, DebugOptions, Instruction, OpCode,
};
pub use self::invocation_target::{InvocationTarget, Invoke, InvokeKind};
pub use self::module::{Module, ModuleKind};
pub use self::op::Op;
pub use self::procedure::*;
pub use self::serde::AstSerdeOptions;
pub use self::visit::{Visit, VisitMut};

pub(crate) type SmallOpsVec = smallvec::SmallVec<[Op; 1]>;

/// Maximum number of procedures in a module.
pub const MAX_LOCAL_PROCS: usize = u16::MAX as usize;

/// Maximum number of re-exported procedures in a module.
pub const MAX_REEXPORTED_PROCS: usize = u16::MAX as usize;

/// Maximum number of bytes for a single documentation comment.
pub const MAX_DOCS_LEN: usize = u16::MAX as usize;

/// Maximum number of nodes in statement body (e.g., procedure body, loop body etc.).
pub const MAX_BODY_LEN: usize = u16::MAX as usize;

/// Maximum number of imported libraries in a module or a program
pub const MAX_IMPORTS: usize = u16::MAX as usize;

/// Maximum number of imported procedures used in a module or a program
pub const MAX_INVOKED_IMPORTED_PROCS: usize = u16::MAX as usize;

/// Maximum stack index at which a full word can start.
pub const MAX_STACK_WORD_OFFSET: u8 = 12;
