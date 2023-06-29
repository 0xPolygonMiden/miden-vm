use super::{
    bound_into_included_u64, AdviceInjectorNode, CodeBody, Deserializable, Felt, Instruction,
    InvocationTarget, LabelError, LibraryPath, LocalConstMap, LocalProcMap, ModuleImports, Node,
    ParsingError, ProcedureAst, ProcedureId, ProcedureName, ReExportedProcMap, RpoDigest,
    SliceReader, StarkField, String, ToString, Token, TokenStream, Vec, MAX_BODY_LEN, MAX_DOCS_LEN,
    MAX_LABEL_LEN, MAX_STACK_WORD_OFFSET,
};
use core::{fmt::Display, ops::RangeBounds};

pub mod adv_ops;
pub mod field_ops;
pub mod io_ops;
pub mod stack_ops;
pub mod u32_ops;

mod context;
pub use context::ParserContext;

mod labels;
pub use labels::{
    decode_hex_rpo_digest_label, CONSTANT_LABEL_PARSER, NAMESPACE_LABEL_PARSER,
    PROCEDURE_LABEL_PARSER,
};

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
                let (name, value) = parse_constant(token)?;

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
fn parse_constant(token: &Token) -> Result<(String, u64), ParsingError> {
    match token.num_parts() {
        0 => unreachable!(),
        1 => Err(ParsingError::missing_param(token)),
        2 => {
            let const_declaration: Vec<&str> = token.parts()[1].split('=').collect();
            match const_declaration.len() {
                0 => unreachable!(),
                1 => Err(ParsingError::missing_param(token)),
                2 => {
                    let name = CONSTANT_LABEL_PARSER
                        .parse_label(const_declaration[0])
                        .map_err(|err| ParsingError::invalid_const_name(token, err))?;
                    let value = parse_const_value(token, const_declaration[1])?;
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

/// Parses a constant value and ensures it falls within bounds specified by the caller
fn parse_const_value(op: &Token, const_value: &str) -> Result<u64, ParsingError> {
    let result = const_value
        .parse::<u64>()
        .map_err(|err| ParsingError::invalid_const_value(op, const_value, &err.to_string()))?;

    let range = 0..Felt::MODULUS;
    range.contains(&result).then_some(result).ok_or_else(|| ParsingError::invalid_const_value(op, const_value, format!(
        "constant value must be greater than or equal to {lower_bound} and less than or equal to {upper_bound}", lower_bound = bound_into_included_u64(range.start_bound(), true),
        upper_bound = bound_into_included_u64(range.end_bound(), false)
    )
    .as_str(),))
}

/// Parses a param from the op token with the specified type and index. If the param is a constant
/// label, it will be looked up in the provided constant map.
fn parse_param_with_constant_lookup<R>(
    op: &Token,
    param_idx: usize,
    constants: &LocalConstMap,
) -> Result<R, ParsingError>
where
    R: TryFrom<u64> + core::str::FromStr,
{
    let param_str = op.parts()[param_idx];
    match CONSTANT_LABEL_PARSER.parse_label(param_str) {
        Ok(_) => {
            let constant = constants
                .get(param_str)
                .cloned()
                .ok_or_else(|| ParsingError::const_not_found(op))?;
            constant
                .try_into()
                .map_err(|_| ParsingError::const_conversion_failed(op, core::any::type_name::<R>()))
        }
        Err(_) => parse_param::<R>(op, param_idx),
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
