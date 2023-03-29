use super::{
    adv_ops, field_ops, io_ops, stack_ops, u32_ops, AbsolutePath, Instruction, LocalConstMap,
    LocalProcMap, Node, ParsingError, ProcedureAst, ProcedureId, Token, TokenStream,
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
    pub imports: BTreeMap<String, AbsolutePath>,
    pub local_procs: LocalProcMap,
    pub local_constants: LocalConstMap,
}

impl ParserContext {
    // STATEMENT PARSERS
    // --------------------------------------------------------------------------------------------

    /// Parses an if-else statement from the provided token stream into an AST node.
    fn parse_if(&self, tokens: &mut TokenStream) -> Result<Node, ParsingError> {
        // record start of the if-else block and consume the 'if' token
        let if_start = tokens.pos();
        tokens.read().expect("no if token").validate_if()?;
        tokens.advance();

        // read the `if` clause
        let mut t_branch = Vec::<Node>::new();
        self.parse_body(tokens, &mut t_branch, true)?;

        // build the `else` clause; if the else clause is specified, then parse it;
        // otherwise, set the `else` to an empty vector
        let f_branch = match tokens.read() {
            Some(token) => match token.parts()[0] {
                Token::ELSE => {
                    // record start of the `else` block and consume the `else` token
                    token.validate_else()?;
                    let else_start = tokens.pos();
                    tokens.advance();

                    // parse the `false` branch
                    let mut f_branch = Vec::<Node>::new();
                    self.parse_body(tokens, &mut f_branch, false)?;

                    // consume the `end` token
                    match tokens.read() {
                        None => {
                            let token = tokens.read_at(else_start).expect("no else token");
                            Err(ParsingError::unmatched_else(token))
                        }
                        Some(token) => match token.parts()[0] {
                            Token::END => token.validate_end(),
                            Token::ELSE => Err(ParsingError::dangling_else(token)),
                            _ => {
                                let token = tokens.read_at(else_start).expect("no else token");
                                Err(ParsingError::unmatched_else(token))
                            }
                        },
                    }?;
                    tokens.advance();

                    // return the `false` branch
                    f_branch
                }
                Token::END => {
                    // consume the `end` token and return an empty vector
                    token.validate_end()?;
                    tokens.advance();
                    Vec::new()
                }
                _ => {
                    let token = tokens.read_at(if_start).expect("no if token");
                    return Err(ParsingError::unmatched_if(token));
                }
            },
            None => {
                let token = tokens.read_at(if_start).expect("no if token");
                return Err(ParsingError::unmatched_if(token));
            }
        };

        Ok(Node::IfElse(t_branch, f_branch))
    }

    /// Parses a while statement from the provided token stream into an AST node.
    fn parse_while(&self, tokens: &mut TokenStream) -> Result<Node, ParsingError> {
        // record start of the while block and consume the 'while' token
        let while_start = tokens.pos();
        tokens.read().expect("no while token").validate_while()?;
        tokens.advance();

        // read the loop body
        let mut loop_body = Vec::<Node>::new();
        self.parse_body(tokens, &mut loop_body, false)?;

        // consume the `end` token
        match tokens.read() {
            None => {
                let token = tokens.read_at(while_start).expect("no while token");
                Err(ParsingError::unmatched_while(token))
            }
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                Token::ELSE => Err(ParsingError::dangling_else(token)),
                _ => {
                    let token = tokens.read_at(while_start).expect("no while token");
                    Err(ParsingError::unmatched_while(token))
                }
            },
        }?;
        tokens.advance();

