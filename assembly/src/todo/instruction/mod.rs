use super::{
    Assembler, AssemblerError, CallSet, CodeBlock, ModuleContext, Operation, ProcedureId,
    SpanBuilder,
};
use crate::parsers::Instruction;

mod crypto_ops;
mod field_ops;
mod procedures;

impl Assembler {
    pub(super) fn compile_instruction(
        &self,
        instruction: &Instruction,
        span: &mut SpanBuilder,
        context: &ModuleContext,
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
            Instruction::Mul => span.add_op(Mul),
            Instruction::MulImm(imm) => field_ops::mul_imm(imm, span),
            Instruction::Div => span.add_ops([Inv, Mul]),
            Instruction::DivImm(imm) => field_ops::div_imm(imm, span),
            Instruction::Neg => span.add_op(Neg),
            Instruction::Inv => span.add_op(Inv),

            Instruction::Pow2 => field_ops::pow2(span),
            Instruction::Exp => field_ops::exp_imm(&64u64.into(), span),
            Instruction::ExpImm(imm) => field_ops::exp_imm(imm, span),
            Instruction::ExpBitLength(bits) => field_ops::exp_bits(bits, span),

            Instruction::Not => span.add_op(Not),
            Instruction::And => span.add_op(And),
            Instruction::Or => span.add_op(Or),
            Instruction::Xor => span.add_ops([Dup0, Dup2, Or, MovDn2, And, Not, And]),

            Instruction::Eq => span.add_op(Eq),
            Instruction::EqImm(imm) => field_ops::eq_imm(imm, span),
            Instruction::Eqw => field_ops::eqw(span),
            Instruction::Neq => span.add_ops([Eq, Not]),
            Instruction::NeqImm(imm) => field_ops::neq_imm(imm, span),

            Instruction::MTreeGet => crypto_ops::mtree_get(span),

            Instruction::PushConstants(imms) => span.add_ops(imms.iter().copied().map(Push)),
            Instruction::ExecLocal(idx) => self.exec_local(*idx, context, callset),
            Instruction::ExecImported(id) => self.exec_imported(id, callset),
            Instruction::CallLocal(idx) => self.call_local(*idx, context, callset),
            Instruction::CallImported(id) => self.call_imported(id, context, callset),
            Instruction::SysCall(id) => self.syscall(id, context, callset),

            _ => todo!(),
        }
    }
}
