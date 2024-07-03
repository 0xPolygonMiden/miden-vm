use crate::{
    AdviceInjector, AssemblyOp, DebugOptions, Decorator, DecoratorList, Operation, OperationData,
    SignatureKind,
};

use super::{decorator::EncodedDecoratorVariant, StringIndex, StringRef};
use alloc::{string::String, vec::Vec};
use miden_crypto::{Felt, ZERO};
use winter_utils::{ByteReader, DeserializationError, SliceReader};

pub struct BasicBlockDataDecoder<'a> {
    data: &'a [u8],
    data_reader: SliceReader<'a>,
    strings: &'a [StringRef],
}

/// Constructors
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn new(data: &'a [u8], strings: &'a [StringRef]) -> Self {
        let data_reader = SliceReader::new(data);

        Self {
            data,
            data_reader,
            strings,
        }
    }
}

/// Mutators
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn decode_operations_and_decorators(
        &mut self,
        num_to_decode: u32,
    ) -> Result<(Vec<Operation>, DecoratorList), DeserializationError> {
        let mut operations: Vec<Operation> = Vec::new();
        let mut decorators: DecoratorList = Vec::new();

        for _ in 0..num_to_decode {
            let first_byte = self.data_reader.read_u8()?;

            if first_byte & 0b1000_0000 == 0 {
                // operation.
                let op_code = first_byte;

                let operation = if op_code == Operation::Assert(0_u32).op_code()
                    || op_code == Operation::MpVerify(0_u32).op_code()
                {
                    let value_le_bytes: [u8; 4] = self.data_reader.read_array()?;
                    let value = u32::from_le_bytes(value_le_bytes);

                    Operation::with_opcode_and_data(op_code, OperationData::U32(value))?
                } else if op_code == Operation::U32assert2(ZERO).op_code()
                    || op_code == Operation::Push(ZERO).op_code()
                {
                    // Felt operation data
                    let value_le_bytes: [u8; 8] = self.data_reader.read_array()?;
                    let value_u64 = u64::from_le_bytes(value_le_bytes);
                    let value_felt = Felt::try_from(value_u64).map_err(|_| {
                        DeserializationError::InvalidValue(format!(
                            "Operation associated data doesn't fit in a field element: {value_u64}"
                        ))
                    })?;

                    Operation::with_opcode_and_data(op_code, OperationData::Felt(value_felt))?
                } else {
                    // No operation data
                    Operation::with_opcode_and_data(op_code, OperationData::None)?
                };

                operations.push(operation);
            } else {
                // decorator.
                let discriminant = first_byte & 0b0111_1111;
                let decorator = self.decode_decorator(discriminant)?;

                decorators.push((operations.len(), decorator));
            }
        }

        Ok((operations, decorators))
    }
}

