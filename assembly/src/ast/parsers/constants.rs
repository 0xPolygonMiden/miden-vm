use super::{Felt, LocalConstMap, ParsingError, StarkField, String, ToString, Token, Vec};
use core::fmt::Display;

// CONSTANT VALUE EXPRESSIONS
// ================================================================================================

/// An operation used in constant expressions
#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    LPar,
    RPar,
    Value(Felt),
}

impl Operation {
    /// Returns operator from [Operation] based on provided character
    pub fn get_operator(char: &char) -> Self {
        match char {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            '(' => Self::LPar,
            ')' => Self::RPar,
            _ => unreachable!("Invalid operator character"),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Operation::*;

        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            IntDiv => write!(f, "//"),
            LPar => write!(f, "("),
            RPar => write!(f, ")"),
            Value(v) => write!(f, "{}", v),
        }
    }
}

/// Calculates expression in the constant value.
pub fn calculate_const_value(
    op: &Token,
    expression: &str,
    constants: &LocalConstMap,
) -> Result<Felt, ParsingError> {
    let postfix_expression = build_postfix_expression(op, expression, constants)?;
    evaluate_postfix_expression(op, expression, postfix_expression)
}

/// Parses constant value expression and transforms it into the postfix notation
fn build_postfix_expression(
    op: &Token,
    expression: &str,
    constants: &LocalConstMap,
) -> Result<Vec<Operation>, ParsingError> {
    let ops = ['+', '-', '*', '/', '(', ')'];

    let mut stack = Vec::new();
    let mut postfix_expression = Vec::new();
    let mut parsed_value = String::new();

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
            if !parsed_value.is_empty() {
                let parsed_number = parse_number(op, expression, constants, parsed_value)?;
                postfix_expression.push(parsed_number);
                parsed_value = "".to_string();
            }

            // if we get `(` push it on the stack
            if current_char == '(' {
                stack.push(Operation::LPar);
            }
            // if we get `)` push operators from the stack to the postfix expression untill we
            // get `(` on stack
            else if current_char == ')' {
                while stack.last() != Some(&Operation::LPar) {
                    postfix_expression.push(stack.pop().unwrap());
                }
                // pop the `(` from stack
                stack.pop();
            }
            // if stack is empty or the last operator on stack is `(` or we got an operator
            // with higher priority than stack top operator -- push obtained operator to the
            // stack
            else if stack.is_empty()
                || stack.last() == Some(&Operation::LPar)
                || left_has_greater_priority(&current_char, stack.last().unwrap())
            {
                // if we get field division
                if current_char == '/' && next_char == Some('/') {
                    stack.push(Operation::IntDiv);
                    next_char = chars.next();
                } else {
                    stack.push(Operation::get_operator(&current_char));
                }
            }
            // if the obtained operator has priority equal or lower than stack top operator
            // push the operators from the stack to the postfix expression until stack is empty
            // or we get from the stack an operation with lower priority
            else {
                postfix_expression.push(stack.pop().unwrap());
                while !stack.is_empty()
                    && !left_has_greater_priority(&current_char, stack.last().unwrap())
                {
                    postfix_expression.push(stack.pop().unwrap());
                }
                // if we get field division
                if current_char == '/' && next_char == Some('/') {
                    stack.push(Operation::IntDiv);
                    next_char = chars.next();
                } else {
                    stack.push(Operation::get_operator(&current_char));
                }
            }
        } else {
            // push character of the number (or its name) to the `parsed_number` string
            parsed_value.push(current_char);
        }

        if next_char.is_none() {
            break;
        }
    }

    // push the remaining number to the postfix expression
    if !parsed_value.is_empty() {
        let parsed_number = parse_number(op, expression, constants, parsed_value)?;
        postfix_expression.push(parsed_number);
    }

    // push remaining on the stack operators to the postfix expression
    while let Some(element) = stack.pop() {
        postfix_expression.push(element);
    }

    Ok(postfix_expression)
}

/// Evaluates constant expression represented in postfix notation
fn evaluate_postfix_expression(
    op: &Token,
    expression: &str,
    postfix_expression: Vec<Operation>,
) -> Result<Felt, ParsingError> {
    let mut stack = Vec::new();

    for operation in postfix_expression.iter() {
        match operation {
            // if the operation is a value
            Operation::Value(value) => stack.push(*value),
            // if the operation is an operator
            _ => {
                let right = stack.pop().expect("stack is empty");
                let left = stack.pop().expect("stack is empty");
                stack.push(compute_statement(left, right, operation, op, expression)?);
            }
        }
    }

    // get the result from the stack
    stack.pop().ok_or_else(|| {
        ParsingError::invalid_const_value(
            op,
            expression,
            &format!("constant expression {} is incorrect", op),
        )
    })
}

// HELPER FUNCTIONS
// ================================================================================================

/// Returns the number in `value` or the constant value if the value is the name of the constant.
fn parse_number(
    op: &Token,
    expression: &str,
    constants: &LocalConstMap,
    value: String,
) -> Result<Operation, ParsingError> {
    let parsed_number = value.parse::<u64>();
    // if the parsed value is a number push it on the stack
    if let Ok(parsed_number) = parsed_number {
        Ok(Operation::Value(Felt::new(parsed_number)))
    }
    // if it is a name of the constant get its value from the `constants` map
    else {
        let parsed_number = constants.get(&value).ok_or_else(|| {
            ParsingError::invalid_const_value(
                op,
                expression,
                &format!("constant with name {} was not initialized", value),
            )
        })?;
        Ok(Operation::Value(Felt::new(*parsed_number)))
    }
}

/// Returns `true` if th left operator has higher priority than the right, `false` otherwise.
fn left_has_greater_priority(left: &char, right: &Operation) -> bool {
    use Operation::*;

    let left_level = match left {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0,
    };
    let right_level = match right {
        Mul | Div | IntDiv => 2,
        Add | Sub => 1,
        _ => 0,
    };
    left_level > right_level
}

/// Computes the expression based on provided `operator` character.
fn compute_statement(
    left: Felt,
    right: Felt,
    operator: &Operation,
    op: &Token,
    expression: &str,
) -> Result<Felt, ParsingError> {
    use Operation::*;
    match operator {
        Add => Ok(left + right),
        Sub => Ok(left - right),
        Mul => Ok(left * right),
        Div => Ok(Felt::new(left.as_int() / right.as_int())),
        IntDiv => Ok(left / right),
        _ => Err(ParsingError::invalid_const_value(
            op,
            expression,
            &format!("expression contains unsupported operator: {}", operator),
        )),
    }
}