        Ok(Node::While(loop_body))
    }

    /// Parses a repeat statement from the provided token stream into an AST node.
    fn parse_repeat(&self, tokens: &mut TokenStream) -> Result<Node, ParsingError> {
        // record start of the repeat block and consume the 'repeat' token
        let repeat_start = tokens.pos();
        let count = tokens.read().expect("no repeat token").parse_repeat()?;
        tokens.advance();

        // read the loop body
        let mut loop_body = Vec::<Node>::new();
        self.parse_body(tokens, &mut loop_body, false)?;

        // consume the `end` token
        match tokens.read() {
            None => {
                let token = tokens.read_at(repeat_start).expect("no repeat token");
                Err(ParsingError::unmatched_repeat(token))
            }
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                Token::ELSE => Err(ParsingError::dangling_else(token)),
                _ => {
                    let token = tokens.read_at(repeat_start).expect("no repeat token");
                    Err(ParsingError::unmatched_repeat(token))
                }
            },
        }?;
        tokens.advance();

        Ok(Node::Repeat(count, loop_body))
    }

    // CALL PARSERS
    // --------------------------------------------------------------------------------------------

    /// Parse an `exec` token into an instruction node.
    fn parse_exec(&self, token: &Token) -> Result<Node, ParsingError> {
        // get the name of the invoked procedure; if the procedure is invoked from an imported
        // module, the module name is also returned
        let (proc_name, module_name) = token.parse_exec()?;

        if let Some(module_name) = module_name {
            let proc_id = self.get_imported_proc_id(proc_name, module_name, token)?;
            Ok(Node::Instruction(Instruction::ExecImported(proc_id)))
        } else {
            let index = self.get_local_proc_index(proc_name, token)?;
            Ok(Node::Instruction(Instruction::ExecLocal(index)))
        }
    }

    /// Parse a `call` token into an instruction node.
    fn parse_call(&self, token: &Token) -> Result<Node, ParsingError> {
        // get the name of the invoked procedure; if the procedure is invoked from an imported
        // module, the module name is also returned
        let (proc_name, module_name) = token.parse_call()?;

        if let Some(module_name) = module_name {
            let proc_id = self.get_imported_proc_id(proc_name, module_name, token)?;
            Ok(Node::Instruction(Instruction::CallImported(proc_id)))
        } else {
            let index = self.get_local_proc_index(proc_name, token)?;
            Ok(Node::Instruction(Instruction::CallLocal(index)))
        }
    }

    /// Parse `syscall` token into an instruction node.
    fn parse_syscall(&self, token: &Token) -> Result<Node, ParsingError> {
        // get the name of the invoked procedure;
        let proc_name = token.parse_syscall()?;

        let proc_id = ProcedureId::from_kernel_name(proc_name);
        Ok(Node::Instruction(Instruction::SysCall(proc_id)))
    }

    // PROCEDURE PARSERS
    // --------------------------------------------------------------------------------------------

    /// Parse procedures in the source and store them in the program
    pub fn parse_procedures(
        &mut self,
        tokens: &mut TokenStream,
        allow_export: bool,
    ) -> Result<(), ParsingError> {
        // parse procedures until all `proc` or `exec` tokens have been consumed
        while let Some(token) = tokens.read() {
            match token.parts()[0] {
                Token::EXPORT => {
                    if !allow_export {
                        let proc_name = token.parts()[1];
                        return Err(ParsingError::proc_export_not_allowed(token, proc_name));
                    }
                }
                Token::PROC => {
                    // no validation needed, parse the procedure below
                }
                _ => break,
            }

            // parse the procedure body and add it to the list of local procedures
            let proc = self.parse_procedure(tokens)?;
            self.local_procs
                .insert(proc.name.to_string(), (self.local_procs.len() as u16, proc));
        }

        Ok(())
    }

    /// Parse procedure from token stream and add it to the procedure map in context.
    fn parse_procedure(&self, tokens: &mut TokenStream) -> Result<ProcedureAst, ParsingError> {
        let proc_start = tokens.pos();

        // parse procedure declaration, make sure the procedure with the same name hasn't been
        // declared previously, and consume the `proc` token.
        let header = tokens.read().expect("missing procedure header");
        let (name, num_locals, is_export) = header.parse_proc()?;
        if self.local_procs.contains_key(name.as_str()) {
            return Err(ParsingError::duplicate_proc_name(header, name.as_str()));
        }
        tokens.advance();

        // attach doc comments (if any) to exported procedures
        let docs = if is_export {
            tokens.take_doc_comment_at(proc_start)
        } else {
            None
        };

        // parse procedure body
        let mut body = Vec::<Node>::new();
        self.parse_body(tokens, &mut body, false)?;

        // consume the 'end' token
        match tokens.read() {
            None => {
                let token = tokens.read_at(proc_start).expect("no proc token");
                Err(ParsingError::unmatched_proc(token, name.as_str()))
            }
            Some(token) => match token.parts()[0] {
                Token::END => token.validate_end(),
                _ => {
                    let token = tokens.read_at(proc_start).expect("no proc token");
                    Err(ParsingError::unmatched_proc(token, name.as_str()))
                }
            },
        }?;
        tokens.advance();

        // build and return the procedure
        let proc = ProcedureAst {
            name,
            docs,
            num_locals,
            is_export,
            body,
        };

        Ok(proc)
    }

    // BODY PARSER
    // --------------------------------------------------------------------------------------------
    /// Parses AST tokens from the token stream and add them to the nodes vector.
    ///
    /// Nodes are added to the list until `if`, `else`, `while`, `repeat`, `end`, `export`, `proc`,
    /// or `begin` tokens are encountered, or an error occurs.
    pub fn parse_body(
        &self,
        tokens: &mut TokenStream,
        nodes: &mut Vec<Node>,
        break_on_else: bool,
    ) -> Result<(), ParsingError> {
        while let Some(token) = tokens.read() {
            match token.parts()[0] {
                Token::IF => nodes.push(self.parse_if(tokens)?),
                Token::ELSE => {
                    token.validate_else()?;
                    if break_on_else {
                        break;
                    }
                    return Err(ParsingError::dangling_else(token));
                }
                Token::WHILE => nodes.push(self.parse_while(tokens)?),
                Token::REPEAT => nodes.push(self.parse_repeat(tokens)?),
                Token::END => {
                    token.validate_end()?;
                    break;
                }
                Token::USE => {
                    return Err(ParsingError::import_inside_body(token));
                }
                Token::EXPORT | Token::PROC | Token::BEGIN => {
                    // break out of the loop; whether this results in an error will be determined
                    // by the function which invoked parse_body()
                    break;
                }
                _ => {
                    nodes.push(self.parse_op_token(token)?);
                    tokens.advance();
                }
            }
        }

        Ok(())
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Parses a token into an instruction node.
    fn parse_op_token(&self, op: &Token) -> Result<Node, ParsingError> {
        use Instruction::*;

        // based on the instruction, invoke the correct parser for the operation
        match op.parts()[0] {
            // ----- field operations -------------------------------------------------------------
            "assert" => simple_instruction(op, Assert),
            "assertz" => simple_instruction(op, Assertz),
            "assert_eq" => simple_instruction(op, AssertEq),
            "assert_eqw" => simple_instruction(op, AssertEqw),

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
            "is_odd" => simple_instruction(op, IsOdd),
            "eqw" => simple_instruction(op, Eqw),

            // ----- ext2 operations -----------------------------------------------------
            "ext2add" => simple_instruction(op, Ext2Add),
            "ext2sub" => simple_instruction(op, Ext2Sub),
            "ext2mul" => simple_instruction(op, Ext2Mul),
            "ext2div" => simple_instruction(op, Ext2Div),
            "ext2neg" => simple_instruction(op, Ext2Neg),
            "ext2inv" => simple_instruction(op, Ext2Inv),

            // ----- u32 operations ---------------------------------------------------------------
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

            "u32checked_popcnt" => simple_instruction(op, U32CheckedPopcnt),
            "u32unchecked_popcnt" => simple_instruction(op, U32UncheckedPopcnt),

            "u32checked_eq" => u32_ops::parse_u32checked_eq(op),
            "u32checked_neq" => u32_ops::parse_u32checked_neq(op),

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

            // ----- stack manipulation -----------------------------------------------------------
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

            // ----- input / output operations ----------------------------------------------------
            "push" => io_ops::parse_push(op, &self.local_constants),

            "sdepth" => simple_instruction(op, Sdepth),
            "locaddr" => io_ops::parse_locaddr(op),
            "caller" => simple_instruction(op, Caller), // TODO: error if not in SYSCALL (issue #551)
            "clk" => simple_instruction(op, Clk),

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

            "adv" => adv_ops::parse_adv_inject(op),

            // ----- cryptographic operations -----------------------------------------------------
            "hash" => simple_instruction(op, Hash),
            "hmerge" => simple_instruction(op, HMerge),
            "hperm" => simple_instruction(op, HPerm),

            "mtree_get" => simple_instruction(op, MTreeGet),
            "mtree_set" => simple_instruction(op, MTreeSet),
            "mtree_merge" => simple_instruction(op, MTreeMerge),

            "fri_ext2fold4" => simple_instruction(op, FriExt2Fold4),

            // ----- procedure invocations --------------------------------------------------------
            "exec" => self.parse_exec(op),
            "call" => self.parse_call(op),
            "syscall" => self.parse_syscall(op),

            // ----- constant statements ----------------------------------------------------------
            "const" => Err(ParsingError::const_invalid_scope(op)),

            // ----- debug decorators -------------------------------------------------------------
            "breakpoint" => simple_instruction(op, Breakpoint),

            // ----- catch all --------------------------------------------------------------------
            _ => Err(ParsingError::invalid_op(op)),
        }
    }

    /// Returns an index of a local procedure for the specified procedure name.
    ///
    /// # Errors
    /// Returns an error if a local procedure with the specified name has not been parsed ye.
    fn get_local_proc_index(&self, proc_name: &str, token: &Token) -> Result<u16, ParsingError> {
        self.local_procs
            .get(proc_name)
            .ok_or_else(|| ParsingError::undefined_local_proc(token, proc_name))
            .map(|(index, _)| *index)
    }

    /// Returns procedure ID of a procedure imported from the specified module.
    ///
    /// # Errors
    /// Return an error if the module with the specified name has not been imported via the `use`
    /// statement.
    fn get_imported_proc_id(
        &self,
        proc_name: &str,
        module_name: &str,
        token: &Token,
    ) -> Result<ProcedureId, ParsingError> {
        let module_path = self
            .imports
            .get(module_name)
            .ok_or_else(|| ParsingError::procedure_module_not_imported(token, module_name))?;
        let proc_id = ProcedureId::from_name(proc_name, module_path.as_str());
        Ok(proc_id)
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Validates that the provided token does not contain any immediate parameters and returns a node
/// for the specified instruction.
///
/// # Errors
/// Returns an error if the token is not a simple operation (i.e., contains immediate values).
fn simple_instruction(op: &Token, instruction: Instruction) -> Result<Node, ParsingError> {
    debug_assert_eq!(op.parts()[0], instruction.to_string());
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Node::Instruction(instruction)),
        _ => Err(ParsingError::extra_param(op)),
    }
}
