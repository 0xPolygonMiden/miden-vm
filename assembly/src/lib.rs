use vm_core::program::{blocks::CodeBlock, Script};

mod context;
use context::ScriptContext;

mod procedures;
use procedures::Procedure;

mod parsers;
use parsers::{parse_code_blocks, parse_proc_blocks};

mod tokens;
use tokens::{Token, TokenStream};

mod errors;
pub use errors::AssemblyError;

#[cfg(test)]
mod tests;

// ASSEMBLER
// ================================================================================================

/// TODO: add comments
pub struct Assembler {}

impl Assembler {
    pub fn new() -> Self {
        Self {}
    }

    /// TODO: add comments
    pub fn compile_script(&self, source: &str) -> Result<Script, AssemblyError> {
        let mut tokens = TokenStream::new(source)?;
        let mut context = ScriptContext::new();

        // parse procedures and add them to the procedure map; procedures are parsed in the order
        // in which they appear in the source, and thus, procedures which come later may invoke
        // preceding procedures
        while let Some(token) = tokens.read() {
            match token.parts()[0] {
                Token::PROC => {
                    let proc = Procedure::parse(&mut tokens, &context)?;
                    context.add_local_proc(proc);
                }
                _ => break,
            }
        }

        // make sure script body is present
        let next_token = tokens
            .read()
            .ok_or_else(|| AssemblyError::unexpected_eof(tokens.pos()))?;
        if next_token.parts()[0] != Token::BEGIN {
            return Err(AssemblyError::unexpected_token(next_token, Token::BEGIN));
        }

        // parse script body and return the resulting script
        let script_root = parse_script(&mut tokens, &context)?;
        Ok(Script::new(script_root))
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

// PARSERS
// ================================================================================================

/// TODO: add comments
fn parse_script(
    tokens: &mut TokenStream,
    context: &ScriptContext,
) -> Result<CodeBlock, AssemblyError> {
    let script_start = tokens.pos();
    // consume the 'begin' token
    let header = tokens.read().expect("missing script header");
    header.validate_begin()?;
    tokens.advance();

    // parse the script body
    let root = parse_code_blocks(tokens, context, 0)?;

    // consume the 'end' token
    match tokens.read() {
        None => Err(AssemblyError::unmatched_begin(
            tokens.read_at(script_start).expect("no begin token"),
        )),
        Some(token) => match token.parts()[0] {
            Token::END => token.validate_end(),
            Token::ELSE => Err(AssemblyError::dangling_else(token)),
            _ => Err(AssemblyError::unmatched_begin(
                tokens.read_at(script_start).expect("no begin token"),
            )),
        },
    }?;
    tokens.advance();

    // make sure there are no instructions after the end
    if let Some(token) = tokens.read() {
        return Err(AssemblyError::dangling_ops_after_script(token));
    }

    Ok(root)
}
