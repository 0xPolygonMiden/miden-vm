use super::{
    AdviceInjector::*,
    Instruction::AdvInject,
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

    let injector = match op.parts()[1] {
        "u64div" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushU64div)
        }
        "keyval" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushMapVal)
        }
        "mem" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(InsertMem)
        }
        "ext2inv" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushExt2inv)
        }
        "ext2intt" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushExt2intt)
        }
        "smtget" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushSmtGet)
        }
        _ => return Err(ParsingError::invalid_op(op)),
    };

    Ok(Instruction(injector))
}
