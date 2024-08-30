use alloc::{string::String, sync::Arc, vec::Vec};
use core::cell::RefCell;

use miden_crypto::Felt;
use winter_utils::{ByteReader, Deserializable, DeserializationError, SliceReader};

use super::{decorator::EncodedDecoratorVariant, DataOffset, StringIndex};
use crate::{
    mast::MastForest, AdviceInjector, AssemblyOp, DebugOptions, Decorator, DecoratorList,
    Operation, SignatureKind,
};

pub struct BasicBlockDataDecoder<'a> {
    data: &'a [u8],
    strings: &'a [DataOffset],
    /// This field is used to allocate an `Arc` for any string in `strings` where the decoder
    /// requests a reference-counted string rather than a fresh allocation as a `String`.
    ///
    /// Currently, this is only used for debug information (source file names), but most cases
    /// where strings are stored in MAST are stored as `Arc` in practice, we just haven't yet
    /// updated all of the decoders.
    ///
    /// We lazily allocate an `Arc` when strings are decoded as an `Arc`, but the underlying
    /// string data corresponds to the same index in `strings`. All future requests for a
    /// ref-counted string we've allocated an `Arc` for, will clone the `Arc` rather than
    /// allocate a fresh string.
    refc_strings: Vec<RefCell<Option<Arc<str>>>>,
}

/// Constructors
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn new(data: &'a [u8], strings: &'a [DataOffset]) -> Self {
        let mut refc_strings = Vec::with_capacity(strings.len());
        refc_strings.resize(strings.len(), RefCell::new(None));
        Self { data, strings, refc_strings }
    }
}

