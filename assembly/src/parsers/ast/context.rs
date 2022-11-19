use super::{
    field_ops, io_ops, stack_ops, u32_ops, AssemblyError, Instruction, LocalProcMap, Node,
    ProcedureAst, ProcedureId, Token, TokenStream, MODULE_PATH_DELIM,
};
use vm_core::utils::{
    collections::{BTreeMap, Vec},
    string::{String, ToString},
};

// PARSER CONTEXT
// ================================================================================================

/// AST Parser context that holds internal state to generate correct ASTs.
#[derive(Default)]
pub struct ParserContext {
    pub imports: BTreeMap<String, String>,
    pub local_procs: LocalProcMap,
}

impl ParserContext {
    // STATEMENT PARSERS
    // --------------------------------------------------------------------------------------------

    // Parses an if-else statement from the provided token stream.
    fn parse_if(&self, tokens: &mut TokenStream) -> Result<Node, AssemblyError> {
        // record start of the if-else block and consume the 'if' token
        let if_start = tokens.pos();
        tokens.advance();

        let mut t_branch = Vec::<Node>::new();
        // read the `if` clause
        self.parse_body(tokens, &mut t_branch, true)?;

        // build the `else` clause; if the else clause is specified, then read it;
        // otherwise, set to a Span with a single noop
        let f_branch = match tokens.read() {
            Some(token) => match token.parts()[0] {
                Token::ELSE => {
                    // record start of the `else` block and consume the `else` token
                    token.validate_else()?;

                    let else_start = tokens.pos();
                    tokens.advance();

                    let mut f_branch = Vec::<Node>::new();
                    // parse the `false` branch
                    self.parse_body(tokens, &mut f_branch, false)?;

                    // consume the `end` token
                    match tokens.read() {
                        None => Err(AssemblyError::unmatched_else(
                            tokens.read_at(else_start).expect("no else token"),
                        )),
                        Some(token) => match token.parts()[0] {
                            Token::END => token.validate_end(),
                            Token::ELSE => Err(AssemblyError::dangling_else(token)),
                            _ => Err(AssemblyError::unmatched_else(
                                tokens.read_at(else_start).expect("no else token"),
                            )),
                        },
                    }?;
                    tokens.advance();

                    // return the `false` branch
                    f_branch
                }
                Token::END => {
                    // consume the `end` token
                    token.validate_end()?;
                    tokens.advance();
                    Vec::new()
                }
                _ => {
                    return Err(AssemblyError::unmatched_if(
                        tokens.read_at(if_start).expect("no if token"),
                    ))
                }
            },
            None => {
                return Err(AssemblyError::unmatched_if(
                    tokens.read_at(if_start).expect("no if token"),
                ))
            }
        };

        Ok(Node::IfElse(t_branch, f_branch))
    }

    /// Parse while token into AST nodes.
    fn parse_while(&self, tokens: &mut TokenStream) -> Result<Node, AssemblyError> {
        // record start of the while block and consume the 'while' token
        let while_start = tokens.pos();
        tokens.advance();

        let mut loop_body = Vec::<Node>::new();
        // read the loop body
        self.parse_body(tokens, &mut loop_body, false)?;

        // consume the `end` token
        match tokens.read() {
            None => Err(AssemblyError::unmatched_while(
                tokens.read_at(while_start).expect("no if token"),
            )),
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                Token::ELSE => Err(AssemblyError::dangling_else(token)),
                _ => Err(AssemblyError::unmatched_while(
                    tokens.read_at(while_start).expect("no if token"),
                )),
            },
        }?;
        tokens.advance();

