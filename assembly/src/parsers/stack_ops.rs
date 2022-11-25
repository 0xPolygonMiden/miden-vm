use super::{
    Instruction::*,
    Node::{self, Instruction},
    ParsingError, Token,
};

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `Dup0` instruction node if no immediate vaule is provided or one of the
/// `Dup1` — `Dup15` instruction nodes according to the immediate value.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 15.
pub fn parse_dup(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "dup");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Dup0)),
        2 => match op.parts()[1] {
            "0" => Ok(Instruction(Dup0)),
            "1" => Ok(Instruction(Dup1)),
            "2" => Ok(Instruction(Dup2)),
            "3" => Ok(Instruction(Dup3)),
            "4" => Ok(Instruction(Dup4)),
            "5" => Ok(Instruction(Dup5)),
            "6" => Ok(Instruction(Dup6)),
            "7" => Ok(Instruction(Dup7)),
            "8" => Ok(Instruction(Dup8)),
            "9" => Ok(Instruction(Dup9)),
            "10" => Ok(Instruction(Dup10)),
            "11" => Ok(Instruction(Dup11)),
            "12" => Ok(Instruction(Dup12)),
            "13" => Ok(Instruction(Dup13)),
            "14" => Ok(Instruction(Dup14)),
            "15" => Ok(Instruction(Dup15)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `DupW0` instruction node if no immediate vaule is provided or one of the
/// `DupW1` — `DupW3` instruction nodes according to the immediate value.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is greater than 3.
pub fn parse_dupw(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "dupw");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(DupW0)),
        2 => match op.parts()[1] {
            "0" => Ok(Instruction(DupW0)),
            "1" => Ok(Instruction(DupW1)),
            "2" => Ok(Instruction(DupW2)),
            "3" => Ok(Instruction(DupW3)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `Swap` node instruction if no immediate vaule is provided or one of the
/// `Swap` — `Swap15` instructions according to the immediate value.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is smaller than 1 or greater than 15.
pub fn parse_swap(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "swap");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(Swap1)),
        2 => match op.parts()[1] {
            "1" => Ok(Instruction(Swap1)),
            "2" => Ok(Instruction(Swap2)),
            "3" => Ok(Instruction(Swap3)),
            "4" => Ok(Instruction(Swap4)),
            "5" => Ok(Instruction(Swap5)),
            "6" => Ok(Instruction(Swap6)),
            "7" => Ok(Instruction(Swap7)),
            "8" => Ok(Instruction(Swap8)),
            "9" => Ok(Instruction(Swap9)),
            "10" => Ok(Instruction(Swap10)),
            "11" => Ok(Instruction(Swap11)),
            "12" => Ok(Instruction(Swap12)),
            "13" => Ok(Instruction(Swap13)),
            "14" => Ok(Instruction(Swap14)),
            "15" => Ok(Instruction(Swap15)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `SwapW` instruction node if no immediate vaule is provided, or one of the
/// `SwapW` — `SwapW3` instruction nodes according to the immediate value.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is smaller than 1 or greater than 3.
pub fn parse_swapw(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "swapw");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction(SwapW1)),
        2 => match op.parts()[1] {
            "1" => Ok(Instruction(SwapW1)),
            "2" => Ok(Instruction(SwapW2)),
            "3" => Ok(Instruction(SwapW3)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of the `MovUp2` — `MovUp15` instruction nodes according to the immediate value.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is smaller than 2 or greater than 15.
pub fn parse_movup(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "movup");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => Ok(Instruction(MovUp2)),
            "3" => Ok(Instruction(MovUp3)),
            "4" => Ok(Instruction(MovUp4)),
            "5" => Ok(Instruction(MovUp5)),
            "6" => Ok(Instruction(MovUp6)),
            "7" => Ok(Instruction(MovUp7)),
            "8" => Ok(Instruction(MovUp8)),
            "9" => Ok(Instruction(MovUp9)),
            "10" => Ok(Instruction(MovUp10)),
            "11" => Ok(Instruction(MovUp11)),
            "12" => Ok(Instruction(MovUp12)),
            "13" => Ok(Instruction(MovUp13)),
            "14" => Ok(Instruction(MovUp14)),
            "15" => Ok(Instruction(MovUp15)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns one of the `MovDn2` — `MovDn15` instruction nodes according to the immediate value.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is smaller than 2 or greater than 15.
pub fn parse_movdn(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "movdn");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => Ok(Instruction(MovDn2)),
            "3" => Ok(Instruction(MovDn3)),
            "4" => Ok(Instruction(MovDn4)),
            "5" => Ok(Instruction(MovDn5)),
            "6" => Ok(Instruction(MovDn6)),
            "7" => Ok(Instruction(MovDn7)),
            "8" => Ok(Instruction(MovDn8)),
            "9" => Ok(Instruction(MovDn9)),
            "10" => Ok(Instruction(MovDn10)),
            "11" => Ok(Instruction(MovDn11)),
            "12" => Ok(Instruction(MovDn12)),
            "13" => Ok(Instruction(MovDn13)),
            "14" => Ok(Instruction(MovDn14)),
            "15" => Ok(Instruction(MovDn15)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `MovUpW2` or `MovUpW3` instruction node according to the immediate value.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not either 2 or 3.
pub fn parse_movupw(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "movupw");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => Ok(Instruction(MovUpW2)),
            "3" => Ok(Instruction(MovUpW3)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}

/// Returns `MovDnW2` or `MovDnW3` instruction node according to the immediate value.
///
/// # Errors
/// Returns an error if the instruction token contains wrong number of parameters, or if the
/// provided parameter is not either 2 or 3.
pub fn parse_movdnw(op: &Token) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], "movdnw");
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(op)),
        2 => match op.parts()[1] {
            "2" => Ok(Instruction(MovDnW2)),
            "3" => Ok(Instruction(MovDnW3)),
            _ => Err(ParsingError::invalid_param(op, 1)),
        },
        _ => Err(ParsingError::extra_param(op)),
    }
}
