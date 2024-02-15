use super::{
    Assembler, AssemblyContext, AssemblyError, CodeBlock, Felt, Instruction, Operation,
    ProcedureId, RpoDigest, SpanBuilder, ONE, ZERO,
};
use crate::utils::bound_into_included_u64;
use core::ops::RangeBounds;
use vm_core::{Decorator, FieldElement};

mod adv_ops;
mod crypto_ops;
mod env_ops;
mod ext2_ops;
mod field_ops;
mod mem_ops;
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
        ctx: &mut AssemblyContext,
    ) -> Result<Option<CodeBlock>, AssemblyError> {
        use Operation::*;

        // if the assembler is in debug mode, start tracking the instruction about to be executed;
        // this will allow us to map the instruction to the sequence of operations which were
        // executed as a part of this instruction.
        if self.in_debug_mode() {
            span.track_instruction(instruction, ctx);
        }

        let result = match instruction {
            Instruction::Assert => span.add_op(Assert(0)),
            Instruction::AssertWithError(err_code) => span.add_op(Assert(*err_code)),
            Instruction::AssertEq => span.add_ops([Eq, Assert(0)]),
            Instruction::AssertEqWithError(err_code) => span.add_ops([Eq, Assert(*err_code)]),
            Instruction::AssertEqw => field_ops::assertw(span, 0),
            Instruction::AssertEqwWithError(err_code) => field_ops::assertw(span, *err_code),
            Instruction::Assertz => span.add_ops([Eqz, Assert(0)]),
            Instruction::AssertzWithError(err_code) => span.add_ops([Eqz, Assert(*err_code)]),

            Instruction::Add => span.add_op(Add),
            Instruction::AddImm(imm) => field_ops::add_imm(span, *imm),
            Instruction::Sub => span.add_ops([Neg, Add]),
            Instruction::SubImm(imm) => field_ops::sub_imm(span, *imm),
            Instruction::Mul => span.add_op(Mul),
            Instruction::MulImm(imm) => field_ops::mul_imm(span, *imm),
            Instruction::Div => span.add_ops([Inv, Mul]),
            Instruction::DivImm(imm) => field_ops::div_imm(span, *imm),
            Instruction::Neg => span.add_op(Neg),
            Instruction::Inv => span.add_op(Inv),
            Instruction::Incr => span.add_op(Incr),

            Instruction::Pow2 => field_ops::pow2(span),
            Instruction::Exp => field_ops::exp(span, 64),
            Instruction::ExpImm(pow) => field_ops::exp_imm(span, *pow),
            Instruction::ExpBitLength(num_pow_bits) => field_ops::exp(span, *num_pow_bits),

            Instruction::Not => span.add_op(Not),
            Instruction::And => span.add_op(And),
            Instruction::Or => span.add_op(Or),
            Instruction::Xor => span.add_ops([Dup0, Dup2, Or, MovDn2, And, Not, And]),

            Instruction::Eq => span.add_op(Eq),
            Instruction::EqImm(imm) => field_ops::eq_imm(span, *imm),
            Instruction::Eqw => field_ops::eqw(span),
            Instruction::Neq => span.add_ops([Eq, Not]),
            Instruction::NeqImm(imm) => field_ops::neq_imm(span, *imm),
            Instruction::Lt => field_ops::lt(span),
            Instruction::Lte => field_ops::lte(span),
            Instruction::Gt => field_ops::gt(span),
            Instruction::Gte => field_ops::gte(span),
            Instruction::IsOdd => field_ops::is_odd(span),

            // ----- ext2 instructions ------------------------------------------------------------
            Instruction::Ext2Add => ext2_ops::ext2_add(span),
            Instruction::Ext2Sub => ext2_ops::ext2_sub(span),
            Instruction::Ext2Mul => ext2_ops::ext2_mul(span),
            Instruction::Ext2Div => ext2_ops::ext2_div(span),
            Instruction::Ext2Neg => ext2_ops::ext2_neg(span),
            Instruction::Ext2Inv => ext2_ops::ext2_inv(span),

            // ----- u32 manipulation -------------------------------------------------------------
            Instruction::U32Test => span.add_ops([Dup0, U32split, Swap, Drop, Eqz]),
            Instruction::U32TestW => u32_ops::u32testw(span),
            Instruction::U32Assert => span.add_ops([Pad, U32assert2(ZERO), Drop]),
            Instruction::U32AssertWithError(err_code) => {
                span.add_ops([Pad, U32assert2(Felt::from(*err_code)), Drop])
            }
            Instruction::U32Assert2 => span.add_op(U32assert2(ZERO)),
            Instruction::U32Assert2WithError(err_code) => {
                span.add_op(U32assert2(Felt::from(*err_code)))
            }
            Instruction::U32AssertW => u32_ops::u32assertw(span, ZERO),
            Instruction::U32AssertWWithError(err_code) => {
                u32_ops::u32assertw(span, Felt::from(*err_code))
            }

            Instruction::U32Cast => span.add_ops([U32split, Drop]),
            Instruction::U32Split => span.add_op(U32split),

            Instruction::U32OverflowingAdd => u32_ops::u32add(span, Overflowing, None),
            Instruction::U32OverflowingAddImm(v) => u32_ops::u32add(span, Overflowing, Some(*v)),
            Instruction::U32WrappingAdd => u32_ops::u32add(span, Wrapping, None),
            Instruction::U32WrappingAddImm(v) => u32_ops::u32add(span, Wrapping, Some(*v)),
            Instruction::U32OverflowingAdd3 => span.add_op(U32add3),
            Instruction::U32WrappingAdd3 => span.add_ops([U32add3, Drop]),

            Instruction::U32OverflowingSub => u32_ops::u32sub(span, Overflowing, None),
            Instruction::U32OverflowingSubImm(v) => u32_ops::u32sub(span, Overflowing, Some(*v)),
            Instruction::U32WrappingSub => u32_ops::u32sub(span, Wrapping, None),
            Instruction::U32WrappingSubImm(v) => u32_ops::u32sub(span, Wrapping, Some(*v)),

            Instruction::U32OverflowingMul => u32_ops::u32mul(span, Overflowing, None),
            Instruction::U32OverflowingMulImm(v) => u32_ops::u32mul(span, Overflowing, Some(*v)),
            Instruction::U32WrappingMul => u32_ops::u32mul(span, Wrapping, None),
            Instruction::U32WrappingMulImm(v) => u32_ops::u32mul(span, Wrapping, Some(*v)),
            Instruction::U32OverflowingMadd => span.add_op(U32madd),
            Instruction::U32WrappingMadd => span.add_ops([U32madd, Drop]),

            Instruction::U32Div => u32_ops::u32div(span, None),
            Instruction::U32DivImm(v) => u32_ops::u32div(span, Some(*v)),
            Instruction::U32Mod => u32_ops::u32mod(span, None),
            Instruction::U32ModImm(v) => u32_ops::u32mod(span, Some(*v)),
            Instruction::U32DivMod => u32_ops::u32divmod(span, None),
            Instruction::U32DivModImm(v) => u32_ops::u32divmod(span, Some(*v)),

            Instruction::U32And => span.add_op(U32and),
            Instruction::U32Or => span.add_ops([Dup1, Dup1, U32and, Neg, Add, Add]),
            Instruction::U32Xor => span.add_op(U32xor),
            Instruction::U32Not => u32_ops::u32not(span),
            Instruction::U32Shl => u32_ops::u32shl(span, None),
            Instruction::U32ShlImm(v) => u32_ops::u32shl(span, Some(*v)),
            Instruction::U32Shr => u32_ops::u32shr(span, None),
            Instruction::U32ShrImm(v) => u32_ops::u32shr(span, Some(*v)),
            Instruction::U32Rotl => u32_ops::u32rotl(span, None),
            Instruction::U32RotlImm(v) => u32_ops::u32rotl(span, Some(*v)),
            Instruction::U32Rotr => u32_ops::u32rotr(span, None),
            Instruction::U32RotrImm(v) => u32_ops::u32rotr(span, Some(*v)),
            Instruction::U32Popcnt => u32_ops::u32popcnt(span),

            Instruction::U32Lt => u32_ops::u32lt(span),
            Instruction::U32Lte => u32_ops::u32lte(span),
            Instruction::U32Gt => u32_ops::u32gt(span),
            Instruction::U32Gte => u32_ops::u32gte(span),
            Instruction::U32Min => u32_ops::u32min(span),
            Instruction::U32Max => u32_ops::u32max(span),

            // ----- stack manipulation -----------------------------------------------------------
            Instruction::Drop => span.add_op(Drop),
            Instruction::DropW => span.add_ops([Drop; 4]),
            Instruction::PadW => span.add_ops([Pad; 4]),
            Instruction::Dup0 => span.add_op(Dup0),
            Instruction::Dup1 => span.add_op(Dup1),
            Instruction::Dup2 => span.add_op(Dup2),
            Instruction::Dup3 => span.add_op(Dup3),
            Instruction::Dup4 => span.add_op(Dup4),
            Instruction::Dup5 => span.add_op(Dup5),
            Instruction::Dup6 => span.add_op(Dup6),
            Instruction::Dup7 => span.add_op(Dup7),
            Instruction::Dup8 => span.add_ops([Pad, Dup9, Add]),
            Instruction::Dup9 => span.add_op(Dup9),
            Instruction::Dup10 => span.add_ops([Pad, Dup11, Add]),
            Instruction::Dup11 => span.add_op(Dup11),
            Instruction::Dup12 => span.add_ops([Pad, Dup13, Add]),
            Instruction::Dup13 => span.add_op(Dup13),
            Instruction::Dup14 => span.add_ops([Pad, Dup15, Add]),
            Instruction::Dup15 => span.add_op(Dup15),
            Instruction::DupW0 => span.add_ops([Dup3; 4]),
            Instruction::DupW1 => span.add_ops([Dup7; 4]),
            Instruction::DupW2 => span.add_ops([Dup11; 4]),
            Instruction::DupW3 => span.add_ops([Dup15; 4]),
            Instruction::Swap1 => span.add_op(Swap),
            Instruction::Swap2 => span.add_ops([Swap, MovUp2]),
            Instruction::Swap3 => span.add_ops([MovDn2, MovUp3]),
            Instruction::Swap4 => span.add_ops([MovDn3, MovUp4]),
            Instruction::Swap5 => span.add_ops([MovDn4, MovUp5]),
            Instruction::Swap6 => span.add_ops([MovDn5, MovUp6]),
            Instruction::Swap7 => span.add_ops([MovDn6, MovUp7]),
            Instruction::Swap8 => span.add_ops([MovDn7, MovUp8]),
            Instruction::Swap9 => span.add_ops([MovDn8, SwapDW, Swap, SwapDW, MovUp8]),
            Instruction::Swap10 => span.add_ops([MovDn8, SwapDW, Swap, MovUp2, SwapDW, MovUp8]),
            Instruction::Swap11 => span.add_ops([MovDn8, SwapDW, MovDn2, MovUp3, SwapDW, MovUp8]),
            Instruction::Swap12 => span.add_ops([MovDn8, SwapDW, MovDn3, MovUp4, SwapDW, MovUp8]),
            Instruction::Swap13 => span.add_ops([MovDn8, SwapDW, MovDn4, MovUp5, SwapDW, MovUp8]),
            Instruction::Swap14 => span.add_ops([MovDn8, SwapDW, MovDn5, MovUp6, SwapDW, MovUp8]),
            Instruction::Swap15 => span.add_ops([MovDn8, SwapDW, MovDn6, MovUp7, SwapDW, MovUp8]),
            Instruction::SwapW1 => span.add_op(SwapW),
            Instruction::SwapW2 => span.add_op(SwapW2),
            Instruction::SwapW3 => span.add_op(SwapW3),
            Instruction::SwapDw => span.add_op(SwapDW),
            Instruction::MovUp2 => span.add_op(MovUp2),
            Instruction::MovUp3 => span.add_op(MovUp3),
            Instruction::MovUp4 => span.add_op(MovUp4),
            Instruction::MovUp5 => span.add_op(MovUp5),
            Instruction::MovUp6 => span.add_op(MovUp6),
            Instruction::MovUp7 => span.add_op(MovUp7),
            Instruction::MovUp8 => span.add_op(MovUp8),
            Instruction::MovUp9 => span.add_ops([SwapDW, Swap, SwapDW, MovUp8]),
            Instruction::MovUp10 => span.add_ops([SwapDW, MovUp2, SwapDW, MovUp8]),
            Instruction::MovUp11 => span.add_ops([SwapDW, MovUp3, SwapDW, MovUp8]),
            Instruction::MovUp12 => span.add_ops([SwapDW, MovUp4, SwapDW, MovUp8]),
            Instruction::MovUp13 => span.add_ops([SwapDW, MovUp5, SwapDW, MovUp8]),
            Instruction::MovUp14 => span.add_ops([SwapDW, MovUp6, SwapDW, MovUp8]),
            Instruction::MovUp15 => span.add_ops([SwapDW, MovUp7, SwapDW, MovUp8]),
            Instruction::MovUpW2 => span.add_ops([SwapW, SwapW2]),
            Instruction::MovUpW3 => span.add_ops([SwapW, SwapW2, SwapW3]),
            Instruction::MovDn2 => span.add_op(MovDn2),
            Instruction::MovDn3 => span.add_op(MovDn3),
            Instruction::MovDn4 => span.add_op(MovDn4),
            Instruction::MovDn5 => span.add_op(MovDn5),
            Instruction::MovDn6 => span.add_op(MovDn6),
            Instruction::MovDn7 => span.add_op(MovDn7),
            Instruction::MovDn8 => span.add_op(MovDn8),
            Instruction::MovDn9 => span.add_ops([MovDn8, SwapDW, Swap, SwapDW]),
            Instruction::MovDn10 => span.add_ops([MovDn8, SwapDW, MovDn2, SwapDW]),
            Instruction::MovDn11 => span.add_ops([MovDn8, SwapDW, MovDn3, SwapDW]),
            Instruction::MovDn12 => span.add_ops([MovDn8, SwapDW, MovDn4, SwapDW]),
            Instruction::MovDn13 => span.add_ops([MovDn8, SwapDW, MovDn5, SwapDW]),
            Instruction::MovDn14 => span.add_ops([MovDn8, SwapDW, MovDn6, SwapDW]),
            Instruction::MovDn15 => span.add_ops([MovDn8, SwapDW, MovDn7, SwapDW]),
            Instruction::MovDnW2 => span.add_ops([SwapW2, SwapW]),
            Instruction::MovDnW3 => span.add_ops([SwapW3, SwapW2, SwapW]),

            Instruction::CSwap => span.add_op(CSwap),
            Instruction::CSwapW => span.add_op(CSwapW),
            Instruction::CDrop => span.add_ops([CSwap, Drop]),
            Instruction::CDropW => span.add_ops([CSwapW, Drop, Drop, Drop, Drop]),

            // ----- input / output instructions --------------------------------------------------
            Instruction::PushU8(imm) => env_ops::push_one(*imm, span),
            Instruction::PushU16(imm) => env_ops::push_one(*imm, span),
            Instruction::PushU32(imm) => env_ops::push_one(*imm, span),
            Instruction::PushFelt(imm) => env_ops::push_one(*imm, span),
            Instruction::PushWord(imms) => env_ops::push_many(imms, span),
            Instruction::PushU8List(imms) => env_ops::push_many(imms, span),
            Instruction::PushU16List(imms) => env_ops::push_many(imms, span),
            Instruction::PushU32List(imms) => env_ops::push_many(imms, span),
            Instruction::PushFeltList(imms) => env_ops::push_many(imms, span),
            Instruction::Sdepth => span.add_op(SDepth),
            Instruction::Caller => env_ops::caller(span, ctx),
            Instruction::Clk => span.add_op(Clk),
            Instruction::AdvPipe => span.add_op(Pipe),
            Instruction::AdvPush(n) => adv_ops::adv_push(span, *n),
            Instruction::AdvLoadW => span.add_op(AdvPopW),

            Instruction::MemStream => span.add_op(MStream),

            Instruction::Locaddr(v) => env_ops::locaddr(span, *v, ctx),
            Instruction::MemLoad => mem_ops::mem_read(span, ctx, None, false, true),
            Instruction::MemLoadImm(v) => mem_ops::mem_read(span, ctx, Some(*v), false, true),
            Instruction::MemLoadW => mem_ops::mem_read(span, ctx, None, false, false),
            Instruction::MemLoadWImm(v) => mem_ops::mem_read(span, ctx, Some(*v), false, false),
            Instruction::LocLoad(v) => mem_ops::mem_read(span, ctx, Some(*v as u32), true, true),
            Instruction::LocLoadW(v) => mem_ops::mem_read(span, ctx, Some(*v as u32), true, false),
            Instruction::MemStore => span.add_ops([MStore, Drop]),
            Instruction::MemStoreW => span.add_ops([MStoreW]),
            Instruction::MemStoreImm(v) => mem_ops::mem_write_imm(span, ctx, *v, false, true),
            Instruction::MemStoreWImm(v) => mem_ops::mem_write_imm(span, ctx, *v, false, false),
            Instruction::LocStore(v) => mem_ops::mem_write_imm(span, ctx, *v as u32, true, true),
            Instruction::LocStoreW(v) => mem_ops::mem_write_imm(span, ctx, *v as u32, true, false),

            Instruction::AdvInject(injector) => adv_ops::adv_inject(span, injector),

            // ----- cryptographic instructions ---------------------------------------------------
            Instruction::Hash => crypto_ops::hash(span),
            Instruction::HPerm => span.add_op(HPerm),
            Instruction::HMerge => crypto_ops::hmerge(span),
            Instruction::MTreeGet => crypto_ops::mtree_get(span),
            Instruction::MTreeSet => crypto_ops::mtree_set(span),
            Instruction::MTreeMerge => crypto_ops::mtree_merge(span),
            Instruction::MTreeVerify => crypto_ops::mtree_verify(span),

            // ----- STARK proof verification -----------------------------------------------------
            Instruction::FriExt2Fold4 => span.add_op(FriE2F4),
            Instruction::RCombBase => span.add_op(RCombBase),

            // ----- exec/call instructions -------------------------------------------------------
            Instruction::ExecLocal(idx) => self.exec_local(*idx, ctx),
            Instruction::ExecImported(id) => self.exec_imported(id, ctx),
            Instruction::CallLocal(idx) => self.call_local(*idx, ctx),
            Instruction::CallMastRoot(root) => self.call_mast_root(root, ctx),
            Instruction::CallImported(id) => self.call_imported(id, ctx),
            Instruction::SysCall(id) => self.syscall(id, ctx),
            Instruction::DynExec => self.dynexec(),
            Instruction::DynCall => self.dyncall(),
            Instruction::ProcRefLocal(idx) => self.procref_local(*idx, ctx, span),
            Instruction::ProcRefImported(id) => self.procref_imported(id, ctx, span),

            // ----- debug decorators -------------------------------------------------------------
            Instruction::Breakpoint => {
                if self.in_debug_mode() {
                    span.add_op(Noop)?;
                    span.track_instruction(instruction, ctx);
                }
                Ok(None)
            }

            Instruction::Debug(options) => {
                if self.in_debug_mode() {
                    span.push_decorator(Decorator::Debug(*options))
                }
                Ok(None)
            }

            // ----- emit instruction -------------------------------------------------------------
            Instruction::Emit(event_id) => {
                span.push_decorator(Decorator::Event(*event_id));
                Ok(None)
            }

            // ----- trace instruction ------------------------------------------------------------
            Instruction::Trace(trace_id) => {
                span.push_decorator(Decorator::Trace(*trace_id));
                Ok(None)
            }
        };

        // compute and update the cycle count of the instruction which just finished executing
        if self.in_debug_mode() {
            span.set_instruction_cycle_count();
        }

        result
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// This is a helper function that appends a PUSH operation to the span block which puts the
/// provided u32 value onto the stack.
///
/// When the value is 0, PUSH operation is replaced with PAD. When the value is 1, PUSH operation
/// is replaced with PAD INCR because in most cases this will be more efficient than doing a PUSH.
fn push_u32_value(span: &mut SpanBuilder, value: u32) {
    use Operation::*;

    if value == 0 {
        span.push_op(Pad);
    } else if value == 1 {
        span.push_op(Pad);
        span.push_op(Incr);
    } else {
        span.push_op(Push(Felt::from(value)));
    }
}

/// This is a helper function that appends a PUSH operation to the span block which puts the
/// provided field element onto the stack.
///
/// When the value is 0, PUSH operation is replaced with PAD. When the value is 1, PUSH operation
/// is replaced with PAD INCR because in most cases this will be more efficient than doing a PUSH.
fn push_felt(span: &mut SpanBuilder, value: Felt) {
    use Operation::*;

    if value == ZERO {
        span.push_op(Pad);
    } else if value == ONE {
        span.push_op(Pad);
        span.push_op(Incr);
    } else {
        span.push_op(Push(value));
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
        AssemblyError::param_out_of_bounds(
            value.into(),
            bound_into_included_u64(range.start_bound(), true),
            bound_into_included_u64(range.end_bound(), false),
        )
    })
}
