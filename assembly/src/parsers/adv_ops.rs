use super::{
    parse_checked_param, parse_param,
    Instruction::*,
    Node::{self, Instruction},
    ParsingError, Token,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `AdvU64Div`, `AdvKeyval`, or `AdvMem`  instruction node.
///
/// # Errors
/// Returns an error if:
/// - Any of the instructions have a wrong number of parameters.
/// - adv.mem.a.n has a + n > u32::MAX.
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
        "mem" => match op.num_parts() {
            0 | 1 => unreachable!(),
            2 | 3 => Err(ParsingError::missing_param(op)),
            4 => {
                let start_addr = parse_param(op, 2)?;
                let num_words = parse_checked_param(op, 3, 1..=(u32::MAX - start_addr))?;
                Ok(Instruction(AdvMem(start_addr, num_words)))
            }
            _ => Err(ParsingError::extra_param(op)),
        },
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
        _ => Err(ParsingError::invalid_op(op)),
    }
}
