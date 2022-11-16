use super::{
    Assembler, AssemblerContext, AssemblerError, CallSet, CodeBlock, Operation, ProcedureId,
    SpanBuilder,
};
use crate::parsers::Instruction;

mod field_ops;
mod procedures;

impl Assembler {
    pub(super) fn compile_instruction(
        &self,
        instruction: &Instruction,
        span: &mut SpanBuilder,
        context: &AssemblerContext,
        callset: &mut CallSet,
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
            Instruction::ExecLocal(idx) => self.exec_local(*idx, context, callset),
            Instruction::ExecImported(id) => self.exec_imported(id, callset),
            Instruction::CallLocal(idx) => self.call_local(*idx, context, callset),
            Instruction::CallImported(id) => self.call_imported(id, callset),
            _ => todo!(),
        }
    }
}
