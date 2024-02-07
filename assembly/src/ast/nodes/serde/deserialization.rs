use super::{
    super::AdviceInjectorNode, debug, ByteReader, CodeBody, Deserializable, DeserializationError,
    Felt, Instruction, Node, OpCode, ProcedureId, RpoDigest, ToString, MAX_PUSH_INPUTS,
};

// NODE DESERIALIZATION
// ================================================================================================

impl Deserializable for Node {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let first_byte = source.peek_u8()?;

        if first_byte == OpCode::IfElse as u8 {
            source.read_u8()?;

            let if_block_len = source.read_u16()? as usize;
            let nodes = source.read_many::<Node>(if_block_len)?;
            let true_case = CodeBody::new(nodes);

            let else_block_len = source.read_u16()? as usize;
            let nodes = source.read_many::<Node>(else_block_len)?;
            let false_case = CodeBody::new(nodes);

            Ok(Node::IfElse {
                true_case,
                false_case,
            })
        } else if first_byte == OpCode::Repeat as u8 {
            source.read_u8()?;

            let times = source.read_u32()?;

            let nodes_len = source.read_u16()? as usize;
            let nodes = source.read_many::<Node>(nodes_len)?;
            let body = CodeBody::new(nodes);

            Ok(Node::Repeat { times, body })
        } else if first_byte == OpCode::While as u8 {
            source.read_u8()?;

            let nodes_len = source.read_u16()? as usize;
            let nodes = source.read_many::<Node>(nodes_len)?;
            let body = CodeBody::new(nodes);

            Ok(Node::While { body })
        } else {
            let inner = Deserializable::read_from(source)?;
            Ok(Node::Instruction(inner))
        }
    }
}

// INSTRUCTION DESERIALIZATION
// ================================================================================================

impl Deserializable for Instruction {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let opcode = OpCode::read_from(source)?;

