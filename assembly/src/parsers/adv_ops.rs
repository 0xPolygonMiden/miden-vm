use super::{
    Instruction::*,
    Node::{self, Instruction},
    ParsingError, Token,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `AdvU64Div`, `AdvKeyval`, `AdvMem`, `AdvExt2Inv`, `AdvExt2INTT`, or `AdvSmtGet`
/// instruction node.
///
/// # Errors
/// Returns an error if:
/// - Any of the instructions have a wrong number of parameters.
pub fn parse_adv_inject(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "adv");
    if op.num_parts() < 2 {
        return Err(ParsingError::missing_param(op));
    }

    match op.parts()[1] {
        "u64div" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            Ok(Instruction(AdvU64Div))
        }
        "keyval" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            Ok(Instruction(AdvKeyval))
        }
        "mem" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            Ok(Instruction(AdvMem))
        }
        "ext2inv" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            Ok(Instruction(AdvExt2Inv))
        }
        "ext2intt" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            Ok(Instruction(AdvExt2INTT))
        }
        "smtget" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            Ok(Instruction(AdvSmtGet))
        }
        _ => Err(ParsingError::invalid_op(op)),
    }
}
