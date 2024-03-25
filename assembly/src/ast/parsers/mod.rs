use super::{
    bound_into_included_u64, AdviceInjectorNode, CodeBody, Deserializable, Felt, Instruction,
    InvocationTarget, LabelError, LibraryPath, LocalConstMap, LocalProcMap, ModuleImports, Node,
    ParsingError, ProcedureAst, ProcedureId, ProcedureName, ReExportedProcMap, RpoDigest,
    SliceReader, StarkField, Token, TokenStream, MAX_BODY_LEN, MAX_DOCS_LEN, MAX_LABEL_LEN,
    MAX_STACK_WORD_OFFSET,
};
use crate::HEX_CHUNK_SIZE;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::{fmt::Display, ops::RangeBounds};

mod adv_ops;
mod debug;
mod events;
mod field_ops;
mod io_ops;
mod stack_ops;
mod sys_ops;
mod u32_ops;

mod constants;
use constants::calculate_const_value;

mod context;
pub use context::ParserContext;

mod labels;
pub use labels::{
    decode_hex_rpo_digest_label, CONSTANT_LABEL_PARSER, NAMESPACE_LABEL_PARSER,
    PROCEDURE_LABEL_PARSER,
};

/// Helper enum for endianness determination in the parsing functions.
#[derive(Debug)]
pub enum Endianness {
    Little,
    Big,
}

// PARSERS FUNCTIONS
// ================================================================================================

/// Parses all `const` statements into a map which maps a const name to a value
pub fn parse_constants(tokens: &mut TokenStream) -> Result<LocalConstMap, ParsingError> {
    // instantiate new constant map for this module
    let mut constants = LocalConstMap::new();

    // iterate over tokens until we find a const declaration
    while let Some(token) = tokens.read() {
        match token.parts()[0] {
            Token::CONST => {
                let (name, value) = parse_constant(token, &constants)?;

                if constants.contains_key(&name) {
                    return Err(ParsingError::duplicate_const_name(token, &name));
                }

                constants.insert(name, value);
                tokens.advance();
            }
            _ => break,
        }
    }

    Ok(constants)
}

/// Parses a constant token and returns a (constant_name, constant_value) tuple
fn parse_constant(token: &Token, constants: &LocalConstMap) -> Result<(String, u64), ParsingError> {
    match token.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(token, "const.<name>=<value>")),
        2 => {
            let const_declaration: Vec<&str> = token.parts()[1].split('=').collect();
            match const_declaration.len() {
                0 => unreachable!(),
                1 => Err(ParsingError::missing_param(token, "const.<name>=<value>")),
                2 => {
                    let name = CONSTANT_LABEL_PARSER
                        .parse_label(const_declaration[0])
                        .map_err(|err| ParsingError::invalid_const_name(token, err))?;
                    let value = parse_const_value(token, const_declaration[1], constants)?;
                    Ok((name.to_string(), value))
                }
                _ => Err(ParsingError::extra_param(token)),
            }
        }
        _ => Err(ParsingError::extra_param(token)),
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// If `constant_name` is a valid constant name, returns the value of this constant or an error if
/// the constant does not exist in set of available constants.
///
/// If `constant_name` is not a valid constant name, returns None.
fn try_get_constant_value(
    op: &Token,
    const_name: &str,
    constants: &LocalConstMap,
) -> Result<Option<u64>, ParsingError> {
    match CONSTANT_LABEL_PARSER.parse_label(const_name) {
        Ok(_) => constants
            .get(const_name)
            .ok_or_else(|| ParsingError::const_not_found(op))
            .map(|v| Some(*v)),
        Err(_) => Ok(None),
    }
}

/// Parses a constant value and ensures it falls within bounds specified by the caller.
fn parse_const_value(
    op: &Token,
    const_value: &str,
    constants: &LocalConstMap,
) -> Result<u64, ParsingError> {
    let result = match const_value.parse::<u64>() {
        Ok(value) => value,
        Err(_) => match const_value.strip_prefix("0x") {
            Some(param_str) => parse_hex_value(op, param_str, 1, Endianness::Big)?,
            None => calculate_const_value(op, const_value, constants)?.as_int(),
        },
    };

    if result >= Felt::MODULUS {
        let reason = format!("constant value must be smaller than {}", Felt::MODULUS);
        Err(ParsingError::invalid_const_value(op, const_value, &reason))
    } else {
        Ok(result)
    }
}

/// Parses a param from the op token with the specified type and index. If the param is a constant
/// label, it will be looked up in the provided constant map.
pub(crate) fn parse_param_with_constant_lookup<R>(
    op: &Token,
    param_idx: usize,
    constants: &LocalConstMap,
) -> Result<R, ParsingError>
where
    R: TryFrom<u64> + core::str::FromStr,
{
    let param_str = op.parts()[param_idx];
    match try_get_constant_value(op, param_str, constants)? {
        Some(val) => val
            .try_into()
            .map_err(|_| ParsingError::const_conversion_failed(op, core::any::type_name::<R>())),
        None => parse_param::<R>(op, param_idx),
    }
}

