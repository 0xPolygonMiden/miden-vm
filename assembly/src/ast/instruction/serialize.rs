use crate::{
    ast::{Instruction, OpCode},
    ByteWriter, Serializable,
};

impl Serializable for Instruction {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        match self {
            Self::Assert => OpCode::Assert.write_into(target),
            Self::AssertWithError(err_code) => {
                OpCode::AssertWithError.write_into(target);
                target.write_u32(err_code.expect_value());
            }
            Self::AssertEq => OpCode::AssertEq.write_into(target),
            Self::AssertEqWithError(err_code) => {
                OpCode::AssertEqWithError.write_into(target);
                target.write_u32(err_code.expect_value());
            }
            Self::AssertEqw => OpCode::AssertEqw.write_into(target),
            Self::AssertEqwWithError(err_code) => {
                OpCode::AssertEqwWithError.write_into(target);
                target.write_u32(err_code.expect_value());
            }
            Self::Assertz => OpCode::Assertz.write_into(target),
            Self::AssertzWithError(err_code) => {
                OpCode::AssertzWithError.write_into(target);
                target.write_u32(err_code.expect_value());
            }
            Self::Add => OpCode::Add.write_into(target),
            Self::AddImm(v) => {
                OpCode::AddImm.write_into(target);
                v.expect_value().write_into(target);
            }
            Self::Sub => OpCode::Sub.write_into(target),
            Self::SubImm(v) => {
                OpCode::SubImm.write_into(target);
                v.expect_value().write_into(target);
            }
            Self::Mul => OpCode::Mul.write_into(target),
            Self::MulImm(v) => {
                OpCode::MulImm.write_into(target);
                v.expect_value().write_into(target);
            }
            Self::Div => OpCode::Div.write_into(target),
            Self::DivImm(v) => {
                OpCode::DivImm.write_into(target);
                v.expect_value().write_into(target);
            }
            Self::Neg => OpCode::Neg.write_into(target),
            Self::ILog2 => OpCode::ILog2.write_into(target),
            Self::Inv => OpCode::Inv.write_into(target),
            Self::Incr => OpCode::Incr.write_into(target),
            Self::Pow2 => OpCode::Pow2.write_into(target),
            Self::Exp => OpCode::Exp.write_into(target),
            Self::ExpImm(v) => {
                OpCode::ExpImm.write_into(target);
                v.expect_value().write_into(target);
            }
            Self::ExpBitLength(v) => {
                OpCode::ExpBitLength.write_into(target);
                target.write_u8(*v);
            }
            Self::Not => OpCode::Not.write_into(target),
            Self::And => OpCode::And.write_into(target),
            Self::Or => OpCode::Or.write_into(target),
            Self::Xor => OpCode::Xor.write_into(target),
            Self::Eq => OpCode::Eq.write_into(target),
            Self::EqImm(v) => {
                OpCode::EqImm.write_into(target);
                v.expect_value().write_into(target);
            }
            Self::Neq => OpCode::Neq.write_into(target),
            Self::NeqImm(v) => {
                OpCode::NeqImm.write_into(target);
                v.expect_value().write_into(target);
            }
            Self::Eqw => OpCode::Eqw.write_into(target),
            Self::Lt => OpCode::Lt.write_into(target),
            Self::Lte => OpCode::Lte.write_into(target),
            Self::Gt => OpCode::Gt.write_into(target),
            Self::Gte => OpCode::Gte.write_into(target),
            Self::IsOdd => OpCode::IsOdd.write_into(target),

            // ----- ext2 operations --------------------------------------------------------------
            Self::Ext2Add => OpCode::Ext2Add.write_into(target),
            Self::Ext2Sub => OpCode::Ext2Sub.write_into(target),
            Self::Ext2Mul => OpCode::Ext2Mul.write_into(target),
            Self::Ext2Div => OpCode::Ext2Div.write_into(target),
            Self::Ext2Neg => OpCode::Ext2Neg.write_into(target),
            Self::Ext2Inv => OpCode::Ext2Inv.write_into(target),

