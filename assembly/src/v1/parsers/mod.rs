use super::{AssemblyError, Token, TokenStream};
pub use blocks::parse_code_blocks;
use vm_core::{
    v1::{program::Operation, BaseElement, StarkField},
    FieldElement,
};

mod blocks;
mod field_ops;
mod io_ops;

// OP PARSER
// ================================================================================================

/// Transforms an assembly instruction into a sequence of one or more VM instructions.
fn parse_op_token(op: &Token, span_ops: &mut Vec<Operation>) -> Result<(), AssemblyError> {
    // based on the instruction, invoke the correct parser for the operation
    match op.parts()[0] {
        // ----- field operations -----------------------------------------------------------------
        "assert" => field_ops::parse_assert(span_ops, op),

        "add" => field_ops::parse_add(span_ops, op),
        "sub" => field_ops::parse_sub(span_ops, op),
        "mul" => field_ops::parse_mul(span_ops, op),
        "div" => field_ops::parse_div(span_ops, op),
        "neg" => field_ops::parse_neg(span_ops, op),
        "inv" => field_ops::parse_inv(span_ops, op),

        "not" => field_ops::parse_not(span_ops, op),
        "and" => field_ops::parse_and(span_ops, op),
        "or" => field_ops::parse_or(span_ops, op),
        "xor" => field_ops::parse_xor(span_ops, op),

        "eq" => field_ops::parse_eq(span_ops, op),
        "neq" => field_ops::parse_neq(span_ops, op),
        "lt" => field_ops::parse_lt(span_ops, op),
        "lte" => field_ops::parse_lte(span_ops, op),
        "gt" => field_ops::parse_gt(span_ops, op),
        "gte" => field_ops::parse_gte(span_ops, op),
        "eqw" => field_ops::parse_eqw(span_ops, op),

        // ----- input / output operations --------------------------------------------------------
        "push" => io_ops::parse_push(span_ops, op),

        // ----- catch all ------------------------------------------------------------------------
        _ => return Err(AssemblyError::invalid_op(op)),
    }?;

    Ok(())
}

// HELPER FUNCTIONS
// ================================================================================================

fn parse_element_param(op: &Token, param_idx: usize) -> Result<BaseElement, AssemblyError> {
    // make sure that the parameter value is available
    if op.num_parts() <= param_idx {
        return Err(AssemblyError::missing_param(op));
    }
    let param_value = op.parts()[param_idx];

    let result = if let Some(param_value) = param_value.strip_prefix("0x") {
        // parse hexadecimal number
        match u64::from_str_radix(param_value, 16) {
            Ok(i) => i,
            Err(_) => return Err(AssemblyError::invalid_param(op, param_idx)),
        }
    } else {
        // parse decimal number
        match param_value.parse::<u64>() {
            Ok(i) => i,
            Err(_) => return Err(AssemblyError::invalid_param(op, param_idx)),
        }
    };

    // make sure the value is a valid field element
    if result >= BaseElement::MODULUS {
        return Err(AssemblyError::invalid_param_with_reason(
            op,
            param_idx,
            format!(
                "parameter value must be smaller than {}",
                BaseElement::MODULUS
            )
            .as_str(),
        ));
    }

    Ok(BaseElement::new(result))
}
