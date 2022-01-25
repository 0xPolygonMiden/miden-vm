use super::{AssemblyError, Token, TokenStream};
pub use blocks::parse_code_blocks;
use vm_core::{
    program::blocks::CodeBlock, Felt as BaseElement, FieldElement, Operation, StarkField,
};

mod blocks;
mod crypto_ops;
mod field_ops;
mod io_ops;
mod stack_ops;
mod u32_ops;

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

        // ----- u32 operations -------------------------------------------------------------------
        "u32test" => u32_ops::parse_u32test(span_ops, op),
        "u32testw" => u32_ops::parse_u32testw(span_ops, op),
        "u32assert" => u32_ops::parse_u32assert(span_ops, op),
        "u32assertw" => u32_ops::parse_u32assertw(span_ops, op),
        "u32cast" => u32_ops::parse_u32cast(span_ops, op),
        "u32split" => u32_ops::parse_u32split(span_ops, op),

        "u32add" => u32_ops::parse_u32add(span_ops, op),
        "u32addc" => u32_ops::parse_u32addc(span_ops, op),
        "u32sub" => u32_ops::parse_u32sub(span_ops, op),
        "u32mul" => u32_ops::parse_u32mul(span_ops, op),
        "u32madd" => u32_ops::parse_u32madd(span_ops, op),
        "u32div" => u32_ops::parse_u32div(span_ops, op),
        "u32mod" => u32_ops::parse_u32mod(span_ops, op),

        "u32and" => u32_ops::parse_u32and(span_ops, op),
        "u32or" => u32_ops::parse_u32or(span_ops, op),
        "u32xor" => u32_ops::parse_u32xor(span_ops, op),
        "u32not" => u32_ops::parse_u32not(span_ops, op),
        "u32shr" => u32_ops::parse_u32shr(span_ops, op),
        "u32shl" => u32_ops::parse_u32shl(span_ops, op),
        "u32rotr" => u32_ops::parse_u32rotr(span_ops, op),
        "u32rotl" => u32_ops::parse_u32rotl(span_ops, op),

        "u32eq" => u32_ops::parse_u32eq(span_ops, op),
        "u32neq" => u32_ops::parse_u32neq(span_ops, op),
        "u32lt" => u32_ops::parse_u32lt(span_ops, op),
        "u32lte" => u32_ops::parse_u32lte(span_ops, op),
        "u32gt" => u32_ops::parse_u32gt(span_ops, op),
        "u32gte" => u32_ops::parse_u32gte(span_ops, op),
        "u32min" => u32_ops::parse_u32min(span_ops, op),
        "u32max" => u32_ops::parse_u32max(span_ops, op),

        // ----- stack manipulation ---------------------------------------------------------------
        "drop" => stack_ops::parse_drop(span_ops, op),
        "dropw" => stack_ops::parse_dropw(span_ops, op),
        "padw" => stack_ops::parse_padw(span_ops, op),
        "dup" => stack_ops::parse_dup(span_ops, op),
        "dupw" => stack_ops::parse_dupw(span_ops, op),
        "swap" => stack_ops::parse_swap(span_ops, op),
        "swapw" => stack_ops::parse_swapw(span_ops, op),
        "movup" => stack_ops::parse_movup(span_ops, op),
        "movupw" => stack_ops::parse_movupw(span_ops, op),
        "movdn" => stack_ops::parse_movdn(span_ops, op),
        "movdnw" => stack_ops::parse_movdnw(span_ops, op),

        "cswap" => stack_ops::parse_cswap(span_ops, op),
        "cswapw" => stack_ops::parse_cswapw(span_ops, op),
        "cdrop" => stack_ops::parse_cdrop(span_ops, op),
        "cdropw" => stack_ops::parse_cdropw(span_ops, op),

        // ----- input / output operations --------------------------------------------------------
        "push" => io_ops::parse_push(span_ops, op),
        "pushw" => io_ops::parse_pushw(span_ops, op),
        "env" => io_ops::parse_env(span_ops, op),
        "adv" => io_ops::parse_adv(span_ops, op),
        "mem" => io_ops::parse_mem(span_ops, op),

        // ----- cryptographic operations ---------------------------------------------------------
        "rphash" => crypto_ops::parse_rphash(span_ops, op),
        "rpperm" => crypto_ops::parse_rpperm(span_ops, op),

        "mtree" => crypto_ops::parse_mtree(span_ops, op),

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

/// This is a helper function that parses the parameter at the specified op index as an integer and
/// ensures that it falls within the bounds specified by the caller.
///
/// # Errors
/// Returns an invalid param AssemblyError if:
/// - the parsing attempt fails.
/// - the parameter is outside the specified lower and upper bounds.
fn parse_int_param(
    op: &Token,
    param_idx: usize,
    lower_bound: u32,
    upper_bound: u32,
) -> Result<u32, AssemblyError> {
    let param_value = op.parts()[param_idx];

    // attempt to parse the parameter value as an integer
    let result = match param_value.parse::<u32>() {
        Ok(i) => i,
        Err(_) => return Err(AssemblyError::invalid_param(op, param_idx)),
    };

    // check that the parameter is within the specified bounds
    if result < lower_bound || result > upper_bound {
        return Err(AssemblyError::invalid_param_with_reason(
            op,
            param_idx,
            format!(
                "parameter value must be greater than {} and less than than {}",
                lower_bound, upper_bound
            )
            .as_str(),
        ));
    }

    Ok(result)
}

/// This is a helper function that validates the length of an assembly instruction and returns
/// an error if the instruction is too short or too long.
///
/// `instr_parts` expects the number of non-parameter parts in the instruction, e.g. 2 for the
/// "mem.pop" instruction. `min_params` and `max_params` expect the minimum and maximum number of
/// parameters accepted by the operation respectively.
///
/// # Errors
///
/// This function will return an AssemblyError if the instruction part of the operation is
/// too short or if too many or too few parameters are provided.
fn validate_op_len(
    op: &Token,
    instr_parts: usize,
    min_params: usize,
    max_params: usize,
) -> Result<(), AssemblyError> {
    let num_parts = op.num_parts();

    // token has too few parts to contain the full instruction
    if num_parts < instr_parts {
        return Err(AssemblyError::invalid_op(op));
    }
    // token has too few parts to contain the required parameters
    if num_parts < instr_parts + min_params {
        return Err(AssemblyError::missing_param(op));
    }
    // token has more than the maximum number of parts
    if num_parts > instr_parts + max_params {
        return Err(AssemblyError::extra_param(op));
    }

    Ok(())
}

/// This is a helper function that appends a PUSH operation to the span block which puts the
/// provided value parameter onto the stack.
///
/// When the value is 0, PUSH operation is replaced with PAD. When the value is 1, PUSH operation
/// is replaced with PAD INCR because in most cases this will be more efficient than doing a PUSH.
fn push_value(span_ops: &mut Vec<Operation>, value: BaseElement) {
    if value == BaseElement::ZERO {
        span_ops.push(Operation::Pad);
    } else if value == BaseElement::ONE {
        span_ops.push(Operation::Pad);
        span_ops.push(Operation::Incr);
    } else {
        span_ops.push(Operation::Push(value));
    }
}
