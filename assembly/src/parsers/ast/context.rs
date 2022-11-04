use crate::ProcedureId;

use super::{
    io_ops, stack_ops, u32_ops, AssemblyError, Instruction, LocalProcMap, Node, ProcedureAst,
    Token, TokenStream, MODULE_PATH_DELIM,
};
use vm_core::utils::{
    collections::{BTreeMap, Vec},
    string::{String, ToString},
};

// Context
// ================================================================================================

/// AST Parser context that holds internal state to generate correct ASTs.
#[derive(Default)]
pub struct ParserContext {
    pub imports: BTreeMap<String, String>,
    pub local_procs: LocalProcMap,
}

impl ParserContext {
    // STATEMENT PARSERS
    // ================================================================================================

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

        Ok(Node::Repeat(repeat_start, loop_body))
    }

    // CALL PARSERS
    // ================================================================================================

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

    // PROCEDURE PARSERS
    // ================================================================================================

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

                    if self.local_procs.contains_key("test") {
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
            num_locals,
            is_export,
            body,
        };

        Ok(proc)
    }

    // BODY PARSERS
    // ================================================================================================

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
                    if break_on_else {
                        break;
                    }
                    return Err(AssemblyError::dangling_else(token));
                }
                Token::IF => {
                    token.validate_if()?;
                    nodes.push(self.parse_if(tokens)?);
                }
                Token::WHILE => nodes.push(self.parse_while(tokens)?),
                Token::REPEAT => nodes.push(self.parse_repeat(tokens)?),
                Token::EXEC => {
                    let label = token.parse_exec()?;
                    nodes.push(self.parse_exec(label, tokens)?);
                }
                Token::CALL => {
                    let label = token.parse_call()?;
                    nodes.push(self.parse_call(label, tokens)?);
                }
                Token::END => {
                    token.validate_end()?;
                    break;
                }
                Token::USE | Token::EXPORT | Token::PROC | Token::BEGIN => {
                    unreachable!("invalid control token (use|export|proc|begin) found in body");
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
        let (module_name, proc_name) = short_name.split_once(MODULE_PATH_DELIM).unwrap();
        let full_module_name = self.imports.get(module_name).unwrap();
        format!("{full_module_name}{MODULE_PATH_DELIM}{proc_name}")
    }
}