        Ok(Node::While(loop_body))
    }

    /// Parse repeat token into AST nodes.
    fn parse_repeat(&self, tokens: &mut TokenStream) -> Result<Node, AssemblyError> {
        // record start of the repeat block and consume the 'repeat' token
        let repeat_start = tokens.pos();
        let count = match tokens.read() {
            Some(token) => token.parse_repeat()? as usize,
            None => {
                return Err(AssemblyError::missing_param(
                    tokens.read_at(repeat_start).expect("no repeat token"),
                ))
            }
        };
        tokens.advance();

        let mut loop_body = Vec::<Node>::new();
        // read the loop body
        self.parse_body(tokens, &mut loop_body, false)?;

        // consume the `end` token
        match tokens.read() {
            None => Err(AssemblyError::unmatched_repeat(
                tokens.read_at(repeat_start).expect("no repeat token"),
            )),
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                Token::ELSE => Err(AssemblyError::dangling_else(token)),
                _ => Err(AssemblyError::unmatched_repeat(
                    tokens.read_at(repeat_start).expect("no repeat token"),
                )),
            },
        }?;
        tokens.advance();

        Ok(Node::Repeat(count, loop_body))
    }

    // CALL PARSERS
    // --------------------------------------------------------------------------------------------

    /// Parse exec token into AST nodes.
    fn parse_exec(&self, label: String, tokens: &mut TokenStream) -> Result<Node, AssemblyError> {
        tokens.advance();

        if label.contains(MODULE_PATH_DELIM) {
            let full_proc_name = self.get_full_imported_proc_name(label);
            let proc_id = ProcedureId::new(full_proc_name);
            Ok(Node::Instruction(Instruction::ExecImported(proc_id)))
        } else {
            let index = self
                .local_procs
                .get(&label)
                .ok_or_else(|| AssemblyError::undefined_proc(tokens.read().unwrap(), &label))?
                .0;

            Ok(Node::Instruction(Instruction::ExecLocal(index)))
        }
    }

    /// Parse call token into AST nodes.
    fn parse_call(&self, label: String, tokens: &mut TokenStream) -> Result<Node, AssemblyError> {
        tokens.advance();
        if label.contains(MODULE_PATH_DELIM) {
            let full_proc_name = self.get_full_imported_proc_name(label);
            let proc_id = ProcedureId::new(full_proc_name);
            Ok(Node::Instruction(Instruction::CallImported(proc_id)))
        } else {
            let index = self
                .local_procs
                .get(&label)
                .ok_or_else(|| AssemblyError::undefined_proc(tokens.read().unwrap(), &label))?
                .0;

            Ok(Node::Instruction(Instruction::CallLocal(index)))
        }
    }

    /// Parse syscall token into AST nodes.
    fn parse_syscall(
        &self,
        label: String,
        tokens: &mut TokenStream,
    ) -> Result<Node, AssemblyError> {
        tokens.advance();
        let proc_id = ProcedureId::from_kernel_name(label.as_str());
        Ok(Node::Instruction(Instruction::SysCall(proc_id)))
    }

    // PROCEDURE PARSERS
    // --------------------------------------------------------------------------------------------

    /// Parse procedures in the source and store them in the program
    pub fn parse_procedures(
        &mut self,
        tokens: &mut TokenStream,
        allow_export: bool,
    ) -> Result<(), AssemblyError> {
        while let Some(token) = tokens.read() {
            match token.parts()[0] {
                Token::EXPORT | Token::PROC => {
                    let (label, _, is_export) = token.parse_proc()?;
                    if !allow_export && is_export {
                        return Err(AssemblyError::proc_export_not_allowed(token, &label));
                    }

                    if self.local_procs.contains_key(&label) {
                        return Err(AssemblyError::duplicate_proc_label(token, &label));
                    }

                    let proc = self.parse_procedure(tokens)?;
                    self.local_procs
                        .insert(label.to_string(), (self.local_procs.len() as u16, proc));
                }
                _ => break,
            }
        }

        Ok(())
    }

    /// Parse procedure from token stream and add it to the procedure map in context.
    fn parse_procedure(&self, tokens: &mut TokenStream) -> Result<ProcedureAst, AssemblyError> {
        let proc_start = tokens.pos();

        // read procedure name and consume the procedure header token
        let header = tokens.read().expect("missing procedure header");
        let (label, num_locals, is_export) = header.parse_proc()?;
        let docs = if is_export {
            tokens.take_doc_comment_at(proc_start)
        } else {
            None
        };

        tokens.advance();

        let mut body = Vec::<Node>::new();
        // parse procedure body
        self.parse_body(tokens, &mut body, false)?;

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
        let proc = ProcedureAst {
            name: label,
            docs,
            num_locals,
            is_export,
            body,
        };

        Ok(proc)
    }

    // BODY PARSER
    // --------------------------------------------------------------------------------------------
    /// Parses a token from the token stream in a body, which generates a series of AST nodes.
    pub fn parse_body(
        &self,
        tokens: &mut TokenStream,
        nodes: &mut Vec<Node>,
        break_on_else: bool,
    ) -> Result<(), AssemblyError> {
        while let Some(token) = tokens.read() {
            match token.parts()[0] {
                Token::ELSE => {
                    token.validate_else()?;
                    if break_on_else {
                        break;
                    }
                    return Err(AssemblyError::dangling_else(token));
                }
                Token::IF => {
                    token.validate_if()?;
                    nodes.push(self.parse_if(tokens)?);
                }
                Token::WHILE => {
                    token.validate_while()?;
                    nodes.push(self.parse_while(tokens)?);
                }
                Token::REPEAT => nodes.push(self.parse_repeat(tokens)?),
                Token::EXEC => {
                    let label = token.parse_exec()?;
                    nodes.push(self.parse_exec(label, tokens)?);
                }
                Token::CALL => {
                    let label = token.parse_call()?;
                    nodes.push(self.parse_call(label, tokens)?);
                }
                Token::SYSCALL => {
                    let label = token.parse_syscall()?;
                    nodes.push(self.parse_syscall(label, tokens)?);
                }
                Token::END => {
                    token.validate_end()?;
                    break;
                }
                Token::USE | Token::EXPORT | Token::PROC | Token::BEGIN => {
                    // TODO improve the error with the originating block
                    // https://github.com/maticnetwork/miden/issues/514
                    return Err(AssemblyError::unexpected_body_end(token));
                }
                _ => {
                    // Process non control tokens.
                    while let Some(op) = tokens.read() {
                        if op.is_control_token() {
                            break;
                        }
                        nodes.push(parse_op_token(op)?);
                        tokens.advance();
                    }
                }
            }
        }

        Ok(())
    }

    // HELPER FUNCTIONS
    // ================================================================================================

    fn get_full_imported_proc_name(&self, short_name: String) -> String {
        let (module_name, proc_name) = short_name.rsplit_once(MODULE_PATH_DELIM).unwrap();
        let full_module_name = self.imports.get(module_name).unwrap();
        ProcedureId::path(proc_name, full_module_name)
    }
}

