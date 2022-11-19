use super::super::{parse_decimal_param, parse_element_param, parse_hex_param};
use super::{
    parse_checked_param, parse_param,
    Instruction::*,
    Node::{self, Instruction},
    Vec,
};
use crate::{validate_operation, AssemblyError, Token, ADVICE_READ_LIMIT, MAX_PUSH_INPUTS};
use vm_core::Felt;

// CONSTANTS
// ================================================================================================

/// The required length of the hexadecimal representation for an input value when more than one hex
/// input is provided to `push` without period separators.
const HEX_CHUNK_SIZE: usize = 16;

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `PushConstants` instruction node.
///
/// # Errors
/// Returns an error if the instruction token has invalid values or inappropriate number of
/// values.
pub fn parse_push(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "push", 1..MAX_PUSH_INPUTS);

    let constants = parse_constants(op)?;
    Ok(Instruction(PushConstants(constants)))
}

/// Returns `Locaddr` instruction node.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u16 value.
pub fn parse_locaddr(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Err(AssemblyError::missing_param(op)),
        2 => {
            let index = parse_param::<u16>(op, 1)?;
            Ok(Instruction(Locaddr(index)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `Caller` instruction node.
///
/// # Errors
/// Returns an error if the instruction token is malformed.
pub fn parse_caller(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Instruction(Caller))
}

/// Returns `AdvPush` instruction node.
///
/// # Errors
/// Returns an error if the instruction token does not have exactly one parameter, or if the
/// parameter is smaller than 1 or greater than 16.
pub fn parse_adv_push(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Err(AssemblyError::missing_param(op)),
        2 => {
            let num_vals = parse_checked_param(op, 1, 1, ADVICE_READ_LIMIT)?;
            Ok(Instruction(AdvPush(num_vals)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `AdvU64Div`, `AdvKeyval`, or `AdvMem`  instruction node.
///
/// # Errors
/// Returns an error if:
/// - Any of the instructions have a wrong number of parameters.
/// - adv.mem.a.n has a + n > u32::MAX.
pub fn parse_adv_inject(op: &Token) -> Result<Node, AssemblyError> {
    match op.parts()[1] {
        "u64div" => {
            validate_operation!(op, "adv.u64div", 0);
            Ok(Instruction(AdvU64Div))
        }
        "keyval" => {
            validate_operation!(op, "adv.keyval", 0);
            Ok(Instruction(AdvKeyval))
        }
        "mem" => {
            validate_operation!(op, "adv.mem", 2);
            let start_addr = parse_param(op, 2)?;
            let num_words = parse_checked_param(op, 3, 1, u32::MAX - start_addr)?;
            Ok(Instruction(AdvMem(start_addr, num_words)))
        }
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

/// Returns `MemLoad` instruction node if no immediate value is provided, or `MemLoadImm`
/// instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_mem_load(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Ok(Instruction(MemLoad)),
        2 => {
            let address = parse_param::<u32>(op, 1)?;
            Ok(Instruction(MemLoadImm(address)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `LocLoad` instruction node.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u16 value.
pub fn parse_loc_load(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Err(AssemblyError::missing_param(op)),
        2 => {
            let index = parse_param::<u16>(op, 1)?;
            Ok(Instruction(LocLoad(index)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `MemLoadW` instruction node if no immediate value is provided, or `MemLoadWImm`
/// instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_mem_loadw(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Ok(Instruction(MemLoadW)),
        2 => {
            let address = parse_param::<u32>(op, 1)?;
            Ok(Instruction(MemLoadWImm(address)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `LocLoadW` instruction node.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u16 value.
pub fn parse_loc_loadw(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Err(AssemblyError::missing_param(op)),
        2 => {
            let index = parse_param::<u16>(op, 1)?;
            Ok(Instruction(LocLoadW(index)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `MemStore` instruction node if no immediate value is provided, or `MemStoreImm`
/// instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_mem_store(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Ok(Instruction(MemStore)),
        2 => {
            let address = parse_param::<u32>(op, 1)?;
            Ok(Instruction(MemStoreImm(address)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `LocStore` instruction node.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u16 value.
pub fn parse_loc_store(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Err(AssemblyError::missing_param(op)),
        2 => {
            let index = parse_param::<u16>(op, 1)?;
            Ok(Instruction(LocStore(index)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `MemStoreW` instruction node if no immediate value is provided, or `MemStoreWImm`
/// instruction node otherwise.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u32 value.
pub fn parse_mem_storew(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Ok(Instruction(MemStoreW)),
        2 => {
            let address = parse_param::<u32>(op, 1)?;
            Ok(Instruction(MemStoreWImm(address)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

/// Returns `LocStoreW` instruction node.
///
/// # Errors
/// Returns an error if the instruction token contains a wrong number of parameters, or if
/// the provided parameter is not a u16 value.
pub fn parse_loc_storew(op: &Token) -> Result<Node, AssemblyError> {
    match op.num_parts() {
        1 => Err(AssemblyError::missing_param(op)),
        2 => {
            let index = parse_param::<u16>(op, 1)?;
            Ok(Instruction(LocStoreW(index)))
        }
        _ => Err(AssemblyError::extra_param(op)),
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn parse_constants(op: &Token) -> Result<Vec<Felt>, AssemblyError> {
    let mut constants = Vec::new();
    let param_idx = 1;
    let param_count = op.num_parts() - param_idx;

    // for multiple input parameters, parse & push each one onto the stack in order, then return
    if param_count > 1 {
        for param_idx in param_idx..=param_count {
            let value = parse_element_param(op, param_idx)?;
            constants.push(value);
        }
        return Ok(constants);
    }

    // for a single input, there could be one value or there could be a series of many hexadecimal
    // values without separators
    let param_str = op.parts()[param_idx];
    if let Some(param_str) = param_str.strip_prefix("0x") {
        // parse 1 or more hexadecimal values
        let values = parse_hex_params(op, param_idx, param_str)?;
        // push each value onto the stack in order
        for &value in values.iter() {
            constants.push(value);
        }
    } else {
        // parse 1 decimal value and push it onto the stack
        let value = parse_decimal_param(op, param_idx, param_str)?;
        constants.push(value);
    }

    Ok(constants)
}

fn parse_hex_params(
    op: &Token,
    param_idx: usize,
    param_str: &str,
) -> Result<Vec<Felt>, AssemblyError> {
    // handle error cases where the hex string is poorly formed
    let is_single_element = if param_str.len() <= HEX_CHUNK_SIZE {
        if param_str.len() % 2 != 0 {
            // parameter string is not a valid hex representation
            return Err(AssemblyError::invalid_param(op, param_idx));
        }
        true
    } else {
        if param_str.len() % HEX_CHUNK_SIZE != 0 {
            // hex string doesn't contain a valid number of bytes
            return Err(AssemblyError::invalid_param(op, param_idx));
        }
        false
    };

    // parse the hex string into one or more valid field elements
    if is_single_element {
        // parse a single element in hex representation
        let parsed_param = parse_hex_param(op, param_idx, param_str)?;
        Ok(vec![parsed_param])
    } else {
        // iterate over the multi-value hex string and parse each 8-byte chunk into a valid element
        (0..param_str.len())
            .step_by(HEX_CHUNK_SIZE)
            .map(|i| parse_hex_param(op, param_idx, &param_str[i..i + HEX_CHUNK_SIZE]))
            .collect()
    }
}
