use super::super::{parse_decimal_param, parse_element_param, parse_hex_param};
use super::{parse_param, Instruction, Node, Vec};
use crate::{validate_operation, AssemblyError, Token};
use vm_core::Felt;

// CONSTANTS
// ================================================================================================

/// The maximum number of constant inputs allowed by `push` operation.
const MAX_CONST_INPUTS: usize = 16;

/// The required length of the hexadecimal representation for an input value when more than one hex
/// input is provided to `push` without period separators.
const HEX_CHUNK_SIZE: usize = 16;

// INSTRUCTION PARSERS
// ================================================================================================

pub(super) fn parse_push(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "push", 1..MAX_CONST_INPUTS);

    let constants = parse_constants(op)?;
    Ok(Node::Instruction(Instruction::PushConstants(constants)))
}

pub(super) fn parse_locaddr(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "locaddr", 1);

    let param = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::Locaddr(Felt::from(param))))
}

pub(super) fn parse_adv_push(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv_push", 1);

    let param = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::Locaddr(Felt::from(param))))
}

pub(super) fn parse_adv_loadw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv_loadw", 1);

    let param = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::Locaddr(Felt::from(param))))
}

pub(super) fn parse_adv_inject(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "adv.u64div");

    let node = match op.parts()[1] {
        "u64div" => Node::Instruction(Instruction::AdvU64Div),
        _ => return Err(AssemblyError::invalid_op(op)),
    };

    Ok(node)
}

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

pub(super) fn parse_loc_load(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_load", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocLoad(Felt::from(index))))
}

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

pub(super) fn parse_loc_loadw(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_loadw", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocLoadW(Felt::from(index))))
}

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

pub(super) fn parse_loc_store(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_store", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocStore(Felt::from(index))))
}

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

pub(super) fn parse_loc_storew(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "loc_storew", 1);
    let index = parse_param::<u32>(op, 1)?;
    Ok(Node::Instruction(Instruction::LocStoreW(Felt::from(index))))
}

pub(super) fn parse_mem_stream(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "mem_stream", 0);
    Ok(Node::Instruction(Instruction::MemStream))
}

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
