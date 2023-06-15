use super::{
    AdviceInjector::*,
    Instruction::AdvInject,
    Node::{self, Instruction},
    ParsingError, Token,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `AdvInject` instruction node with an appropriate internal advice injector variant.
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
        "push_u64div" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushU64div)
        }
        "push_ext2intt" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushExt2intt)
        }
        "push_smtget" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushSmtGet)
        }
        "push_mapval" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(PushMapVal)
        }
        "insert_mem" => {
            if op.num_parts() > 2 {
                return Err(ParsingError::extra_param(op));
            }
            AdvInject(InsertMem)
        }
        _ => return Err(ParsingError::invalid_op(op)),
    };

    Ok(Instruction(injector))
}
