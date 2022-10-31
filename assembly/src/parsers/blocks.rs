use crate::ModuleMap;

use super::{
    ast::nodes::{Instruction, Node},
    parse_op_instruction, AssemblyContext, AssemblyError, CodeBlock, Operation, Vec,
};
use vm_core::{utils::group_vector_elements, CodeBlockTable, DecoratorList};

// BLOCK PARSER
// ================================================================================================

/// parses a program body which is a list of ast nodes, that can be either control flow
/// or instructions.
pub fn parse_body(
    nodes: &Vec<Node>,
    context: &AssemblyContext,
    cb_table: &mut CodeBlockTable,
    parsed_modules: &mut ModuleMap,
    num_proc_locals: u32,
) -> Result<CodeBlock, AssemblyError> {
    // parse the sequence of blocks and add each block to the list
    let mut blocks = Vec::new();
    let mut decorators = DecoratorList::new();
    let mut span_ops = Vec::<Operation>::new();
    for node in nodes {
        let block = parse_node(
            node,
            context,
            cb_table,
            &mut decorators,
            &mut span_ops,
            parsed_modules,
            num_proc_locals,
        )?;
        if let Some(cb) = block {
            blocks.push(cb);
        }
    }

    if !span_ops.is_empty() {
        blocks.push(CodeBlock::new_span_with_decorators(span_ops, decorators))
    }

    // make sure at least one block has been read
    if blocks.is_empty() {
        Err(AssemblyError::invalid_ast_body())
    } else {
        // build a binary tree out of the parsed list of blocks
        Ok(combine_blocks(blocks))
    }
}

// AST NODE PARSER
// ================================================================================================

// parses the ast node and optionally return a codeblock.
fn parse_node<'a>(
    node: &Node,
    context: &AssemblyContext<'a>,
    cb_table: &mut CodeBlockTable,
    decorators: &mut DecoratorList,
    span_ops: &mut Vec<Operation>,
    parsed_modules: &'a mut ModuleMap,
    num_proc_locals: u32,
) -> Result<Option<CodeBlock>, AssemblyError> {
    match node {
        // ------------------------------------------------------------------------------------
        Node::Instruction(instruction) => {
            match instruction {
                Instruction::ExecImported(label) => {
                    context.parse_imports(
                        &String::from(module_name),
                        &mut Vec::new(),
                        parsed_modules,
                        cb_table,
                    )?;

                    // retrieve the procedure block from the proc map and consume the 'exec' token
                    let proc_block = context
                        .get_proc_code(label)
                        .ok_or_else(|| AssemblyError::undefined_proc(label))?;
                    Ok(Some(proc_block.clone()))
                }
                Instruction::ExecLocal(index) => {
                    // retrieve the procedure block from the proc map and consume the 'exec' token
                    let proc_block = context
                        .get_local_proc_code(*index)
                        .ok_or_else(|| AssemblyError::undefined_local_proc(*index))?;
                    Ok(Some(proc_block.clone()))
                }
                Instruction::CallLocal(index) => {
                    // making a function call in kernel context is not allowed
                    if context.in_kernel() {
                        return Err(AssemblyError::call_in_kernel());
                    }

                    // retrieve the procedure block from the proc map and consume the 'call' token
                    let proc_block = context
                        .get_local_proc_code(*index)
                        .ok_or_else(|| AssemblyError::undefined_local_proc(*index))?;

                    // if the procedure hasn't been inserted into code block table yet, insert it
                    if !cb_table.has(proc_block.hash()) {
                        cb_table.insert(proc_block.clone());
                    }

                    Ok(Some(CodeBlock::new_call(proc_block.hash())))
                }
                Instruction::CallImported(label) => {
                    // making a function call in kernel context is not allowed
                    if context.in_kernel() {
                        return Err(AssemblyError::call_in_kernel());
                    }

                    context.parse_imports(
                        &String::from(module_name),
                        &mut Vec::new(),
                        parsed_modules,
                        cb_table,
                    )?;

                    // retrieve the procedure block from the proc map and consume the 'call' token
                    let proc_block = context
                        .get_proc_code(label)
                        .ok_or_else(|| AssemblyError::undefined_proc(label))?;

                    // if the procedure hasn't been inserted into code block table yet, insert it
                    if !cb_table.has(proc_block.hash()) {
                        cb_table.insert(proc_block.clone());
                    }

                    Ok(Some(CodeBlock::new_call(proc_block.hash())))
                }
                Instruction::SysCall(label) => {
                    // making a syscall in kernel context is not allowed
                    if context.in_kernel() {
                        AssemblyError::syscall_in_kernel();
                    }

                    // retrieve the procedure block from the proc map and consume the 'syscall' token
                    let proc_block = context
                        .get_kernel_proc_code(label)
                        .ok_or_else(|| AssemblyError::undefined_kernel_proc(label))?;

                    // if the procedure hasn't been inserted into code block table yet, insert it
                    if !cb_table.has(proc_block.hash()) {
                        cb_table.insert(proc_block.clone());
                    }

                    Ok(Some(CodeBlock::new_syscall(proc_block.hash())))
                }
                _ => {
                    parse_op_instruction(
                        instruction,
                        span_ops,
                        num_proc_locals,
                        decorators,
                        context.in_debug_mode(),
                    )?;
                    Ok(None)
                }
            }
        }
        // ------------------------------------------------------------------------------------
        Node::IfElse(t_branch, f_branch) => {
            // read the `if` clause
            let t_branch_block =
                parse_body(t_branch, context, cb_table, parsed_modules, num_proc_locals)?;

            // build the `else` clause; if the else clause is specified, then read it;
            // otherwise, set to a Span with a single noop
            let f_branch_block = match f_branch.len() {
                0 => CodeBlock::new_span(vec![Operation::Noop]),
                _ => parse_body(f_branch, context, cb_table, parsed_modules, num_proc_locals)?,
            };

            Ok(Some(CodeBlock::new_split(t_branch_block, f_branch_block)))
        }
        // ------------------------------------------------------------------------------------
        Node::While(body) => {
            // read the loop body
            let loop_body = parse_body(body, context, cb_table, parsed_modules, num_proc_locals)?;
            Ok(Some(CodeBlock::new_loop(loop_body)))
        }
        // ------------------------------------------------------------------------------------
        Node::Repeat(iter_count, body) => {
            // read the loop body
            let loop_body = parse_body(body, context, cb_table, parsed_modules, num_proc_locals)?;

            // if the body of the loop consists of a single span, unroll the loop as a single
            // span; otherwise unroll the loop as a sequence of join blocks
            if let CodeBlock::Span(span) = loop_body {
                Ok(Some(CodeBlock::Span(span.replicate(*iter_count as usize))))
            } else {
                // TODO: transform the loop to a while loop instead?
                let blocks = (0..*iter_count)
                    .map(|_| loop_body.clone())
                    .collect::<Vec<_>>();
                Ok(Some(combine_blocks(blocks)))
            }
        }
    }
}