/// Parses a Token into a node instruction.
fn parse_op_token(op: &Token) -> Result<Node, AssemblyError> {
    use Instruction::*;

    // based on the instruction, invoke the correct parser for the operation
    match op.parts()[0] {
        // ----- field operations -----------------------------------------------------------------
        "assert" => simple_instruction(op, Assert),
        "assertz" => simple_instruction(op, Assertz),
        "assert_eq" => simple_instruction(op, AssertEq),

        "add" => field_ops::parse_add(op),
        "sub" => field_ops::parse_sub(op),
        "mul" => field_ops::parse_mul(op),
        "div" => field_ops::parse_div(op),
        "neg" => simple_instruction(op, Neg),
        "inv" => simple_instruction(op, Inv),

        "pow2" => simple_instruction(op, Pow2),
        "exp" => field_ops::parse_exp(op),

        "not" => simple_instruction(op, Not),
        "and" => simple_instruction(op, And),
        "or" => simple_instruction(op, Or),
        "xor" => simple_instruction(op, Xor),

        "eq" => field_ops::parse_eq(op),
        "neq" => field_ops::parse_neq(op),
        "lt" => simple_instruction(op, Lt),
        "lte" => simple_instruction(op, Lte),
        "gt" => simple_instruction(op, Gt),
        "gte" => simple_instruction(op, Gte),
        "eqw" => simple_instruction(op, Eqw),

        // ----- u32 operations -------------------------------------------------------------------
        "u32test" => simple_instruction(op, U32Test),
        "u32testw" => simple_instruction(op, U32TestW),
        "u32assert" => u32_ops::parse_u32assert(op),
        "u32assertw" => simple_instruction(op, U32AssertW),
        "u32cast" => simple_instruction(op, U32Cast),
        "u32split" => simple_instruction(op, U32Split),

        "u32checked_add" => u32_ops::parse_u32checked_add(op),
        "u32wrapping_add" => u32_ops::parse_u32wrapping_add(op),
        "u32overflowing_add" => u32_ops::parse_u32overflowing_add(op),

        "u32overflowing_add3" => simple_instruction(op, U32OverflowingAdd3),
        "u32wrapping_add3" => simple_instruction(op, U32WrappingAdd3),

        "u32checked_sub" => u32_ops::parse_u32checked_sub(op),
        "u32wrapping_sub" => u32_ops::parse_u32wrapping_sub(op),
        "u32overflowing_sub" => u32_ops::parse_u32overflowing_sub(op),

        "u32checked_mul" => u32_ops::parse_u32checked_mul(op),
        "u32wrapping_mul" => u32_ops::parse_u32wrapping_mul(op),
        "u32overflowing_mul" => u32_ops::parse_u32overflowing_mul(op),

        "u32overflowing_madd" => simple_instruction(op, U32OverflowingMadd),
        "u32wrapping_madd" => simple_instruction(op, U32WrappingMadd),

        "u32checked_div" => u32_ops::parse_u32_div(op, true),
        "u32unchecked_div" => u32_ops::parse_u32_div(op, false),

        "u32checked_mod" => u32_ops::parse_u32_mod(op, true),
        "u32unchecked_mod" => u32_ops::parse_u32_mod(op, false),

        "u32checked_divmod" => u32_ops::parse_u32_divmod(op, true),
        "u32unchecked_divmod" => u32_ops::parse_u32_divmod(op, false),

        "u32checked_and" => simple_instruction(op, U32CheckedAnd),
        "u32checked_or" => simple_instruction(op, U32CheckedOr),
        "u32checked_xor" => simple_instruction(op, U32CheckedXor),
        "u32checked_not" => simple_instruction(op, U32CheckedNot),

        "u32checked_shr" => u32_ops::parse_u32_shr(op, true),
        "u32unchecked_shr" => u32_ops::parse_u32_shr(op, false),

        "u32checked_shl" => u32_ops::parse_u32_shl(op, true),
        "u32unchecked_shl" => u32_ops::parse_u32_shl(op, false),

        "u32checked_rotr" => u32_ops::parse_u32_rotr(op, true),
        "u32unchecked_rotr" => u32_ops::parse_u32_rotr(op, false),

        "u32checked_rotl" => u32_ops::parse_u32_rotl(op, true),
        "u32unchecked_rotl" => u32_ops::parse_u32_rotl(op, false),

        "u32checked_eq" => simple_instruction(op, U32CheckedEq),
        "u32checked_neq" => simple_instruction(op, U32CheckedNeq),

        "u32checked_lt" => simple_instruction(op, U32CheckedLt),
        "u32unchecked_lt" => simple_instruction(op, U32UncheckedLt),

        "u32checked_lte" => simple_instruction(op, U32CheckedLte),
        "u32unchecked_lte" => simple_instruction(op, U32UncheckedLte),

        "u32checked_gt" => simple_instruction(op, U32CheckedGt),
        "u32unchecked_gt" => simple_instruction(op, U32UncheckedGt),

        "u32checked_gte" => simple_instruction(op, U32CheckedGte),
        "u32unchecked_gte" => simple_instruction(op, U32UncheckedGte),

        "u32checked_min" => simple_instruction(op, U32CheckedMin),
        "u32unchecked_min" => simple_instruction(op, U32UncheckedMin),

        "u32checked_max" => simple_instruction(op, U32CheckedMax),
        "u32unchecked_max" => simple_instruction(op, U32UncheckedMax),

        // ----- stack manipulation ---------------------------------------------------------------
        "drop" => simple_instruction(op, Drop),
        "dropw" => simple_instruction(op, DropW),
        "padw" => simple_instruction(op, PadW),
        "dup" => stack_ops::parse_dup(op),
        "dupw" => stack_ops::parse_dupw(op),
        "swap" => stack_ops::parse_swap(op),
        "swapw" => stack_ops::parse_swapw(op),
        "swapdw" => simple_instruction(op, SwapDw),
        "movup" => stack_ops::parse_movup(op),
        "movupw" => stack_ops::parse_movupw(op),
        "movdn" => stack_ops::parse_movdn(op),
        "movdnw" => stack_ops::parse_movdnw(op),

        "cswap" => simple_instruction(op, CSwap),
        "cswapw" => simple_instruction(op, CSwapW),
        "cdrop" => simple_instruction(op, CDrop),
        "cdropw" => simple_instruction(op, CDropW),

        // ----- input / output operations --------------------------------------------------------
        "push" => io_ops::parse_push(op),

        "sdepth" => simple_instruction(op, Sdepth),
        "locaddr" => io_ops::parse_locaddr(op),
        "caller" => io_ops::parse_caller(op), // TODO: error if not in SYSCALL

        "mem_load" => io_ops::parse_mem_load(op),
        "loc_load" => io_ops::parse_loc_load(op),

        "mem_loadw" => io_ops::parse_mem_loadw(op),
        "loc_loadw" => io_ops::parse_loc_loadw(op),

        "mem_store" => io_ops::parse_mem_store(op),
        "loc_store" => io_ops::parse_loc_store(op),

        "mem_storew" => io_ops::parse_mem_storew(op),
        "loc_storew" => io_ops::parse_loc_storew(op),

        "mem_stream" => simple_instruction(op, MemStream),
        "adv_pipe" => simple_instruction(op, AdvPipe),

        "adv_push" => io_ops::parse_adv_push(op),
        "adv_loadw" => simple_instruction(op, AdvLoadW),

        "adv" => io_ops::parse_adv_inject(op),

        // ----- cryptographic operations ---------------------------------------------------------
        "rphash" => simple_instruction(op, RpHash),
        "rpperm" => simple_instruction(op, RpPerm),

        "mtree_get" => simple_instruction(op, MTreeGet),
        "mtree_set" => simple_instruction(op, MTreeSet),
        "mtree_cwm" => simple_instruction(op, MTreeCwm),

        // ----- catch all ------------------------------------------------------------------------
        _ => Err(AssemblyError::invalid_op(op)),
    }
}

/// Validates that the provided token does not contain any immediate parameters and returns a node
/// for the specified instruction.
///
/// # Errors
/// Returns an error if the token is not a simple operation (i.e., contains immediate values).
fn simple_instruction(op: &Token, instruction: Instruction) -> Result<Node, AssemblyError> {
    if op.num_parts() > 1 {
        return Err(AssemblyError::extra_param(op));
    }
    Ok(Node::Instruction(instruction))
}
