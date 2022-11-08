use crate::validate_operation;

use super::{
    super::parse_bit_len_param, super::parse_element_param, AssemblyError, Instruction, Node, Token,
};

pub fn parse_exp(op: &Token) -> Result<Node, AssemblyError> {
    let instruction = match op.num_parts() {
        1 => Instruction::Exp,
        2 => {
            let param_value = op.parts()[1];

            if param_value.strip_prefix('u').is_some() {
                // parse the bits length of the exponent from the immediate value.
                let bits_len = parse_bit_len_param(op, 1)?;

                // the specified bits length can not be more than 64 bits.
                if bits_len > 64 {
                    return Err(AssemblyError::invalid_param_with_reason(
                        op,
                        1,
                        format!("parameter can at max be a u64 but found u{}", bits_len).as_str(),
                    ));
                }

                Instruction::ExpBitLength(bits_len as u32)
            } else {
                // parse immediate value.
                let imm = parse_element_param(op, 1)?;
                Instruction::ExpImm(imm)
            }
        }
        _ => return Err(AssemblyError::extra_param(op)),
    };

    Ok(Node::Instruction(instruction))
}

pub fn parse_eq(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "eq", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::EqImm(value))
        }
        _ => Node::Instruction(Instruction::Eq),
    };

    Ok(node)
}

pub fn parse_neq(op: &Token) -> Result<Node, AssemblyError> {
    validate_operation!(op, "neq", 0..1);

    let node = match op.num_parts() {
        2 => {
            let value = parse_element_param(op, 1)?;
            Node::Instruction(Instruction::NeqImm(value))
        }
        _ => Node::Instruction(Instruction::Neq),
    };

    Ok(node)
}
