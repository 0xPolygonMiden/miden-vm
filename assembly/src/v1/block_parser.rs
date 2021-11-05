use super::{parse_op_token, AssemblyError, TokenStream};
use vm_core::v1::program::{blocks::CodeBlock, Operation};
use winter_utils::group_vector_elements;

// CONTROL TOKENS
// ================================================================================================

const IF: &str = "if";
const ELSE: &str = "else";
const WHILE: &str = "while";
const REPEAT: &str = "repeat";
const END: &str = "end";

// CODE BLOCK PARSER
// ================================================================================================

#[derive(Debug)]
pub enum BlockParser {
    Span,
    IfElse,
    While,
    Repeat(u32),
}

impl BlockParser {
    pub fn parse(&self, tokens: &mut TokenStream) -> Result<CodeBlock, AssemblyError> {
        match self {
            Self::Span => {
                // --------------------------------------------------------------------------------
                let mut span_ops = Vec::new();
                while let Some(op) = tokens.read() {
                    if is_control_token(op) {
                        break;
                    }
                    parse_op_token(op, &mut span_ops, tokens.pos())?;
                    tokens.advance();
                }
                Ok(CodeBlock::new_span(span_ops))
            }
            Self::IfElse => {
                // --------------------------------------------------------------------------------
                // record start of the if-else block and consume the 'if' token
                let block_start = tokens.pos();
                tokens.advance();

                // read the `if` clause
                let t_branch = parse_block_body(tokens)?;

                // build the `else` clause; if the else clause is specified, then read it;
                // otherwise, set to a Span with a single noop
                let f_branch = match tokens.read() {
                    Some(token) => match token[0] {
                        ELSE => {
                            // record start of the `else` block and consume the `else` token
                            validate_else_token(token, tokens.pos())?;
                            let else_start = tokens.pos();
                            tokens.advance();

                            // parse the `false` branch
                            let f_branch = parse_block_body(tokens)?;

                            // consume the `end` token
                            match tokens.read() {
                                None => Err(AssemblyError::unmatched_else(else_start)),
                                Some(token) => match token[0] {
                                    END => validate_end_token(token, tokens.pos()),
                                    _ => Err(AssemblyError::unmatched_else(else_start)),
                                },
                            }?;
                            tokens.advance();

                            // return the `false` branch
                            f_branch
                        }
                        END => {
                            // consume the `end` token
                            validate_end_token(token, tokens.pos())?;
                            tokens.advance();

                            // when no `else` clause was specified, a Span with a single noop
                            CodeBlock::new_span(vec![Operation::Noop])
                        }
                        _ => return Err(AssemblyError::unmatched_if(block_start)),
                    },
                    None => return Err(AssemblyError::unmatched_if(block_start)),
                };

                Ok(CodeBlock::new_split(t_branch, f_branch))
            }
            Self::While => {
                // --------------------------------------------------------------------------------
                // record start of the while block and consume the 'while' token
                let block_start = tokens.pos();
                tokens.advance();

                // read the loop body
                let loop_body = parse_block_body(tokens)?;

                // consume the `end` token
                match tokens.read() {
                    None => Err(AssemblyError::unmatched_while(block_start)),
                    Some(token) => match token[0] {
                        END => validate_end_token(token, tokens.pos()),
                        _ => Err(AssemblyError::unmatched_while(block_start)),
                    },
                }?;
                tokens.advance();

                Ok(CodeBlock::new_loop(loop_body))
            }
            Self::Repeat(iter_count) => {
                // --------------------------------------------------------------------------------
                // record start of the repeat block and consume the 'repeat' token
                let block_start = tokens.pos();
                tokens.advance();

                // read the loop body
                let loop_body = parse_block_body(tokens)?;

                // consume the `end` token
                match tokens.read() {
                    None => Err(AssemblyError::unmatched_while(block_start)),
                    Some(token) => match token[0] {
                        END => validate_end_token(token, tokens.pos()),
                        _ => Err(AssemblyError::unmatched_while(block_start)),
                    },
                }?;
                tokens.advance();

                // if the body of the loop consists of a single span, unroll the loop as a single
                // span; otherwise unroll the loop as a sequence of join blocks
                if let CodeBlock::Span(span) = loop_body {
                    Ok(CodeBlock::Span(span.replicate(*iter_count as usize)))
                } else {
                    // TODO: transform the loop to a while loop instead?
                    let blocks = (0..*iter_count)
                        .map(|_| loop_body.clone())
                        .collect::<Vec<_>>();
                    Ok(combine_blocks(blocks))
                }
            }
        }
    }

