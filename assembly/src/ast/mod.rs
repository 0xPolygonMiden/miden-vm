//! Abstract syntax tree (AST) components of Miden programs, modules, and procedures.

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

pub use self::{
    block::Block,
    constants::{Constant, ConstantExpr, ConstantOp},
    form::Form,
    ident::{CaseKindError, Ident, IdentError},
    immediate::{ErrorCode, ImmFelt, ImmU16, ImmU32, ImmU8, Immediate},
    imports::Import,
    instruction::{advice::SignatureKind, AdviceInjectorNode, DebugOptions, Instruction, OpCode},
    invocation_target::{InvocationTarget, Invoke, InvokeKind},
    module::{Module, ModuleKind},
    op::Op,
    procedure::*,
    serde::AstSerdeOptions,
    visit::{Visit, VisitMut},
};

pub(crate) type SmallOpsVec = smallvec::SmallVec<[Op; 1]>;

/// Maximum stack index at which a full word can start.
pub const MAX_STACK_WORD_OFFSET: u8 = 12;
