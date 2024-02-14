use super::{Felt, LocalConstMap, ParsingError, String, Token, Vec};
use core::fmt::Display;

// CONSTANT VALUE EXPRESSIONS
// ================================================================================================

const OPERATORS: [char; 6] = ['+', '-', '*', '/', '(', ')'];

/// An operation used in constant expressions
#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    FeltDiv,
    IntDiv,
    LPar,
    RPar,
    Value(Felt),
}

impl Display for Operation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Operation::*;

        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            FeltDiv => write!(f, "/"),
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
    let mut stack = Vec::new();
    let mut postfix_expression = Vec::new();

    let mut operation_iterator = OperationIterator::new(op, expression, constants);

    while let Some(operation) = operation_iterator.next()? {
        match operation {
            // if we get some value push it to the postfix expression
            Operation::Value(_) => postfix_expression.push(operation),
            // if we get `(` push it on the stack
            Operation::LPar => stack.push(Operation::LPar),
            // if we get `)` push operators from the stack to the postfix expression untill we
            // get `(` on stack
            Operation::RPar => {
                while stack.last() != Some(&Operation::LPar) {
                    postfix_expression.push(stack.pop().unwrap());
                }
                // pop the `(` from stack
                stack.pop();
            }
            // if stack is empty or the last operator on stack is `(` or we got an operator
            // with higher priority than stack top operator -- push obtained operator to the
            // stack
            _ if stack.is_empty()
                || stack.last() == Some(&Operation::LPar)
                || left_has_greater_precedence(&operation, stack.last().unwrap()) =>
            {
                stack.push(operation)
            }
            // if the obtained operator has priority equal or lower than stack top operator
            // push the operators from the stack to the postfix expression until stack is empty
            // or we get from the stack an operation with lower priority
            _ => {
                postfix_expression.push(stack.pop().unwrap());
                while stack.last().is_some()
                    && !left_has_greater_precedence(&operation, stack.last().unwrap())
                {
                    postfix_expression.push(stack.pop().unwrap());
                }
                stack.push(operation);
            }
        }
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
                stack.push(compute_statement(op, left, right, operation)?);
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

/// Used to iterate over operations in `expressions` string.
///
/// `original_expression` stay unchanged during `next` method. It is used to obtain original
/// expression for ParsingError.
struct OperationIterator<'a> {
    op: &'a Token<'a>,
    original_expression: &'a str,
    expression: &'a str,
    constants: &'a LocalConstMap,
}

impl<'a> OperationIterator<'a> {
    /// Returns a new instance of the [OperationIterator].
    pub fn new(op: &'a Token<'a>, expression: &'a str, constants: &'a LocalConstMap) -> Self {
        OperationIterator {
            op,
            original_expression: expression,
            expression,
            constants,
        }
    }

    /// Parses and returns next [Operation] in the expression string.
    pub fn next(&mut self) -> Result<Option<Operation>, ParsingError> {
        let mut parsed_value = String::new();
        let mut char_iter = self.expression.chars();
        match char_iter.next() {
            Some('+') => {
                self.expression = &self.expression[1..];
                Ok(Some(Operation::Add))
            }
            Some('-') => {
                self.expression = &self.expression[1..];
                Ok(Some(Operation::Sub))
            }
            Some('(') => {
                self.expression = &self.expression[1..];
                Ok(Some(Operation::LPar))
            }
            Some(')') => {
                self.expression = &self.expression[1..];
                Ok(Some(Operation::RPar))
            }
            Some('*') => {
                self.expression = &self.expression[1..];
                Ok(Some(Operation::Mul))
            }
            Some('/') => match char_iter.next() {
                Some('/') => {
                    self.expression = &self.expression[2..];
                    Ok(Some(Operation::IntDiv))
                }
                _ => {
                    self.expression = &self.expression[1..];
                    Ok(Some(Operation::FeltDiv))
                }
            },
            Some(value) => {
                parsed_value.push(value);
                let mut next_char = char_iter.next();
                self.expression = &self.expression[1..];
                while next_char.is_some() && !OPERATORS.contains(&next_char.unwrap()) {
                    parsed_value.push(next_char.unwrap());
                    next_char = char_iter.next();
                    self.expression = &self.expression[1..];
                }
                Ok(Some(parse_operand(
                    self.op,
                    self.original_expression,
                    self.constants,
                    parsed_value,
                )?))
            }
            None => Ok(None),
        }
    }
}