/// Parses a param from the op token with the specified type.
fn parse_param<I: core::str::FromStr>(op: &Token, param_idx: usize) -> Result<I, ParsingError> {
    let param_value = op.parts()[param_idx];

    let result = match param_value.parse::<I>() {
        Ok(i) => i,
        Err(_) => return Err(ParsingError::invalid_param(op, param_idx)),
    };

    Ok(result)
}

/// Parses a param from the op token with the specified type and ensures that it falls within the
/// bounds specified by the caller.
fn parse_checked_param<I, R>(op: &Token, param_idx: usize, range: R) -> Result<I, ParsingError>
where
    I: core::str::FromStr + Ord + Clone + Into<u64> + Display,
    R: RangeBounds<I>,
{
    let param_value = op.parts()[param_idx];

    let result = match param_value.parse::<I>() {
        Ok(i) => i,
        Err(_) => return Err(ParsingError::invalid_param(op, param_idx)),
    };

    // check that the parameter is within the specified bounds
    range.contains(&result).then_some(result).ok_or_else(||
        ParsingError::invalid_param_with_reason(
            op,
            param_idx,
            format!(
                "parameter value must be greater than or equal to {lower_bound} and less than or equal to {upper_bound}", lower_bound = bound_into_included_u64(range.start_bound(), true),
                upper_bound = bound_into_included_u64(range.end_bound(), false)
            )
            .as_str(),
        )
    )
}

/// Returns an error if the passed in value is 0.
///
/// This is intended to be used when parsing instructions which need to perform division by
/// immediate value.
fn check_div_by_zero(value: u64, op: &Token, param_idx: usize) -> Result<(), ParsingError> {
    if value == 0 {
        Err(ParsingError::invalid_param_with_reason(op, param_idx, "division by zero"))
    } else {
        Ok(())
    }
}

/// Parses the error code declaration for an assertion instruction, and returns the value of the
/// code.
///
/// The code is expected to be specified via the first instruction parameter and have the form
/// `err=<code>`.
fn parse_error_code(token: &Token, constants: &LocalConstMap) -> Result<u32, ParsingError> {
    let inst = token.parts()[0];
    let err_code_parts: Vec<&str> = token.parts()[1].split('=').collect();
    match err_code_parts.len() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(token, format!("{inst}.err=<code>").as_str())),
        2 => {
            if err_code_parts[0] != "err" {
                return Err(ParsingError::invalid_param(token, 1));
            }

            let err_code_str = err_code_parts[1];
            let err_code = match try_get_constant_value(token, err_code_str, constants)? {
                Some(val) => val.try_into().map_err(|_| ParsingError::invalid_param(token, 1))?,
                None => err_code_str.parse().map_err(|_| ParsingError::invalid_param(token, 1))?,
            };
            Ok(err_code)
        }
        _ => Err(ParsingError::extra_param(token)),
    }
}

/// Parses a hexadecimal parameter value into a u64.
///
/// # Errors
/// Returns an error if:
/// - The length of a short hex string (big-endian) is not even.
/// - The length of a short hex string (big-endian) is greater than 16.
/// - The length of the chunk of a long hex string (little-endian) is not equal to 16.
/// - If the string does not contain a valid hexadecimal value.
/// - If the parsed value is greater than or equal to the field modulus.
fn parse_hex_value(
    op: &Token,
    hex_str: &str,
    param_idx: usize,
    endianness: Endianness,
) -> Result<u64, ParsingError> {
    let value = match endianness {
        Endianness::Big => {
            if hex_str.len() % 2 != 0 {
                return Err(ParsingError::invalid_param_with_reason(
                    op,
                    param_idx,
                    &format!(
                        "hex string '{hex_str}' does not contain an even number of characters"
                    ),
                ));
            }
            if hex_str.len() > HEX_CHUNK_SIZE {
                return Err(ParsingError::invalid_param_with_reason(
                    op,
                    param_idx,
                    &format!("hex string '{hex_str}' contains too many characters"),
                ));
            }
            u64::from_str_radix(hex_str, 16)
                .map_err(|_| ParsingError::invalid_param(op, param_idx))?
        }
        Endianness::Little => {
            if hex_str.len() != HEX_CHUNK_SIZE {
                return Err(ParsingError::invalid_param_with_reason(
                    op,
                    param_idx,
                    &format!("hex string chunk '{hex_str}' must contain exactly 16 characters"),
                ));
            }
            u64::from_str_radix(hex_str, 16)
                .map(|v| v.swap_bytes())
                .map_err(|_| ParsingError::invalid_param(op, param_idx))?
        }
    };

    if value >= Felt::MODULUS {
        Err(ParsingError::invalid_param_with_reason(
            op,
            param_idx,
            &format!("hex string '{hex_str}' contains value greater than field modulus"),
        ))
    } else {
        Ok(value)
    }
}
