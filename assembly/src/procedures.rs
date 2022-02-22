use super::{parse_proc_blocks, AssemblyContext, AssemblyError, CodeBlock, Token, TokenStream};

// PROCEDURE
// ================================================================================================

/// TODO: add docs
pub struct Procedure {
    label: String,
    #[allow(dead_code)]
    num_locals: u32,
    code_root: CodeBlock,
}

impl Procedure {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a root of this procedure's MAST.
    pub fn code_root(&self) -> &CodeBlock {
        &self.code_root
    }

    /// Returns a label of this procedure.
    pub fn label(&self) -> &str {
        &self.label
    }

    // PARSER
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn parse(
        tokens: &mut TokenStream,
        context: &AssemblyContext,
    ) -> Result<Self, AssemblyError> {
        let proc_start = tokens.pos();

        // read procedure name and consume the procedure header token
        let header = tokens.read().expect("missing procedure header");
        let (label, num_locals) = header.parse_proc()?;
        if context.contains_proc(&label) {
            return Err(AssemblyError::duplicate_proc_label(header, &label));
        }
        tokens.advance();

        // parse procedure body, and handle memory allocation/deallocation of locals if any are declared
        let code_root = parse_proc_blocks(tokens, context, num_locals)?;

        // consume the 'end' token
        match tokens.read() {
            None => Err(AssemblyError::unmatched_proc(
                tokens.read_at(proc_start).expect("no proc token"),
            )),
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                _ => Err(AssemblyError::unmatched_proc(
                    tokens.read_at(proc_start).expect("no proc token"),
                )),
            },
        }?;
        tokens.advance();

        // build and return the procedure
        Ok(Self {
            label,
            num_locals,
            code_root,
        })
    }
}
