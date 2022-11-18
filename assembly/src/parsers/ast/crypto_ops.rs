use super::{AssemblyError, Instruction, Node};
use crate::Token;

/// Returns `RPHash` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_rphash(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::RPHash))
}

/// Returns `RPPerm` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_rpperm(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::RPPerm))
}

/// Returns `MTreeGet` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_mtree_get(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::MTreeGet))
}

/// Returns `MTreeSet` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_mtree_set(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::MTreeSet))
}

/// Returns `MTreeCwm` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_mtree_cwm(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::MTreeCwm))
}
