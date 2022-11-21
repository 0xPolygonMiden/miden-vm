use super::{AssemblyError, CodeBlock, Token, TokenStream};
pub use blocks::combine_blocks;
use vm_core::{utils::collections::Vec, Felt, Operation, StarkField};

mod blocks;

mod ast;
pub use ast::{parse_module, ModuleAst, NamedModuleAst, ProcedureAst};
pub(crate) use ast::{parse_program, Instruction, Node, ProgramAst};

// HELPER FUNCTIONS
// ================================================================================================

/// Parses a single parameter into a valid field element.
fn parse_element_param(op: &Token, param_idx: usize) -> Result<Felt, AssemblyError> {
    // make sure that the parameter value is available
    if op.num_parts() <= param_idx {
        return Err(AssemblyError::missing_param(op));
    }
    let param_value = op.parts()[param_idx];

    if let Some(param_value) = param_value.strip_prefix("0x") {
        // parse hexadecimal number
        parse_hex_param(op, param_idx, param_value)
    } else {
        // parse decimal number
        parse_decimal_param(op, param_idx, param_value)
    }
}

/// Parses a decimal parameter value into valid a field element.
fn parse_decimal_param(
    op: &Token,
    param_idx: usize,
    param_str: &str,
) -> Result<Felt, AssemblyError> {
    match param_str.parse::<u64>() {
        Ok(value) => get_valid_felt(op, param_idx, value),
        Err(_) => Err(AssemblyError::invalid_param(op, param_idx)),
    }
}

/// Parses a hexadecimal parameter value into a valid field element.
fn parse_hex_param(op: &Token, param_idx: usize, param_str: &str) -> Result<Felt, AssemblyError> {
    match u64::from_str_radix(param_str, 16) {
        Ok(value) => get_valid_felt(op, param_idx, value),
        Err(_) => Err(AssemblyError::invalid_param(op, param_idx)),
    }
}

/// Checks that the u64 parameter value is a valid field element value and returns it as a field
/// element.
fn get_valid_felt(op: &Token, param_idx: usize, param: u64) -> Result<Felt, AssemblyError> {
    if param >= Felt::MODULUS {
        return Err(AssemblyError::invalid_param_with_reason(
            op,
            param_idx,
            format!("parameter value must be smaller than {}", Felt::MODULUS).as_str(),
        ));
    }

    Ok(Felt::new(param))
}

/// Validates an op Token against a provided instruction string and/or an expected number of
/// parameter inputs and returns an appropriate AssemblyError if the operation Token is invalid.
///
/// * To fully validate an operation, pass all of the following:
/// - the parsed operation Token
/// - a string describing a valid instruction, with variants separated by '|' and parameters
///   excluded.
/// - an integer or range for the number of allowed parameters
/// This will attempt to fully validate the operation, so a full-length instruction must be
/// described. For example, `popw.mem` accepts 0 or 1 inputs and can be validated by:
/// ```validate_operation!(op_token, "popw.mem", 0..1)```
///
/// * To validate only the operation parameters, specify @only_params before passing the same inputs
/// used for full validation (above). This will skip validating each part of the instruction.
/// For example, to validate only the parameters of `popw.mem` use:
/// ```validate_operation!(@only_params op_token, "popw.mem", 0..1)```
///
/// * To validate only the instruction portion of the operation, exclude the specification for the
/// number of parameters. This will only validate up to the number of parts in the provided
/// instruction string. For example, `pop.local` and `pop.mem` are the two valid instruction
/// variants for `pop`, so the first 2 parts of `pop` can be validated by:
/// ```validate_operation!(op_token, "pop.local|mem")```
/// or the first part can be validated by:
/// ```validate_operation!(op_token, "pop")```
#[macro_export]
macro_rules! validate_operation {
    // validate that the number of parameters is within the allowed range
    (@only_params $token:expr, $instr:literal, $min_params:literal..$max_params:expr ) => {
        let num_parts = $token.num_parts();
        let num_instr_parts = $instr.split(".").count();

        // token has too few parts to contain the required parameters
        if num_parts < num_instr_parts + $min_params {
            return Err(AssemblyError::missing_param($token));
        }
        // token has more than the maximum number of parts
        if num_parts > num_instr_parts + $max_params {
            return Err(AssemblyError::extra_param($token));
        }
    };
    // validate the exact number of parameters
    (@only_params $token:expr, $instr:literal, $num_params:literal) => {
        validate_operation!(@only_params $token, $instr, $num_params..$num_params);
    };

    // validate the instruction string and an optional parameter range
    ($token:expr, $instr:literal $(, $min_params:literal..$max_params:expr)?) => {
        // split the expected instruction into a vector of parts
        let instr_parts: Vec<Vec<&str>> = $instr
            .split(".")
            .map(|part| part.split("|").collect())
            .collect();

        let num_parts = $token.num_parts();
        let num_instr_parts = instr_parts.len();

        // token has too few parts to contain the full instruction
        if num_parts < num_instr_parts {
            return Err(AssemblyError::invalid_op($token));
        }

        // compare the parts to make sure they match
        for (part_variants, token_part) in instr_parts.iter().zip($token.parts()) {
            if !part_variants.contains(token_part) {
                return Err(AssemblyError::unexpected_token($token, $instr));
            }
        }

        $(
            // validate the parameter range, if provided
            validate_operation!(@only_params $token, $instr, $min_params..$max_params);
        )?
    };
    // validate the instruction string and an exact number of parameters
    ($token:expr, $instr:literal, $num_params:literal) => {
        validate_operation!($token, $instr, $num_params..$num_params);
    };
}
