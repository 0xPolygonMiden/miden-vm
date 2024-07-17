use crate::{
    AdviceInjector, AssemblyOp, DebugOptions, Decorator, DecoratorList, Operation, SignatureKind,
};

use super::{decorator::EncodedDecoratorVariant, DataOffset, StringIndex};
use alloc::{string::String, vec::Vec};
use miden_crypto::Felt;
use winter_utils::{ByteReader, Deserializable, DeserializationError, SliceReader};

pub struct BasicBlockDataDecoder<'a> {
    data: &'a [u8],
    strings: &'a [DataOffset],
}

/// Constructors
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn new(data: &'a [u8], strings: &'a [DataOffset]) -> Self {
        Self { data, strings }
    }
}

/// Mutators
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn decode_operations_and_decorators(
        &self,
        offset: DataOffset,
        num_to_decode: u32,
    ) -> Result<(Vec<Operation>, DecoratorList), DeserializationError> {
        let mut operations: Vec<Operation> = Vec::new();
        let mut decorators: DecoratorList = Vec::new();

        let mut data_reader = SliceReader::new(&self.data[offset as usize..]);
        for _ in 0..num_to_decode {
            let first_byte = data_reader.peek_u8()?;

            if first_byte & 0b1000_0000 == 0 {
                // operation.
                operations.push(Operation::read_from(&mut data_reader)?);
            } else {
                // decorator.
                let decorator = self.decode_decorator(&mut data_reader)?;
                decorators.push((operations.len(), decorator));
            }
        }

        Ok((operations, decorators))
    }
}

/// Helpers
impl<'a> BasicBlockDataDecoder<'a> {
    fn decode_decorator(
        &self,
        data_reader: &mut SliceReader,
    ) -> Result<Decorator, DeserializationError> {
        let discriminant = data_reader.read_u8()?;

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
                let include_len = data_reader.read_bool()?;
                let key_offset = data_reader.read_usize()?;

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
                let domain = data_reader.read_u64()?;
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
                let num_cycles = data_reader.read_u8()?;
                let should_break = data_reader.read_bool()?;

                let context_name = {
                    let str_index_in_table = data_reader.read_usize()?;
                    self.read_string(str_index_in_table)?
                };

                let op = {
                    let str_index_in_table = data_reader.read_usize()?;
                    self.read_string(str_index_in_table)?
                };

                Ok(Decorator::AsmOp(AssemblyOp::new(context_name, num_cycles, op, should_break)))
            }
            EncodedDecoratorVariant::DebugOptionsStackAll => {
                Ok(Decorator::Debug(DebugOptions::StackAll))
            }
            EncodedDecoratorVariant::DebugOptionsStackTop => {
                let value = data_reader.read_u8()?;

                Ok(Decorator::Debug(DebugOptions::StackTop(value)))
            }
            EncodedDecoratorVariant::DebugOptionsMemAll => {
                Ok(Decorator::Debug(DebugOptions::MemAll))
            }
            EncodedDecoratorVariant::DebugOptionsMemInterval => {
                let start = data_reader.read_u32()?;
                let end = data_reader.read_u32()?;

                Ok(Decorator::Debug(DebugOptions::MemInterval(start, end)))
            }
            EncodedDecoratorVariant::DebugOptionsLocalInterval => {
                let start = data_reader.read_u16()?;
                let second = data_reader.read_u16()?;
                let end = data_reader.read_u16()?;

                Ok(Decorator::Debug(DebugOptions::LocalInterval(start, second, end)))
            }
            EncodedDecoratorVariant::Event => {
                let value = data_reader.read_u32()?;

                Ok(Decorator::Event(value))
            }
            EncodedDecoratorVariant::Trace => {
                let value = data_reader.read_u32()?;

                Ok(Decorator::Trace(value))
            }
        }
    }

    fn read_string(&self, str_idx: StringIndex) -> Result<String, DeserializationError> {
        let str_offset = self.strings.get(str_idx).copied().ok_or_else(|| {
            DeserializationError::InvalidValue(format!("invalid index in strings table: {str_idx}"))
        })? as usize;

        let mut reader = SliceReader::new(&self.data[str_offset..]);
        reader.read()
    }
}