    fn next(tokens: &mut TokenStream) -> Result<Option<Self>, AssemblyError> {
        let parser = match tokens.read() {
            None => None,
            Some(token) => match token[0] {
                IF => {
                    validate_if_token(token, tokens.pos())?;
                    Some(Self::IfElse)
                }
                ELSE => {
                    validate_else_token(token, tokens.pos())?;
                    None
                }
                WHILE => {
                    validate_while_token(token, tokens.pos())?;
                    Some(Self::While)
                }
                REPEAT => {
                    let iter_count = validate_repeat_token(token, tokens.pos())?;
                    Some(Self::Repeat(iter_count))
                }
                END => {
                    validate_end_token(token, tokens.pos())?;
                    None
                }
                _ => Some(Self::Span),
            },
        };

        Ok(parser)
    }
}

// VALIDATORS
// ================================================================================================

fn validate_if_token(token: &[&str], pos: usize) -> Result<(), AssemblyError> {
    assert_eq!(IF, token[0], "not an if");
    if token.len() == 1 || token[1] != "true" {
        Err(AssemblyError::invalid_param_reason(
            token,
            pos,
            "expected if.true".to_string(),
        ))
    } else {
        Ok(())
    }
}

fn validate_else_token(token: &[&str], pos: usize) -> Result<(), AssemblyError> {
    assert_eq!(ELSE, token[0], "not an else");
    if token.len() > 1 {
        Err(AssemblyError::invalid_param_reason(
            token,
            pos,
            "expected else".to_string(),
        ))
    } else {
        Ok(())
    }
}

fn validate_while_token(token: &[&str], pos: usize) -> Result<(), AssemblyError> {
    assert_eq!(WHILE, token[0], "not a while");
    if token.len() == 1 || token[1] != "true" {
        Err(AssemblyError::invalid_param_reason(
            token,
            pos,
            "expected while.true".to_string(),
        ))
    } else {
        Ok(())
    }
}

fn validate_repeat_token(token: &[&str], pos: usize) -> Result<u32, AssemblyError> {
    assert_eq!(REPEAT, token[0], "not a repeat");
    if token.len() > 2 {
        return Err(AssemblyError::extra_param(token, pos));
    }

    // try to parse the parameter value
    token[1]
        .parse::<u32>()
        .map_err(|_| AssemblyError::invalid_param(token, pos))
}

fn validate_end_token(token: &[&str], pos: usize) -> Result<(), AssemblyError> {
    assert_eq!(END, token[0], "not an end");
    if token.len() > 1 {
        Err(AssemblyError::invalid_param_reason(
            token,
            pos,
            "expected end".to_string(),
        ))
    } else {
        Ok(())
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn is_control_token(token: &[&str]) -> bool {
    matches!(token[0], IF | ELSE | WHILE | REPEAT | END)
}

pub fn parse_block_body(tokens: &mut TokenStream) -> Result<CodeBlock, AssemblyError> {
    let mut blocks = Vec::new();
    while let Some(parser) = BlockParser::next(tokens)? {
        let block = parser.parse(tokens)?;
        blocks.push(block);
    }
    // TODO: check that at least one block has been read
    Ok(combine_blocks(blocks))
}

fn combine_blocks(mut blocks: Vec<CodeBlock>) -> CodeBlock {
    while blocks.len() > 1 {
        let last_block = if blocks.len() % 2 == 0 {
            None
        } else {
            blocks.pop()
        };

        let mut grouped_blocks = Vec::new();
        core::mem::swap(&mut blocks, &mut grouped_blocks);
        let mut grouped_blocks = group_vector_elements::<CodeBlock, 2>(grouped_blocks);
        grouped_blocks.drain(0..).for_each(|pair| {
            blocks.push(CodeBlock::new_join(pair));
        });

        if let Some(block) = last_block {
            blocks.push(block);
        }
    }

    blocks.remove(0)
}
