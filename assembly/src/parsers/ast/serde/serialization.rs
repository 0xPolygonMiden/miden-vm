use super::{
    super::nodes::{Instruction, Node},
    OpCode, IF_ELSE_OPCODE, REPEAT_OPCODE, WHILE_OPCODE,
};
use crate::{errors::SerializationError, ProcedureId};
use vm_core::{utils::collections::Vec, utils::string::String, Felt, StarkField};

const MAX_STRING_LENGTH: u8 = 100;

// BYTE WRITER IMPLEMENTATION
// ================================================================================================

/// Contains a vector for storing serialized objects
pub struct ByteWriter(Vec<u8>);

impl ByteWriter {
    pub fn new() -> Self {
        let vec_bytes = Vec::new();
        Self(vec_bytes)
    }

    pub fn write_bool(&mut self, val: bool) {
        self.write_u8(val as u8);
    }

    pub fn write_u8(&mut self, val: u8) {
        self.0.push(val);
    }

    pub fn write_u16(&mut self, val: u16) {
        self.0.append(&mut val.to_le_bytes().to_vec());
    }

    pub fn write_u32(&mut self, val: u32) {
        self.0.append(&mut val.to_le_bytes().to_vec());
    }

    pub fn write_u64(&mut self, val: u64) {
        self.0.append(&mut val.to_le_bytes().to_vec());
    }

    pub fn write_string(&mut self, val: &String) -> Result<(), SerializationError> {
        let val_bytes = val.as_bytes();
        let val_bytes_len = val_bytes.len() as u8;
        if val_bytes_len > MAX_STRING_LENGTH {
            return Err(SerializationError::StringTooLong);
        } else {
            self.write_u8(val_bytes_len);
            self.0.append(&mut val_bytes.to_vec());
        }
        Ok(())
    }

    pub fn write_procedure_id(&mut self, val: &ProcedureId) {
        self.0.append(&mut val.to_vec());
    }

    pub fn write_felt(&mut self, val: Felt) {
        self.write_u64(val.as_int());
    }

