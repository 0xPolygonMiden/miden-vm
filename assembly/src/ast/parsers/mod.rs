use super::{
    bound_into_included_u64, AdviceInjectorNode, BTreeMap, CodeBody, Deserializable, Felt,
    Instruction, InvocationTarget, LabelError, LibraryPath, LocalConstMap, LocalProcMap,
    ModuleImports, Node, ParsingError, ProcedureAst, ProcedureId, ProcedureName, ReExportedProcMap,
    RpoDigest, SliceReader, StarkField, String, ToString, Token, TokenStream, Vec, MAX_BODY_LEN,
    MAX_DOCS_LEN, MAX_LABEL_LEN, MAX_STACK_WORD_OFFSET,
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
fn parse_constant(
    token: &Token,
    constants: &BTreeMap<String, u64>,
) -> Result<(String, u64), ParsingError> {
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

/// Parses a constant value and ensures it falls within bounds specified by the caller
fn parse_const_value(
    op: &Token,
    const_value: &str,
    constants: &BTreeMap<String, u64>,
) -> Result<u64, ParsingError> {
    let result = match const_value.parse::<u64>() {
        Ok(value) => value,
        Err(_) => calculate_const_value_with_stack(op, const_value, constants)? as u64,
    };

    let range = 0..Felt::MODULUS;
    range.contains(&result).then_some(result).ok_or_else(|| ParsingError::invalid_const_value(op, const_value, format!(
        "constant value must be greater than or equal to {lower_bound} and less than or equal to {upper_bound}", lower_bound = bound_into_included_u64(range.start_bound(), true),
        upper_bound = bound_into_included_u64(range.end_bound(), false)
    )
    .as_str(),))
}

/// Parses and calculates expression in the constant value.
///
/// The idea is to split expression by operators in the correct order and invoke this function on
/// the obtained prefix and suffix.
#[allow(dead_code)]
fn calculate_const_value_without_stack(
    op: &Token,
    expression: &str,
    constants: &BTreeMap<String, u64>,
) -> Result<i64, ParsingError> {
    // handle the parentheses
    let l_par_split = expression.split_once('(');
    if let Some(l_par_split) = l_par_split {
        let right_side = l_par_split.1;
        // get the expression in the parentheses
        let (in_par_string, remainder) = split_parenthesis_content(right_side);
        // calculate the statement in the parentheses
        let par_result = calculate_const_value_without_stack(op, in_par_string, constants)?;
        let precalc_expression = [l_par_split.0, &par_result.to_string(), remainder].concat();
        //
        return calculate_const_value_without_stack(op, &precalc_expression, constants);
    }

    // handle the addition
    let res = expression.split_once('+');
    if let Some(res) = res {
        return Ok(calculate_const_value_without_stack(op, res.0, constants)?
            + calculate_const_value_without_stack(op, res.1, constants)?);
    }

    // handle the subtraction
    // notice that subtraction splitting is performed from the end of the expression
    let res = expression.rsplit_once('-');
    if let Some(res) = res {
        return Ok(calculate_const_value_without_stack(op, res.0, constants)?
            - calculate_const_value_without_stack(op, res.1, constants)?);
    }

    let div_split = expression.split_once('/');
    let mul_split = expression.split_once('*');

    match (div_split, mul_split) {
        (None, None) => {}
        (Some(parts), None) => return handle_division(op, constants, parts),
        (None, Some(parts)) => return handle_multiplication(op, constants, parts),
        (Some(div_parts), Some(mul_parts)) => {
            // if multiplication and division are only left, perform the first operation on the
            // right (less prioritized)
            if div_parts.0.len() > mul_parts.0.len() {
                return handle_division(op, constants, div_parts);
            } else {
                return handle_multiplication(op, constants, mul_parts);
            }
        }
    }

    // handle a constant value
    let number = expression.parse::<i64>();
    if let Ok(number) = number {
        // just return it if it is a number
        Ok(number)
    } else {
        // get its value from the constants map if it is a constant name
        constants
            .get(&expression.to_string())
            .ok_or_else(|| {
                ParsingError::invalid_const_value(
                    op,
                    expression,
                    &format!("constant with name {} was not initialized", expression),
                )
            })
            .map(|&value| value as i64)
    }
}

/// Returns the result of division performed in the constant value. Supports `Felt` and `i64`
/// divison operations.
pub fn handle_division(
    op: &Token,
    constants: &BTreeMap<String, u64>,
    parts: (&str, &str),
) -> Result<i64, ParsingError> {
    if parts.1.starts_with('/') {
        let felt_result =
            Felt::from(calculate_const_value_without_stack(op, parts.0, constants)? as u64)
                / Felt::from(
                    calculate_const_value_without_stack(op, &parts.1[1..], constants)? as u64
                );
        return Ok(felt_result.as_int() as i64);
    }
    calculate_const_value_without_stack(op, parts.0, constants)?
        .checked_div(calculate_const_value_without_stack(op, parts.1, constants)?)
        .ok_or_else(|| ParsingError::const_division_by_zero(op))
}

/// Returns the result of multiplication performed in the constant value.
pub fn handle_multiplication(
    op: &Token,
    constants: &BTreeMap<String, u64>,
    parts: (&str, &str),
) -> Result<i64, ParsingError> {
    Ok(calculate_const_value_without_stack(op, parts.0, constants)?
        * calculate_const_value_without_stack(op, parts.1, constants)?)
}

/// Returns the internals of the current parsing parentheses along with remaining string.
///
/// # Example:
/// If the expression is `3+(2+1))*(5/2)` this function will return `("3+(2+1)", "*(5/2)")`.
///
/// The first parenthesis of the expression is absent because it was consumed on the parsing stage.
pub fn split_parenthesis_content(expression: &str) -> (&str, &str) {
    let mut counter = 0;
    let mut result = "";
    let mut remainder = expression;
    for (index, char) in expression.chars().enumerate() {
        match char {
            '(' => counter += 1,
            ')' if counter == 0 => {
                result = &expression[..index];
                remainder = &expression[index + 1..];
                break;
            }
            ')' if counter != 0 => counter -= 1,
            _ => {}
        }
    }
    (result, remainder)
}

/// Parses and calculates expression in the constant value.
///
/// The idea is to rebuild expression in the postfix representation and then evaluate it.
pub fn calculate_const_value_with_stack(
    op: &Token,
    expression: &str,
    constants: &BTreeMap<String, u64>,
) -> Result<i64, ParsingError> {
    let ops = ['+', '-', '*', '/', '(', ')'];

    let mut stack = Vec::new();
    let mut postfix_expression = Vec::new();
    let mut parsed_number = String::new();

    // Create a "window" of chars containing current and next char of the expression. It is needed
    // to determine field division `//`.
    let mut chars = expression.chars();
    let mut next_char = chars.next();
    let mut current_char: Option<char>;

    loop {
        current_char = next_char;
        next_char = chars.next();

        let current_char = current_char.unwrap();

        // if char is an operator
        if ops.contains(&current_char) {
            // if we already parsed some value before push it to the postfix expression
            if !parsed_number.is_empty() {
                postfix_expression.push(parsed_number);
                parsed_number = "".to_string();
            }

            // if we get `(` push it on the stack
            if current_char == '(' {
                stack.push(current_char.to_string());
            }
            // if we get `)` push operators from the stack to the postfix expression untill we
            // get `(` on stack
            else if current_char == ')' {
                while stack.last() != Some(&"(".to_string()) {
                    postfix_expression.push(stack.pop().unwrap().to_string());
                }
                // pop the `(` from stack
                stack.pop();
            }
            // if stack is empty or the last operator on stack is `(` or we got an operator
            // with higher priority than stack top operator -- push obtained operator to the
            // stack
            else if stack.is_empty()
                || *stack.last().unwrap() == "("
                || left_has_greater_priority(&current_char, stack.last().unwrap())
            {
                // if we get field division
                if current_char == '/' && next_char == Some('/') {
                    stack.push("//".to_string());
                    next_char = chars.next();
                } else {
                    stack.push(current_char.to_string());
                }
            }
            // if the obtained operator has priority equal or lower than stack top operator
            // push the operators from the stack to the postfix expression until stack is empty
            // or we get from the stack an operation with lower priority
            else {
                postfix_expression.push(stack.pop().unwrap().to_string());
                while !stack.is_empty()
                    && !left_has_greater_priority(&current_char, stack.last().unwrap())
                {
                    postfix_expression.push(stack.pop().unwrap().to_string());
                }
                // if we get field division
                if current_char == '/' && next_char == Some('/') {
                    stack.push("//".to_string());
                    next_char = chars.next();
                } else {
                    stack.push(current_char.to_string());
                }
            }
        } else {
            // push character of the number (or its name) to the `parsed_number` string
            parsed_number.push(current_char);
        }

        if next_char.is_none() {
            break;
        }
    }

    // push the remaining number to the postfix expression
    if !parsed_number.is_empty() {
        postfix_expression.push(parsed_number);
    }

    // push remaining on the stack operators to the postfix expression
    while !stack.is_empty() {
        postfix_expression.push(stack.pop().unwrap().to_string());
    }

    let mut stack = Vec::new();
    for elem in postfix_expression.iter() {
        if ops.contains(&elem.chars().next().unwrap()) {
            // if the parsed element is an operator
            let right = stack.pop().expect("stack is empty");
            let left = stack.pop().expect("stack is empty");
            stack.push(compute_statement(left, right, elem, op, expression)?);
        } else {
            let number = elem.parse::<i64>();
            // if the parsed element is a number
            if let Ok(number) = number {
                stack.push(number);
            }
            // if it is a name of the constant get its value from the `constants` map
            else {
                let const_value = constants.get(elem).ok_or_else(|| {
                    ParsingError::invalid_const_value(
                        op,
                        expression,
                        &format!("constant with name {} was not initialized", expression),
                    )
                })?;
                stack.push(*const_value as i64);
            }
        }
    }

    // get the result from the stack
    stack
        .last()
        .ok_or_else(|| {
            ParsingError::invalid_const_value(
                op,
                expression,
                &format!("constant expression {} is incorrect", op),
            )
        })
        .map(|&value| value)
}

/// Returns `true` if th left operator has higher priority than the right, `false` otherwise.
fn left_has_greater_priority(left: &char, right: &str) -> bool {
    let left_level = match left {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0,
    };
    let right_level = match right {
        "*" | "/" | "//" => 2,
        "+" | "-" => 1,
        _ => 0,
    };
    left_level > right_level
}

/// Computes the expression based on provided `operator` character.
fn compute_statement(
    left: i64,
    right: i64,
    operator: &String,
    op: &Token,
    expression: &str,
) -> Result<i64, ParsingError> {
    match operator.as_str() {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => Ok(left.checked_div(right).unwrap()),
        "//" => Ok((Felt::from(left as u64) / Felt::from(right as u64)).as_int() as i64),
        _ => Err(ParsingError::invalid_const_value(
            op,
            expression,
            &format!("expression contains unsupported operator: {}", operator),
        )),
    }
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
