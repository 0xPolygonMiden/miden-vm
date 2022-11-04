use super::{
    super::{
        nodes::{Instruction, Node},
        PROC_DIGEST_SIZE,
    },
    OpCode, IF_ELSE_OPCODE, REPEAT_OPCODE, WHILE_OPCODE,
};
use crate::errors::SerializationError;
use vm_core::{utils::collections::Vec, utils::string::String, Felt};

// BYTE READER IMPLEMENTATION
// ================================================================================================

/// Contains bytes for deserialization and current reading position
pub struct ByteReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ByteReader { bytes, pos: 0 }
    }

    pub fn read_bool(&mut self) -> Result<bool, SerializationError> {
        self.check_eor(1)?;
        let result = self.bytes[self.pos];
        self.pos += 1;
        u8_to_bool(result)
    }

    pub fn read_u8(&mut self) -> Result<u8, SerializationError> {
        self.check_eor(1)?;
        let result = self.bytes[self.pos];
        self.pos += 1;
        Ok(result)
    }

    pub fn peek_u8(&self) -> Result<u8, SerializationError> {
        self.check_eor(1)?;
        let result = self.bytes[self.pos];
        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16, SerializationError> {
        self.check_eor(2)?;
        let result = &self.bytes[self.pos..self.pos + 2];
        self.pos += 2;
        Ok(u16::from_le_bytes(
            result.try_into().expect("u16 conversion failure"),
        ))
    }

    pub fn read_u32(&mut self) -> Result<u32, SerializationError> {
        self.check_eor(4)?;
        let result = &self.bytes[self.pos..self.pos + 4];
        self.pos += 4;
        Ok(u32::from_le_bytes(
            result.try_into().expect("u32 conversion failure"),
        ))
    }

    pub fn read_u64(&mut self) -> Result<u64, SerializationError> {
        self.check_eor(8)?;
        let result = &self.bytes[self.pos..self.pos + 8];
        self.pos += 8;
        Ok(u64::from_le_bytes(
            result.try_into().expect("u64 conversion failure"),
        ))
    }

    pub fn read_string(&mut self) -> Result<String, SerializationError> {
        self.check_eor(1)?;
        let length = self.bytes[self.pos];
        self.pos += 1;
        self.check_eor(length as usize)?;
        let string_bytes = &self.bytes[self.pos..self.pos + length as usize];
        self.pos += length as usize;
        Ok(String::from_utf8(string_bytes.to_vec()).expect("String conversion failure"))
    }

    pub fn read_proc_hash(&mut self) -> Result<[u8; PROC_DIGEST_SIZE], SerializationError> {
        self.check_eor(PROC_DIGEST_SIZE)?;
        let mut hash = [0; PROC_DIGEST_SIZE];
        hash.copy_from_slice(&self.bytes[self.pos..self.pos + PROC_DIGEST_SIZE]);
        self.pos += PROC_DIGEST_SIZE;
        Ok(hash)
    }

    pub fn read_opcode(&mut self) -> Result<OpCode, SerializationError> {
        let value = self.read_u8()?;
        OpCode::try_from(value).map_err(|_| SerializationError::InvalidOpCode)
    }

    pub fn read_felt(&mut self) -> Result<Felt, SerializationError> {
        Ok(Felt::new(
            self.read_u64()
                .map_err(|_| SerializationError::InvalidFieldElement)?,
        ))
    }

    /// Checks if it is possible to read at least `num_bytes` bytes from ByteReader
    ///
    /// # Errors
    /// Returns an error if, when reading the requested number of bytes, we go beyond the boundaries of the array
    fn check_eor(&self, num_bytes: usize) -> Result<(), SerializationError> {
        if self.pos + num_bytes > self.bytes.len() {
            return Err(SerializationError::EndOfReader);
        }
        Ok(())
    }
}

// DESERIALIZABLE TRAIT IMPLEMENTATIONS
// ================================================================================================

/// Returns `self` from its byte representation stored in provided `ByteReader` struct.
pub trait Deserializable: Sized {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError>;
}

impl Deserializable for Vec<Node> {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let mut vec_node: Vec<Node> = Vec::new();
        let vec_len = bytes.read_u16()?;

        for _ in 0..vec_len {
            vec_node.push(Deserializable::read_from(bytes)?);
        }

        Ok(vec_node)
    }
}

impl Deserializable for Node {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let first_byte = bytes.peek_u8()?;