/// Returns the number in `value` or the constant value if the value is the name of the constant.
fn parse_operand(
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
fn left_has_greater_precedence(left: &Operation, right: &Operation) -> bool {
    use Operation::*;

    let left_level = match left {
        Mul | FeltDiv | IntDiv => 2,
        Add | Sub => 1,
        _ => 0,
    };
    let right_level = match right {
        Mul | FeltDiv | IntDiv => 2,
        Add | Sub => 1,
        _ => 0,
    };
    left_level > right_level
}

/// Computes the expression based on provided `operator` character.
fn compute_statement(
    op: &Token,
    left: Felt,
    right: Felt,
    operator: &Operation,
) -> Result<Felt, ParsingError> {
    use Operation::*;
    match operator {
        Add => Ok(left + right),
        Sub => Ok(left - right),
        Mul => Ok(left * right),
        IntDiv => {
            if right.as_int() == 0 {
                return Err(ParsingError::const_division_by_zero(op));
            }
            Ok(Felt::new(left.as_int() / right.as_int()))
        }
        FeltDiv => {
            if right.as_int() == 0 {
                return Err(ParsingError::const_division_by_zero(op));
            }
            Ok(left / right)
        }
        _ => unreachable!(),
    }
}

// TESTS
// ================================================================================================
#[cfg(test)]
mod tests {
    use super::{Felt, LocalConstMap, Token};
    use crate::{
        ast::parsers::constants::{
            build_postfix_expression, evaluate_postfix_expression, Operation,
        },
        ONE,
    };
    use Operation::*;

    #[test]
    fn test_build_postfix_expression() {
        let constants = LocalConstMap::from([("A".to_string(), 3), ("B".to_string(), 10)]);

        let expression = "51-A+22";
        let result = build_postfix_expression(&Token::new_dummy(), expression, &constants).unwrap();
        let expected =
            vec![Value(Felt::new(51)), Value(Felt::new(3)), Sub, Value(Felt::new(22)), Add];
        assert_eq!(result, expected);

        let expression = "12*3+(2*B-(A/3+1))-2*3";
        let result = build_postfix_expression(&Token::new_dummy(), expression, &constants).unwrap();
        let expected = vec![
            Value(Felt::new(12)),
            Value(Felt::new(3)),
            Mul,
            Value(Felt::new(2)),
            Value(Felt::new(10)),
            Mul,
            Value(Felt::new(3)),
            Value(Felt::new(3)),
            FeltDiv,
            Value(ONE),
            Add,
            Sub,
            Add,
            Value(Felt::new(2)),
            Value(Felt::new(3)),
            Mul,
            Sub,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_postfix_expression() {
        let expression = "51-A+22";
        let postfix_expression =
            vec![Value(Felt::new(51)), Value(Felt::new(3)), Sub, Value(Felt::new(22)), Add];
        let result =
            evaluate_postfix_expression(&Token::new_dummy(), expression, postfix_expression)
                .unwrap();
        let expected = Felt::new(70);
        assert_eq!(result, expected);

        let expression = "12*3+(2*B-(A/3+1))-2*3";
        let postfix_expression = vec![
            Value(Felt::new(12)),
            Value(Felt::new(3)),
            Mul,
            Value(Felt::new(2)),
            Value(Felt::new(10)),
            Mul,
            Value(Felt::new(3)),
            Value(Felt::new(3)),
            FeltDiv,
            Value(ONE),
            Add,
            Sub,
            Add,
            Value(Felt::new(2)),
            Value(Felt::new(3)),
            Mul,
            Sub,
        ];
        let result =
            evaluate_postfix_expression(&Token::new_dummy(), expression, postfix_expression)
                .unwrap();
        let expected = Felt::new(48);
        assert_eq!(result, expected);
    }
}
