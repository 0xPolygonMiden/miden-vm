use crate::AssemblyError;
use vm_core::v1::program::Script;

mod block_parser;

mod op_parser;
use op_parser::parse_op_token;

mod token_stream;
use token_stream::TokenStream;

#[cfg(test)]
mod tests;

// ASSEMBLY COMPILER
// ================================================================================================
pub fn compile_script(source: &str) -> Result<Script, AssemblyError> {
    let mut tokens = TokenStream::new(source);
    let root = block_parser::parse_block_body(&mut tokens)?;
    Ok(Script::new(root))
}