        match opcode {
            OpCode::Assert => Ok(Instruction::Assert),
            OpCode::AssertWithError => Ok(Instruction::AssertWithError(source.read_u32()?)),
            OpCode::AssertEq => Ok(Instruction::AssertEq),
            OpCode::AssertEqWithError => Ok(Instruction::AssertEqWithError(source.read_u32()?)),
            OpCode::AssertEqw => Ok(Instruction::AssertEqw),
            OpCode::AssertEqwWithError => Ok(Instruction::AssertEqwWithError(source.read_u32()?)),
            OpCode::Assertz => Ok(Instruction::Assertz),
            OpCode::AssertzWithError => Ok(Instruction::AssertzWithError(source.read_u32()?)),
            OpCode::Add => Ok(Instruction::Add),
            OpCode::AddImm => Ok(Instruction::AddImm(Felt::read_from(source)?)),
            OpCode::Sub => Ok(Instruction::Sub),
            OpCode::SubImm => Ok(Instruction::SubImm(Felt::read_from(source)?)),
            OpCode::Mul => Ok(Instruction::Mul),
            OpCode::MulImm => Ok(Instruction::MulImm(Felt::read_from(source)?)),
            OpCode::Div => Ok(Instruction::Div),
            OpCode::DivImm => Ok(Instruction::DivImm(Felt::read_from(source)?)),
            OpCode::Neg => Ok(Instruction::Neg),
            OpCode::Inv => Ok(Instruction::Inv),
            OpCode::Incr => Ok(Instruction::Incr),
            OpCode::Pow2 => Ok(Instruction::Pow2),
            OpCode::Exp => Ok(Instruction::Exp),
            OpCode::ExpImm => Ok(Instruction::ExpImm(Felt::read_from(source)?)),
            OpCode::ExpBitLength => Ok(Instruction::ExpBitLength(source.read_u8()?)),
            OpCode::Not => Ok(Instruction::Not),
            OpCode::And => Ok(Instruction::And),
            OpCode::Or => Ok(Instruction::Or),
            OpCode::Xor => Ok(Instruction::Xor),
            OpCode::Eq => Ok(Instruction::Eq),
            OpCode::EqImm => Ok(Instruction::EqImm(Felt::read_from(source)?)),
            OpCode::Neq => Ok(Instruction::Neq),
            OpCode::NeqImm => Ok(Instruction::NeqImm(Felt::read_from(source)?)),
            OpCode::Eqw => Ok(Instruction::Eqw),
            OpCode::Lt => Ok(Instruction::Lt),
            OpCode::Lte => Ok(Instruction::Lte),
            OpCode::Gt => Ok(Instruction::Gt),
            OpCode::Gte => Ok(Instruction::Gte),
            OpCode::IsOdd => Ok(Instruction::IsOdd),

            // ----- ext2 operations --------------------------------------------------------------
            OpCode::Ext2Add => Ok(Instruction::Ext2Add),
            OpCode::Ext2Sub => Ok(Instruction::Ext2Sub),
            OpCode::Ext2Mul => Ok(Instruction::Ext2Mul),
            OpCode::Ext2Div => Ok(Instruction::Ext2Div),
            OpCode::Ext2Neg => Ok(Instruction::Ext2Neg),
            OpCode::Ext2Inv => Ok(Instruction::Ext2Inv),

            // ----- u32 manipulation -------------------------------------------------------------
            OpCode::U32Test => Ok(Instruction::U32Test),
            OpCode::U32TestW => Ok(Instruction::U32TestW),
            OpCode::U32Assert => Ok(Instruction::U32Assert),
            OpCode::U32AssertWithError => Ok(Instruction::U32AssertWithError(source.read_u32()?)),
            OpCode::U32Assert2 => Ok(Instruction::U32Assert2),
            OpCode::U32Assert2WithError => Ok(Instruction::U32Assert2WithError(source.read_u32()?)),
            OpCode::U32AssertW => Ok(Instruction::U32AssertW),
            OpCode::U32AssertWWithError => Ok(Instruction::U32AssertWWithError(source.read_u32()?)),
            OpCode::U32Split => Ok(Instruction::U32Split),
            OpCode::U32Cast => Ok(Instruction::U32Cast),
            OpCode::U32WrappingAdd => Ok(Instruction::U32WrappingAdd),
            OpCode::U32WrappingAddImm => Ok(Instruction::U32WrappingAddImm(source.read_u32()?)),
            OpCode::U32OverflowingAdd => Ok(Instruction::U32OverflowingAdd),
            OpCode::U32OverflowingAddImm => {
                Ok(Instruction::U32OverflowingAddImm(source.read_u32()?))
            }
            OpCode::U32OverflowingAdd3 => Ok(Instruction::U32OverflowingAdd3),
            OpCode::U32WrappingAdd3 => Ok(Instruction::U32WrappingAdd3),
            OpCode::U32WrappingSub => Ok(Instruction::U32WrappingSub),
            OpCode::U32WrappingSubImm => Ok(Instruction::U32WrappingSubImm(source.read_u32()?)),
            OpCode::U32OverflowingSub => Ok(Instruction::U32OverflowingSub),
            OpCode::U32OverflowingSubImm => {
                Ok(Instruction::U32OverflowingSubImm(source.read_u32()?))
            }
            OpCode::U32WrappingMul => Ok(Instruction::U32WrappingMul),
            OpCode::U32WrappingMulImm => Ok(Instruction::U32WrappingMulImm(source.read_u32()?)),
            OpCode::U32OverflowingMul => Ok(Instruction::U32OverflowingMul),
            OpCode::U32OverflowingMulImm => {
                Ok(Instruction::U32OverflowingMulImm(source.read_u32()?))
            }
            OpCode::U32OverflowingMadd => Ok(Instruction::U32OverflowingMadd),
            OpCode::U32WrappingMadd => Ok(Instruction::U32WrappingMadd),
            OpCode::U32Div => Ok(Instruction::U32Div),
            OpCode::U32DivImm => Ok(Instruction::U32DivImm(source.read_u32()?)),
            OpCode::U32Mod => Ok(Instruction::U32Mod),
            OpCode::U32ModImm => Ok(Instruction::U32ModImm(source.read_u32()?)),
            OpCode::U32DivMod => Ok(Instruction::U32DivMod),
            OpCode::U32DivModImm => Ok(Instruction::U32DivModImm(source.read_u32()?)),
            OpCode::U32And => Ok(Instruction::U32And),
            OpCode::U32Or => Ok(Instruction::U32Or),
            OpCode::U32Xor => Ok(Instruction::U32Xor),
            OpCode::U32Not => Ok(Instruction::U32Not),
            OpCode::U32Shr => Ok(Instruction::U32Shr),
            OpCode::U32ShrImm => Ok(Instruction::U32ShrImm(source.read_u8()?)),
            OpCode::U32Shl => Ok(Instruction::U32Shl),
            OpCode::U32ShlImm => Ok(Instruction::U32ShlImm(source.read_u8()?)),
            OpCode::U32Rotr => Ok(Instruction::U32Rotr),
            OpCode::U32RotrImm => Ok(Instruction::U32RotrImm(source.read_u8()?)),
            OpCode::U32Rotl => Ok(Instruction::U32Rotl),
            OpCode::U32RotlImm => Ok(Instruction::U32RotlImm(source.read_u8()?)),
            OpCode::U32Popcnt => Ok(Instruction::U32Popcnt),
            OpCode::U32Lt => Ok(Instruction::U32Lt),
            OpCode::U32Lte => Ok(Instruction::U32Lte),
            OpCode::U32Gt => Ok(Instruction::U32Gt),
            OpCode::U32Gte => Ok(Instruction::U32Gte),
            OpCode::U32Min => Ok(Instruction::U32Min),
            OpCode::U32Max => Ok(Instruction::U32Max),

            // ----- stack manipulation -----------------------------------------------------------
            OpCode::Drop => Ok(Instruction::Drop),
            OpCode::DropW => Ok(Instruction::DropW),
            OpCode::PadW => Ok(Instruction::PadW),
            OpCode::Dup0 => Ok(Instruction::Dup0),
            OpCode::Dup1 => Ok(Instruction::Dup1),
            OpCode::Dup2 => Ok(Instruction::Dup2),
            OpCode::Dup3 => Ok(Instruction::Dup3),
            OpCode::Dup4 => Ok(Instruction::Dup4),
            OpCode::Dup5 => Ok(Instruction::Dup5),
            OpCode::Dup6 => Ok(Instruction::Dup6),
            OpCode::Dup7 => Ok(Instruction::Dup7),
            OpCode::Dup8 => Ok(Instruction::Dup8),
            OpCode::Dup9 => Ok(Instruction::Dup9),
            OpCode::Dup10 => Ok(Instruction::Dup10),
            OpCode::Dup11 => Ok(Instruction::Dup11),
            OpCode::Dup12 => Ok(Instruction::Dup12),
            OpCode::Dup13 => Ok(Instruction::Dup13),
            OpCode::Dup14 => Ok(Instruction::Dup14),
            OpCode::Dup15 => Ok(Instruction::Dup15),
            OpCode::DupW0 => Ok(Instruction::DupW0),
            OpCode::DupW1 => Ok(Instruction::DupW1),
            OpCode::DupW2 => Ok(Instruction::DupW2),
            OpCode::DupW3 => Ok(Instruction::DupW3),
            OpCode::Swap1 => Ok(Instruction::Swap1),
            OpCode::Swap2 => Ok(Instruction::Swap2),
            OpCode::Swap3 => Ok(Instruction::Swap3),
            OpCode::Swap4 => Ok(Instruction::Swap4),
            OpCode::Swap5 => Ok(Instruction::Swap5),
            OpCode::Swap6 => Ok(Instruction::Swap6),
            OpCode::Swap7 => Ok(Instruction::Swap7),
            OpCode::Swap8 => Ok(Instruction::Swap8),
            OpCode::Swap9 => Ok(Instruction::Swap9),
            OpCode::Swap10 => Ok(Instruction::Swap10),
            OpCode::Swap11 => Ok(Instruction::Swap11),
            OpCode::Swap12 => Ok(Instruction::Swap12),
            OpCode::Swap13 => Ok(Instruction::Swap13),
            OpCode::Swap14 => Ok(Instruction::Swap14),
            OpCode::Swap15 => Ok(Instruction::Swap15),
            OpCode::SwapW1 => Ok(Instruction::SwapW1),
            OpCode::SwapW2 => Ok(Instruction::SwapW2),
            OpCode::SwapW3 => Ok(Instruction::SwapW3),
            OpCode::SwapDW => Ok(Instruction::SwapDw),
            OpCode::MovUp2 => Ok(Instruction::MovUp2),
            OpCode::MovUp3 => Ok(Instruction::MovUp3),
            OpCode::MovUp4 => Ok(Instruction::MovUp4),
            OpCode::MovUp5 => Ok(Instruction::MovUp5),
            OpCode::MovUp6 => Ok(Instruction::MovUp6),
            OpCode::MovUp7 => Ok(Instruction::MovUp7),
            OpCode::MovUp8 => Ok(Instruction::MovUp8),
            OpCode::MovUp9 => Ok(Instruction::MovUp9),
            OpCode::MovUp10 => Ok(Instruction::MovUp10),
            OpCode::MovUp11 => Ok(Instruction::MovUp11),
            OpCode::MovUp12 => Ok(Instruction::MovUp12),
            OpCode::MovUp13 => Ok(Instruction::MovUp13),
            OpCode::MovUp14 => Ok(Instruction::MovUp14),
            OpCode::MovUp15 => Ok(Instruction::MovUp15),
            OpCode::MovUpW2 => Ok(Instruction::MovUpW2),
            OpCode::MovUpW3 => Ok(Instruction::MovUpW3),
            OpCode::MovDn2 => Ok(Instruction::MovDn2),
            OpCode::MovDn3 => Ok(Instruction::MovDn3),
            OpCode::MovDn4 => Ok(Instruction::MovDn4),
            OpCode::MovDn5 => Ok(Instruction::MovDn5),
            OpCode::MovDn6 => Ok(Instruction::MovDn6),
            OpCode::MovDn7 => Ok(Instruction::MovDn7),
            OpCode::MovDn8 => Ok(Instruction::MovDn8),
            OpCode::MovDn9 => Ok(Instruction::MovDn9),
            OpCode::MovDn10 => Ok(Instruction::MovDn10),
            OpCode::MovDn11 => Ok(Instruction::MovDn11),
            OpCode::MovDn12 => Ok(Instruction::MovDn12),
            OpCode::MovDn13 => Ok(Instruction::MovDn13),
            OpCode::MovDn14 => Ok(Instruction::MovDn14),
            OpCode::MovDn15 => Ok(Instruction::MovDn15),
            OpCode::MovDnW2 => Ok(Instruction::MovDnW2),
            OpCode::MovDnW3 => Ok(Instruction::MovDnW3),
            OpCode::CSwap => Ok(Instruction::CSwap),
            OpCode::CSwapW => Ok(Instruction::CSwapW),
            OpCode::CDrop => Ok(Instruction::CDrop),
            OpCode::CDropW => Ok(Instruction::CDropW),

            // ----- input / output operations ----------------------------------------------------
            OpCode::PushU8 => Ok(Instruction::PushU8(source.read_u8()?)),
            OpCode::PushU16 => Ok(Instruction::PushU16(source.read_u16()?)),
            OpCode::PushU32 => Ok(Instruction::PushU32(source.read_u32()?)),
            OpCode::PushFelt => Ok(Instruction::PushFelt(Felt::read_from(source)?)),
            OpCode::PushWord => Ok(Instruction::PushWord([
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
                    .map(Instruction::PushU8List)
            }
            OpCode::PushU16List => {
                let length = parse_num_push_params(source)?;
                (0..length)
                    .map(|_| source.read_u16())
                    .collect::<Result<_, _>>()
                    .map(Instruction::PushU16List)
            }
            OpCode::PushU32List => {
                let length = parse_num_push_params(source)?;
                (0..length)
                    .map(|_| source.read_u32())
                    .collect::<Result<_, _>>()
                    .map(Instruction::PushU32List)
            }
            OpCode::PushFeltList => {
                let length = parse_num_push_params(source)?;
                (0..length)
                    .map(|_| Felt::read_from(source))
                    .collect::<Result<_, _>>()
                    .map(Instruction::PushFeltList)
            }

            OpCode::Locaddr => Ok(Instruction::Locaddr(source.read_u16()?)),
            OpCode::Sdepth => Ok(Instruction::Sdepth),
            OpCode::Caller => Ok(Instruction::Caller),
            OpCode::Clk => Ok(Instruction::Clk),

            OpCode::MemLoad => Ok(Instruction::MemLoad),
            OpCode::MemLoadImm => Ok(Instruction::MemLoadImm(source.read_u32()?)),
            OpCode::MemLoadW => Ok(Instruction::MemLoadW),
            OpCode::MemLoadWImm => Ok(Instruction::MemLoadWImm(source.read_u32()?)),
            OpCode::LocLoad => Ok(Instruction::LocLoad(source.read_u16()?)),
            OpCode::LocLoadW => Ok(Instruction::LocLoadW(source.read_u16()?)),
            OpCode::MemStore => Ok(Instruction::MemStore),
            OpCode::MemStoreImm => Ok(Instruction::MemStoreImm(source.read_u32()?)),
            OpCode::LocStore => Ok(Instruction::LocStore(source.read_u16()?)),
            OpCode::MemStoreW => Ok(Instruction::MemStoreW),
            OpCode::MemStoreWImm => Ok(Instruction::MemStoreWImm(source.read_u32()?)),
            OpCode::LocStoreW => Ok(Instruction::LocStoreW(source.read_u16()?)),

            OpCode::MemStream => Ok(Instruction::MemStream),
            OpCode::AdvPipe => Ok(Instruction::AdvPipe),

            OpCode::AdvPush => Ok(Instruction::AdvPush(source.read_u8()?)),
            OpCode::AdvLoadW => Ok(Instruction::AdvLoadW),

            OpCode::AdvInject => Ok(Instruction::AdvInject(AdviceInjectorNode::read_from(source)?)),

            // ----- cryptographic operations -----------------------------------------------------
            OpCode::Hash => Ok(Instruction::Hash),
            OpCode::HMerge => Ok(Instruction::HMerge),
            OpCode::HPerm => Ok(Instruction::HPerm),
            OpCode::MTreeGet => Ok(Instruction::MTreeGet),
            OpCode::MTreeSet => Ok(Instruction::MTreeSet),
            OpCode::MTreeMerge => Ok(Instruction::MTreeMerge),
            OpCode::MTreeVerify => Ok(Instruction::MTreeVerify),

            // ----- STARK proof verification -----------------------------------------------------
            OpCode::FriExt2Fold4 => Ok(Instruction::FriExt2Fold4),
            OpCode::RCombBase => Ok(Instruction::RCombBase),

            // ----- exec / call ------------------------------------------------------------------
            OpCode::ExecLocal => Ok(Instruction::ExecLocal(source.read_u16()?)),
            OpCode::ExecImported => Ok(Instruction::ExecImported(ProcedureId::read_from(source)?)),
            OpCode::CallLocal => Ok(Instruction::CallLocal(source.read_u16()?)),
            OpCode::CallMastRoot => Ok(Instruction::CallMastRoot(RpoDigest::read_from(source)?)),
            OpCode::CallImported => Ok(Instruction::CallImported(ProcedureId::read_from(source)?)),
            OpCode::SysCall => Ok(Instruction::SysCall(ProcedureId::read_from(source)?)),
            OpCode::DynExec => Ok(Instruction::DynExec),
            OpCode::DynCall => Ok(Instruction::DynCall),
            OpCode::ProcRefLocal => Ok(Instruction::ProcRefLocal(source.read_u16()?)),
            OpCode::ProcRefImported => {
                Ok(Instruction::ProcRefImported(ProcedureId::read_from(source)?))
            }

            // ----- debugging --------------------------------------------------------------------
            OpCode::Debug => {
                let options = debug::read_options_from(source)?;
                Ok(Instruction::Debug(options))
            }

            // ----- event decorators -------------------------------------------------------------
            OpCode::Emit => Ok(Instruction::Emit(source.read_u32()?)),
            OpCode::Trace => Ok(Instruction::Trace(source.read_u32()?)),

            // ----- control flow -----------------------------------------------------------------
            // control flow instructions should be parsed as a part of Node::read_from() and we
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
    let length = source.read_u8()? as usize;
    if !(1..=MAX_PUSH_INPUTS).contains(&length) {
        Err(DeserializationError::InvalidValue("invalid push values argument".to_string()))
    } else {
        Ok(length as u8)
    }
}
