use super::super::{parse_decimal_param, parse_element_param, parse_hex_param};
use super::{parse_checked_param, parse_param, Instruction, Node, Vec};
use crate::{validate_operation, AssemblyError, Token};
use vm_core::Felt;

// CONSTANTS
// ================================================================================================

/// The maximum number of constant inputs allowed by `push` operation.
const MAX_CONST_INPUTS: usize = 16;

/// The required length of the hexadecimal representation for an input value when more than one hex
/// input is provided to `push` without period separators.
const HEX_CHUNK_SIZE: usize = 16;

/// The maximum number of elements that can be read from the advice tape in a single `push`
/// operation.
const ADVICE_READ_LIMIT: u8 = 16;

// INSTRUCTION PARSERS
// ================================================================================================

/// Returns `PushConstants` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has invalid values or inappropriate number of
/// values
pub(super) fn parse_push(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "push", 1..MAX_CONST_INPUTS);

    let constants = parse_constants(op)?;
    Ok(Node::Instruction(Instruction::PushConstants(constants)))
}

/// Returns `Sdepth` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_sdepth(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Sdepth))
}

/// Returns `Locaddr` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub(super) fn parse_locaddr(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "locaddr", 1);

    let param = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::Locaddr(Felt::from(param))))
}

/// Returns `Caller` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub fn parse_caller(op: &Token) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(Instruction::Caller))
}

/// Returns `AdvPush` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub(super) fn parse_adv_push(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv_push", 1);

    let param = parse_checked_param(op, 1, 1, ADVICE_READ_LIMIT)?;
    Ok(Node::Instruction(Instruction::AdvPush(param)))
}

/// Returns `AdvLoadW` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub(super) fn parse_adv_loadw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv_loadw", 0);

    Ok(Node::Instruction(Instruction::AdvLoadW))
}

/// Returns `AdvU64Div` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has immediate value different from `u64div`
pub(super) fn parse_adv_inject(op: &Token) -> Result<Node, AssemblyError> {
    match op.parts()[1] {
        "u64div" => {
            validate_operation!(op, "adv.u64div", 0);
            Ok(Node::Instruction(Instruction::AdvU64Div))
        }
        "keyval" => {
            validate_operation!(op, "adv.keyval", 0);
            Ok(Node::Instruction(Instruction::AdvKeyval))
        }
        "mem" => {
            validate_operation!(op, "adv.mem", 2);
            let start_addr = parse_param(op, 2)?;
            let num_words = parse_checked_param(op, 3, 1, u32::MAX - start_addr)?;
            Ok(Node::Instruction(Instruction::AdvMem(
                start_addr, num_words,
            )))
        }
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

/// Returns `MemLoad` node instruction if no immediate vaule is provided or `MemLoadImm` otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub(super) fn parse_mem_load(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_load", 0..1);
    let node = match op.num_parts() {
        2 => {
            let address = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::MemLoadImm(address))
        }
        _ => Node::Instruction(Instruction::MemLoad),
    };

    Ok(node)
}

/// Returns `LocLoad` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub(super) fn parse_loc_load(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_load", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocLoad(Felt::from(index))))
}

/// Returns `MemLoadW` node instruction if no immediate vaule is provided or `MemLoadWImm`
/// otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub(super) fn parse_mem_loadw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_loadw", 0..1);
    let node = match op.num_parts() {
        2 => {
            let address = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::MemLoadWImm(address))
        }
        _ => Node::Instruction(Instruction::MemLoadW),
    };

    Ok(node)
}

/// Returns `LocLoadW` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub(super) fn parse_loc_loadw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_loadw", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocLoadW(Felt::from(index))))
}

/// Returns `MemStore` node instruction if no immediate vaule is provided or `MemStoreImm`
/// otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub(super) fn parse_mem_store(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_store", 0..1);
    let node = match op.num_parts() {
        2 => {
            let address = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::MemStoreImm(address))
        }
        _ => Node::Instruction(Instruction::MemStore),
    };

    Ok(node)
}

/// Returns `LocStore` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub(super) fn parse_loc_store(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_store", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocStore(Felt::from(index))))
}

/// Returns `MemStoreW` node instruction if no immediate vaule is provided or `MemStoreWImm`
/// otherwise
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param or more than one param
pub(super) fn parse_mem_storew(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_storew", 0..1);
    let node = match op.num_parts() {
        2 => {
            let address = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::MemStoreWImm(address))
        }
        _ => Node::Instruction(Instruction::MemStoreW),
    };

    Ok(node)
}

/// Returns `LocStoreW` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has invalid param, no params or more than one
/// param
pub(super) fn parse_loc_storew(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_storew", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocStoreW(Felt::from(index))))
}

/// Returns `MemStream` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub(super) fn parse_mem_stream(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_stream", 0);
    Ok(Node::Instruction(Instruction::MemStream))
}

/// Returns `AdvPipe` node instruction
///
/// # Errors
/// Returns an error if the assembly operation token has any immediate value
pub(super) fn parse_adv_pipe(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv_pipe", 0);
    Ok(Node::Instruction(Instruction::AdvPipe))
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
