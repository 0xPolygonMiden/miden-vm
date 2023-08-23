use super::{
    parse_checked_param, parse_param, parse_param_with_constant_lookup,
    AdviceInjectorNode::*,
    Felt,
    Instruction::AdvInject,
    LocalConstMap,
    Node::{self, Instruction},
    ParsingError, Token, MAX_STACK_WORD_OFFSET,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `AdvInject` instruction node with an appropriate internal advice injector variant.
///
/// # Errors
/// Returns an error if parsing of the internal advice injector variant fails due to wrong number
/// of parameters or invalid parameter values.
pub fn parse_adv_inject(op: &Token, constants: &LocalConstMap) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "adv");
    if op.num_parts() < 2 {
        return Err(ParsingError::missing_param(op));
    }

    let injector = match op.parts()[1] {
        "push_u64div" => match op.num_parts() {
            2 => AdvInject(PushU64div),
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_ext2intt" => match op.num_parts() {
            2 => AdvInject(PushExt2intt),
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_smtget" => match op.num_parts() {
            2 => AdvInject(PushSmtGet),
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_smtinsert" => match op.num_parts() {
            2 => AdvInject(PushSmtInsert),
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_mapval" => match op.num_parts() {
            2 => AdvInject(PushMapVal),
            3 => {
                let offset = parse_checked_param::<u8, _>(op, 2, 0..=MAX_STACK_WORD_OFFSET)?;
                if offset == 0 {
                    AdvInject(PushMapVal)
                } else {
                    AdvInject(PushMapValImm { offset })
                }
            }
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_mapvaln" => match op.num_parts() {
            2 => AdvInject(PushMapValN),
            3 => {
                let offset = parse_checked_param::<u8, _>(op, 2, 0..=MAX_STACK_WORD_OFFSET)?;
                if offset == 0 {
                    AdvInject(PushMapValN)
                } else {
                    AdvInject(PushMapValNImm { offset })
                }
            }
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_mapval_const" => match op.num_parts() {
            // if only one parameter provided, assume the key to be [param, 0, 0, 0]
            3 => {
                let key = [
                    Felt::from(parse_param_with_constant_lookup::<u64>(op, 2, constants)?),
                    Felt::from(0u64),
                    Felt::from(0u64),
                    Felt::from(0u64),
                ];
                AdvInject(PushMapValC { key })
            }
            _ if op.num_parts() < 6 => return Err(ParsingError::missing_param(op)),
            6 => {
                let key = [
                    Felt::from(parse_param::<u64>(op, 2)?),
                    Felt::from(parse_param::<u64>(op, 3)?),
                    Felt::from(parse_param::<u64>(op, 4)?),
                    Felt::from(parse_param::<u64>(op, 5)?),
                ];
                AdvInject(PushMapValC { key })
            }
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_mapvaln_const" => match op.num_parts() {
            // if only one parameter provided, assume the key to be [param, 0, 0, 0]
            3 => {
                let key = [
                    Felt::from(parse_param_with_constant_lookup::<u64>(op, 2, constants)?),
                    Felt::from(0u64),
                    Felt::from(0u64),
                    Felt::from(0u64),
                ];
                AdvInject(PushMapValNC { key })
            }
            _ if op.num_parts() < 6 => return Err(ParsingError::missing_param(op)),
            6 => {
                let key = [
                    Felt::from(parse_param::<u64>(op, 2)?),
                    Felt::from(parse_param::<u64>(op, 3)?),
                    Felt::from(parse_param::<u64>(op, 4)?),
                    Felt::from(parse_param::<u64>(op, 5)?),
                ];
                AdvInject(PushMapValNC { key })
            }
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_mapval_mem" => match op.num_parts() {
            2 => return Err(ParsingError::missing_param(op)),
            3 => {
                let addr = Felt::from(parse_param_with_constant_lookup::<u64>(op, 2, constants)?);
                AdvInject(PushMapValM { addr })
            }
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_mapvaln_mem" => match op.num_parts() {
            2 => return Err(ParsingError::missing_param(op)),
            3 => {
                let addr = Felt::from(parse_param_with_constant_lookup::<u64>(op, 2, constants)?);
                AdvInject(PushMapValNM { addr })
            }
            _ => return Err(ParsingError::extra_param(op)),
        },
        "push_mtnode" => match op.num_parts() {
            2 => AdvInject(PushMtNode),
            _ => return Err(ParsingError::extra_param(op)),
        },
        "insert_mem" => match op.num_parts() {
            2 => AdvInject(InsertMem),
            _ => return Err(ParsingError::extra_param(op)),
        },
        "insert_hdword" => match op.num_parts() {
            2 => AdvInject(InsertHdword),
            3 => {
                let domain = parse_checked_param::<u8, _>(op, 2, 0..=u8::MAX)?;
                if domain == 0 {
                    AdvInject(InsertHdword)
                } else {
                    AdvInject(InsertHdwordImm { domain })
                }
            }
            _ => return Err(ParsingError::extra_param(op)),
        },
        "insert_hperm" => match op.num_parts() {
            2 => AdvInject(InsertHperm),
            _ => return Err(ParsingError::extra_param(op)),
        },
        _ => return Err(ParsingError::invalid_op(op)),
    };

    Ok(Instruction(injector))
}