    pub fn write_opcode(&mut self, val: OpCode) {
        self.write_u8(val as u8);
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

// SERIALIZABLE TRAIT IMPLEMENTATIONS
// ================================================================================================

/// Converts `self` into bytes and writes them to the provided `ByteWriter` struct
pub trait Serializable: Sized {
    fn write_into(&self, target: &mut ByteWriter);
}

impl Serializable for Vec<Node> {
    fn write_into(&self, target: &mut ByteWriter) {
        target.write_u16(self.len() as u16);

        for node in self {
            node.write_into(target);
        }
    }
}

impl Serializable for Node {
    fn write_into(&self, target: &mut ByteWriter) {
        match self {
            Self::Instruction(i) => {
                i.write_into(target);
            }
            Self::IfElse(if_clause, else_clause) => {
                target.write_u8(IF_ELSE_OPCODE);

                if_clause.write_into(target);

                else_clause.write_into(target);
            }
            Self::Repeat(times, nodes) => {
                target.write_u8(REPEAT_OPCODE);

                target.write_u16(*times as u16);

                nodes.write_into(target);
            }
            Self::While(nodes) => {
                target.write_u8(WHILE_OPCODE);

                nodes.write_into(target);
            }
        };
    }
}

impl Serializable for Instruction {
    fn write_into(&self, target: &mut ByteWriter) {
        match self {
            Self::Assert => target.write_opcode(OpCode::Assert),
            Self::AssertEq => target.write_opcode(OpCode::AssertEq),
            Self::Assertz => target.write_opcode(OpCode::Assertz),
            Self::Add => target.write_opcode(OpCode::Add),
            Self::AddImm(v) => {
                target.write_opcode(OpCode::AddImm);
                target.write_felt(*v);
            }
            Self::Sub => target.write_opcode(OpCode::Sub),
            Self::SubImm(v) => {
                target.write_opcode(OpCode::SubImm);
                target.write_felt(*v);
            }
            Self::Mul => target.write_opcode(OpCode::Mul),
            Self::MulImm(v) => {
                target.write_opcode(OpCode::MulImm);
                target.write_felt(*v);
            }
            Self::Div => target.write_opcode(OpCode::Div),
            Self::DivImm(v) => {
                target.write_opcode(OpCode::DivImm);
                target.write_felt(*v);
            }
            Self::Neg => target.write_opcode(OpCode::Neg),
            Self::Inv => target.write_opcode(OpCode::Inv),
            Self::Pow2 => target.write_opcode(OpCode::Pow2),
            Self::Exp => target.write_opcode(OpCode::Exp),
            Self::Not => target.write_opcode(OpCode::Not),
            Self::And => target.write_opcode(OpCode::And),
            Self::Or => target.write_opcode(OpCode::Or),
            Self::Xor => target.write_opcode(OpCode::Xor),
            Self::Eq => target.write_opcode(OpCode::Eq),
            Self::EqImm(v) => {
                target.write_opcode(OpCode::EqImm);
                target.write_felt(*v);
            }
            Self::Neq => target.write_opcode(OpCode::Neq),
            Self::NeqImm(v) => {
                target.write_opcode(OpCode::NeqImm);
                target.write_felt(*v);
            }
            Self::Eqw => target.write_opcode(OpCode::Eqw),
            Self::Lt => target.write_opcode(OpCode::Lt),
            Self::Lte => target.write_opcode(OpCode::Lte),
            Self::Gt => target.write_opcode(OpCode::Gt),
            Self::Gte => target.write_opcode(OpCode::Gte),

            // ----- u32 operations ---------------------------------------------------------------
            Self::U32Test => target.write_opcode(OpCode::U32Test),
            Self::U32TestW => target.write_opcode(OpCode::U32TestW),
            Self::U32Assert => target.write_opcode(OpCode::U32Assert),
            Self::U32AssertW => target.write_opcode(OpCode::U32AssertW),
            Self::U32Split => target.write_opcode(OpCode::U32Split),
            Self::U32Cast => target.write_opcode(OpCode::U32Cast),
            Self::U32CheckedAdd => target.write_opcode(OpCode::U32CheckedAdd),
            Self::U32CheckedAddImm(v) => {
                target.write_opcode(OpCode::U32CheckedAddImm);
                target.write_u32(*v);
            }
            Self::U32WrappingAdd => target.write_opcode(OpCode::U32WrappingAdd),
            Self::U32WrappingAddImm(v) => {
                target.write_opcode(OpCode::U32WrappingAddImm);
                target.write_u32(*v);
            }
            Self::U32OverflowingAdd => target.write_opcode(OpCode::U32OverflowingAdd),
            Self::U32OverflowingAddImm(v) => {
                target.write_opcode(OpCode::U32OverflowingAddImm);
                target.write_u32(*v);
            }
            Self::U32OverflowingAdd3 => target.write_opcode(OpCode::U32OverflowingAdd3),
            Self::U32WrappingAdd3 => target.write_opcode(OpCode::U32WrappingAdd3),
            Self::U32CheckedSub => target.write_opcode(OpCode::U32CheckedSub),
            Self::U32CheckedSubImm(v) => {
                target.write_opcode(OpCode::U32CheckedSubImm);
                target.write_u32(*v);
            }
            Self::U32WrappingSub => target.write_opcode(OpCode::U32WrappingSub),
            Self::U32WrappingSubImm(v) => {
                target.write_opcode(OpCode::U32WrappingSubImm);
                target.write_u32(*v);
            }
            Self::U32OverflowingSub => target.write_opcode(OpCode::U32OverflowingSub),
            Self::U32OverflowingSubImm(v) => {
                target.write_opcode(OpCode::U32OverflowingSubImm);
                target.write_u32(*v);
            }
            Self::U32CheckedMul => target.write_opcode(OpCode::U32CheckedMul),
            Self::U32CheckedMulImm(v) => {
                target.write_opcode(OpCode::U32CheckedMulImm);
                target.write_u32(*v);
            }
            Self::U32WrappingMul => target.write_opcode(OpCode::U32WrappingMul),
            Self::U32WrappingMulImm(v) => {
                target.write_opcode(OpCode::U32WrappingMulImm);
                target.write_u32(*v);
            }
            Self::U32OverflowingMul => target.write_opcode(OpCode::U32OverflowingMul),
            Self::U32OverflowingMulImm(v) => {
                target.write_opcode(OpCode::U32OverflowingMulImm);
                target.write_u32(*v);
            }
            Self::U32OverflowingMadd => target.write_opcode(OpCode::U32OverflowingMadd),
            Self::U32WrappingMadd => target.write_opcode(OpCode::U32WrappingMadd),
            Self::U32CheckedDiv => target.write_opcode(OpCode::U32CheckedDiv),
            Self::U32CheckedDivImm(v) => {
                target.write_opcode(OpCode::U32CheckedDivImm);
                target.write_u32(*v);
            }
            Self::U32UncheckedDiv => target.write_opcode(OpCode::U32UncheckedDiv),
            Self::U32UncheckedDivImm(v) => {
                target.write_opcode(OpCode::U32UncheckedDivImm);
                target.write_u32(*v);
            }
            Self::U32CheckedMod => target.write_opcode(OpCode::U32CheckedMod),
            Self::U32CheckedModImm(v) => {
                target.write_opcode(OpCode::U32CheckedModImm);
                target.write_u32(*v);
            }
            Self::U32UncheckedMod => target.write_opcode(OpCode::U32UncheckedMod),
            Self::U32UncheckedModImm(v) => {
                target.write_opcode(OpCode::U32UncheckedModImm);
                target.write_u32(*v);
            }
            Self::U32CheckedDivMod => target.write_opcode(OpCode::U32CheckedDivMod),
            Self::U32CheckedDivModImm(v) => {
                target.write_opcode(OpCode::U32CheckedDivModImm);
                target.write_u32(*v);
            }
            Self::U32UncheckedDivMod => target.write_opcode(OpCode::U32UncheckedDivMod),
            Self::U32UncheckedDivModImm(v) => {
                target.write_opcode(OpCode::U32UncheckedDivModImm);
                target.write_u32(*v);
            }
            Self::U32CheckedAnd => target.write_opcode(OpCode::U32CheckedAnd),
            Self::U32CheckedOr => target.write_opcode(OpCode::U32CheckedOr),
            Self::U32CheckedXor => target.write_opcode(OpCode::U32CheckedXor),
            Self::U32CheckedNot => target.write_opcode(OpCode::U32CheckedNot),
            Self::U32CheckedShr => target.write_opcode(OpCode::U32CheckedShr),
            Self::U32CheckedShrImm(v) => {
                target.write_opcode(OpCode::U32CheckedShrImm);
                target.write_u8(*v);
            }
            Self::U32UncheckedShr => target.write_opcode(OpCode::U32UncheckedShr),
            Self::U32UncheckedShrImm(v) => {
                target.write_opcode(OpCode::U32UncheckedShrImm);
                target.write_u8(*v);
            }
            Self::U32CheckedShl => target.write_opcode(OpCode::U32CheckedShl),
            Self::U32CheckedShlImm(v) => {
                target.write_opcode(OpCode::U32CheckedShlImm);
                target.write_u8(*v);
            }
            Self::U32UncheckedShl => target.write_opcode(OpCode::U32UncheckedShl),
            Self::U32UncheckedShlImm(v) => {
                target.write_opcode(OpCode::U32UncheckedShlImm);
                target.write_u8(*v);
            }
            Self::U32CheckedRotr => target.write_opcode(OpCode::U32CheckedRotr),
            Self::U32CheckedRotrImm(v) => {
                target.write_opcode(OpCode::U32CheckedRotrImm);
                target.write_u8(*v);
            }
            Self::U32UncheckedRotr => target.write_opcode(OpCode::U32UncheckedRotr),
            Self::U32UncheckedRotrImm(v) => {
                target.write_opcode(OpCode::U32UncheckedRotrImm);
                target.write_u8(*v);
            }
            Self::U32CheckedRotl => target.write_opcode(OpCode::U32CheckedRotl),
            Self::U32CheckedRotlImm(v) => {
                target.write_opcode(OpCode::U32CheckedRotlImm);
                target.write_u8(*v);
            }
            Self::U32UncheckedRotl => target.write_opcode(OpCode::U32UncheckedRotl),
            Self::U32UncheckedRotlImm(v) => {
                target.write_opcode(OpCode::U32UncheckedRotlImm);
                target.write_u8(*v);
            }
            Self::U32CheckedEq => target.write_opcode(OpCode::U32CheckedEq),
            Self::U32CheckedEqImm(v) => {
                target.write_opcode(OpCode::U32CheckedEqImm);
                target.write_u32(*v);
            }
            Self::U32CheckedNeq => target.write_opcode(OpCode::U32CheckedNeq),
            Self::U32CheckedNeqImm(v) => {
                target.write_opcode(OpCode::U32CheckedNeqImm);
                target.write_u32(*v);
            }
            Self::U32CheckedLt => target.write_opcode(OpCode::U32CheckedLt),
            Self::U32UncheckedLt => target.write_opcode(OpCode::U32UncheckedLt),
            Self::U32CheckedLte => target.write_opcode(OpCode::U32CheckedLte),
            Self::U32UncheckedLte => target.write_opcode(OpCode::U32UncheckedLte),
            Self::U32CheckedGt => target.write_opcode(OpCode::U32CheckedGt),
            Self::U32UncheckedGt => target.write_opcode(OpCode::U32UncheckedGt),
            Self::U32CheckedGte => target.write_opcode(OpCode::U32CheckedGte),
            Self::U32UncheckedGte => target.write_opcode(OpCode::U32UncheckedGte),
            Self::U32CheckedMin => target.write_opcode(OpCode::U32CheckedMin),
            Self::U32UncheckedMin => target.write_opcode(OpCode::U32UncheckedMin),
            Self::U32CheckedMax => target.write_opcode(OpCode::U32CheckedMax),
            Self::U32UncheckedMax => target.write_opcode(OpCode::U32UncheckedMax),

            // ----- stack manipulation ---------------------------------------------------------------
            Self::Drop => target.write_opcode(OpCode::Drop),
            Self::DropW => target.write_opcode(OpCode::DropW),
            Self::PadW => target.write_opcode(OpCode::PadW),
            Self::Dup0 => target.write_opcode(OpCode::Dup0),
            Self::Dup1 => target.write_opcode(OpCode::Dup1),
            Self::Dup2 => target.write_opcode(OpCode::Dup2),
            Self::Dup3 => target.write_opcode(OpCode::Dup3),
            Self::Dup4 => target.write_opcode(OpCode::Dup4),
            Self::Dup5 => target.write_opcode(OpCode::Dup5),
            Self::Dup6 => target.write_opcode(OpCode::Dup6),
            Self::Dup7 => target.write_opcode(OpCode::Dup7),
            Self::Dup8 => target.write_opcode(OpCode::Dup8),
            Self::Dup9 => target.write_opcode(OpCode::Dup9),
            Self::Dup10 => target.write_opcode(OpCode::Dup10),
            Self::Dup11 => target.write_opcode(OpCode::Dup11),
            Self::Dup12 => target.write_opcode(OpCode::Dup12),
            Self::Dup13 => target.write_opcode(OpCode::Dup13),
            Self::Dup14 => target.write_opcode(OpCode::Dup14),
            Self::Dup15 => target.write_opcode(OpCode::Dup15),
            Self::DupW0 => target.write_opcode(OpCode::DupW0),
            Self::DupW1 => target.write_opcode(OpCode::DupW1),
            Self::DupW2 => target.write_opcode(OpCode::DupW2),
            Self::DupW3 => target.write_opcode(OpCode::DupW3),
            Self::Swap => target.write_opcode(OpCode::Swap),
            Self::Swap2 => target.write_opcode(OpCode::Swap2),
            Self::Swap3 => target.write_opcode(OpCode::Swap3),
            Self::Swap4 => target.write_opcode(OpCode::Swap4),
            Self::Swap5 => target.write_opcode(OpCode::Swap5),
            Self::Swap6 => target.write_opcode(OpCode::Swap6),
            Self::Swap7 => target.write_opcode(OpCode::Swap7),
            Self::Swap8 => target.write_opcode(OpCode::Swap8),
            Self::Swap9 => target.write_opcode(OpCode::Swap9),
            Self::Swap10 => target.write_opcode(OpCode::Swap10),
            Self::Swap11 => target.write_opcode(OpCode::Swap11),
            Self::Swap12 => target.write_opcode(OpCode::Swap12),
            Self::Swap13 => target.write_opcode(OpCode::Swap13),
            Self::Swap14 => target.write_opcode(OpCode::Swap14),
            Self::Swap15 => target.write_opcode(OpCode::Swap15),
            Self::SwapW => target.write_opcode(OpCode::SwapW),
            Self::SwapW2 => target.write_opcode(OpCode::SwapW2),
            Self::SwapW3 => target.write_opcode(OpCode::SwapW3),
            Self::SwapDW => target.write_opcode(OpCode::SwapDW),
            Self::MovUp2 => target.write_opcode(OpCode::MovUp2),
            Self::MovUp3 => target.write_opcode(OpCode::MovUp3),
            Self::MovUp4 => target.write_opcode(OpCode::MovUp4),
            Self::MovUp5 => target.write_opcode(OpCode::MovUp5),
            Self::MovUp6 => target.write_opcode(OpCode::MovUp6),
            Self::MovUp7 => target.write_opcode(OpCode::MovUp7),
            Self::MovUp8 => target.write_opcode(OpCode::MovUp8),
            Self::MovUp9 => target.write_opcode(OpCode::MovUp9),
            Self::MovUp10 => target.write_opcode(OpCode::MovUp10),
            Self::MovUp11 => target.write_opcode(OpCode::MovUp11),
            Self::MovUp12 => target.write_opcode(OpCode::MovUp12),
            Self::MovUp13 => target.write_opcode(OpCode::MovUp13),
            Self::MovUp14 => target.write_opcode(OpCode::MovUp14),
            Self::MovUp15 => target.write_opcode(OpCode::MovUp15),
            Self::MovUpW2 => target.write_opcode(OpCode::MovUpW2),
            Self::MovUpW3 => target.write_opcode(OpCode::MovUpW3),
            Self::MovDn2 => target.write_opcode(OpCode::MovDn2),
            Self::MovDn3 => target.write_opcode(OpCode::MovDn3),
            Self::MovDn4 => target.write_opcode(OpCode::MovDn4),
            Self::MovDn5 => target.write_opcode(OpCode::MovDn5),
            Self::MovDn6 => target.write_opcode(OpCode::MovDn6),
            Self::MovDn7 => target.write_opcode(OpCode::MovDn7),
            Self::MovDn8 => target.write_opcode(OpCode::MovDn8),
            Self::MovDn9 => target.write_opcode(OpCode::MovDn9),
            Self::MovDn10 => target.write_opcode(OpCode::MovDn10),
            Self::MovDn11 => target.write_opcode(OpCode::MovDn11),
            Self::MovDn12 => target.write_opcode(OpCode::MovDn12),
            Self::MovDn13 => target.write_opcode(OpCode::MovDn13),
            Self::MovDn14 => target.write_opcode(OpCode::MovDn14),
            Self::MovDn15 => target.write_opcode(OpCode::MovDn15),
            Self::MovDnW2 => target.write_opcode(OpCode::MovDnW2),
            Self::MovDnW3 => target.write_opcode(OpCode::MovDnW3),
            Self::CSwap => target.write_opcode(OpCode::CSwap),
            Self::CSwapW => target.write_opcode(OpCode::CSwapW),
            Self::CDrop => target.write_opcode(OpCode::CDrop),
            Self::CDropW => target.write_opcode(OpCode::CDropW),

            // ----- input / output operations --------------------------------------------------------
            Self::Locaddr(v) => {
                target.write_opcode(OpCode::Locaddr);
                target.write_felt(*v);
            }
            Self::Sdepth => target.write_opcode(OpCode::Sdepth),
            Self::Caller => target.write_opcode(OpCode::Caller),
            Self::MemLoad => target.write_opcode(OpCode::MemLoad),
            Self::MemLoadImm(v) => {
                target.write_opcode(OpCode::MemLoadImm);
                target.write_felt(*v);
            }
            Self::MemLoadW => target.write_opcode(OpCode::MemLoadW),
            Self::MemLoadWImm(v) => {
                target.write_opcode(OpCode::MemLoadWImm);
                target.write_felt(*v);
            }
            Self::LocLoad(v) => {
                target.write_opcode(OpCode::LocLoad);
                target.write_felt(*v);
            }
            Self::LocLoadW(v) => {
                target.write_opcode(OpCode::LocLoadW);
                target.write_felt(*v);
            }
            Self::MemStore => target.write_opcode(OpCode::MemStore),
            Self::MemStoreImm(v) => {
                target.write_opcode(OpCode::MemStoreImm);
                target.write_felt(*v);
            }
            Self::LocStore(v) => {
                target.write_opcode(OpCode::LocStore);
                target.write_felt(*v);
            }
            Self::MemStoreW => target.write_opcode(OpCode::MemStoreW),
            Self::MemStoreWImm(v) => {
                target.write_opcode(OpCode::MemStoreWImm);
                target.write_felt(*v);
            }
            Self::LocStoreW(v) => {
                target.write_opcode(OpCode::LocStoreW);
                target.write_felt(*v);
            }

            Self::PushConstants(constants) => {
                target.write_opcode(OpCode::PushConstants);
                let length = constants.len();
                target.write_u8(length as u8);
                for v in constants {
                    target.write_felt(*v);
                }
            }
            Self::AdvU64Div => target.write_opcode(OpCode::AdvU64Div),
            Self::AdvPush(v) => {
                target.write_opcode(OpCode::AdvPush);
                target.write_felt(*v);
            }
            Self::AdvLoadW(v) => {
                target.write_opcode(OpCode::AdvLoadW);
                target.write_felt(*v);
            }

            // ----- cryptographic operations ---------------------------------------------------------
            Self::RPHash => target.write_opcode(OpCode::RPHash),
            Self::RPPerm => target.write_opcode(OpCode::RPPerm),
            Self::MTreeGet => target.write_opcode(OpCode::MTreeGet),
            Self::MTreeSet => target.write_opcode(OpCode::MTreeSet),
            Self::MTreeCWM => target.write_opcode(OpCode::MTreeCWM),

            // ----- exec / call ----------------------------------------------------------------------
            Self::ExecLocal(v) => {
                target.write_opcode(OpCode::ExecLocal);
                target.write_u16(*v);
            }
            Self::ExecImported(imported) => {
                target.write_opcode(OpCode::ExecImported);
                target.write_procedure_id(imported);
            }
            Self::CallLocal(v) => {
                target.write_opcode(OpCode::CallLocal);
                target.write_u16(*v);
            }
            Self::CallImported(imported) => {
                target.write_opcode(OpCode::CallImported);
                target.write_procedure_id(imported);
            }
        }
    }
}