/// Helpers
impl<'a> BasicBlockDataDecoder<'a> {
    fn decode_decorator(&mut self, discriminant: u8) -> Result<Decorator, DeserializationError> {
        let decorator_variant = EncodedDecoratorVariant::from_discriminant(discriminant)
            .ok_or_else(|| {
                DeserializationError::InvalidValue(format!(
                    "invalid decorator variant discriminant: {discriminant}"
                ))
            })?;

        match decorator_variant {
            EncodedDecoratorVariant::AdviceInjectorMerkleNodeMerge => {
                Ok(Decorator::Advice(AdviceInjector::MerkleNodeMerge))
            }
            EncodedDecoratorVariant::AdviceInjectorMerkleNodeToStack => {
                Ok(Decorator::Advice(AdviceInjector::MerkleNodeToStack))
            }
            EncodedDecoratorVariant::AdviceInjectorUpdateMerkleNode => {
                Ok(Decorator::Advice(AdviceInjector::UpdateMerkleNode))
            }
            EncodedDecoratorVariant::AdviceInjectorMapValueToStack => {
                let include_len = self.data_reader.read_bool()?;
                let key_offset = self.data_reader.read_usize()?;

                Ok(Decorator::Advice(AdviceInjector::MapValueToStack {
                    include_len,
                    key_offset,
                }))
            }
            EncodedDecoratorVariant::AdviceInjectorU64Div => {
                Ok(Decorator::Advice(AdviceInjector::U64Div))
            }
            EncodedDecoratorVariant::AdviceInjectorExt2Inv => {
                Ok(Decorator::Advice(AdviceInjector::Ext2Inv))
            }
            EncodedDecoratorVariant::AdviceInjectorExt2Intt => {
                Ok(Decorator::Advice(AdviceInjector::Ext2Intt))
            }
            EncodedDecoratorVariant::AdviceInjectorSmtGet => {
                Ok(Decorator::Advice(AdviceInjector::SmtGet))
            }
            EncodedDecoratorVariant::AdviceInjectorSmtSet => {
                Ok(Decorator::Advice(AdviceInjector::SmtSet))
            }
            EncodedDecoratorVariant::AdviceInjectorSmtPeek => {
                Ok(Decorator::Advice(AdviceInjector::SmtPeek))
            }
            EncodedDecoratorVariant::AdviceInjectorU32Clz => {
                Ok(Decorator::Advice(AdviceInjector::U32Clz))
            }
            EncodedDecoratorVariant::AdviceInjectorU32Ctz => {
                Ok(Decorator::Advice(AdviceInjector::U32Ctz))
            }
            EncodedDecoratorVariant::AdviceInjectorU32Clo => {
                Ok(Decorator::Advice(AdviceInjector::U32Clo))
            }
            EncodedDecoratorVariant::AdviceInjectorU32Cto => {
                Ok(Decorator::Advice(AdviceInjector::U32Cto))
            }
            EncodedDecoratorVariant::AdviceInjectorILog2 => {
                Ok(Decorator::Advice(AdviceInjector::ILog2))
            }
            EncodedDecoratorVariant::AdviceInjectorMemToMap => {
                Ok(Decorator::Advice(AdviceInjector::MemToMap))
            }
            EncodedDecoratorVariant::AdviceInjectorHdwordToMap => {
                let domain = self.data_reader.read_u64()?;
                let domain = Felt::try_from(domain).map_err(|err| {
                    DeserializationError::InvalidValue(format!(
                        "Error when deserializing HdwordToMap decorator domain: {err}"
                    ))
                })?;

                Ok(Decorator::Advice(AdviceInjector::HdwordToMap { domain }))
            }
            EncodedDecoratorVariant::AdviceInjectorHpermToMap => {
                Ok(Decorator::Advice(AdviceInjector::HpermToMap))
            }
            EncodedDecoratorVariant::AdviceInjectorSigToStack => {
                Ok(Decorator::Advice(AdviceInjector::SigToStack {
                    kind: SignatureKind::RpoFalcon512,
                }))
            }
            EncodedDecoratorVariant::AssemblyOp => {
                let num_cycles = self.data_reader.read_u8()?;
                let should_break = self.data_reader.read_bool()?;

                let context_name = {
                    let str_index_in_table = self.data_reader.read_usize()?;
                    self.read_string(str_index_in_table)?
                };

                let op = {
                    let str_index_in_table = self.data_reader.read_usize()?;
                    self.read_string(str_index_in_table)?
                };

                Ok(Decorator::AsmOp(AssemblyOp::new(context_name, num_cycles, op, should_break)))
            }
            EncodedDecoratorVariant::DebugOptionsStackAll => {
                Ok(Decorator::Debug(DebugOptions::StackAll))
            }
            EncodedDecoratorVariant::DebugOptionsStackTop => {
                let value = self.data_reader.read_u8()?;

                Ok(Decorator::Debug(DebugOptions::StackTop(value)))
            }
            EncodedDecoratorVariant::DebugOptionsMemAll => {
                Ok(Decorator::Debug(DebugOptions::MemAll))
            }
            EncodedDecoratorVariant::DebugOptionsMemInterval => {
                let start = u32::from_le_bytes(self.data_reader.read_array::<4>()?);
                let end = u32::from_le_bytes(self.data_reader.read_array::<4>()?);

                Ok(Decorator::Debug(DebugOptions::MemInterval(start, end)))
            }
            EncodedDecoratorVariant::DebugOptionsLocalInterval => {
                let start = u16::from_le_bytes(self.data_reader.read_array::<2>()?);
                let second = u16::from_le_bytes(self.data_reader.read_array::<2>()?);
                let end = u16::from_le_bytes(self.data_reader.read_array::<2>()?);

                Ok(Decorator::Debug(DebugOptions::LocalInterval(start, second, end)))
            }
            EncodedDecoratorVariant::Event => {
                let value = u32::from_le_bytes(self.data_reader.read_array::<4>()?);

                Ok(Decorator::Event(value))
            }
            EncodedDecoratorVariant::Trace => {
                let value = u32::from_le_bytes(self.data_reader.read_array::<4>()?);

                Ok(Decorator::Trace(value))
            }
        }
    }

    fn read_string(&self, str_idx: StringIndex) -> Result<String, DeserializationError> {
        let str_ref = self.strings.get(str_idx).ok_or_else(|| {
            DeserializationError::InvalidValue(format!("invalid index in strings table: {str_idx}"))
        })?;

        let str_bytes = {
            let start = str_ref.offset as usize;
            let end = (str_ref.offset + str_ref.len) as usize;

            self.data.get(start..end).ok_or_else(|| {
                DeserializationError::InvalidValue(format!(
                    "invalid string ref in strings table. Offset: {},  length: {}",
                    str_ref.offset, str_ref.len
                ))
            })?
        };

        String::from_utf8(str_bytes.to_vec()).map_err(|_| {
            DeserializationError::InvalidValue(format!(
                "Invalid UTF-8 string in strings table: {str_bytes:?}"
            ))
        })
    }
}
