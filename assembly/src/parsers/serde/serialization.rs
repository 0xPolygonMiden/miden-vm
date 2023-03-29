use super::{ByteWriter, Instruction, Node, OpCode, Serializable, SerializationError};

// NODE SERIALIZATION
// ================================================================================================

impl Serializable for Node {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        match self {
            Self::Instruction(i) => i.write_into(target),
            Self::IfElse(if_clause, else_clause) => {
                OpCode::IfElse.write_into(target)?;
                if_clause.write_into(target)?;
                else_clause.write_into(target)
            }
            Self::Repeat(times, nodes) => {
                OpCode::Repeat.write_into(target)?;
                target.write_u32(*times);
                nodes.write_into(target)
            }
            Self::While(nodes) => {
                OpCode::While.write_into(target)?;
                nodes.write_into(target)
            }
        }
    }
}

// INSTRUCTION SERIALIZATION
// ================================================================================================

impl Serializable for Instruction {
    fn write_into(&self, target: &mut ByteWriter) -> Result<(), SerializationError> {
        match self {
            Self::Assert => OpCode::Assert.write_into(target)?,
            Self::AssertEq => OpCode::AssertEq.write_into(target)?,
            Self::AssertEqw => OpCode::AssertEqw.write_into(target)?,
            Self::Assertz => OpCode::Assertz.write_into(target)?,
            Self::Add => OpCode::Add.write_into(target)?,
            Self::AddImm(v) => {
                OpCode::AddImm.write_into(target)?;
                target.write_felt(*v);
            }
            Self::Sub => OpCode::Sub.write_into(target)?,
            Self::SubImm(v) => {
                OpCode::SubImm.write_into(target)?;
                target.write_felt(*v);
            }
            Self::Mul => OpCode::Mul.write_into(target)?,
            Self::MulImm(v) => {
                OpCode::MulImm.write_into(target)?;
                target.write_felt(*v);
            }
            Self::Div => OpCode::Div.write_into(target)?,
            Self::DivImm(v) => {
                OpCode::DivImm.write_into(target)?;
                target.write_felt(*v);
            }
            Self::Neg => OpCode::Neg.write_into(target)?,
            Self::Inv => OpCode::Inv.write_into(target)?,
            Self::Incr => OpCode::Incr.write_into(target)?,
            Self::Pow2 => OpCode::Pow2.write_into(target)?,
            Self::Exp => OpCode::Exp.write_into(target)?,
            Self::ExpImm(v) => {
                OpCode::ExpImm.write_into(target)?;
                target.write_felt(*v);
            }
            Self::ExpBitLength(v) => {
                OpCode::ExpBitLength.write_into(target)?;
                target.write_u8(*v);
            }
            Self::Not => OpCode::Not.write_into(target)?,
            Self::And => OpCode::And.write_into(target)?,
            Self::Or => OpCode::Or.write_into(target)?,
            Self::Xor => OpCode::Xor.write_into(target)?,
            Self::Eq => OpCode::Eq.write_into(target)?,
            Self::EqImm(v) => {
                OpCode::EqImm.write_into(target)?;
                target.write_felt(*v);
            }
            Self::Neq => OpCode::Neq.write_into(target)?,
            Self::NeqImm(v) => {
                OpCode::NeqImm.write_into(target)?;
                target.write_felt(*v);
            }
            Self::Eqw => OpCode::Eqw.write_into(target)?,
            Self::Lt => OpCode::Lt.write_into(target)?,
            Self::Lte => OpCode::Lte.write_into(target)?,
            Self::Gt => OpCode::Gt.write_into(target)?,
            Self::Gte => OpCode::Gte.write_into(target)?,
            Self::IsOdd => OpCode::IsOdd.write_into(target)?,

            // ----- ext2 operations --------------------------------------------------------------
            Self::Ext2Add => OpCode::Ext2Add.write_into(target)?,
            Self::Ext2Sub => OpCode::Ext2Sub.write_into(target)?,
            Self::Ext2Mul => OpCode::Ext2Mul.write_into(target)?,
            Self::Ext2Div => OpCode::Ext2Div.write_into(target)?,
            Self::Ext2Neg => OpCode::Ext2Neg.write_into(target)?,
            Self::Ext2Inv => OpCode::Ext2Inv.write_into(target)?,

            // ----- u32 operations ---------------------------------------------------------------
            Self::U32Test => OpCode::U32Test.write_into(target)?,
            Self::U32TestW => OpCode::U32TestW.write_into(target)?,
            Self::U32Assert => OpCode::U32Assert.write_into(target)?,
            Self::U32Assert2 => OpCode::U32Assert2.write_into(target)?,
            Self::U32AssertW => OpCode::U32AssertW.write_into(target)?,
            Self::U32Split => OpCode::U32Split.write_into(target)?,
            Self::U32Cast => OpCode::U32Cast.write_into(target)?,
            Self::U32CheckedAdd => OpCode::U32CheckedAdd.write_into(target)?,
            Self::U32CheckedAddImm(v) => {
                OpCode::U32CheckedAddImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32WrappingAdd => OpCode::U32WrappingAdd.write_into(target)?,
            Self::U32WrappingAddImm(v) => {
                OpCode::U32WrappingAddImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32OverflowingAdd => OpCode::U32OverflowingAdd.write_into(target)?,
            Self::U32OverflowingAddImm(v) => {
                OpCode::U32OverflowingAddImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32OverflowingAdd3 => OpCode::U32OverflowingAdd3.write_into(target)?,
            Self::U32WrappingAdd3 => OpCode::U32WrappingAdd3.write_into(target)?,
            Self::U32CheckedSub => OpCode::U32CheckedSub.write_into(target)?,
            Self::U32CheckedSubImm(v) => {
                OpCode::U32CheckedSubImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32WrappingSub => OpCode::U32WrappingSub.write_into(target)?,
            Self::U32WrappingSubImm(v) => {
                OpCode::U32WrappingSubImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32OverflowingSub => OpCode::U32OverflowingSub.write_into(target)?,
            Self::U32OverflowingSubImm(v) => {
                OpCode::U32OverflowingSubImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32CheckedMul => OpCode::U32CheckedMul.write_into(target)?,
            Self::U32CheckedMulImm(v) => {
                OpCode::U32CheckedMulImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32WrappingMul => OpCode::U32WrappingMul.write_into(target)?,
            Self::U32WrappingMulImm(v) => {
                OpCode::U32WrappingMulImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32OverflowingMul => OpCode::U32OverflowingMul.write_into(target)?,
            Self::U32OverflowingMulImm(v) => {
                OpCode::U32OverflowingMulImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32OverflowingMadd => OpCode::U32OverflowingMadd.write_into(target)?,
            Self::U32WrappingMadd => OpCode::U32WrappingMadd.write_into(target)?,
            Self::U32CheckedDiv => OpCode::U32CheckedDiv.write_into(target)?,
            Self::U32CheckedDivImm(v) => {
                OpCode::U32CheckedDivImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32UncheckedDiv => OpCode::U32UncheckedDiv.write_into(target)?,
            Self::U32UncheckedDivImm(v) => {
                OpCode::U32UncheckedDivImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32CheckedMod => OpCode::U32CheckedMod.write_into(target)?,
            Self::U32CheckedModImm(v) => {
                OpCode::U32CheckedModImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32UncheckedMod => OpCode::U32UncheckedMod.write_into(target)?,
            Self::U32UncheckedModImm(v) => {
                OpCode::U32UncheckedModImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32CheckedDivMod => OpCode::U32CheckedDivMod.write_into(target)?,
            Self::U32CheckedDivModImm(v) => {
                OpCode::U32CheckedDivModImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32UncheckedDivMod => OpCode::U32UncheckedDivMod.write_into(target)?,
            Self::U32UncheckedDivModImm(v) => {
                OpCode::U32UncheckedDivModImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32CheckedAnd => OpCode::U32CheckedAnd.write_into(target)?,
            Self::U32CheckedOr => OpCode::U32CheckedOr.write_into(target)?,
            Self::U32CheckedXor => OpCode::U32CheckedXor.write_into(target)?,
            Self::U32CheckedNot => OpCode::U32CheckedNot.write_into(target)?,
            Self::U32CheckedShr => OpCode::U32CheckedShr.write_into(target)?,
            Self::U32CheckedShrImm(v) => {
                OpCode::U32CheckedShrImm.write_into(target)?;
                target.write_u8(*v);
            }
            Self::U32UncheckedShr => OpCode::U32UncheckedShr.write_into(target)?,
            Self::U32UncheckedShrImm(v) => {
                OpCode::U32UncheckedShrImm.write_into(target)?;
                target.write_u8(*v);
            }
            Self::U32CheckedShl => OpCode::U32CheckedShl.write_into(target)?,
            Self::U32CheckedShlImm(v) => {
                OpCode::U32CheckedShlImm.write_into(target)?;
                target.write_u8(*v);
            }
            Self::U32UncheckedShl => OpCode::U32UncheckedShl.write_into(target)?,
            Self::U32UncheckedShlImm(v) => {
                OpCode::U32UncheckedShlImm.write_into(target)?;
                target.write_u8(*v);
            }
            Self::U32CheckedRotr => OpCode::U32CheckedRotr.write_into(target)?,
            Self::U32CheckedRotrImm(v) => {
                OpCode::U32CheckedRotrImm.write_into(target)?;
                target.write_u8(*v);
            }
            Self::U32UncheckedRotr => OpCode::U32UncheckedRotr.write_into(target)?,
            Self::U32UncheckedRotrImm(v) => {
                OpCode::U32UncheckedRotrImm.write_into(target)?;
                target.write_u8(*v);
            }
            Self::U32CheckedRotl => OpCode::U32CheckedRotl.write_into(target)?,
            Self::U32CheckedRotlImm(v) => {
                OpCode::U32CheckedRotlImm.write_into(target)?;
                target.write_u8(*v);
            }
            Self::U32UncheckedRotl => OpCode::U32UncheckedRotl.write_into(target)?,
            Self::U32UncheckedRotlImm(v) => {
                OpCode::U32UncheckedRotlImm.write_into(target)?;
                target.write_u8(*v);
            }
            Self::U32CheckedPopcnt => OpCode::U32CheckedPopcnt.write_into(target)?,
            Self::U32UncheckedPopcnt => OpCode::U32UncheckedPopcnt.write_into(target)?,
            Self::U32CheckedEq => OpCode::U32CheckedEq.write_into(target)?,
            Self::U32CheckedEqImm(v) => {
                OpCode::U32CheckedEqImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32CheckedNeq => OpCode::U32CheckedNeq.write_into(target)?,
            Self::U32CheckedNeqImm(v) => {
                OpCode::U32CheckedNeqImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::U32CheckedLt => OpCode::U32CheckedLt.write_into(target)?,
            Self::U32UncheckedLt => OpCode::U32UncheckedLt.write_into(target)?,
            Self::U32CheckedLte => OpCode::U32CheckedLte.write_into(target)?,
            Self::U32UncheckedLte => OpCode::U32UncheckedLte.write_into(target)?,
            Self::U32CheckedGt => OpCode::U32CheckedGt.write_into(target)?,
            Self::U32UncheckedGt => OpCode::U32UncheckedGt.write_into(target)?,
            Self::U32CheckedGte => OpCode::U32CheckedGte.write_into(target)?,
            Self::U32UncheckedGte => OpCode::U32UncheckedGte.write_into(target)?,
            Self::U32CheckedMin => OpCode::U32CheckedMin.write_into(target)?,
            Self::U32UncheckedMin => OpCode::U32UncheckedMin.write_into(target)?,
            Self::U32CheckedMax => OpCode::U32CheckedMax.write_into(target)?,
            Self::U32UncheckedMax => OpCode::U32UncheckedMax.write_into(target)?,

            // ----- stack manipulation ---------------------------------------------------------------
            Self::Drop => OpCode::Drop.write_into(target)?,
            Self::DropW => OpCode::DropW.write_into(target)?,
            Self::PadW => OpCode::PadW.write_into(target)?,
            Self::Dup0 => OpCode::Dup0.write_into(target)?,
            Self::Dup1 => OpCode::Dup1.write_into(target)?,
            Self::Dup2 => OpCode::Dup2.write_into(target)?,
            Self::Dup3 => OpCode::Dup3.write_into(target)?,
            Self::Dup4 => OpCode::Dup4.write_into(target)?,
            Self::Dup5 => OpCode::Dup5.write_into(target)?,
            Self::Dup6 => OpCode::Dup6.write_into(target)?,
            Self::Dup7 => OpCode::Dup7.write_into(target)?,
            Self::Dup8 => OpCode::Dup8.write_into(target)?,
            Self::Dup9 => OpCode::Dup9.write_into(target)?,
            Self::Dup10 => OpCode::Dup10.write_into(target)?,
            Self::Dup11 => OpCode::Dup11.write_into(target)?,
            Self::Dup12 => OpCode::Dup12.write_into(target)?,
            Self::Dup13 => OpCode::Dup13.write_into(target)?,
            Self::Dup14 => OpCode::Dup14.write_into(target)?,
            Self::Dup15 => OpCode::Dup15.write_into(target)?,
            Self::DupW0 => OpCode::DupW0.write_into(target)?,
            Self::DupW1 => OpCode::DupW1.write_into(target)?,
            Self::DupW2 => OpCode::DupW2.write_into(target)?,
            Self::DupW3 => OpCode::DupW3.write_into(target)?,
            Self::Swap1 => OpCode::Swap1.write_into(target)?,
            Self::Swap2 => OpCode::Swap2.write_into(target)?,
            Self::Swap3 => OpCode::Swap3.write_into(target)?,
            Self::Swap4 => OpCode::Swap4.write_into(target)?,
            Self::Swap5 => OpCode::Swap5.write_into(target)?,
            Self::Swap6 => OpCode::Swap6.write_into(target)?,
            Self::Swap7 => OpCode::Swap7.write_into(target)?,
            Self::Swap8 => OpCode::Swap8.write_into(target)?,
            Self::Swap9 => OpCode::Swap9.write_into(target)?,
            Self::Swap10 => OpCode::Swap10.write_into(target)?,
            Self::Swap11 => OpCode::Swap11.write_into(target)?,
            Self::Swap12 => OpCode::Swap12.write_into(target)?,
            Self::Swap13 => OpCode::Swap13.write_into(target)?,
            Self::Swap14 => OpCode::Swap14.write_into(target)?,
            Self::Swap15 => OpCode::Swap15.write_into(target)?,
            Self::SwapW1 => OpCode::SwapW1.write_into(target)?,
            Self::SwapW2 => OpCode::SwapW2.write_into(target)?,
            Self::SwapW3 => OpCode::SwapW3.write_into(target)?,
            Self::SwapDw => OpCode::SwapDW.write_into(target)?,
            Self::MovUp2 => OpCode::MovUp2.write_into(target)?,
            Self::MovUp3 => OpCode::MovUp3.write_into(target)?,
            Self::MovUp4 => OpCode::MovUp4.write_into(target)?,
            Self::MovUp5 => OpCode::MovUp5.write_into(target)?,
            Self::MovUp6 => OpCode::MovUp6.write_into(target)?,
            Self::MovUp7 => OpCode::MovUp7.write_into(target)?,
            Self::MovUp8 => OpCode::MovUp8.write_into(target)?,
            Self::MovUp9 => OpCode::MovUp9.write_into(target)?,
            Self::MovUp10 => OpCode::MovUp10.write_into(target)?,
            Self::MovUp11 => OpCode::MovUp11.write_into(target)?,
            Self::MovUp12 => OpCode::MovUp12.write_into(target)?,
            Self::MovUp13 => OpCode::MovUp13.write_into(target)?,
            Self::MovUp14 => OpCode::MovUp14.write_into(target)?,
            Self::MovUp15 => OpCode::MovUp15.write_into(target)?,
            Self::MovUpW2 => OpCode::MovUpW2.write_into(target)?,
            Self::MovUpW3 => OpCode::MovUpW3.write_into(target)?,
            Self::MovDn2 => OpCode::MovDn2.write_into(target)?,
            Self::MovDn3 => OpCode::MovDn3.write_into(target)?,
            Self::MovDn4 => OpCode::MovDn4.write_into(target)?,
            Self::MovDn5 => OpCode::MovDn5.write_into(target)?,
            Self::MovDn6 => OpCode::MovDn6.write_into(target)?,
            Self::MovDn7 => OpCode::MovDn7.write_into(target)?,
            Self::MovDn8 => OpCode::MovDn8.write_into(target)?,
            Self::MovDn9 => OpCode::MovDn9.write_into(target)?,
            Self::MovDn10 => OpCode::MovDn10.write_into(target)?,
            Self::MovDn11 => OpCode::MovDn11.write_into(target)?,
            Self::MovDn12 => OpCode::MovDn12.write_into(target)?,
            Self::MovDn13 => OpCode::MovDn13.write_into(target)?,
            Self::MovDn14 => OpCode::MovDn14.write_into(target)?,
            Self::MovDn15 => OpCode::MovDn15.write_into(target)?,
            Self::MovDnW2 => OpCode::MovDnW2.write_into(target)?,
            Self::MovDnW3 => OpCode::MovDnW3.write_into(target)?,
            Self::CSwap => OpCode::CSwap.write_into(target)?,
            Self::CSwapW => OpCode::CSwapW.write_into(target)?,
            Self::CDrop => OpCode::CDrop.write_into(target)?,
            Self::CDropW => OpCode::CDropW.write_into(target)?,

            // ----- input / output operations --------------------------------------------------------
            Self::PushU8(value) => {
                OpCode::PushU8.write_into(target)?;
                target.write_u8(*value);
            }
            Self::PushU16(value) => {
                OpCode::PushU16.write_into(target)?;
                target.write_u16(*value);
            }
            Self::PushU32(value) => {
                OpCode::PushU32.write_into(target)?;
                target.write_u32(*value);
            }
            Self::PushFelt(value) => {
                OpCode::PushFelt.write_into(target)?;
                target.write_felt(*value);
            }
            Self::PushWord(values) => {
                OpCode::PushWord.write_into(target)?;
                values.iter().for_each(|&v| target.write_felt(v));
            }
            Self::PushU8List(values) => {
                OpCode::PushU8List.write_into(target)?;
                target.write_u8(values.len() as u8);
                values.iter().for_each(|&v| target.write_u8(v));
            }
            Self::PushU16List(values) => {
                OpCode::PushU16List.write_into(target)?;
                target.write_u8(values.len() as u8);
                values.iter().for_each(|&v| target.write_u16(v));
            }
            Self::PushU32List(values) => {
                OpCode::PushU32List.write_into(target)?;
                target.write_u8(values.len() as u8);
                values.iter().for_each(|&v| target.write_u32(v));
            }
            Self::PushFeltList(values) => {
                OpCode::PushFeltList.write_into(target)?;
                target.write_u8(values.len() as u8);
                values.iter().for_each(|&v| target.write_felt(v));
            }
            Self::Locaddr(v) => {
                OpCode::Locaddr.write_into(target)?;
                target.write_u16(*v);
            }
            Self::Sdepth => OpCode::Sdepth.write_into(target)?,
            Self::Caller => OpCode::Caller.write_into(target)?,
            Self::Clk => OpCode::Clk.write_into(target)?,

            Self::MemLoad => OpCode::MemLoad.write_into(target)?,
            Self::MemLoadImm(v) => {
                OpCode::MemLoadImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::MemLoadW => OpCode::MemLoadW.write_into(target)?,
            Self::MemLoadWImm(v) => {
                OpCode::MemLoadWImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::LocLoad(v) => {
                OpCode::LocLoad.write_into(target)?;
                target.write_u16(*v);
            }
            Self::LocLoadW(v) => {
                OpCode::LocLoadW.write_into(target)?;
                target.write_u16(*v);
            }
            Self::MemStore => OpCode::MemStore.write_into(target)?,
            Self::MemStoreImm(v) => {
                OpCode::MemStoreImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::LocStore(v) => {
                OpCode::LocStore.write_into(target)?;
                target.write_u16(*v);
            }
            Self::MemStoreW => OpCode::MemStoreW.write_into(target)?,
            Self::MemStoreWImm(v) => {
                OpCode::MemStoreWImm.write_into(target)?;
                target.write_u32(*v);
            }
            Self::LocStoreW(v) => {
                OpCode::LocStoreW.write_into(target)?;
                target.write_u16(*v);
            }

            Self::MemStream => OpCode::MemStream.write_into(target)?,
            Self::AdvPipe => OpCode::AdvPipe.write_into(target)?,

            Self::AdvU64Div => OpCode::AdvU64Div.write_into(target)?,
            Self::AdvKeyval => OpCode::AdvKeyval.write_into(target)?,
            Self::AdvMem(start_addr, num_words) => {
                OpCode::AdvMem.write_into(target)?;
                target.write_u32(*start_addr);
                target.write_u32(*num_words);
            }
            Self::AdvPush(v) => {
                OpCode::AdvPush.write_into(target)?;
                target.write_u8(*v);
            }
            Self::AdvLoadW => OpCode::AdvLoadW.write_into(target)?,
            Self::AdvExt2Inv => OpCode::AdvExt2Inv.write_into(target)?,
            Self::AdvExt2INTT => OpCode::AdvExt2INTT.write_into(target)?,

            // ----- cryptographic operations -----------------------------------------------------
            Self::Hash => OpCode::Hash.write_into(target)?,
            Self::HMerge => OpCode::HMerge.write_into(target)?,
            Self::HPerm => OpCode::HPerm.write_into(target)?,
            Self::MTreeGet => OpCode::MTreeGet.write_into(target)?,
            Self::MTreeSet => OpCode::MTreeSet.write_into(target)?,
            Self::MTreeMerge => OpCode::MTreeMerge.write_into(target)?,
            Self::FriExt2Fold4 => OpCode::FriExt2Fold4.write_into(target)?,

            // ----- exec / call ------------------------------------------------------------------
            Self::ExecLocal(v) => {
                OpCode::ExecLocal.write_into(target)?;
                target.write_u16(*v);
            }
            Self::ExecImported(imported) => {
                OpCode::ExecImported.write_into(target)?;
                imported.write_into(target)?
            }
            Self::CallLocal(v) => {
                OpCode::CallLocal.write_into(target)?;
                target.write_u16(*v);
            }
            Self::CallImported(imported) => {
                OpCode::CallImported.write_into(target)?;
                imported.write_into(target)?
            }
            Self::SysCall(imported) => {
                OpCode::SysCall.write_into(target)?;
                imported.write_into(target)?
            }

            // ----- debug decorators -------------------------------------------------------------
            Self::Breakpoint => {
                // this is a transparent instruction and will not be encoded into the library
            }
        }
        Ok(())
    }
}
