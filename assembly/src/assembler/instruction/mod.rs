use core::ops::RangeBounds;

use miette::miette;
use vm_core::{mast::MastNodeId, Decorator, ONE, ZERO};

use super::{
    ast::InvokeKind, mast_forest_builder::MastForestBuilder, Assembler, BasicBlockBuilder, Felt,
    Operation, ProcedureContext,
};
use crate::{ast::Instruction, utils::bound_into_included_u64, AssemblyError, Span};

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
        instruction: &Span<Instruction>,
        span_builder: &mut BasicBlockBuilder,
        proc_ctx: &mut ProcedureContext,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        // if the assembler is in debug mode, start tracking the instruction about to be executed;
        // this will allow us to map the instruction to the sequence of operations which were
        // executed as a part of this instruction.
        if self.in_debug_mode() {
            span_builder.track_instruction(instruction, proc_ctx);
        }

        let result = self.compile_instruction_impl(
            instruction,
            span_builder,
            proc_ctx,
            mast_forest_builder,
        )?;

        // compute and update the cycle count of the instruction which just finished executing
        if self.in_debug_mode() {
            span_builder.set_instruction_cycle_count();
        }

        Ok(result)
    }

    fn compile_instruction_impl(
        &self,
        instruction: &Span<Instruction>,
        basic_block_builder: &mut BasicBlockBuilder,
        proc_ctx: &mut ProcedureContext,
        mast_forest_builder: &mut MastForestBuilder,
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        use Operation::*;

        match &**instruction {
            Instruction::Nop => basic_block_builder.push_op(Noop),
            Instruction::Assert => basic_block_builder.push_op(Assert(0)),
            Instruction::AssertWithError(err_code) => {
                basic_block_builder.push_op(Assert(err_code.expect_value()))
            },
            Instruction::AssertEq => basic_block_builder.push_ops([Eq, Assert(0)]),
            Instruction::AssertEqWithError(err_code) => {
                basic_block_builder.push_ops([Eq, Assert(err_code.expect_value())])
            },
            Instruction::AssertEqw => field_ops::assertw(basic_block_builder, 0),
            Instruction::AssertEqwWithError(err_code) => {
                field_ops::assertw(basic_block_builder, err_code.expect_value())
            },
            Instruction::Assertz => basic_block_builder.push_ops([Eqz, Assert(0)]),
            Instruction::AssertzWithError(err_code) => {
                basic_block_builder.push_ops([Eqz, Assert(err_code.expect_value())])
            },

            Instruction::Add => basic_block_builder.push_op(Add),
            Instruction::AddImm(imm) => field_ops::add_imm(basic_block_builder, imm.expect_value()),
            Instruction::Sub => basic_block_builder.push_ops([Neg, Add]),
            Instruction::SubImm(imm) => field_ops::sub_imm(basic_block_builder, imm.expect_value()),
            Instruction::Mul => basic_block_builder.push_op(Mul),
            Instruction::MulImm(imm) => field_ops::mul_imm(basic_block_builder, imm.expect_value()),
            Instruction::Div => basic_block_builder.push_ops([Inv, Mul]),
            Instruction::DivImm(imm) => {
                field_ops::div_imm(basic_block_builder, proc_ctx, imm.expect_spanned_value())?;
            },
            Instruction::Neg => basic_block_builder.push_op(Neg),
            Instruction::Inv => basic_block_builder.push_op(Inv),
            Instruction::Incr => basic_block_builder.push_op(Incr),

            Instruction::Pow2 => field_ops::pow2(basic_block_builder),
            Instruction::Exp => field_ops::exp(basic_block_builder, 64)?,
            Instruction::ExpImm(pow) => {
                field_ops::exp_imm(basic_block_builder, pow.expect_value())?
            },
            Instruction::ExpBitLength(num_pow_bits) => {
                field_ops::exp(basic_block_builder, *num_pow_bits)?
            },
            Instruction::ILog2 => field_ops::ilog2(basic_block_builder),

            Instruction::Not => basic_block_builder.push_op(Not),
            Instruction::And => basic_block_builder.push_op(And),
            Instruction::Or => basic_block_builder.push_op(Or),
            Instruction::Xor => {
                basic_block_builder.push_ops([Dup0, Dup2, Or, MovDn2, And, Not, And])
            },

            Instruction::Eq => basic_block_builder.push_op(Eq),
            Instruction::EqImm(imm) => field_ops::eq_imm(basic_block_builder, imm.expect_value()),
            Instruction::Eqw => field_ops::eqw(basic_block_builder),
            Instruction::Neq => basic_block_builder.push_ops([Eq, Not]),
            Instruction::NeqImm(imm) => field_ops::neq_imm(basic_block_builder, imm.expect_value()),
            Instruction::Lt => field_ops::lt(basic_block_builder),
            Instruction::Lte => field_ops::lte(basic_block_builder),
            Instruction::Gt => field_ops::gt(basic_block_builder),
            Instruction::Gte => field_ops::gte(basic_block_builder),
            Instruction::IsOdd => field_ops::is_odd(basic_block_builder),

            // ----- ext2 instructions ------------------------------------------------------------
            Instruction::Ext2Add => ext2_ops::ext2_add(basic_block_builder),
            Instruction::Ext2Sub => ext2_ops::ext2_sub(basic_block_builder),
            Instruction::Ext2Mul => ext2_ops::ext2_mul(basic_block_builder),
            Instruction::Ext2Div => ext2_ops::ext2_div(basic_block_builder),
            Instruction::Ext2Neg => ext2_ops::ext2_neg(basic_block_builder),
            Instruction::Ext2Inv => ext2_ops::ext2_inv(basic_block_builder),

            // ----- u32 manipulation -------------------------------------------------------------
            Instruction::U32Test => basic_block_builder.push_ops([Dup0, U32split, Swap, Drop, Eqz]),
            Instruction::U32TestW => u32_ops::u32testw(basic_block_builder),
            Instruction::U32Assert => basic_block_builder.push_ops([Pad, U32assert2(0), Drop]),
            Instruction::U32AssertWithError(err_code) => {
                basic_block_builder.push_ops([Pad, U32assert2(err_code.expect_value()), Drop])
            },
            Instruction::U32Assert2 => basic_block_builder.push_op(U32assert2(0)),
            Instruction::U32Assert2WithError(err_code) => {
                basic_block_builder.push_op(U32assert2(err_code.expect_value()))
            },
            Instruction::U32AssertW => u32_ops::u32assertw(basic_block_builder, 0),
            Instruction::U32AssertWWithError(err_code) => {
                u32_ops::u32assertw(basic_block_builder, err_code.expect_value())
            },

            Instruction::U32Cast => basic_block_builder.push_ops([U32split, Drop]),
            Instruction::U32Split => basic_block_builder.push_op(U32split),

            Instruction::U32OverflowingAdd => {
                u32_ops::u32add(basic_block_builder, Overflowing, None)
            },
            Instruction::U32OverflowingAddImm(v) => {
                u32_ops::u32add(basic_block_builder, Overflowing, Some(v.expect_value()))
            },
            Instruction::U32WrappingAdd => u32_ops::u32add(basic_block_builder, Wrapping, None),
            Instruction::U32WrappingAddImm(v) => {
                u32_ops::u32add(basic_block_builder, Wrapping, Some(v.expect_value()))
            },
            Instruction::U32OverflowingAdd3 => basic_block_builder.push_op(U32add3),
            Instruction::U32WrappingAdd3 => basic_block_builder.push_ops([U32add3, Drop]),

            Instruction::U32OverflowingSub => {
                u32_ops::u32sub(basic_block_builder, Overflowing, None)
            },
            Instruction::U32OverflowingSubImm(v) => {
                u32_ops::u32sub(basic_block_builder, Overflowing, Some(v.expect_value()))
            },
            Instruction::U32WrappingSub => u32_ops::u32sub(basic_block_builder, Wrapping, None),
            Instruction::U32WrappingSubImm(v) => {
                u32_ops::u32sub(basic_block_builder, Wrapping, Some(v.expect_value()))
            },

            Instruction::U32OverflowingMul => {
                u32_ops::u32mul(basic_block_builder, Overflowing, None)
            },
            Instruction::U32OverflowingMulImm(v) => {
                u32_ops::u32mul(basic_block_builder, Overflowing, Some(v.expect_value()))
            },
            Instruction::U32WrappingMul => u32_ops::u32mul(basic_block_builder, Wrapping, None),
            Instruction::U32WrappingMulImm(v) => {
                u32_ops::u32mul(basic_block_builder, Wrapping, Some(v.expect_value()))
            },
            Instruction::U32OverflowingMadd => basic_block_builder.push_op(U32madd),
            Instruction::U32WrappingMadd => basic_block_builder.push_ops([U32madd, Drop]),

            Instruction::U32Div => u32_ops::u32div(basic_block_builder, proc_ctx, None)?,
            Instruction::U32DivImm(v) => {
                u32_ops::u32div(basic_block_builder, proc_ctx, Some(v.expect_spanned_value()))?
            },
            Instruction::U32Mod => u32_ops::u32mod(basic_block_builder, proc_ctx, None)?,
            Instruction::U32ModImm(v) => {
                u32_ops::u32mod(basic_block_builder, proc_ctx, Some(v.expect_spanned_value()))?
            },
            Instruction::U32DivMod => u32_ops::u32divmod(basic_block_builder, proc_ctx, None)?,
            Instruction::U32DivModImm(v) => {
                u32_ops::u32divmod(basic_block_builder, proc_ctx, Some(v.expect_spanned_value()))?
            },
            Instruction::U32And => basic_block_builder.push_op(U32and),
            Instruction::U32Or => basic_block_builder.push_ops([Dup1, Dup1, U32and, Neg, Add, Add]),
            Instruction::U32Xor => basic_block_builder.push_op(U32xor),
            Instruction::U32Not => u32_ops::u32not(basic_block_builder),
            Instruction::U32Shl => u32_ops::u32shl(basic_block_builder, None)?,
            Instruction::U32ShlImm(v) => {
                u32_ops::u32shl(basic_block_builder, Some(v.expect_value()))?
            },
            Instruction::U32Shr => u32_ops::u32shr(basic_block_builder, None)?,
            Instruction::U32ShrImm(v) => {
                u32_ops::u32shr(basic_block_builder, Some(v.expect_value()))?
            },
            Instruction::U32Rotl => u32_ops::u32rotl(basic_block_builder, None)?,
            Instruction::U32RotlImm(v) => {
                u32_ops::u32rotl(basic_block_builder, Some(v.expect_value()))?
            },
            Instruction::U32Rotr => u32_ops::u32rotr(basic_block_builder, None)?,
            Instruction::U32RotrImm(v) => {
                u32_ops::u32rotr(basic_block_builder, Some(v.expect_value()))?
            },
            Instruction::U32Popcnt => u32_ops::u32popcnt(basic_block_builder),
            Instruction::U32Clz => u32_ops::u32clz(basic_block_builder),
            Instruction::U32Ctz => u32_ops::u32ctz(basic_block_builder),
            Instruction::U32Clo => u32_ops::u32clo(basic_block_builder),
            Instruction::U32Cto => u32_ops::u32cto(basic_block_builder),
            Instruction::U32Lt => u32_ops::u32lt(basic_block_builder),
            Instruction::U32Lte => u32_ops::u32lte(basic_block_builder),
            Instruction::U32Gt => u32_ops::u32gt(basic_block_builder),
            Instruction::U32Gte => u32_ops::u32gte(basic_block_builder),
            Instruction::U32Min => u32_ops::u32min(basic_block_builder),
            Instruction::U32Max => u32_ops::u32max(basic_block_builder),

            // ----- stack manipulation -----------------------------------------------------------
            Instruction::Drop => basic_block_builder.push_op(Drop),
            Instruction::DropW => basic_block_builder.push_ops([Drop; 4]),
            Instruction::PadW => basic_block_builder.push_ops([Pad; 4]),
            Instruction::Dup0 => basic_block_builder.push_op(Dup0),
            Instruction::Dup1 => basic_block_builder.push_op(Dup1),
            Instruction::Dup2 => basic_block_builder.push_op(Dup2),
            Instruction::Dup3 => basic_block_builder.push_op(Dup3),
            Instruction::Dup4 => basic_block_builder.push_op(Dup4),
            Instruction::Dup5 => basic_block_builder.push_op(Dup5),
            Instruction::Dup6 => basic_block_builder.push_op(Dup6),
            Instruction::Dup7 => basic_block_builder.push_op(Dup7),
            Instruction::Dup8 => basic_block_builder.push_ops([Pad, Dup9, Add]),
            Instruction::Dup9 => basic_block_builder.push_op(Dup9),
            Instruction::Dup10 => basic_block_builder.push_ops([Pad, Dup11, Add]),
            Instruction::Dup11 => basic_block_builder.push_op(Dup11),
            Instruction::Dup12 => basic_block_builder.push_ops([Pad, Dup13, Add]),
            Instruction::Dup13 => basic_block_builder.push_op(Dup13),
            Instruction::Dup14 => basic_block_builder.push_ops([Pad, Dup15, Add]),
            Instruction::Dup15 => basic_block_builder.push_op(Dup15),
            Instruction::DupW0 => basic_block_builder.push_ops([Dup3; 4]),
            Instruction::DupW1 => basic_block_builder.push_ops([Dup7; 4]),
            Instruction::DupW2 => basic_block_builder.push_ops([Dup11; 4]),
            Instruction::DupW3 => basic_block_builder.push_ops([Dup15; 4]),
            Instruction::Swap1 => basic_block_builder.push_op(Swap),
            Instruction::Swap2 => basic_block_builder.push_ops([Swap, MovUp2]),
            Instruction::Swap3 => basic_block_builder.push_ops([MovDn2, MovUp3]),
            Instruction::Swap4 => basic_block_builder.push_ops([MovDn3, MovUp4]),
            Instruction::Swap5 => basic_block_builder.push_ops([MovDn4, MovUp5]),
            Instruction::Swap6 => basic_block_builder.push_ops([MovDn5, MovUp6]),
            Instruction::Swap7 => basic_block_builder.push_ops([MovDn6, MovUp7]),
            Instruction::Swap8 => basic_block_builder.push_ops([MovDn7, MovUp8]),
            Instruction::Swap9 => {
                basic_block_builder.push_ops([MovDn8, SwapDW, Swap, SwapDW, MovUp8])
            },
            Instruction::Swap10 => {
                basic_block_builder.push_ops([MovDn8, SwapDW, Swap, MovUp2, SwapDW, MovUp8])
            },
            Instruction::Swap11 => {
                basic_block_builder.push_ops([MovDn8, SwapDW, MovDn2, MovUp3, SwapDW, MovUp8])
            },
            Instruction::Swap12 => {
                basic_block_builder.push_ops([MovDn8, SwapDW, MovDn3, MovUp4, SwapDW, MovUp8])
            },
            Instruction::Swap13 => {
                basic_block_builder.push_ops([MovDn8, SwapDW, MovDn4, MovUp5, SwapDW, MovUp8])
            },
            Instruction::Swap14 => {
                basic_block_builder.push_ops([MovDn8, SwapDW, MovDn5, MovUp6, SwapDW, MovUp8])
            },
            Instruction::Swap15 => {
                basic_block_builder.push_ops([MovDn8, SwapDW, MovDn6, MovUp7, SwapDW, MovUp8])
            },
            Instruction::SwapW1 => basic_block_builder.push_op(SwapW),
            Instruction::SwapW2 => basic_block_builder.push_op(SwapW2),
            Instruction::SwapW3 => basic_block_builder.push_op(SwapW3),
            Instruction::SwapDw => basic_block_builder.push_op(SwapDW),
            Instruction::MovUp2 => basic_block_builder.push_op(MovUp2),
            Instruction::MovUp3 => basic_block_builder.push_op(MovUp3),
            Instruction::MovUp4 => basic_block_builder.push_op(MovUp4),
            Instruction::MovUp5 => basic_block_builder.push_op(MovUp5),
            Instruction::MovUp6 => basic_block_builder.push_op(MovUp6),
            Instruction::MovUp7 => basic_block_builder.push_op(MovUp7),
            Instruction::MovUp8 => basic_block_builder.push_op(MovUp8),
            Instruction::MovUp9 => basic_block_builder.push_ops([SwapDW, Swap, SwapDW, MovUp8]),
            Instruction::MovUp10 => basic_block_builder.push_ops([SwapDW, MovUp2, SwapDW, MovUp8]),
            Instruction::MovUp11 => basic_block_builder.push_ops([SwapDW, MovUp3, SwapDW, MovUp8]),
            Instruction::MovUp12 => basic_block_builder.push_ops([SwapDW, MovUp4, SwapDW, MovUp8]),
            Instruction::MovUp13 => basic_block_builder.push_ops([SwapDW, MovUp5, SwapDW, MovUp8]),
            Instruction::MovUp14 => basic_block_builder.push_ops([SwapDW, MovUp6, SwapDW, MovUp8]),
            Instruction::MovUp15 => basic_block_builder.push_ops([SwapDW, MovUp7, SwapDW, MovUp8]),
            Instruction::MovUpW2 => basic_block_builder.push_ops([SwapW, SwapW2]),
            Instruction::MovUpW3 => basic_block_builder.push_ops([SwapW, SwapW2, SwapW3]),
            Instruction::MovDn2 => basic_block_builder.push_op(MovDn2),
            Instruction::MovDn3 => basic_block_builder.push_op(MovDn3),
            Instruction::MovDn4 => basic_block_builder.push_op(MovDn4),
            Instruction::MovDn5 => basic_block_builder.push_op(MovDn5),
            Instruction::MovDn6 => basic_block_builder.push_op(MovDn6),
            Instruction::MovDn7 => basic_block_builder.push_op(MovDn7),
            Instruction::MovDn8 => basic_block_builder.push_op(MovDn8),
            Instruction::MovDn9 => basic_block_builder.push_ops([MovDn8, SwapDW, Swap, SwapDW]),
            Instruction::MovDn10 => basic_block_builder.push_ops([MovDn8, SwapDW, MovDn2, SwapDW]),
            Instruction::MovDn11 => basic_block_builder.push_ops([MovDn8, SwapDW, MovDn3, SwapDW]),
            Instruction::MovDn12 => basic_block_builder.push_ops([MovDn8, SwapDW, MovDn4, SwapDW]),
            Instruction::MovDn13 => basic_block_builder.push_ops([MovDn8, SwapDW, MovDn5, SwapDW]),
            Instruction::MovDn14 => basic_block_builder.push_ops([MovDn8, SwapDW, MovDn6, SwapDW]),
            Instruction::MovDn15 => basic_block_builder.push_ops([MovDn8, SwapDW, MovDn7, SwapDW]),
            Instruction::MovDnW2 => basic_block_builder.push_ops([SwapW2, SwapW]),
            Instruction::MovDnW3 => basic_block_builder.push_ops([SwapW3, SwapW2, SwapW]),

            Instruction::CSwap => basic_block_builder.push_op(CSwap),
            Instruction::CSwapW => basic_block_builder.push_op(CSwapW),
            Instruction::CDrop => basic_block_builder.push_ops([CSwap, Drop]),
            Instruction::CDropW => basic_block_builder.push_ops([CSwapW, Drop, Drop, Drop, Drop]),

            // ----- input / output instructions --------------------------------------------------
            Instruction::Push(imm) => env_ops::push_one(imm.expect_value(), basic_block_builder),
            Instruction::PushU8(imm) => env_ops::push_one(*imm, basic_block_builder),
            Instruction::PushU16(imm) => env_ops::push_one(*imm, basic_block_builder),
            Instruction::PushU32(imm) => env_ops::push_one(*imm, basic_block_builder),
            Instruction::PushFelt(imm) => env_ops::push_one(*imm, basic_block_builder),
            Instruction::PushWord(imms) => env_ops::push_many(imms, basic_block_builder),
            Instruction::PushU8List(imms) => env_ops::push_many(imms, basic_block_builder),
            Instruction::PushU16List(imms) => env_ops::push_many(imms, basic_block_builder),
            Instruction::PushU32List(imms) => env_ops::push_many(imms, basic_block_builder),
            Instruction::PushFeltList(imms) => env_ops::push_many(imms, basic_block_builder),
            Instruction::Sdepth => basic_block_builder.push_op(SDepth),
            Instruction::Caller => {
                env_ops::caller(basic_block_builder, proc_ctx, instruction.span())?
            },
            Instruction::Clk => basic_block_builder.push_op(Clk),
            Instruction::AdvPipe => basic_block_builder.push_op(Pipe),
            Instruction::AdvPush(n) => adv_ops::adv_push(basic_block_builder, n.expect_value())?,
            Instruction::AdvLoadW => basic_block_builder.push_op(AdvPopW),

            Instruction::MemStream => basic_block_builder.push_op(MStream),
            Instruction::Locaddr(v) => {
                env_ops::locaddr(basic_block_builder, v.expect_value(), proc_ctx)?
            },
            Instruction::MemLoad => {
                mem_ops::mem_read(basic_block_builder, proc_ctx, None, false, true)?
            },
            Instruction::MemLoadImm(v) => mem_ops::mem_read(
                basic_block_builder,
                proc_ctx,
                Some(v.expect_value()),
                false,
                true,
            )?,
            Instruction::MemLoadW => {
                mem_ops::mem_read(basic_block_builder, proc_ctx, None, false, false)?
            },
            Instruction::MemLoadWImm(v) => mem_ops::mem_read(
                basic_block_builder,
                proc_ctx,
                Some(v.expect_value()),
                false,
                false,
            )?,
            Instruction::LocLoad(v) => mem_ops::mem_read(
                basic_block_builder,
                proc_ctx,
                Some(v.expect_value() as u32),
                true,
                true,
            )?,
            Instruction::LocLoadW(v) => mem_ops::mem_read(
                basic_block_builder,
                proc_ctx,
                Some(v.expect_value() as u32),
                true,
                false,
            )?,
            Instruction::MemStore => basic_block_builder.push_ops([MStore, Drop]),
            Instruction::MemStoreW => basic_block_builder.push_ops([MStoreW]),
            Instruction::MemStoreImm(v) => mem_ops::mem_write_imm(
                basic_block_builder,
                proc_ctx,
                v.expect_value(),
                false,
                true,
            )?,
            Instruction::MemStoreWImm(v) => mem_ops::mem_write_imm(
                basic_block_builder,
                proc_ctx,
                v.expect_value(),
                false,
                false,
            )?,
            Instruction::LocStore(v) => mem_ops::mem_write_imm(
                basic_block_builder,
                proc_ctx,
                v.expect_value() as u32,
                true,
                true,
            )?,
            Instruction::LocStoreW(v) => mem_ops::mem_write_imm(
                basic_block_builder,
                proc_ctx,
                v.expect_value() as u32,
                true,
                false,
            )?,

            Instruction::AdvInject(injector) => adv_ops::adv_inject(basic_block_builder, injector),

            // ----- cryptographic instructions ---------------------------------------------------
            Instruction::Hash => crypto_ops::hash(basic_block_builder),
            Instruction::HPerm => basic_block_builder.push_op(HPerm),
            Instruction::HMerge => crypto_ops::hmerge(basic_block_builder),
            Instruction::MTreeGet => crypto_ops::mtree_get(basic_block_builder),
            Instruction::MTreeSet => crypto_ops::mtree_set(basic_block_builder),
            Instruction::MTreeMerge => crypto_ops::mtree_merge(basic_block_builder),
            Instruction::MTreeVerify => basic_block_builder.push_op(MpVerify(0)),
            Instruction::MTreeVerifyWithError(err_code) => {
                basic_block_builder.push_op(MpVerify(err_code.expect_value()))
            },

            // ----- STARK proof verification -----------------------------------------------------
            Instruction::FriExt2Fold4 => basic_block_builder.push_op(FriE2F4),
            Instruction::RCombBase => basic_block_builder.push_op(RCombBase),

            // ----- exec/call instructions -------------------------------------------------------
            Instruction::Exec(ref callee) => {
                return self.invoke(InvokeKind::Exec, callee, proc_ctx, mast_forest_builder)
            },
            Instruction::Call(ref callee) => {
                return self.invoke(InvokeKind::Call, callee, proc_ctx, mast_forest_builder)
            },
            Instruction::SysCall(ref callee) => {
                return self.invoke(InvokeKind::SysCall, callee, proc_ctx, mast_forest_builder)
            },
            Instruction::DynExec => return self.dynexec(mast_forest_builder),
            Instruction::DynCall => return self.dyncall(mast_forest_builder),
            Instruction::ProcRef(ref callee) => {
                self.procref(callee, proc_ctx, basic_block_builder, mast_forest_builder)?
            },

            // ----- debug decorators -------------------------------------------------------------
            Instruction::Breakpoint => {
                if self.in_debug_mode() {
                    basic_block_builder.push_op(Noop);
                    basic_block_builder.track_instruction(instruction, proc_ctx);
                }
            },

            Instruction::Debug(options) => {
                if self.in_debug_mode() {
                    basic_block_builder.push_decorator(Decorator::Debug(
                        options.clone().try_into().expect("unresolved constant"),
                    ))
                }
            },

            // ----- emit instruction -------------------------------------------------------------
            Instruction::Emit(event_id) => {
                basic_block_builder.push_decorator(Decorator::Event(event_id.expect_value()));
            },

            // ----- trace instruction ------------------------------------------------------------
            Instruction::Trace(trace_id) => {
                basic_block_builder.push_decorator(Decorator::Trace(trace_id.expect_value()));
            },
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
fn push_u32_value(span_builder: &mut BasicBlockBuilder, value: u32) {
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
fn push_felt(span_builder: &mut BasicBlockBuilder, value: Felt) {
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
            miette!(
                "parameter value must be greater than or equal to {min} and \
            less than or equal to {max}, but was {value}",
            )
            .into(),
        )
    })
}
