use super::{
    ast::InvokeKind, Assembler, AssemblyContext, CodeBlock, Felt, Instruction, Operation,
    SpanBuilder, ONE, ZERO,
};
use crate::{diagnostics::Report, utils::bound_into_included_u64, AssemblyError};
use core::ops::RangeBounds;
use vm_core::Decorator;

mod adv_ops;
mod crypto_ops;
mod env_ops;
mod ext2_ops;
mod field_ops;
mod mem_ops;
mod procedures;
mod u32_ops;

use self::u32_ops::U32OpMode::*;

/// Instruction Compilation
impl Assembler {
    pub(super) fn compile_instruction(
        &self,
        instruction: &Instruction,
        span_builder: &mut SpanBuilder,
        ctx: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        // if the assembler is in debug mode, start tracking the instruction about to be executed;
        // this will allow us to map the instruction to the sequence of operations which were
        // executed as a part of this instruction.
        if self.in_debug_mode() {
            span_builder.track_instruction(instruction, ctx);
        }

        let result = self.compile_instruction_impl(instruction, span_builder, ctx)?;

        // compute and update the cycle count of the instruction which just finished executing
        if self.in_debug_mode() {
            span_builder.set_instruction_cycle_count();
        }

        Ok(result)
    }

    fn compile_instruction_impl(
        &self,
        instruction: &Instruction,
        span_builder: &mut SpanBuilder,
        ctx: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        use Operation::*;

        match instruction {
            Instruction::Assert => span_builder.add_op(Assert(0)),
            Instruction::AssertWithError(err_code) => {
                span_builder.add_op(Assert(err_code.expect_value()))
            }
            Instruction::AssertEq => span_builder.add_ops([Eq, Assert(0)]),
            Instruction::AssertEqWithError(err_code) => {
                span_builder.add_ops([Eq, Assert(err_code.expect_value())])
            }
            Instruction::AssertEqw => field_ops::assertw(span_builder, 0),
            Instruction::AssertEqwWithError(err_code) => {
                field_ops::assertw(span_builder, err_code.expect_value())
            }
            Instruction::Assertz => span_builder.add_ops([Eqz, Assert(0)]),
            Instruction::AssertzWithError(err_code) => {
                span_builder.add_ops([Eqz, Assert(err_code.expect_value())])
            }

            Instruction::Add => span_builder.add_op(Add),
            Instruction::AddImm(imm) => field_ops::add_imm(span_builder, imm.expect_value()),
            Instruction::Sub => span_builder.add_ops([Neg, Add]),
            Instruction::SubImm(imm) => field_ops::sub_imm(span_builder, imm.expect_value()),
            Instruction::Mul => span_builder.add_op(Mul),
            Instruction::MulImm(imm) => field_ops::mul_imm(span_builder, imm.expect_value()),
            Instruction::Div => span_builder.add_ops([Inv, Mul]),
            Instruction::DivImm(imm) => {
                field_ops::div_imm(span_builder, ctx, imm.expect_spanned_value())?;
            }
            Instruction::Neg => span_builder.add_op(Neg),
            Instruction::Inv => span_builder.add_op(Inv),
            Instruction::Incr => span_builder.add_op(Incr),

            Instruction::Pow2 => field_ops::pow2(span_builder),
            Instruction::Exp => field_ops::exp(span_builder, 64)?,
            Instruction::ExpImm(pow) => field_ops::exp_imm(span_builder, pow.expect_value())?,
            Instruction::ExpBitLength(num_pow_bits) => field_ops::exp(span_builder, *num_pow_bits)?,
            Instruction::ILog2 => field_ops::ilog2(span_builder),

            Instruction::Not => span_builder.add_op(Not),
            Instruction::And => span_builder.add_op(And),
            Instruction::Or => span_builder.add_op(Or),
            Instruction::Xor => span_builder.add_ops([Dup0, Dup2, Or, MovDn2, And, Not, And]),

            Instruction::Eq => span_builder.add_op(Eq),
            Instruction::EqImm(imm) => field_ops::eq_imm(span_builder, imm.expect_value()),
            Instruction::Eqw => field_ops::eqw(span_builder),
            Instruction::Neq => span_builder.add_ops([Eq, Not]),
            Instruction::NeqImm(imm) => field_ops::neq_imm(span_builder, imm.expect_value()),
            Instruction::Lt => field_ops::lt(span_builder),
            Instruction::Lte => field_ops::lte(span_builder),
            Instruction::Gt => field_ops::gt(span_builder),
            Instruction::Gte => field_ops::gte(span_builder),
            Instruction::IsOdd => field_ops::is_odd(span_builder),

            // ----- ext2 instructions ------------------------------------------------------------
            Instruction::Ext2Add => ext2_ops::ext2_add(span_builder),
            Instruction::Ext2Sub => ext2_ops::ext2_sub(span_builder),
            Instruction::Ext2Mul => ext2_ops::ext2_mul(span_builder),
            Instruction::Ext2Div => ext2_ops::ext2_div(span_builder),
            Instruction::Ext2Neg => ext2_ops::ext2_neg(span_builder),
            Instruction::Ext2Inv => ext2_ops::ext2_inv(span_builder),

            // ----- u32 manipulation -------------------------------------------------------------
            Instruction::U32Test => span_builder.add_ops([Dup0, U32split, Swap, Drop, Eqz]),
            Instruction::U32TestW => u32_ops::u32testw(span_builder),
            Instruction::U32Assert => span_builder.add_ops([Pad, U32assert2(ZERO), Drop]),
            Instruction::U32AssertWithError(err_code) => {
                span_builder.add_ops([Pad, U32assert2(Felt::from(err_code.expect_value())), Drop])
            }
            Instruction::U32Assert2 => span_builder.add_op(U32assert2(ZERO)),
            Instruction::U32Assert2WithError(err_code) => {
                span_builder.add_op(U32assert2(Felt::from(err_code.expect_value())))
            }
            Instruction::U32AssertW => u32_ops::u32assertw(span_builder, ZERO),
            Instruction::U32AssertWWithError(err_code) => {
                u32_ops::u32assertw(span_builder, Felt::from(err_code.expect_value()))
            }

            Instruction::U32Cast => span_builder.add_ops([U32split, Drop]),
            Instruction::U32Split => span_builder.add_op(U32split),

            Instruction::U32OverflowingAdd => u32_ops::u32add(span_builder, Overflowing, None),
            Instruction::U32OverflowingAddImm(v) => {
                u32_ops::u32add(span_builder, Overflowing, Some(v.expect_value()))
            }
            Instruction::U32WrappingAdd => u32_ops::u32add(span_builder, Wrapping, None),
            Instruction::U32WrappingAddImm(v) => {
                u32_ops::u32add(span_builder, Wrapping, Some(v.expect_value()))
            }
            Instruction::U32OverflowingAdd3 => span_builder.add_op(U32add3),
            Instruction::U32WrappingAdd3 => span_builder.add_ops([U32add3, Drop]),

            Instruction::U32OverflowingSub => u32_ops::u32sub(span_builder, Overflowing, None),
            Instruction::U32OverflowingSubImm(v) => {
                u32_ops::u32sub(span_builder, Overflowing, Some(v.expect_value()))
            }
            Instruction::U32WrappingSub => u32_ops::u32sub(span_builder, Wrapping, None),
            Instruction::U32WrappingSubImm(v) => {
                u32_ops::u32sub(span_builder, Wrapping, Some(v.expect_value()))
            }

            Instruction::U32OverflowingMul => u32_ops::u32mul(span_builder, Overflowing, None),
            Instruction::U32OverflowingMulImm(v) => {
                u32_ops::u32mul(span_builder, Overflowing, Some(v.expect_value()))
            }
            Instruction::U32WrappingMul => u32_ops::u32mul(span_builder, Wrapping, None),
            Instruction::U32WrappingMulImm(v) => {
                u32_ops::u32mul(span_builder, Wrapping, Some(v.expect_value()))
            }
            Instruction::U32OverflowingMadd => span_builder.add_op(U32madd),
            Instruction::U32WrappingMadd => span_builder.add_ops([U32madd, Drop]),

            Instruction::U32Div => u32_ops::u32div(span_builder, ctx, None)?,
            Instruction::U32DivImm(v) => {
                u32_ops::u32div(span_builder, ctx, Some(v.expect_spanned_value()))?
            }
            Instruction::U32Mod => u32_ops::u32mod(span_builder, ctx, None)?,
            Instruction::U32ModImm(v) => {
                u32_ops::u32mod(span_builder, ctx, Some(v.expect_spanned_value()))?
            }
            Instruction::U32DivMod => u32_ops::u32divmod(span_builder, ctx, None)?,
            Instruction::U32DivModImm(v) => {
                u32_ops::u32divmod(span_builder, ctx, Some(v.expect_spanned_value()))?
            }
            Instruction::U32And => span_builder.add_op(U32and),
            Instruction::U32Or => span_builder.add_ops([Dup1, Dup1, U32and, Neg, Add, Add]),
            Instruction::U32Xor => span_builder.add_op(U32xor),
            Instruction::U32Not => u32_ops::u32not(span_builder),
            Instruction::U32Shl => u32_ops::u32shl(span_builder, None)?,
            Instruction::U32ShlImm(v) => u32_ops::u32shl(span_builder, Some(v.expect_value()))?,
            Instruction::U32Shr => u32_ops::u32shr(span_builder, None)?,
            Instruction::U32ShrImm(v) => u32_ops::u32shr(span_builder, Some(v.expect_value()))?,
            Instruction::U32Rotl => u32_ops::u32rotl(span_builder, None)?,
            Instruction::U32RotlImm(v) => u32_ops::u32rotl(span_builder, Some(v.expect_value()))?,
            Instruction::U32Rotr => u32_ops::u32rotr(span_builder, None)?,
            Instruction::U32RotrImm(v) => u32_ops::u32rotr(span_builder, Some(v.expect_value()))?,
            Instruction::U32Popcnt => u32_ops::u32popcnt(span_builder),
            Instruction::U32Clz => u32_ops::u32clz(span_builder),
            Instruction::U32Ctz => u32_ops::u32ctz(span_builder),
            Instruction::U32Clo => u32_ops::u32clo(span_builder),
            Instruction::U32Cto => u32_ops::u32cto(span_builder),
            Instruction::U32Lt => u32_ops::u32lt(span_builder),
            Instruction::U32Lte => u32_ops::u32lte(span_builder),
            Instruction::U32Gt => u32_ops::u32gt(span_builder),
            Instruction::U32Gte => u32_ops::u32gte(span_builder),
            Instruction::U32Min => u32_ops::u32min(span_builder),
            Instruction::U32Max => u32_ops::u32max(span_builder),

            // ----- stack manipulation -----------------------------------------------------------
            Instruction::Drop => span_builder.add_op(Drop),
            Instruction::DropW => span_builder.add_ops([Drop; 4]),
            Instruction::PadW => span_builder.add_ops([Pad; 4]),
            Instruction::Dup0 => span_builder.add_op(Dup0),
            Instruction::Dup1 => span_builder.add_op(Dup1),
            Instruction::Dup2 => span_builder.add_op(Dup2),
            Instruction::Dup3 => span_builder.add_op(Dup3),
            Instruction::Dup4 => span_builder.add_op(Dup4),
            Instruction::Dup5 => span_builder.add_op(Dup5),
            Instruction::Dup6 => span_builder.add_op(Dup6),
            Instruction::Dup7 => span_builder.add_op(Dup7),
            Instruction::Dup8 => span_builder.add_ops([Pad, Dup9, Add]),
            Instruction::Dup9 => span_builder.add_op(Dup9),
            Instruction::Dup10 => span_builder.add_ops([Pad, Dup11, Add]),
            Instruction::Dup11 => span_builder.add_op(Dup11),
            Instruction::Dup12 => span_builder.add_ops([Pad, Dup13, Add]),
            Instruction::Dup13 => span_builder.add_op(Dup13),
            Instruction::Dup14 => span_builder.add_ops([Pad, Dup15, Add]),
            Instruction::Dup15 => span_builder.add_op(Dup15),
            Instruction::DupW0 => span_builder.add_ops([Dup3; 4]),
            Instruction::DupW1 => span_builder.add_ops([Dup7; 4]),
            Instruction::DupW2 => span_builder.add_ops([Dup11; 4]),
            Instruction::DupW3 => span_builder.add_ops([Dup15; 4]),
            Instruction::Swap1 => span_builder.add_op(Swap),
            Instruction::Swap2 => span_builder.add_ops([Swap, MovUp2]),
            Instruction::Swap3 => span_builder.add_ops([MovDn2, MovUp3]),
            Instruction::Swap4 => span_builder.add_ops([MovDn3, MovUp4]),
            Instruction::Swap5 => span_builder.add_ops([MovDn4, MovUp5]),
            Instruction::Swap6 => span_builder.add_ops([MovDn5, MovUp6]),
            Instruction::Swap7 => span_builder.add_ops([MovDn6, MovUp7]),
            Instruction::Swap8 => span_builder.add_ops([MovDn7, MovUp8]),
            Instruction::Swap9 => span_builder.add_ops([MovDn8, SwapDW, Swap, SwapDW, MovUp8]),
            Instruction::Swap10 => {
                span_builder.add_ops([MovDn8, SwapDW, Swap, MovUp2, SwapDW, MovUp8])
            }
            Instruction::Swap11 => {
                span_builder.add_ops([MovDn8, SwapDW, MovDn2, MovUp3, SwapDW, MovUp8])
            }
            Instruction::Swap12 => {
                span_builder.add_ops([MovDn8, SwapDW, MovDn3, MovUp4, SwapDW, MovUp8])
            }
            Instruction::Swap13 => {
                span_builder.add_ops([MovDn8, SwapDW, MovDn4, MovUp5, SwapDW, MovUp8])
            }
            Instruction::Swap14 => {
                span_builder.add_ops([MovDn8, SwapDW, MovDn5, MovUp6, SwapDW, MovUp8])
            }
            Instruction::Swap15 => {
                span_builder.add_ops([MovDn8, SwapDW, MovDn6, MovUp7, SwapDW, MovUp8])
            }
            Instruction::SwapW1 => span_builder.add_op(SwapW),
            Instruction::SwapW2 => span_builder.add_op(SwapW2),
            Instruction::SwapW3 => span_builder.add_op(SwapW3),
            Instruction::SwapDw => span_builder.add_op(SwapDW),
            Instruction::MovUp2 => span_builder.add_op(MovUp2),
            Instruction::MovUp3 => span_builder.add_op(MovUp3),
            Instruction::MovUp4 => span_builder.add_op(MovUp4),
            Instruction::MovUp5 => span_builder.add_op(MovUp5),
            Instruction::MovUp6 => span_builder.add_op(MovUp6),
            Instruction::MovUp7 => span_builder.add_op(MovUp7),
            Instruction::MovUp8 => span_builder.add_op(MovUp8),
            Instruction::MovUp9 => span_builder.add_ops([SwapDW, Swap, SwapDW, MovUp8]),
            Instruction::MovUp10 => span_builder.add_ops([SwapDW, MovUp2, SwapDW, MovUp8]),
            Instruction::MovUp11 => span_builder.add_ops([SwapDW, MovUp3, SwapDW, MovUp8]),
            Instruction::MovUp12 => span_builder.add_ops([SwapDW, MovUp4, SwapDW, MovUp8]),
            Instruction::MovUp13 => span_builder.add_ops([SwapDW, MovUp5, SwapDW, MovUp8]),
            Instruction::MovUp14 => span_builder.add_ops([SwapDW, MovUp6, SwapDW, MovUp8]),
            Instruction::MovUp15 => span_builder.add_ops([SwapDW, MovUp7, SwapDW, MovUp8]),
            Instruction::MovUpW2 => span_builder.add_ops([SwapW, SwapW2]),
            Instruction::MovUpW3 => span_builder.add_ops([SwapW, SwapW2, SwapW3]),
            Instruction::MovDn2 => span_builder.add_op(MovDn2),
            Instruction::MovDn3 => span_builder.add_op(MovDn3),
            Instruction::MovDn4 => span_builder.add_op(MovDn4),
            Instruction::MovDn5 => span_builder.add_op(MovDn5),
            Instruction::MovDn6 => span_builder.add_op(MovDn6),
            Instruction::MovDn7 => span_builder.add_op(MovDn7),
            Instruction::MovDn8 => span_builder.add_op(MovDn8),
            Instruction::MovDn9 => span_builder.add_ops([MovDn8, SwapDW, Swap, SwapDW]),
            Instruction::MovDn10 => span_builder.add_ops([MovDn8, SwapDW, MovDn2, SwapDW]),
            Instruction::MovDn11 => span_builder.add_ops([MovDn8, SwapDW, MovDn3, SwapDW]),
            Instruction::MovDn12 => span_builder.add_ops([MovDn8, SwapDW, MovDn4, SwapDW]),
            Instruction::MovDn13 => span_builder.add_ops([MovDn8, SwapDW, MovDn5, SwapDW]),
            Instruction::MovDn14 => span_builder.add_ops([MovDn8, SwapDW, MovDn6, SwapDW]),
            Instruction::MovDn15 => span_builder.add_ops([MovDn8, SwapDW, MovDn7, SwapDW]),
            Instruction::MovDnW2 => span_builder.add_ops([SwapW2, SwapW]),
            Instruction::MovDnW3 => span_builder.add_ops([SwapW3, SwapW2, SwapW]),

            Instruction::CSwap => span_builder.add_op(CSwap),
            Instruction::CSwapW => span_builder.add_op(CSwapW),
            Instruction::CDrop => span_builder.add_ops([CSwap, Drop]),
            Instruction::CDropW => span_builder.add_ops([CSwapW, Drop, Drop, Drop, Drop]),

            // ----- input / output instructions --------------------------------------------------
            Instruction::Push(imm) => env_ops::push_one(imm.expect_value(), span_builder),
            Instruction::PushU8(imm) => env_ops::push_one(*imm, span_builder),
            Instruction::PushU16(imm) => env_ops::push_one(*imm, span_builder),
            Instruction::PushU32(imm) => env_ops::push_one(*imm, span_builder),
            Instruction::PushFelt(imm) => env_ops::push_one(*imm, span_builder),
            Instruction::PushWord(imms) => env_ops::push_many(imms, span_builder),
            Instruction::PushU8List(imms) => env_ops::push_many(imms, span_builder),
            Instruction::PushU16List(imms) => env_ops::push_many(imms, span_builder),
            Instruction::PushU32List(imms) => env_ops::push_many(imms, span_builder),
            Instruction::PushFeltList(imms) => env_ops::push_many(imms, span_builder),
            Instruction::Sdepth => span_builder.add_op(SDepth),
            Instruction::Caller => env_ops::caller(span_builder, ctx)?,
            Instruction::Clk => span_builder.add_op(Clk),
            Instruction::AdvPipe => span_builder.add_op(Pipe),
            Instruction::AdvPush(n) => adv_ops::adv_push(span_builder, n.expect_value())?,
            Instruction::AdvLoadW => span_builder.add_op(AdvPopW),

            Instruction::MemStream => span_builder.add_op(MStream),
            Instruction::Locaddr(v) => env_ops::locaddr(span_builder, v.expect_value(), ctx)?,
            Instruction::MemLoad => mem_ops::mem_read(span_builder, ctx, None, false, true)?,
            Instruction::MemLoadImm(v) => {
                mem_ops::mem_read(span_builder, ctx, Some(v.expect_value()), false, true)?
            }
            Instruction::MemLoadW => mem_ops::mem_read(span_builder, ctx, None, false, false)?,
            Instruction::MemLoadWImm(v) => {
                mem_ops::mem_read(span_builder, ctx, Some(v.expect_value()), false, false)?
            }
            Instruction::LocLoad(v) => {
                mem_ops::mem_read(span_builder, ctx, Some(v.expect_value() as u32), true, true)?
            }
            Instruction::LocLoadW(v) => {
                mem_ops::mem_read(span_builder, ctx, Some(v.expect_value() as u32), true, false)?
            }
            Instruction::MemStore => span_builder.add_ops([MStore, Drop]),
            Instruction::MemStoreW => span_builder.add_ops([MStoreW]),
            Instruction::MemStoreImm(v) => {
                mem_ops::mem_write_imm(span_builder, ctx, v.expect_value(), false, true)?
            }
            Instruction::MemStoreWImm(v) => {
                mem_ops::mem_write_imm(span_builder, ctx, v.expect_value(), false, false)?
            }
            Instruction::LocStore(v) => {
                mem_ops::mem_write_imm(span_builder, ctx, v.expect_value() as u32, true, true)?
            }
            Instruction::LocStoreW(v) => {
                mem_ops::mem_write_imm(span_builder, ctx, v.expect_value() as u32, true, false)?
            }

            Instruction::AdvInject(injector) => adv_ops::adv_inject(span_builder, injector),

            // ----- cryptographic instructions ---------------------------------------------------
            Instruction::Hash => crypto_ops::hash(span_builder),
            Instruction::HPerm => span_builder.add_op(HPerm),
            Instruction::HMerge => crypto_ops::hmerge(span_builder),
            Instruction::MTreeGet => crypto_ops::mtree_get(span_builder),
            Instruction::MTreeSet => crypto_ops::mtree_set(span_builder),
            Instruction::MTreeMerge => crypto_ops::mtree_merge(span_builder),
            Instruction::MTreeVerify => crypto_ops::mtree_verify(span_builder),

            // ----- STARK proof verification -----------------------------------------------------
            Instruction::FriExt2Fold4 => span_builder.add_op(FriE2F4),
            Instruction::RCombBase => span_builder.add_op(RCombBase),

            // ----- exec/call instructions -------------------------------------------------------
            Instruction::Exec(ref callee) => return self.invoke(InvokeKind::Exec, callee, ctx),
            Instruction::Call(ref callee) => return self.invoke(InvokeKind::Call, callee, ctx),
            Instruction::SysCall(ref callee) => {
                return self.invoke(InvokeKind::SysCall, callee, ctx)
            }
            Instruction::DynExec => return self.dynexec(),
            Instruction::DynCall => return self.dyncall(),
            Instruction::ProcRef(ref callee) => self.procref(callee, ctx, span_builder)?,

            // ----- debug decorators -------------------------------------------------------------
            Instruction::Breakpoint => {
                if self.in_debug_mode() {
                    span_builder.add_op(Noop);
                    span_builder.track_instruction(instruction, ctx);
                }
            }

            Instruction::Debug(options) => {
                if self.in_debug_mode() {
                    span_builder.push_decorator(Decorator::Debug(
                        options.clone().try_into().expect("unresolved constant"),
                    ))
                }
            }

            // ----- emit instruction -------------------------------------------------------------
            Instruction::Emit(event_id) => {
                span_builder.push_decorator(Decorator::Event(event_id.expect_value()));
            }

            // ----- trace instruction ------------------------------------------------------------
            Instruction::Trace(trace_id) => {
                span_builder.push_decorator(Decorator::Trace(trace_id.expect_value()));
            }
        }

        Ok(None)
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// This is a helper function that appends a PUSH operation to the span block which puts the
/// provided u32 value onto the stack.
///
/// When the value is 0, PUSH operation is replaced with PAD. When the value is 1, PUSH operation
/// is replaced with PAD INCR because in most cases this will be more efficient than doing a PUSH.
fn push_u32_value(span_builder: &mut SpanBuilder, value: u32) {
    use Operation::*;

    if value == 0 {
        span_builder.push_op(Pad);
    } else if value == 1 {
        span_builder.push_op(Pad);
        span_builder.push_op(Incr);
    } else {
        span_builder.push_op(Push(Felt::from(value)));
    }
}

/// This is a helper function that appends a PUSH operation to the span block which puts the
/// provided field element onto the stack.
///
/// When the value is 0, PUSH operation is replaced with PAD. When the value is 1, PUSH operation
/// is replaced with PAD INCR because in most cases this will be more efficient than doing a PUSH.
fn push_felt(span_builder: &mut SpanBuilder, value: Felt) {
    use Operation::*;

    if value == ZERO {
        span_builder.push_op(Pad);
    } else if value == ONE {
        span_builder.push_op(Pad);
        span_builder.push_op(Incr);
    } else {
        span_builder.push_op(Push(value));
    }
}

/// Returns an error if the specified value is smaller than or equal to min or greater than or
/// equal to max. Otherwise, returns Ok(()).
fn validate_param<I, R>(value: I, range: R) -> Result<(), AssemblyError>
where
    I: Ord + Clone + Into<u64>,
    R: RangeBounds<I>,
{
    range.contains(&value).then_some(()).ok_or_else(|| {
        let value: u64 = value.into();
        let min = bound_into_included_u64(range.start_bound(), true);
        let max = bound_into_included_u64(range.end_bound(), false);
        AssemblyError::Other(
            Report::msg(format!(
                "parameter value must be greater than or equal to {min} and \
            less than or equal to {max}, but was {value}",
            ))
            .into(),
        )
    })
}
