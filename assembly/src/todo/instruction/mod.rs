use super::{
    Assembler, AssemblerError, CallSet, CodeBlock, Felt, ModuleContext, Operation, ProcedureId,
    SpanBuilder,
};
use crate::parsers::Instruction;

mod crypto_ops;
mod field_ops;
mod procedures;
mod u32_ops;

use u32_ops::U32OpMode::*;

// INSTRUCTION HANDLERS
// ================================================================================================

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

            Instruction::U32Test => span.add_ops([Dup0, U32split, Swap, Drop, Eqz]),
            Instruction::U32TestW => u32_ops::u32testw(span),
            Instruction::U32Assert => span.add_ops([Pad, U32assert2, Drop]),
            Instruction::U32Assert2 => span.add_op(U32assert2),
            Instruction::U32AssertW => u32_ops::u32assertw(span),
            Instruction::U32Cast => span.add_ops([U32split, Drop]),
            Instruction::U32Split => span.add_op(U32split),

            Instruction::U32CheckedAdd => u32_ops::u32add(span, Checked, None),
            Instruction::U32CheckedAddImm(v) => u32_ops::u32add(span, Checked, Some(*v)),
            Instruction::U32OverflowingAdd => u32_ops::u32add(span, Overflowing, None),
            Instruction::U32OverflowingAddImm(v) => u32_ops::u32add(span, Overflowing, Some(*v)),
            Instruction::U32WrappingAdd => u32_ops::u32add(span, Wrapping, None),
            Instruction::U32WrappingAddImm(v) => u32_ops::u32add(span, Wrapping, Some(*v)),
            Instruction::U32OverflowingAdd3 => span.add_op(U32add3),
            Instruction::U32WrappingAdd3 => span.add_ops([U32add3, Drop]),

            Instruction::U32CheckedSub => u32_ops::u32sub(span, Checked, None),
            Instruction::U32CheckedSubImm(v) => u32_ops::u32sub(span, Checked, Some(*v)),
            Instruction::U32OverflowingSub => u32_ops::u32sub(span, Overflowing, None),
            Instruction::U32OverflowingSubImm(v) => u32_ops::u32sub(span, Overflowing, Some(*v)),
            Instruction::U32WrappingSub => u32_ops::u32sub(span, Wrapping, None),
            Instruction::U32WrappingSubImm(v) => u32_ops::u32sub(span, Wrapping, Some(*v)),

            Instruction::U32CheckedMul => u32_ops::u32mul(span, Checked, None),
            Instruction::U32CheckedMulImm(v) => u32_ops::u32mul(span, Checked, Some(*v)),
            Instruction::U32OverflowingMul => u32_ops::u32mul(span, Overflowing, None),
            Instruction::U32OverflowingMulImm(v) => u32_ops::u32mul(span, Overflowing, Some(*v)),
            Instruction::U32WrappingMul => u32_ops::u32mul(span, Wrapping, None),
            Instruction::U32WrappingMulImm(v) => u32_ops::u32mul(span, Wrapping, Some(*v)),
            Instruction::U32OverflowingMadd => span.add_op(U32madd),
            Instruction::U32WrappingMadd => span.add_ops([U32madd, Drop]),

            Instruction::U32CheckedDiv => u32_ops::u32div(span, Checked, None),
            Instruction::U32CheckedDivImm(v) => u32_ops::u32div(span, Checked, Some(*v)),
            Instruction::U32UncheckedDiv => u32_ops::u32div(span, Unchecked, None),
            Instruction::U32UncheckedDivImm(v) => u32_ops::u32div(span, Unchecked, Some(*v)),
            Instruction::U32CheckedMod => u32_ops::u32mod(span, Checked, None),
            Instruction::U32CheckedModImm(v) => u32_ops::u32mod(span, Checked, Some(*v)),
            Instruction::U32UncheckedMod => u32_ops::u32mod(span, Unchecked, None),
            Instruction::U32UncheckedModImm(v) => u32_ops::u32mod(span, Unchecked, Some(*v)),
            Instruction::U32CheckedDivMod => u32_ops::u32divmod(span, Checked, None),
            Instruction::U32CheckedDivModImm(v) => u32_ops::u32divmod(span, Checked, Some(*v)),
            Instruction::U32UncheckedDivMod => u32_ops::u32divmod(span, Unchecked, None),
            Instruction::U32UncheckedDivModImm(v) => u32_ops::u32divmod(span, Unchecked, Some(*v)),

            Instruction::U32CheckedAnd => span.add_op(U32and),
            Instruction::U32CheckedOr => span.add_ops([Dup1, Dup1, U32and, Neg, Add, Add]),
            Instruction::U32CheckedXor => span.add_op(U32xor),
            Instruction::U32CheckedNot => u32_ops::u32not(span),
            Instruction::U32CheckedShl => u32_ops::u32shl(span, Checked, None),
            Instruction::U32CheckedShlImm(v) => u32_ops::u32shl(span, Checked, Some(*v)),
            Instruction::U32UncheckedShl => u32_ops::u32shl(span, Unchecked, None),
            Instruction::U32UncheckedShlImm(v) => u32_ops::u32shl(span, Unchecked, Some(*v)),
            Instruction::U32CheckedShr => u32_ops::u32shr(span, Checked, None),
            Instruction::U32CheckedShrImm(v) => u32_ops::u32shr(span, Checked, Some(*v)),
            Instruction::U32UncheckedShr => u32_ops::u32shr(span, Unchecked, None),
            Instruction::U32UncheckedShrImm(v) => u32_ops::u32shr(span, Unchecked, Some(*v)),
            Instruction::U32CheckedRotl => u32_ops::u32rotl(span, Checked, None),
            Instruction::U32CheckedRotlImm(v) => u32_ops::u32rotl(span, Checked, Some(*v)),
            Instruction::U32UncheckedRotl => u32_ops::u32rotl(span, Unchecked, None),
            Instruction::U32UncheckedRotlImm(v) => u32_ops::u32rotl(span, Unchecked, Some(*v)),
            Instruction::U32CheckedRotr => u32_ops::u32rotr(span, Checked, None),
            Instruction::U32CheckedRotrImm(v) => u32_ops::u32rotr(span, Checked, Some(*v)),
            Instruction::U32UncheckedRotr => u32_ops::u32rotr(span, Unchecked, None),
            Instruction::U32UncheckedRotrImm(v) => u32_ops::u32rotr(span, Unchecked, Some(*v)),

            Instruction::RPPerm => span.add_op(RpPerm),
            Instruction::RPHash => crypto_ops::rphash(span),
            Instruction::MTreeGet => crypto_ops::mtree_get(span),
            Instruction::MTreeSet => crypto_ops::mtree_set(span),
            Instruction::MTreeCwm => crypto_ops::mtree_cwm(span),

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
