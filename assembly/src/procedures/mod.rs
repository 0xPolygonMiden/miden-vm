use super::{
    combine_blocks, parse_body, parsers::ast::nodes::Node, AssemblyContext, AssemblyError,
    CodeBlock, CodeBlockTable, ModuleMap, String, Vec,
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
    index: u32,
}

impl Procedure {
    pub fn new(
        label: String,
        is_export: bool,
        num_locals: u32,
        code_root: CodeBlock,
        index: u32,
    ) -> Self {
        Self {
            label,
            is_export,
            num_locals,
            code_root,
            index,
        }
    }
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

    /// Returns the index of this procedure.
    pub fn index(&self) -> u32 {
        self.index
    }
}

// HELPER FUNCTIONS
// ================================================================================================

pub fn parse_proc_blocks(
    nodes: &Vec<Node>,
    context: &AssemblyContext,
    cb_table: &mut CodeBlockTable,
    parsed_modules: &mut ModuleMap,
    num_proc_locals: u32,
) -> Result<CodeBlock, AssemblyError> {
    // parse the procedure body
    let body = parse_body(nodes, context, cb_table, parsed_modules, num_proc_locals)?;

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
