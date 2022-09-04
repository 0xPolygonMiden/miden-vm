use super::{
    combine_blocks, parse_code_blocks, ArgsMap, AssemblyContext, AssemblyError, CodeBlock,
    CodeBlockTable, String, Token, TokenStream, Vec,
};
use vm_core::{Felt, Operation};

// PROCEDURE
// ================================================================================================

/// Contains metadata and code of a procedure.
pub struct Procedure {
    label: String,
    is_export: bool,
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

    /// Returns `true` if this is an exported procedure.
    pub fn is_export(&self) -> bool {
        self.is_export
    }

    // PARSER
    // --------------------------------------------------------------------------------------------

    /// Parses and returns a single procedure from the provided token stream.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The token stream does not contain a procedure header token at the current position.
    /// - Parsing of procedure header token fails (e.g., invalid procedure label).
    /// - The procedure is an exported procedure and `allow_export` is false.
    /// - A procedure with the same label already exists in the provided context.
    /// - Parsing of procedure body fails for any reason.
    /// - The procedure body does not terminate with the `END` token.
    pub fn parse(
        tokens: &mut TokenStream,
        context: &mut AssemblyContext,
        cb_table: &mut CodeBlockTable,
        allow_export: bool,
        args_map: &ArgsMap,
        is_declaration: bool,
    ) -> Result<Self, AssemblyError> {
        let proc_start = tokens.pos();

        // read procedure name and consume the procedure header token
        let header = tokens.read().expect("missing procedure header");
        let (label, num_locals, is_export, params) = header.parse_proc(is_declaration)?;
        let mut proc_tokens = Vec::new();
        //TODO: Remove unwrap
        proc_tokens.push(tokens.read().unwrap().to_string());
        // if procedure has parameters
        if !allow_export && is_export {
            return Err(AssemblyError::proc_export_not_allowed(header, &label));
        }
        if context.contains_proc(&label) && is_declaration {
            return Err(AssemblyError::duplicate_proc_label(header, &label));
        }
        tokens.advance();
        let copy_proc = !params.is_empty();

        // parse procedure body, and handle memory allocation/deallocation of locals if any are declared
        let code_root = parse_proc_blocks(
            tokens,
            context,
            cb_table,
            num_locals,
            args_map,
            &mut proc_tokens,
            copy_proc,
        )?;

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
        tokens.advance_with_copy(copy_proc, &mut proc_tokens);
        if copy_proc {
            context.add_proc_with_params(label.clone(), params, proc_tokens.join(" "));
        }
        // build and return the procedure
        Ok(Self {
            label,
            is_export,
            num_locals,
            code_root,
        })
    }
}

// HELPER FUNCTIONS
// ================================================================================================

pub fn parse_proc_blocks(
    tokens: &mut TokenStream,
    context: &mut AssemblyContext,
    cb_table: &mut CodeBlockTable,
    num_proc_locals: u32,
    proc_args: &ArgsMap,
    proc_tokens: &mut Vec<String>,
    copy_proc: bool,
) -> Result<CodeBlock, AssemblyError> {
    // parse the procedure body
    let body = parse_code_blocks(
        tokens,
        context,
        cb_table,
        num_proc_locals,
        proc_args,
        proc_tokens,
        copy_proc,
    )?;

    if num_proc_locals == 0 {
        // if no allocation of locals is required, return the procedure body
        return Ok(body);
    }

    let mut blocks = Vec::new();
    let locals_felt = Felt::new(num_proc_locals as u64);

    // allocate procedure locals before the procedure body
    let alloc_ops = vec![Operation::Push(locals_felt), Operation::FmpUpdate];
    blocks.push(CodeBlock::new_span(alloc_ops));

    // add the procedure body code block
    blocks.push(body);

    // deallocate procedure locals after the procedure body
    let dealloc_ops = vec![Operation::Push(-locals_felt), Operation::FmpUpdate];
    blocks.push(CodeBlock::new_span(dealloc_ops));

    // combine the local memory alloc/dealloc blocks with the procedure body code block
    Ok(combine_blocks(blocks))
}