/// Mutators
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn decode_operations_and_decorators(
        &self,
        offset: DataOffset,
        num_to_decode: u32,
        mast_forest: &mut MastForest,
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
                let decorator_id = mast_forest.add_decorator(decorator).map_err(|err| {
                    DeserializationError::InvalidValue(format!(
                        "failed to add decorator to MAST forest: {err}"
                    ))
                })?;
                decorators.push((operations.len(), decorator_id));
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
            },
            EncodedDecoratorVariant::AdviceInjectorMerkleNodeToStack => {
                Ok(Decorator::Advice(AdviceInjector::MerkleNodeToStack))
            },
            EncodedDecoratorVariant::AdviceInjectorUpdateMerkleNode => {
                Ok(Decorator::Advice(AdviceInjector::UpdateMerkleNode))
            },
            EncodedDecoratorVariant::AdviceInjectorMapValueToStack => {
                let include_len = data_reader.read_bool()?;
                let key_offset = data_reader.read_usize()?;

                Ok(Decorator::Advice(AdviceInjector::MapValueToStack { include_len, key_offset }))
            },
            EncodedDecoratorVariant::AdviceInjectorU64Div => {
                Ok(Decorator::Advice(AdviceInjector::U64Div))
            },
            EncodedDecoratorVariant::AdviceInjectorExt2Inv => {
                Ok(Decorator::Advice(AdviceInjector::Ext2Inv))
            },
            EncodedDecoratorVariant::AdviceInjectorExt2Intt => {
                Ok(Decorator::Advice(AdviceInjector::Ext2Intt))
            },
            EncodedDecoratorVariant::AdviceInjectorSmtGet => {
                Ok(Decorator::Advice(AdviceInjector::SmtGet))
            },
            EncodedDecoratorVariant::AdviceInjectorSmtSet => {
                Ok(Decorator::Advice(AdviceInjector::SmtSet))
            },
            EncodedDecoratorVariant::AdviceInjectorSmtPeek => {
                Ok(Decorator::Advice(AdviceInjector::SmtPeek))
            },
            EncodedDecoratorVariant::AdviceInjectorU32Clz => {
                Ok(Decorator::Advice(AdviceInjector::U32Clz))
            },
            EncodedDecoratorVariant::AdviceInjectorU32Ctz => {
                Ok(Decorator::Advice(AdviceInjector::U32Ctz))
            },
            EncodedDecoratorVariant::AdviceInjectorU32Clo => {
                Ok(Decorator::Advice(AdviceInjector::U32Clo))
            },
            EncodedDecoratorVariant::AdviceInjectorU32Cto => {
                Ok(Decorator::Advice(AdviceInjector::U32Cto))
            },
            EncodedDecoratorVariant::AdviceInjectorILog2 => {
                Ok(Decorator::Advice(AdviceInjector::ILog2))
            },
            EncodedDecoratorVariant::AdviceInjectorMemToMap => {
                Ok(Decorator::Advice(AdviceInjector::MemToMap))
            },
            EncodedDecoratorVariant::AdviceInjectorHdwordToMap => {
                let domain = data_reader.read_u64()?;
                let domain = Felt::try_from(domain).map_err(|err| {
                    DeserializationError::InvalidValue(format!(
                        "Error when deserializing HdwordToMap decorator domain: {err}"
                    ))
                })?;

                Ok(Decorator::Advice(AdviceInjector::HdwordToMap { domain }))
            },
            EncodedDecoratorVariant::AdviceInjectorHpermToMap => {
                Ok(Decorator::Advice(AdviceInjector::HpermToMap))
            },
            EncodedDecoratorVariant::AdviceInjectorSigToStack => {
                Ok(Decorator::Advice(AdviceInjector::SigToStack {
                    kind: SignatureKind::RpoFalcon512,
                }))
            },
            EncodedDecoratorVariant::AssemblyOp => {
                let num_cycles = data_reader.read_u8()?;
                let should_break = data_reader.read_bool()?;

                // source location
                let location = if data_reader.read_bool()? {
                    let str_index_in_table = data_reader.read_usize()?;
                    let path = self.read_arc_str(str_index_in_table)?;
                    let start = data_reader.read_u32()?;
                    let end = data_reader.read_u32()?;
                    Some(crate::debuginfo::Location {
                        path,
                        start: start.into(),
                        end: end.into(),
                    })
                } else {
                    None
                };

                let context_name = {
                    let str_index_in_table = data_reader.read_usize()?;
                    self.read_string(str_index_in_table)?
                };

                let op = {
                    let str_index_in_table = data_reader.read_usize()?;
                    self.read_string(str_index_in_table)?
                };

                Ok(Decorator::AsmOp(AssemblyOp::new(
                    location,
                    context_name,
                    num_cycles,
                    op,
                    should_break,
                )))
            },
            EncodedDecoratorVariant::DebugOptionsStackAll => {
                Ok(Decorator::Debug(DebugOptions::StackAll))
            },
            EncodedDecoratorVariant::DebugOptionsStackTop => {
                let value = data_reader.read_u8()?;

                Ok(Decorator::Debug(DebugOptions::StackTop(value)))
            },
            EncodedDecoratorVariant::DebugOptionsMemAll => {
                Ok(Decorator::Debug(DebugOptions::MemAll))
            },
            EncodedDecoratorVariant::DebugOptionsMemInterval => {
                let start = data_reader.read_u32()?;
                let end = data_reader.read_u32()?;

                Ok(Decorator::Debug(DebugOptions::MemInterval(start, end)))
            },
            EncodedDecoratorVariant::DebugOptionsLocalInterval => {
                let start = data_reader.read_u16()?;
                let second = data_reader.read_u16()?;
                let end = data_reader.read_u16()?;

                Ok(Decorator::Debug(DebugOptions::LocalInterval(start, second, end)))
            },
            EncodedDecoratorVariant::Event => {
                let value = data_reader.read_u32()?;

                Ok(Decorator::Event(value))
            },
            EncodedDecoratorVariant::Trace => {
                let value = data_reader.read_u32()?;

                Ok(Decorator::Trace(value))
            },
        }
    }

    fn read_arc_str(&self, str_idx: StringIndex) -> Result<Arc<str>, DeserializationError> {
        if let Some(cached) = self.refc_strings.get(str_idx).and_then(|cell| cell.borrow().clone())
        {
            return Ok(cached);
        }

        let string = Arc::from(self.read_string(str_idx)?.into_boxed_str());
        *self.refc_strings[str_idx].borrow_mut() = Some(Arc::clone(&string));
        Ok(string)
    }

    fn read_string(&self, str_idx: StringIndex) -> Result<String, DeserializationError> {
        let str_offset = self.strings.get(str_idx).copied().ok_or_else(|| {
            DeserializationError::InvalidValue(format!("invalid index in strings table: {str_idx}"))
        })? as usize;

        let mut reader = SliceReader::new(&self.data[str_offset..]);
        reader.read()
    }
}
