use super::{Assembler, AssemblerContext, SpanBuilder};
use crate::{parsers::Instruction, AssemblerError};
use vm_core::{code_blocks::CodeBlock, CodeBlockTable, Operation};

mod field_ops;
mod procedures;

impl Assembler {
    pub(super) fn compile_instruction(
        &self,
        instruction: &Instruction,
        span: &mut SpanBuilder,
        context: &AssemblerContext,
        cb_table: &mut CodeBlockTable,
    ) -> Result<Option<CodeBlock>, AssemblerError> {
        use Operation::*;

        match instruction {
            Instruction::Assert => span.add_op(Assert),
            Instruction::AssertEq => span.add_ops([Eq, Assert]),
            Instruction::Assertz => span.add_ops([Eqz, Assert]),
            Instruction::Add => span.add_op(Add),
            Instruction::AddImm(imm) => field_ops::add_imm(imm, span),
            Instruction::Sub => span.add_ops([Neg, Add]),
            Instruction::SubImm(imm) => span.add_ops([Push(-*imm), Add]),
            Instruction::PushConstants(imms) => span.add_ops(imms.iter().copied().map(Push)),
            Instruction::ExecLocal(idx) => self.exec_local(*idx, context),
            Instruction::ExecImported(id) => self.exec_imported(id, cb_table),
            Instruction::CallLocal(idx) => self.call_local(*idx, context, cb_table),
            Instruction::CallImported(id) => self.call_imported(id, cb_table),
            _ => todo!(),
        }
    }
}
