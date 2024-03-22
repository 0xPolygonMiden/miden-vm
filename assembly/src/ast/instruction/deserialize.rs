use crate::{
    ast::{AdviceInjectorNode, DebugOptions, Instruction, InvocationTarget, OpCode},
    ByteReader, Deserializable, DeserializationError, Felt, MAX_PUSH_INPUTS,
};

impl Deserializable for Instruction {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let opcode = OpCode::read_from(source)?;

        match opcode {
            OpCode::Assert => Ok(Self::Assert),
            OpCode::AssertWithError => Ok(Self::AssertWithError(source.read_u32()?.into())),
            OpCode::AssertEq => Ok(Self::AssertEq),
            OpCode::AssertEqWithError => Ok(Self::AssertEqWithError(source.read_u32()?.into())),
            OpCode::AssertEqw => Ok(Self::AssertEqw),
            OpCode::AssertEqwWithError => Ok(Self::AssertEqwWithError(source.read_u32()?.into())),
            OpCode::Assertz => Ok(Self::Assertz),
            OpCode::AssertzWithError => Ok(Self::AssertzWithError(source.read_u32()?.into())),
            OpCode::Add => Ok(Self::Add),
            OpCode::AddImm => Ok(Self::AddImm(Felt::read_from(source)?.into())),
            OpCode::Sub => Ok(Self::Sub),
            OpCode::SubImm => Ok(Self::SubImm(Felt::read_from(source)?.into())),
            OpCode::Mul => Ok(Self::Mul),
            OpCode::MulImm => Ok(Self::MulImm(Felt::read_from(source)?.into())),
            OpCode::Div => Ok(Self::Div),
            OpCode::DivImm => Ok(Self::DivImm(Felt::read_from(source)?.into())),
            OpCode::Neg => Ok(Self::Neg),
            OpCode::ILog2 => Ok(Self::ILog2),
            OpCode::Inv => Ok(Self::Inv),
            OpCode::Incr => Ok(Self::Incr),
            OpCode::Pow2 => Ok(Self::Pow2),
            OpCode::Exp => Ok(Self::Exp),
            OpCode::ExpImm => Ok(Self::ExpImm(Felt::read_from(source)?.into())),
            OpCode::ExpBitLength => Ok(Self::ExpBitLength(source.read_u8()?)),
            OpCode::Not => Ok(Self::Not),
            OpCode::And => Ok(Self::And),
            OpCode::Or => Ok(Self::Or),
            OpCode::Xor => Ok(Self::Xor),
            OpCode::Eq => Ok(Self::Eq),
            OpCode::EqImm => Ok(Self::EqImm(Felt::read_from(source)?.into())),
            OpCode::Neq => Ok(Self::Neq),
            OpCode::NeqImm => Ok(Self::NeqImm(Felt::read_from(source)?.into())),
            OpCode::Eqw => Ok(Self::Eqw),
            OpCode::Lt => Ok(Self::Lt),
            OpCode::Lte => Ok(Self::Lte),
            OpCode::Gt => Ok(Self::Gt),
            OpCode::Gte => Ok(Self::Gte),
            OpCode::IsOdd => Ok(Self::IsOdd),

            // ----- ext2 operations --------------------------------------------------------------
            OpCode::Ext2Add => Ok(Self::Ext2Add),
            OpCode::Ext2Sub => Ok(Self::Ext2Sub),
            OpCode::Ext2Mul => Ok(Self::Ext2Mul),
            OpCode::Ext2Div => Ok(Self::Ext2Div),
            OpCode::Ext2Neg => Ok(Self::Ext2Neg),
            OpCode::Ext2Inv => Ok(Self::Ext2Inv),

            // ----- u32 manipulation -------------------------------------------------------------
            OpCode::U32Test => Ok(Self::U32Test),
            OpCode::U32TestW => Ok(Self::U32TestW),
            OpCode::U32Assert => Ok(Self::U32Assert),
            OpCode::U32AssertWithError => Ok(Self::U32AssertWithError(source.read_u32()?.into())),
            OpCode::U32Assert2 => Ok(Self::U32Assert2),
            OpCode::U32Assert2WithError => Ok(Self::U32Assert2WithError(source.read_u32()?.into())),
            OpCode::U32AssertW => Ok(Self::U32AssertW),
            OpCode::U32AssertWWithError => Ok(Self::U32AssertWWithError(source.read_u32()?.into())),
            OpCode::U32Split => Ok(Self::U32Split),
            OpCode::U32Cast => Ok(Self::U32Cast),
            OpCode::U32WrappingAdd => Ok(Self::U32WrappingAdd),
            OpCode::U32WrappingAddImm => Ok(Self::U32WrappingAddImm(source.read_u32()?.into())),
            OpCode::U32OverflowingAdd => Ok(Self::U32OverflowingAdd),
            OpCode::U32OverflowingAddImm => {
                Ok(Self::U32OverflowingAddImm(source.read_u32()?.into()))
            }
            OpCode::U32OverflowingAdd3 => Ok(Self::U32OverflowingAdd3),
            OpCode::U32WrappingAdd3 => Ok(Self::U32WrappingAdd3),
            OpCode::U32WrappingSub => Ok(Self::U32WrappingSub),
            OpCode::U32WrappingSubImm => Ok(Self::U32WrappingSubImm(source.read_u32()?.into())),
            OpCode::U32OverflowingSub => Ok(Self::U32OverflowingSub),
            OpCode::U32OverflowingSubImm => {
                Ok(Self::U32OverflowingSubImm(source.read_u32()?.into()))
            }
            OpCode::U32WrappingMul => Ok(Self::U32WrappingMul),
            OpCode::U32WrappingMulImm => Ok(Self::U32WrappingMulImm(source.read_u32()?.into())),
            OpCode::U32OverflowingMul => Ok(Self::U32OverflowingMul),
            OpCode::U32OverflowingMulImm => {
                Ok(Self::U32OverflowingMulImm(source.read_u32()?.into()))
            }
            OpCode::U32OverflowingMadd => Ok(Self::U32OverflowingMadd),
            OpCode::U32WrappingMadd => Ok(Self::U32WrappingMadd),
            OpCode::U32Div => Ok(Self::U32Div),
            OpCode::U32DivImm => Ok(Self::U32DivImm(source.read_u32()?.into())),
            OpCode::U32Mod => Ok(Self::U32Mod),
            OpCode::U32ModImm => Ok(Self::U32ModImm(source.read_u32()?.into())),
            OpCode::U32DivMod => Ok(Self::U32DivMod),
            OpCode::U32DivModImm => Ok(Self::U32DivModImm(source.read_u32()?.into())),
            OpCode::U32And => Ok(Self::U32And),
            OpCode::U32Or => Ok(Self::U32Or),
            OpCode::U32Xor => Ok(Self::U32Xor),
            OpCode::U32Not => Ok(Self::U32Not),
            OpCode::U32Shr => Ok(Self::U32Shr),
            OpCode::U32ShrImm => Ok(Self::U32ShrImm(source.read_u8()?.into())),
            OpCode::U32Shl => Ok(Self::U32Shl),
            OpCode::U32ShlImm => Ok(Self::U32ShlImm(source.read_u8()?.into())),
            OpCode::U32Rotr => Ok(Self::U32Rotr),
            OpCode::U32RotrImm => Ok(Self::U32RotrImm(source.read_u8()?.into())),
            OpCode::U32Rotl => Ok(Self::U32Rotl),
            OpCode::U32RotlImm => Ok(Self::U32RotlImm(source.read_u8()?.into())),
            OpCode::U32Popcnt => Ok(Self::U32Popcnt),
            OpCode::U32Ctz => Ok(Self::U32Ctz),
            OpCode::U32Clz => Ok(Self::U32Clz),
            OpCode::U32Clo => Ok(Self::U32Clo),
            OpCode::U32Cto => Ok(Self::U32Cto),
            OpCode::U32Lt => Ok(Self::U32Lt),
            OpCode::U32Lte => Ok(Self::U32Lte),
            OpCode::U32Gt => Ok(Self::U32Gt),
            OpCode::U32Gte => Ok(Self::U32Gte),
            OpCode::U32Min => Ok(Self::U32Min),
            OpCode::U32Max => Ok(Self::U32Max),

            // ----- stack manipulation -----------------------------------------------------------
            OpCode::Drop => Ok(Self::Drop),
            OpCode::DropW => Ok(Self::DropW),
            OpCode::PadW => Ok(Self::PadW),
            OpCode::Dup0 => Ok(Self::Dup0),
            OpCode::Dup1 => Ok(Self::Dup1),
            OpCode::Dup2 => Ok(Self::Dup2),
            OpCode::Dup3 => Ok(Self::Dup3),
            OpCode::Dup4 => Ok(Self::Dup4),
            OpCode::Dup5 => Ok(Self::Dup5),
            OpCode::Dup6 => Ok(Self::Dup6),
            OpCode::Dup7 => Ok(Self::Dup7),
            OpCode::Dup8 => Ok(Self::Dup8),
            OpCode::Dup9 => Ok(Self::Dup9),
            OpCode::Dup10 => Ok(Self::Dup10),
            OpCode::Dup11 => Ok(Self::Dup11),
            OpCode::Dup12 => Ok(Self::Dup12),
            OpCode::Dup13 => Ok(Self::Dup13),
            OpCode::Dup14 => Ok(Self::Dup14),
            OpCode::Dup15 => Ok(Self::Dup15),
            OpCode::DupW0 => Ok(Self::DupW0),
            OpCode::DupW1 => Ok(Self::DupW1),
            OpCode::DupW2 => Ok(Self::DupW2),
            OpCode::DupW3 => Ok(Self::DupW3),
            OpCode::Swap1 => Ok(Self::Swap1),
            OpCode::Swap2 => Ok(Self::Swap2),
            OpCode::Swap3 => Ok(Self::Swap3),
            OpCode::Swap4 => Ok(Self::Swap4),
            OpCode::Swap5 => Ok(Self::Swap5),
            OpCode::Swap6 => Ok(Self::Swap6),
            OpCode::Swap7 => Ok(Self::Swap7),
            OpCode::Swap8 => Ok(Self::Swap8),
            OpCode::Swap9 => Ok(Self::Swap9),
            OpCode::Swap10 => Ok(Self::Swap10),
            OpCode::Swap11 => Ok(Self::Swap11),
            OpCode::Swap12 => Ok(Self::Swap12),
            OpCode::Swap13 => Ok(Self::Swap13),
            OpCode::Swap14 => Ok(Self::Swap14),
            OpCode::Swap15 => Ok(Self::Swap15),
            OpCode::SwapW1 => Ok(Self::SwapW1),
            OpCode::SwapW2 => Ok(Self::SwapW2),
            OpCode::SwapW3 => Ok(Self::SwapW3),
            OpCode::SwapDW => Ok(Self::SwapDw),
            OpCode::MovUp2 => Ok(Self::MovUp2),
            OpCode::MovUp3 => Ok(Self::MovUp3),
            OpCode::MovUp4 => Ok(Self::MovUp4),
            OpCode::MovUp5 => Ok(Self::MovUp5),
            OpCode::MovUp6 => Ok(Self::MovUp6),
            OpCode::MovUp7 => Ok(Self::MovUp7),
            OpCode::MovUp8 => Ok(Self::MovUp8),
            OpCode::MovUp9 => Ok(Self::MovUp9),
            OpCode::MovUp10 => Ok(Self::MovUp10),
            OpCode::MovUp11 => Ok(Self::MovUp11),
            OpCode::MovUp12 => Ok(Self::MovUp12),
            OpCode::MovUp13 => Ok(Self::MovUp13),
            OpCode::MovUp14 => Ok(Self::MovUp14),
            OpCode::MovUp15 => Ok(Self::MovUp15),
            OpCode::MovUpW2 => Ok(Self::MovUpW2),
            OpCode::MovUpW3 => Ok(Self::MovUpW3),
            OpCode::MovDn2 => Ok(Self::MovDn2),
            OpCode::MovDn3 => Ok(Self::MovDn3),
            OpCode::MovDn4 => Ok(Self::MovDn4),
            OpCode::MovDn5 => Ok(Self::MovDn5),
            OpCode::MovDn6 => Ok(Self::MovDn6),
            OpCode::MovDn7 => Ok(Self::MovDn7),
            OpCode::MovDn8 => Ok(Self::MovDn8),
            OpCode::MovDn9 => Ok(Self::MovDn9),
            OpCode::MovDn10 => Ok(Self::MovDn10),
            OpCode::MovDn11 => Ok(Self::MovDn11),
            OpCode::MovDn12 => Ok(Self::MovDn12),
            OpCode::MovDn13 => Ok(Self::MovDn13),
            OpCode::MovDn14 => Ok(Self::MovDn14),
            OpCode::MovDn15 => Ok(Self::MovDn15),
            OpCode::MovDnW2 => Ok(Self::MovDnW2),
            OpCode::MovDnW3 => Ok(Self::MovDnW3),
            OpCode::CSwap => Ok(Self::CSwap),
            OpCode::CSwapW => Ok(Self::CSwapW),
            OpCode::CDrop => Ok(Self::CDrop),
            OpCode::CDropW => Ok(Self::CDropW),

            // ----- input / output operations ----------------------------------------------------
            OpCode::PushU8 => Ok(Self::PushU8(source.read_u8()?)),
            OpCode::PushU16 => Ok(Self::PushU16(source.read_u16()?)),
            OpCode::PushU32 => Ok(Self::PushU32(source.read_u32()?)),
            OpCode::PushFelt => Ok(Self::PushFelt(Felt::read_from(source)?)),
            OpCode::PushWord => Ok(Self::PushWord([
                Felt::read_from(source)?,
                Felt::read_from(source)?,
                Felt::read_from(source)?,
                Felt::read_from(source)?,
            ])),
            OpCode::PushU8List => {
                let length = parse_num_push_params(source)?;
                (0..length)
                    .map(|_| source.read_u8())
                    .collect::<Result<_, _>>()
                    .map(Self::PushU8List)
            }
            OpCode::PushU16List => {
                let length = parse_num_push_params(source)?;
                (0..length)
                    .map(|_| source.read_u16())
                    .collect::<Result<_, _>>()
                    .map(Self::PushU16List)
            }
            OpCode::PushU32List => {
                let length = parse_num_push_params(source)?;
                (0..length)
                    .map(|_| source.read_u32())
                    .collect::<Result<_, _>>()
                    .map(Self::PushU32List)
            }
            OpCode::PushFeltList => {
                let length = parse_num_push_params(source)?;
                (0..length)
                    .map(|_| Felt::read_from(source))
                    .collect::<Result<_, _>>()
                    .map(Self::PushFeltList)
            }

            OpCode::Locaddr => Ok(Self::Locaddr(source.read_u16()?.into())),
            OpCode::Sdepth => Ok(Self::Sdepth),
            OpCode::Caller => Ok(Self::Caller),
            OpCode::Clk => Ok(Self::Clk),

            OpCode::MemLoad => Ok(Self::MemLoad),
            OpCode::MemLoadImm => Ok(Self::MemLoadImm(source.read_u32()?.into())),
            OpCode::MemLoadW => Ok(Self::MemLoadW),
            OpCode::MemLoadWImm => Ok(Self::MemLoadWImm(source.read_u32()?.into())),
            OpCode::LocLoad => Ok(Self::LocLoad(source.read_u16()?.into())),
            OpCode::LocLoadW => Ok(Self::LocLoadW(source.read_u16()?.into())),
            OpCode::MemStore => Ok(Self::MemStore),
            OpCode::MemStoreImm => Ok(Self::MemStoreImm(source.read_u32()?.into())),
            OpCode::LocStore => Ok(Self::LocStore(source.read_u16()?.into())),
            OpCode::MemStoreW => Ok(Self::MemStoreW),
            OpCode::MemStoreWImm => Ok(Self::MemStoreWImm(source.read_u32()?.into())),
            OpCode::LocStoreW => Ok(Self::LocStoreW(source.read_u16()?.into())),

            OpCode::MemStream => Ok(Self::MemStream),
            OpCode::AdvPipe => Ok(Self::AdvPipe),

            OpCode::AdvPush => Ok(Self::AdvPush(source.read_u8()?.into())),
            OpCode::AdvLoadW => Ok(Self::AdvLoadW),

            OpCode::AdvInject => Ok(Self::AdvInject(AdviceInjectorNode::read_from(source)?)),

            // ----- cryptographic operations -----------------------------------------------------
            OpCode::Hash => Ok(Self::Hash),
            OpCode::HMerge => Ok(Self::HMerge),
            OpCode::HPerm => Ok(Self::HPerm),
            OpCode::MTreeGet => Ok(Self::MTreeGet),
            OpCode::MTreeSet => Ok(Self::MTreeSet),
            OpCode::MTreeMerge => Ok(Self::MTreeMerge),
            OpCode::MTreeVerify => Ok(Self::MTreeVerify),

            // ----- STARK proof verification -----------------------------------------------------
            OpCode::FriExt2Fold4 => Ok(Self::FriExt2Fold4),
            OpCode::RCombBase => Ok(Self::RCombBase),

            // ----- exec / call ------------------------------------------------------------------
            OpCode::Exec => InvocationTarget::read_from(source).map(Self::Exec),
            OpCode::Call => InvocationTarget::read_from(source).map(Self::Call),
            OpCode::SysCall => InvocationTarget::read_from(source).map(Self::SysCall),
            OpCode::DynExec => Ok(Self::DynExec),
            OpCode::DynCall => Ok(Self::DynCall),
            OpCode::ProcRef => InvocationTarget::read_from(source).map(Self::ProcRef),

            // ----- debugging --------------------------------------------------------------------
            OpCode::Debug => Ok(Self::Debug(DebugOptions::read_from(source)?)),

            // ----- event decorators -------------------------------------------------------------
            OpCode::Emit => Ok(Self::Emit(source.read_u32()?.into())),
            OpCode::Trace => Ok(Self::Trace(source.read_u32()?.into())),

            // ----- control flow -----------------------------------------------------------------
            // control flow instructions should be parsed as a part of Op::read_from() and we
            // should never get here
            OpCode::IfElse => unreachable!(),
            OpCode::Repeat => unreachable!(),
            OpCode::While => unreachable!(),
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn parse_num_push_params<R: ByteReader>(source: &mut R) -> Result<u8, DeserializationError> {
    use alloc::string::ToString;

    let length = source.read_u8()? as usize;
    if !(1..=MAX_PUSH_INPUTS).contains(&length) {
        Err(DeserializationError::InvalidValue("invalid push values argument".to_string()))
    } else {
        Ok(length as u8)
    }
}