            // ----- u32 operations ---------------------------------------------------------------
            Self::U32Test => OpCode::U32Test.write_into(target),
            Self::U32TestW => OpCode::U32TestW.write_into(target),
            Self::U32Assert => OpCode::U32Assert.write_into(target),
            Self::U32AssertWithError(err_code) => {
                OpCode::U32AssertWithError.write_into(target);
                target.write_u32(err_code.expect_value());
            }
            Self::U32Assert2 => OpCode::U32Assert2.write_into(target),
            Self::U32Assert2WithError(err_code) => {
                OpCode::U32Assert2WithError.write_into(target);
                target.write_u32(err_code.expect_value());
            }
            Self::U32AssertW => OpCode::U32AssertW.write_into(target),
            Self::U32AssertWWithError(err_code) => {
                OpCode::U32AssertWWithError.write_into(target);
                target.write_u32(err_code.expect_value());
            }
            Self::U32Split => OpCode::U32Split.write_into(target),
            Self::U32Cast => OpCode::U32Cast.write_into(target),
            Self::U32WrappingAdd => OpCode::U32WrappingAdd.write_into(target),
            Self::U32WrappingAddImm(v) => {
                OpCode::U32WrappingAddImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32OverflowingAdd => OpCode::U32OverflowingAdd.write_into(target),
            Self::U32OverflowingAddImm(v) => {
                OpCode::U32OverflowingAddImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32OverflowingAdd3 => OpCode::U32OverflowingAdd3.write_into(target),
            Self::U32WrappingAdd3 => OpCode::U32WrappingAdd3.write_into(target),
            Self::U32WrappingSub => OpCode::U32WrappingSub.write_into(target),
            Self::U32WrappingSubImm(v) => {
                OpCode::U32WrappingSubImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32OverflowingSub => OpCode::U32OverflowingSub.write_into(target),
            Self::U32OverflowingSubImm(v) => {
                OpCode::U32OverflowingSubImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32WrappingMul => OpCode::U32WrappingMul.write_into(target),
            Self::U32WrappingMulImm(v) => {
                OpCode::U32WrappingMulImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32OverflowingMul => OpCode::U32OverflowingMul.write_into(target),
            Self::U32OverflowingMulImm(v) => {
                OpCode::U32OverflowingMulImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32OverflowingMadd => OpCode::U32OverflowingMadd.write_into(target),
            Self::U32WrappingMadd => OpCode::U32WrappingMadd.write_into(target),
            Self::U32Div => OpCode::U32Div.write_into(target),
            Self::U32DivImm(v) => {
                OpCode::U32DivImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32Mod => OpCode::U32Mod.write_into(target),
            Self::U32ModImm(v) => {
                OpCode::U32ModImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32DivMod => OpCode::U32DivMod.write_into(target),
            Self::U32DivModImm(v) => {
                OpCode::U32DivModImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::U32And => OpCode::U32And.write_into(target),
            Self::U32Or => OpCode::U32Or.write_into(target),
            Self::U32Xor => OpCode::U32Xor.write_into(target),
            Self::U32Not => OpCode::U32Not.write_into(target),
            Self::U32Shr => OpCode::U32Shr.write_into(target),
            Self::U32ShrImm(v) => {
                OpCode::U32ShrImm.write_into(target);
                target.write_u8(v.expect_value());
            }
            Self::U32Shl => OpCode::U32Shl.write_into(target),
            Self::U32ShlImm(v) => {
                OpCode::U32ShlImm.write_into(target);
                target.write_u8(v.expect_value());
            }
            Self::U32Rotr => OpCode::U32Rotr.write_into(target),
            Self::U32RotrImm(v) => {
                OpCode::U32RotrImm.write_into(target);
                target.write_u8(v.expect_value());
            }
            Self::U32Rotl => OpCode::U32Rotl.write_into(target),
            Self::U32RotlImm(v) => {
                OpCode::U32RotlImm.write_into(target);
                target.write_u8(v.expect_value());
            }
            Self::U32Popcnt => OpCode::U32Popcnt.write_into(target),
            Self::U32Clz => OpCode::U32Clz.write_into(target),
            Self::U32Ctz => OpCode::U32Ctz.write_into(target),
            Self::U32Clo => OpCode::U32Clo.write_into(target),
            Self::U32Cto => OpCode::U32Cto.write_into(target),
            Self::U32Lt => OpCode::U32Lt.write_into(target),
            Self::U32Lte => OpCode::U32Lte.write_into(target),
            Self::U32Gt => OpCode::U32Gt.write_into(target),
            Self::U32Gte => OpCode::U32Gte.write_into(target),
            Self::U32Min => OpCode::U32Min.write_into(target),
            Self::U32Max => OpCode::U32Max.write_into(target),

            // ----- stack manipulation ---------------------------------------------------------------
            Self::Drop => OpCode::Drop.write_into(target),
            Self::DropW => OpCode::DropW.write_into(target),
            Self::PadW => OpCode::PadW.write_into(target),
            Self::Dup0 => OpCode::Dup0.write_into(target),
            Self::Dup1 => OpCode::Dup1.write_into(target),
            Self::Dup2 => OpCode::Dup2.write_into(target),
            Self::Dup3 => OpCode::Dup3.write_into(target),
            Self::Dup4 => OpCode::Dup4.write_into(target),
            Self::Dup5 => OpCode::Dup5.write_into(target),
            Self::Dup6 => OpCode::Dup6.write_into(target),
            Self::Dup7 => OpCode::Dup7.write_into(target),
            Self::Dup8 => OpCode::Dup8.write_into(target),
            Self::Dup9 => OpCode::Dup9.write_into(target),
            Self::Dup10 => OpCode::Dup10.write_into(target),
            Self::Dup11 => OpCode::Dup11.write_into(target),
            Self::Dup12 => OpCode::Dup12.write_into(target),
            Self::Dup13 => OpCode::Dup13.write_into(target),
            Self::Dup14 => OpCode::Dup14.write_into(target),
            Self::Dup15 => OpCode::Dup15.write_into(target),
            Self::DupW0 => OpCode::DupW0.write_into(target),
            Self::DupW1 => OpCode::DupW1.write_into(target),
            Self::DupW2 => OpCode::DupW2.write_into(target),
            Self::DupW3 => OpCode::DupW3.write_into(target),
            Self::Swap1 => OpCode::Swap1.write_into(target),
            Self::Swap2 => OpCode::Swap2.write_into(target),
            Self::Swap3 => OpCode::Swap3.write_into(target),
            Self::Swap4 => OpCode::Swap4.write_into(target),
            Self::Swap5 => OpCode::Swap5.write_into(target),
            Self::Swap6 => OpCode::Swap6.write_into(target),
            Self::Swap7 => OpCode::Swap7.write_into(target),
            Self::Swap8 => OpCode::Swap8.write_into(target),
            Self::Swap9 => OpCode::Swap9.write_into(target),
            Self::Swap10 => OpCode::Swap10.write_into(target),
            Self::Swap11 => OpCode::Swap11.write_into(target),
            Self::Swap12 => OpCode::Swap12.write_into(target),
            Self::Swap13 => OpCode::Swap13.write_into(target),
            Self::Swap14 => OpCode::Swap14.write_into(target),
            Self::Swap15 => OpCode::Swap15.write_into(target),
            Self::SwapW1 => OpCode::SwapW1.write_into(target),
            Self::SwapW2 => OpCode::SwapW2.write_into(target),
            Self::SwapW3 => OpCode::SwapW3.write_into(target),
            Self::SwapDw => OpCode::SwapDW.write_into(target),
            Self::MovUp2 => OpCode::MovUp2.write_into(target),
            Self::MovUp3 => OpCode::MovUp3.write_into(target),
            Self::MovUp4 => OpCode::MovUp4.write_into(target),
            Self::MovUp5 => OpCode::MovUp5.write_into(target),
            Self::MovUp6 => OpCode::MovUp6.write_into(target),
            Self::MovUp7 => OpCode::MovUp7.write_into(target),
            Self::MovUp8 => OpCode::MovUp8.write_into(target),
            Self::MovUp9 => OpCode::MovUp9.write_into(target),
            Self::MovUp10 => OpCode::MovUp10.write_into(target),
            Self::MovUp11 => OpCode::MovUp11.write_into(target),
            Self::MovUp12 => OpCode::MovUp12.write_into(target),
            Self::MovUp13 => OpCode::MovUp13.write_into(target),
            Self::MovUp14 => OpCode::MovUp14.write_into(target),
            Self::MovUp15 => OpCode::MovUp15.write_into(target),
            Self::MovUpW2 => OpCode::MovUpW2.write_into(target),
            Self::MovUpW3 => OpCode::MovUpW3.write_into(target),
            Self::MovDn2 => OpCode::MovDn2.write_into(target),
            Self::MovDn3 => OpCode::MovDn3.write_into(target),
            Self::MovDn4 => OpCode::MovDn4.write_into(target),
            Self::MovDn5 => OpCode::MovDn5.write_into(target),
            Self::MovDn6 => OpCode::MovDn6.write_into(target),
            Self::MovDn7 => OpCode::MovDn7.write_into(target),
            Self::MovDn8 => OpCode::MovDn8.write_into(target),
            Self::MovDn9 => OpCode::MovDn9.write_into(target),
            Self::MovDn10 => OpCode::MovDn10.write_into(target),
            Self::MovDn11 => OpCode::MovDn11.write_into(target),
            Self::MovDn12 => OpCode::MovDn12.write_into(target),
            Self::MovDn13 => OpCode::MovDn13.write_into(target),
            Self::MovDn14 => OpCode::MovDn14.write_into(target),
            Self::MovDn15 => OpCode::MovDn15.write_into(target),
            Self::MovDnW2 => OpCode::MovDnW2.write_into(target),
            Self::MovDnW3 => OpCode::MovDnW3.write_into(target),
            Self::CSwap => OpCode::CSwap.write_into(target),
            Self::CSwapW => OpCode::CSwapW.write_into(target),
            Self::CDrop => OpCode::CDrop.write_into(target),
            Self::CDropW => OpCode::CDropW.write_into(target),

            // ----- input / output operations --------------------------------------------------------
            Self::Push(imm) => {
                OpCode::PushFelt.write_into(target);
                imm.expect_value().write_into(target);
            }
            Self::PushU8(value) => {
                OpCode::PushU8.write_into(target);
                target.write_u8(*value);
            }
            Self::PushU16(value) => {
                OpCode::PushU16.write_into(target);
                target.write_u16(*value);
            }
            Self::PushU32(value) => {
                OpCode::PushU32.write_into(target);
                target.write_u32(*value);
            }
            Self::PushFelt(value) => {
                OpCode::PushFelt.write_into(target);
                value.write_into(target);
            }
            Self::PushWord(values) => {
                OpCode::PushWord.write_into(target);
                values.iter().for_each(|&v| v.write_into(target));
            }
            Self::PushU8List(values) => {
                OpCode::PushU8List.write_into(target);
                target.write_u8(values.len() as u8);
                values.iter().for_each(|&v| target.write_u8(v));
            }
            Self::PushU16List(values) => {
                OpCode::PushU16List.write_into(target);
                target.write_u8(values.len() as u8);
                values.iter().for_each(|&v| target.write_u16(v));
            }
            Self::PushU32List(values) => {
                OpCode::PushU32List.write_into(target);
                target.write_u8(values.len() as u8);
                values.iter().for_each(|&v| target.write_u32(v));
            }
            Self::PushFeltList(values) => {
                OpCode::PushFeltList.write_into(target);
                target.write_u8(values.len() as u8);
                values.iter().for_each(|&v| v.write_into(target));
            }
            Self::Locaddr(v) => {
                OpCode::Locaddr.write_into(target);
                target.write_u16(v.expect_value());
            }
            Self::Sdepth => OpCode::Sdepth.write_into(target),
            Self::Caller => OpCode::Caller.write_into(target),
            Self::Clk => OpCode::Clk.write_into(target),

            Self::MemLoad => OpCode::MemLoad.write_into(target),
            Self::MemLoadImm(v) => {
                OpCode::MemLoadImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::MemLoadW => OpCode::MemLoadW.write_into(target),
            Self::MemLoadWImm(v) => {
                OpCode::MemLoadWImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::LocLoad(v) => {
                OpCode::LocLoad.write_into(target);
                target.write_u16(v.expect_value());
            }
            Self::LocLoadW(v) => {
                OpCode::LocLoadW.write_into(target);
                target.write_u16(v.expect_value());
            }
            Self::MemStore => OpCode::MemStore.write_into(target),
            Self::MemStoreImm(v) => {
                OpCode::MemStoreImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::LocStore(v) => {
                OpCode::LocStore.write_into(target);
                target.write_u16(v.expect_value());
            }
            Self::MemStoreW => OpCode::MemStoreW.write_into(target),
            Self::MemStoreWImm(v) => {
                OpCode::MemStoreWImm.write_into(target);
                target.write_u32(v.expect_value());
            }
            Self::LocStoreW(v) => {
                OpCode::LocStoreW.write_into(target);
                target.write_u16(v.expect_value());
            }

            Self::MemStream => OpCode::MemStream.write_into(target),
            Self::AdvPipe => OpCode::AdvPipe.write_into(target),

            Self::AdvPush(v) => {
                OpCode::AdvPush.write_into(target);
                target.write_u8(v.expect_value());
            }
            Self::AdvLoadW => OpCode::AdvLoadW.write_into(target),

            Self::AdvInject(injector) => {
                OpCode::AdvInject.write_into(target);
                injector.write_into(target);
            }

            // ----- cryptographic operations -----------------------------------------------------
            Self::Hash => OpCode::Hash.write_into(target),
            Self::HMerge => OpCode::HMerge.write_into(target),
            Self::HPerm => OpCode::HPerm.write_into(target),
            Self::MTreeGet => OpCode::MTreeGet.write_into(target),
            Self::MTreeSet => OpCode::MTreeSet.write_into(target),
            Self::MTreeMerge => OpCode::MTreeMerge.write_into(target),
            Self::MTreeVerify => OpCode::MTreeVerify.write_into(target),

            // ----- STARK proof verification -----------------------------------------------------
            Self::FriExt2Fold4 => OpCode::FriExt2Fold4.write_into(target),
            Self::RCombBase => OpCode::RCombBase.write_into(target),

            // ----- exec / call ------------------------------------------------------------------
            Self::Exec(ref callee) => {
                OpCode::Exec.write_into(target);
                callee.write_into(target);
            }
            Self::Call(ref callee) => {
                OpCode::Call.write_into(target);
                callee.write_into(target);
            }
            Self::SysCall(ref callee) => {
                OpCode::SysCall.write_into(target);
                callee.write_into(target);
            }
            Self::DynExec => OpCode::DynExec.write_into(target),
            Self::DynCall => OpCode::DynCall.write_into(target),
            Self::ProcRef(ref callee) => {
                OpCode::ProcRef.write_into(target);
                callee.write_into(target);
            }

            // ----- debug decorators -------------------------------------------------------------
            Self::Breakpoint => {
                // this is a transparent instruction and will not be encoded into the library
            }

            Self::Debug(options) => {
                OpCode::Debug.write_into(target);
                options.write_into(target);
            }

            // ----- event decorators -------------------------------------------------------------
            Self::Emit(event_id) => {
                OpCode::Emit.write_into(target);
                target.write_u32(event_id.expect_value());
            }
            Self::Trace(trace_id) => {
                OpCode::Trace.write_into(target);
                target.write_u32(trace_id.expect_value());
            }
        }
    }
}