// UTILITY FUNCTIONS
// ================================================================================================

pub fn combine_blocks(mut blocks: Vec<CodeBlock>) -> CodeBlock {
    // merge consecutive Span blocks.
    let mut merged_blocks: Vec<CodeBlock> = Vec::with_capacity(blocks.len());
    // Keep track of all the consecutive Span blocks and are merged together when
    // there is a discontinuity.
    let mut contiguous_spans: Vec<CodeBlock> = Vec::new();

    blocks.drain(0..).for_each(|block| {
        if block.is_span() {
            contiguous_spans.push(block);
        } else {
            if !contiguous_spans.is_empty() {
                merged_blocks.push(combine_spans(&mut contiguous_spans));
            }
            merged_blocks.push(block);
        }
    });
    if !contiguous_spans.is_empty() {
        merged_blocks.push(combine_spans(&mut contiguous_spans));
    }

    // build a binary tree of blocks joining them using Join blocks
    let mut blocks = merged_blocks;
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

/// Returns a CodeBlock [Span] from sequence of Span blocks provided as input.
pub fn combine_spans(spans: &mut Vec<CodeBlock>) -> CodeBlock {
    if spans.len() == 1 {
        return spans.remove(0);
    }

    let mut ops = Vec::<Operation>::new();
    let mut decorators = DecoratorList::new();
    spans.drain(0..).for_each(|block| {
        if let CodeBlock::Span(span) = block {
            for decorator in span.decorators() {
                decorators.push((decorator.0 + ops.len(), decorator.1.clone()));
            }
            for batch in span.op_batches() {
                ops.extend_from_slice(batch.ops());
            }
        } else {
            panic!(
                "Codeblock was expected to be a Span Block, got {:?}.",
                block
            );
        }
    });
    CodeBlock::new_span_with_decorators(ops, decorators)
}