/// Parses a Token into a node instruction.
fn parse_op_token(op: &Token) -> Result<Node, AssemblyError> {
    // based on the instruction, invoke the correct parser for the operation
    let node = match op.parts()[0] {
        // ----- field operations -----------------------------------------------------------------
        "assert" => Node::Instruction(Instruction::Assert),
        "assert_eq" => Node::Instruction(Instruction::AssertEq),
        "assertz" => Node::Instruction(Instruction::Assertz),

        "add" => Node::Instruction(Instruction::Add),
        "sub" => Node::Instruction(Instruction::Sub),
        "mul" => Node::Instruction(Instruction::Mul),
        "div" => Node::Instruction(Instruction::Div),
        "neg" => Node::Instruction(Instruction::Neg),
        "inv" => Node::Instruction(Instruction::Inv),

        "pow2" => Node::Instruction(Instruction::Pow2),
        "exp" => Node::Instruction(Instruction::Exp),

        "not" => Node::Instruction(Instruction::Not),
        "and" => Node::Instruction(Instruction::And),
        "or" => Node::Instruction(Instruction::Or),
        "xor" => Node::Instruction(Instruction::Xor),

        "eq" => Node::Instruction(Instruction::Eq),
        "neq" => Node::Instruction(Instruction::Neq),
        "lt" => Node::Instruction(Instruction::Lt),
        "lte" => Node::Instruction(Instruction::Lte),
        "gt" => Node::Instruction(Instruction::Gt),
        "gte" => Node::Instruction(Instruction::Gte),
        "eqw" => Node::Instruction(Instruction::Eqw),

        // ----- u32 operations -------------------------------------------------------------------
        "u32test" => Node::Instruction(Instruction::U32Test),
        "u32testw" => Node::Instruction(Instruction::U32TestW),
        "u32assert" => Node::Instruction(Instruction::U32Assert),
        "u32assertw" => Node::Instruction(Instruction::U32AssertW),
        "u32cast" => Node::Instruction(Instruction::U32Cast),
        "u32split" => Node::Instruction(Instruction::U32Split),

        "u32checked_add" => u32_ops::parse_u32checked_add(op)?,
        "u32wrapping_add" => u32_ops::parse_u32wrapping_add(op)?,
        "u32overflowing_add" => u32_ops::parse_u32overflowing_add(op)?,

        "u32overflowing_add3" => Node::Instruction(Instruction::U32OverflowingAdd3),
        "u32wrapping_add3" => Node::Instruction(Instruction::U32WrappingAdd3),

        "u32checked_sub" => u32_ops::parse_u32checked_sub(op)?,
        "u32wrapping_sub" => u32_ops::parse_u32wrapping_sub(op)?,
        "u32overflowing_sub" => u32_ops::parse_u32overflowing_sub(op)?,

        "u32checked_mul" => u32_ops::parse_u32checked_mul(op)?,
        "u32wrapping_mul" => u32_ops::parse_u32wrapping_mul(op)?,
        "u32overflowing_mul" => u32_ops::parse_u32overflowing_mul(op)?,

        "u32overflowing_madd" => Node::Instruction(Instruction::U32OverflowingMadd),
        "u32wrapping_madd" => Node::Instruction(Instruction::U32WrappingMadd),

        "u32checked_div" => u32_ops::parse_u32_div(op, true)?,
        "u32unchecked_div" => u32_ops::parse_u32_div(op, false)?,

        "u32checked_mod" => u32_ops::parse_u32_mod(op, true)?,
        "u32unchecked_mod" => u32_ops::parse_u32_mod(op, false)?,

        "u32checked_divmod" => u32_ops::parse_u32_divmod(op, true)?,
        "u32unchecked_divmod" => u32_ops::parse_u32_divmod(op, false)?,

        "u32checked_and" => Node::Instruction(Instruction::U32CheckedAnd),
        "u32checked_or" => Node::Instruction(Instruction::U32CheckedOr),
        "u32checked_xor" => Node::Instruction(Instruction::U32CheckedXor),
        "u32checked_not" => Node::Instruction(Instruction::U32CheckedNot),

        "u32checked_shr" => u32_ops::parse_u32_shr(op, true)?,
        "u32unchecked_shr" => u32_ops::parse_u32_shr(op, false)?,

        "u32checked_shl" => u32_ops::parse_u32_shl(op, true)?,
        "u32unchecked_shl" => u32_ops::parse_u32_shl(op, false)?,

        "u32checked_rotr" => u32_ops::parse_u32_rotr(op, true)?,
        "u32unchecked_rotr" => u32_ops::parse_u32_rotr(op, false)?,

        "u32checked_rotl" => u32_ops::parse_u32_rotl(op, true)?,
        "u32unchecked_rotl" => u32_ops::parse_u32_rotl(op, false)?,

        "u32checked_eq" => Node::Instruction(Instruction::U32CheckedEq),
        "u32checked_neq" => Node::Instruction(Instruction::U32CheckedNeq),

        "u32checked_lt" => Node::Instruction(Instruction::U32CheckedLt),
        "u32unchecked_lt" => Node::Instruction(Instruction::U32UncheckedLt),

        "u32checked_lte" => Node::Instruction(Instruction::U32CheckedLte),
        "u32unchecked_lte" => Node::Instruction(Instruction::U32UncheckedLte),

        "u32checked_gt" => Node::Instruction(Instruction::U32CheckedGt),
        "u32unchecked_gt" => Node::Instruction(Instruction::U32UncheckedGt),

        "u32checked_gte" => Node::Instruction(Instruction::U32CheckedGte),
        "u32unchecked_gte" => Node::Instruction(Instruction::U32UncheckedGte),

        "u32checked_min" => Node::Instruction(Instruction::U32CheckedMin),
        "u32unchecked_min" => Node::Instruction(Instruction::U32UncheckedMin),

        "u32checked_max" => Node::Instruction(Instruction::U32CheckedMax),
        "u32unchecked_max" => Node::Instruction(Instruction::U32UncheckedMax),

        // ----- stack manipulation ---------------------------------------------------------------
        "drop" => Node::Instruction(Instruction::Drop),
        "dropw" => Node::Instruction(Instruction::DropW),
        "padw" => Node::Instruction(Instruction::PadW),
        "dup" => stack_ops::parse_dup(op)?,
        "dupw" => stack_ops::parse_dupw(op)?,
        "swap" => stack_ops::parse_swap(op)?,
        "swapw" => stack_ops::parse_swapw(op)?,
        "swapdw" => Node::Instruction(Instruction::SwapDW),
        "movup" => stack_ops::parse_movup(op)?,
        "movupw" => stack_ops::parse_movupw(op)?,
        "movdn" => stack_ops::parse_movdn(op)?,
        "movdnw" => stack_ops::parse_movdnw(op)?,

        "cswap" => Node::Instruction(Instruction::CSwap),
        "cswapw" => Node::Instruction(Instruction::CSwapW),
        "cdrop" => Node::Instruction(Instruction::CDrop),
        "cdropw" => Node::Instruction(Instruction::CDropW),

        // ----- input / output operations --------------------------------------------------------
        "push" => io_ops::parse_push(op)?,

        "sdepth" => Node::Instruction(Instruction::Sdepth),
        "locaddr" => io_ops::parse_locaddr(op)?,

        "mem_load" => io_ops::parse_mem_load(op)?,
        "loc_load" => io_ops::parse_loc_load(op)?,

        "mem_loadw" => io_ops::parse_mem_loadw(op)?,
        "loc_loadw" => io_ops::parse_loc_loadw(op)?,

        "mem_store" => io_ops::parse_mem_store(op)?,
        "loc_store" => io_ops::parse_loc_store(op)?,

        "mem_storew" => io_ops::parse_mem_storew(op)?,
        "loc_storew" => io_ops::parse_loc_storew(op)?,

        "adv_push" => io_ops::parse_adv_push(op)?,
        "adv_loadw" => io_ops::parse_adv_loadw(op)?,

        "adv" => io_ops::parse_adv_inject(op)?,

        // ----- cryptographic operations ---------------------------------------------------------
        "rphash" => Node::Instruction(Instruction::RPHash),
        "rpperm" => Node::Instruction(Instruction::RPPerm),

        "mtree_get" => Node::Instruction(Instruction::MTreeGet),
        "mtree_set" => Node::Instruction(Instruction::MTreeSet),
        "mtree_cwm" => Node::Instruction(Instruction::MTreeCWM),

        // ----- catch all ------------------------------------------------------------------------
        _ => return Err(AssemblyError::invalid_op(op)),
    };

    Ok(node)
}