        match first_byte {
            IF_ELSE_OPCODE => {
                bytes.read_u8()?;
                Ok(Node::IfElse(
                    Deserializable::read_from(bytes)?,
                    Deserializable::read_from(bytes)?,
                ))
            }
            REPEAT_OPCODE => {
                bytes.read_u8()?;
                Ok(Node::Repeat(
                    bytes.read_u16()?.into(),
                    Deserializable::read_from(bytes)?,
                ))
            }
            WHILE_OPCODE => {
                bytes.read_u8()?;
                Ok(Node::While(Deserializable::read_from(bytes)?))
            }
            _ => Ok(Node::Instruction(Deserializable::read_from(bytes)?)),
        }
    }
}

impl Deserializable for Instruction {
    fn read_from(bytes: &mut ByteReader) -> Result<Self, SerializationError> {
        let opcode = bytes.read_opcode()?;

        match opcode {
            OpCode::Assert => Ok(Instruction::Assert),
            OpCode::AssertEq => Ok(Instruction::AssertEq),
            OpCode::Assertz => Ok(Instruction::Assertz),
            OpCode::Add => Ok(Instruction::Add),
            OpCode::AddImm => Ok(Instruction::AddImm(bytes.read_felt()?)),
            OpCode::Sub => Ok(Instruction::Sub),
            OpCode::SubImm => Ok(Instruction::SubImm(bytes.read_felt()?)),
            OpCode::Mul => Ok(Instruction::Mul),
            OpCode::MulImm => Ok(Instruction::MulImm(bytes.read_felt()?)),
            OpCode::Div => Ok(Instruction::Div),
            OpCode::DivImm => Ok(Instruction::DivImm(bytes.read_felt()?)),
            OpCode::Neg => Ok(Instruction::Neg),
            OpCode::Inv => Ok(Instruction::Inv),
            OpCode::Pow2 => Ok(Instruction::Pow2),
            OpCode::Exp => Ok(Instruction::Exp),
            OpCode::Not => Ok(Instruction::Not),
            OpCode::And => Ok(Instruction::And),
            OpCode::Or => Ok(Instruction::Or),
            OpCode::Xor => Ok(Instruction::Xor),
            OpCode::Eq => Ok(Instruction::Eq),
            OpCode::EqImm => Ok(Instruction::EqImm(bytes.read_felt()?)),
            OpCode::Neq => Ok(Instruction::Neq),
            OpCode::NeqImm => Ok(Instruction::NeqImm(bytes.read_felt()?)),
            OpCode::Eqw => Ok(Instruction::Eqw),
            OpCode::Lt => Ok(Instruction::Lt),
            OpCode::Lte => Ok(Instruction::Lte),
            OpCode::Gt => Ok(Instruction::Gt),
            OpCode::Gte => Ok(Instruction::Gte),

            // ----- u32 manipulation ---------------------------------------------------------------
            OpCode::U32Test => Ok(Instruction::U32Test),
            OpCode::U32TestW => Ok(Instruction::U32TestW),
            OpCode::U32Assert => Ok(Instruction::U32Assert),
            OpCode::U32AssertW => Ok(Instruction::U32AssertW),
            OpCode::U32Split => Ok(Instruction::U32Split),
            OpCode::U32Cast => Ok(Instruction::U32Cast),
            OpCode::U32CheckedAdd => Ok(Instruction::U32CheckedAdd),
            OpCode::U32CheckedAddImm => Ok(Instruction::U32CheckedAddImm(bytes.read_u32()?)),
            OpCode::U32WrappingAdd => Ok(Instruction::U32WrappingAdd),
            OpCode::U32WrappingAddImm => Ok(Instruction::U32WrappingAddImm(bytes.read_u32()?)),
            OpCode::U32OverflowingAdd => Ok(Instruction::U32OverflowingAdd),
            OpCode::U32OverflowingAddImm => {
                Ok(Instruction::U32OverflowingAddImm(bytes.read_u32()?))
            }
            OpCode::U32OverflowingAdd3 => Ok(Instruction::U32OverflowingAdd3),
            OpCode::U32WrappingAdd3 => Ok(Instruction::U32WrappingAdd3),
            OpCode::U32CheckedSub => Ok(Instruction::U32CheckedSub),
            OpCode::U32CheckedSubImm => Ok(Instruction::U32CheckedSubImm(bytes.read_u32()?)),
            OpCode::U32WrappingSub => Ok(Instruction::U32WrappingSub),
            OpCode::U32WrappingSubImm => Ok(Instruction::U32WrappingSubImm(bytes.read_u32()?)),
            OpCode::U32OverflowingSub => Ok(Instruction::U32OverflowingSub),
            OpCode::U32OverflowingSubImm => {
                Ok(Instruction::U32OverflowingSubImm(bytes.read_u32()?))
            }
            OpCode::U32CheckedMul => Ok(Instruction::U32CheckedMul),
            OpCode::U32CheckedMulImm => Ok(Instruction::U32CheckedMulImm(bytes.read_u32()?)),
            OpCode::U32WrappingMul => Ok(Instruction::U32WrappingMul),
            OpCode::U32WrappingMulImm => Ok(Instruction::U32WrappingMulImm(bytes.read_u32()?)),
            OpCode::U32OverflowingMul => Ok(Instruction::U32OverflowingMul),
            OpCode::U32OverflowingMulImm => {
                Ok(Instruction::U32OverflowingMulImm(bytes.read_u32()?))
            }
            OpCode::U32OverflowingMadd => Ok(Instruction::U32OverflowingMadd),
            OpCode::U32WrappingMadd => Ok(Instruction::U32WrappingMadd),
            OpCode::U32CheckedDiv => Ok(Instruction::U32CheckedDiv),
            OpCode::U32CheckedDivImm => Ok(Instruction::U32CheckedDivImm(bytes.read_u32()?)),
            OpCode::U32UncheckedDiv => Ok(Instruction::U32UncheckedDiv),
            OpCode::U32UncheckedDivImm => Ok(Instruction::U32UncheckedDivImm(bytes.read_u32()?)),
            OpCode::U32CheckedMod => Ok(Instruction::U32CheckedMod),
            OpCode::U32CheckedModImm => Ok(Instruction::U32CheckedModImm(bytes.read_u32()?)),
            OpCode::U32UncheckedMod => Ok(Instruction::U32UncheckedMod),
            OpCode::U32UncheckedModImm => Ok(Instruction::U32UncheckedModImm(bytes.read_u32()?)),
            OpCode::U32CheckedDivMod => Ok(Instruction::U32CheckedDivMod),
            OpCode::U32CheckedDivModImm => Ok(Instruction::U32CheckedDivModImm(bytes.read_u32()?)),
            OpCode::U32UncheckedDivMod => Ok(Instruction::U32UncheckedDivMod),
            OpCode::U32UncheckedDivModImm => {
                Ok(Instruction::U32UncheckedDivModImm(bytes.read_u32()?))
            }
            OpCode::U32CheckedAnd => Ok(Instruction::U32CheckedAnd),
            OpCode::U32CheckedOr => Ok(Instruction::U32CheckedOr),
            OpCode::U32CheckedXor => Ok(Instruction::U32CheckedXor),
            OpCode::U32CheckedNot => Ok(Instruction::U32CheckedNot),
            OpCode::U32CheckedShr => Ok(Instruction::U32CheckedShr),
            OpCode::U32CheckedShrImm => Ok(Instruction::U32CheckedShrImm(bytes.read_u8()?)),
            OpCode::U32UncheckedShr => Ok(Instruction::U32UncheckedShr),
            OpCode::U32UncheckedShrImm => Ok(Instruction::U32UncheckedShrImm(bytes.read_u8()?)),
            OpCode::U32CheckedShl => Ok(Instruction::U32CheckedShl),
            OpCode::U32CheckedShlImm => Ok(Instruction::U32CheckedShlImm(bytes.read_u8()?)),
            OpCode::U32UncheckedShl => Ok(Instruction::U32UncheckedShl),
            OpCode::U32UncheckedShlImm => Ok(Instruction::U32UncheckedShlImm(bytes.read_u8()?)),
            OpCode::U32CheckedRotr => Ok(Instruction::U32CheckedRotr),
            OpCode::U32CheckedRotrImm => Ok(Instruction::U32CheckedRotrImm(bytes.read_u8()?)),
            OpCode::U32UncheckedRotr => Ok(Instruction::U32UncheckedRotr),
            OpCode::U32UncheckedRotrImm => Ok(Instruction::U32UncheckedRotrImm(bytes.read_u8()?)),
            OpCode::U32CheckedRotl => Ok(Instruction::U32CheckedRotl),
            OpCode::U32CheckedRotlImm => Ok(Instruction::U32CheckedRotlImm(bytes.read_u8()?)),
            OpCode::U32UncheckedRotl => Ok(Instruction::U32UncheckedRotl),
            OpCode::U32UncheckedRotlImm => Ok(Instruction::U32UncheckedRotlImm(bytes.read_u8()?)),
            OpCode::U32CheckedEq => Ok(Instruction::U32CheckedEq),
            OpCode::U32CheckedEqImm => Ok(Instruction::U32CheckedEqImm(bytes.read_u32()?)),
            OpCode::U32CheckedNeq => Ok(Instruction::U32CheckedNeq),
            OpCode::U32CheckedNeqImm => Ok(Instruction::U32CheckedNeqImm(bytes.read_u32()?)),
            OpCode::U32CheckedLt => Ok(Instruction::U32CheckedLt),
            OpCode::U32UncheckedLt => Ok(Instruction::U32UncheckedLt),
            OpCode::U32CheckedLte => Ok(Instruction::U32CheckedLte),
            OpCode::U32UncheckedLte => Ok(Instruction::U32UncheckedLte),
            OpCode::U32CheckedGt => Ok(Instruction::U32CheckedGt),
            OpCode::U32UncheckedGt => Ok(Instruction::U32UncheckedGt),
            OpCode::U32CheckedGte => Ok(Instruction::U32CheckedGte),
            OpCode::U32UncheckedGte => Ok(Instruction::U32UncheckedGte),
            OpCode::U32CheckedMin => Ok(Instruction::U32CheckedMin),
            OpCode::U32UncheckedMin => Ok(Instruction::U32UncheckedMin),
            OpCode::U32CheckedMax => Ok(Instruction::U32CheckedMax),
            OpCode::U32UncheckedMax => Ok(Instruction::U32UncheckedMax),

            // ----- stack manipulation ---------------------------------------------------------------
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
            OpCode::Swap => Ok(Instruction::Swap),
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
            OpCode::SwapW => Ok(Instruction::SwapW),
            OpCode::SwapW2 => Ok(Instruction::SwapW2),
            OpCode::SwapW3 => Ok(Instruction::SwapW3),
            OpCode::SwapDW => Ok(Instruction::SwapDW),
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

            // ----- input / output operations --------------------------------------------------------
            OpCode::Adv => Ok(Instruction::Adv(bytes.read_felt()?)),
            OpCode::Locaddr => Ok(Instruction::Locaddr(bytes.read_felt()?)),
            OpCode::Sdepth => Ok(Instruction::Sdepth),
            OpCode::MemLoad => Ok(Instruction::MemLoad),
            OpCode::MemLoadImm => Ok(Instruction::MemLoadImm(bytes.read_felt()?)),
            OpCode::MemLoadW => Ok(Instruction::MemLoadW),
            OpCode::MemLoadWImm => Ok(Instruction::MemLoadWImm(bytes.read_felt()?)),
            OpCode::LocLoad => Ok(Instruction::LocLoad(bytes.read_felt()?)),
            OpCode::LocLoadW => Ok(Instruction::LocLoadW(bytes.read_felt()?)),
            OpCode::MemStore => Ok(Instruction::MemStore),
            OpCode::MemStoreImm => Ok(Instruction::MemStoreImm(bytes.read_felt()?)),
            OpCode::LocStore => Ok(Instruction::LocStore(bytes.read_felt()?)),
            OpCode::MemStoreW => Ok(Instruction::MemStoreW),
            OpCode::MemStoreWImm => Ok(Instruction::MemStoreWImm(bytes.read_felt()?)),
            OpCode::LocStoreW => Ok(Instruction::LocStoreW(bytes.read_felt()?)),
            OpCode::LoadWAdv => Ok(Instruction::LoadWAdv),

            OpCode::PushConstants => {
                let mut constants = Vec::new();
                let length = bytes.read_u8()?;
                for _ in 0..length {
                    constants.push(bytes.read_felt()?);
                }
                Ok(Instruction::PushConstants(constants))
            }
            OpCode::AdvU64Div => Ok(Instruction::AdvU64Div),
            OpCode::AdvPush => Ok(Instruction::AdvPush(bytes.read_felt()?)),
            OpCode::AdvLoadW => Ok(Instruction::AdvLoadW(bytes.read_felt()?)),

            // ----- cryptographic operations ---------------------------------------------------------
            OpCode::RPHash => Ok(Instruction::RPHash),
            OpCode::RPPerm => Ok(Instruction::RPPerm),
            OpCode::MTreeGet => Ok(Instruction::MTreeGet),
            OpCode::MTreeSet => Ok(Instruction::MTreeSet),
            OpCode::MTreeCWM => Ok(Instruction::MTreeCWM),

            // ----- exec / call ----------------------------------------------------------------------
            OpCode::ExecLocal => Ok(Instruction::ExecLocal(bytes.read_u16()?)),
            OpCode::ExecImported => Ok(Instruction::ExecImported(bytes.read_proc_hash()?)),
            OpCode::CallLocal => Ok(Instruction::CallLocal(bytes.read_u16()?)),
            OpCode::CallImported => Ok(Instruction::CallImported(bytes.read_proc_hash()?)),
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================
fn u8_to_bool(param: u8) -> Result<bool, SerializationError> {
    match param {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(SerializationError::InvalidBoolValue),
    }
}
