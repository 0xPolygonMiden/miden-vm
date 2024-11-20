use alloc::vec::Vec;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use winter_utils::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SliceReader,
};

use super::{
    string_table::{StringTable, StringTableBuilder},
    DecoratorDataOffset,
};
use crate::{AdviceInjector, AssemblyOp, DebugOptions, Decorator};

/// Represents a serialized [`Decorator`].
///
/// The serialized representation of [`DecoratorInfo`] is guaranteed to be fixed width, so that the
/// decorators stored in the `decorators` table of the serialized [`MastForest`] can be accessed
/// quickly by index.
#[derive(Debug)]
pub struct DecoratorInfo {
    variant: EncodedDecoratorVariant,
    decorator_data_offset: DecoratorDataOffset,
}

impl DecoratorInfo {
    pub fn from_decorator(
        decorator: &Decorator,
        data_builder: &mut DecoratorDataBuilder,
        string_table_builder: &mut StringTableBuilder,
    ) -> Self {
        let variant = EncodedDecoratorVariant::from(decorator);
        let decorator_data_offset =
            data_builder.encode_decorator_data(decorator, string_table_builder).unwrap_or(0);

        Self { variant, decorator_data_offset }
    }

    pub fn try_into_decorator(
        &self,
        string_table: &StringTable,
        decorator_data: &[u8],
    ) -> Result<Decorator, DeserializationError> {
        // This is safe because for decorators that don't use the offset, `0` is used (and hence
        // will never access an element outside). Note that in this implementation, we trust the
        // encoder.
        let mut data_reader =
            SliceReader::new(&decorator_data[self.decorator_data_offset as usize..]);
        match self.variant {
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
                Ok(Decorator::Advice(AdviceInjector::MapValueToStack))
            },
            EncodedDecoratorVariant::AdviceInjectorMapValueToStackN => {
                Ok(Decorator::Advice(AdviceInjector::MapValueToStackN))
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
                Ok(Decorator::Advice(AdviceInjector::HdwordToMap))
            },
            EncodedDecoratorVariant::AdviceInjectorHdwordToMapWithDomain => {
                Ok(Decorator::Advice(AdviceInjector::HdwordToMapWithDomain))
            },
            EncodedDecoratorVariant::AdviceInjectorHpermToMap => {
                Ok(Decorator::Advice(AdviceInjector::HpermToMap))
            },
            EncodedDecoratorVariant::AdviceInjectorFalconSigToStack => {
                Ok(Decorator::Advice(AdviceInjector::FalconSigToStack))
            },
            EncodedDecoratorVariant::AssemblyOp => {
                let num_cycles = data_reader.read_u8()?;
                let should_break = data_reader.read_bool()?;

                // source location
                let location = if data_reader.read_bool()? {
                    let str_index_in_table = data_reader.read_usize()?;
                    let path = string_table.read_arc_str(str_index_in_table)?;
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
                    string_table.read_string(str_index_in_table)?
                };

                let op = {
                    let str_index_in_table = data_reader.read_usize()?;
                    string_table.read_string(str_index_in_table)?
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
            EncodedDecoratorVariant::Trace => {
                let value = data_reader.read_u32()?;

                Ok(Decorator::Trace(value))
            },
        }
    }
}

impl Serializable for DecoratorInfo {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { variant, decorator_data_offset } = self;

        variant.write_into(target);
        decorator_data_offset.write_into(target);
    }
}

impl Deserializable for DecoratorInfo {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let variant = source.read()?;
        let decorator_data_offset = source.read()?;

        Ok(Self { variant, decorator_data_offset })
    }
}

// ENCODED DATA VARIANT
// ===============================================================================================

/// Stores all the possible [`Decorator`] variants, without any associated data.
///
/// This is effectively equivalent to a set of constants, and designed to convert between variant
/// discriminant and enum variant conveniently.
#[derive(Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum EncodedDecoratorVariant {
    AdviceInjectorMerkleNodeMerge,
    AdviceInjectorMerkleNodeToStack,
    AdviceInjectorUpdateMerkleNode,
    AdviceInjectorMapValueToStack,
    AdviceInjectorMapValueToStackN,
    AdviceInjectorU64Div,
    AdviceInjectorExt2Inv,
    AdviceInjectorExt2Intt,
    AdviceInjectorSmtPeek,
    AdviceInjectorU32Clz,
    AdviceInjectorU32Ctz,
    AdviceInjectorU32Clo,
    AdviceInjectorU32Cto,
    AdviceInjectorILog2,
    AdviceInjectorMemToMap,
    AdviceInjectorHdwordToMap,
    AdviceInjectorHdwordToMapWithDomain,
    AdviceInjectorHpermToMap,
    AdviceInjectorFalconSigToStack,
    AssemblyOp,
    DebugOptionsStackAll,
    DebugOptionsStackTop,
    DebugOptionsMemAll,
    DebugOptionsMemInterval,
    DebugOptionsLocalInterval,
    Trace,
}

impl EncodedDecoratorVariant {
    /// Returns the discriminant of the given decorator variant.
    ///
    /// To distinguish them from [`crate::Operation`] discriminants, the most significant bit of
    /// decorator discriminant is always set to 1.
    pub fn discriminant(&self) -> u8 {
        self.to_u8().expect("guaranteed to fit in a `u8` due to #[repr(u8)]")
    }

    /// The inverse operation of [`Self::discriminant`].
    pub fn from_discriminant(discriminant: u8) -> Option<Self> {
        Self::from_u8(discriminant)
    }
}

impl From<&Decorator> for EncodedDecoratorVariant {
    fn from(decorator: &Decorator) -> Self {
        match decorator {
            Decorator::Advice(advice_injector) => match advice_injector {
                AdviceInjector::MerkleNodeMerge => Self::AdviceInjectorMerkleNodeMerge,
                AdviceInjector::MerkleNodeToStack => Self::AdviceInjectorMerkleNodeToStack,
                AdviceInjector::UpdateMerkleNode => Self::AdviceInjectorUpdateMerkleNode,
                AdviceInjector::MapValueToStack => Self::AdviceInjectorMapValueToStack,
                AdviceInjector::MapValueToStackN => Self::AdviceInjectorMapValueToStackN,
                AdviceInjector::U64Div => Self::AdviceInjectorU64Div,
                AdviceInjector::Ext2Inv => Self::AdviceInjectorExt2Inv,
                AdviceInjector::Ext2Intt => Self::AdviceInjectorExt2Intt,
                AdviceInjector::SmtPeek => Self::AdviceInjectorSmtPeek,
                AdviceInjector::U32Clz => Self::AdviceInjectorU32Clz,
                AdviceInjector::U32Ctz => Self::AdviceInjectorU32Ctz,
                AdviceInjector::U32Clo => Self::AdviceInjectorU32Clo,
                AdviceInjector::U32Cto => Self::AdviceInjectorU32Cto,
                AdviceInjector::ILog2 => Self::AdviceInjectorILog2,
                AdviceInjector::MemToMap => Self::AdviceInjectorMemToMap,
                AdviceInjector::HdwordToMap => Self::AdviceInjectorHdwordToMap,
                AdviceInjector::HdwordToMapWithDomain => Self::AdviceInjectorHdwordToMapWithDomain,
                AdviceInjector::HpermToMap => Self::AdviceInjectorHpermToMap,
                AdviceInjector::FalconSigToStack => Self::AdviceInjectorFalconSigToStack,
            },
            Decorator::AsmOp(_) => Self::AssemblyOp,
            Decorator::Debug(debug_options) => match debug_options {
                DebugOptions::StackAll => Self::DebugOptionsStackAll,
                DebugOptions::StackTop(_) => Self::DebugOptionsStackTop,
                DebugOptions::MemAll => Self::DebugOptionsMemAll,
                DebugOptions::MemInterval(..) => Self::DebugOptionsMemInterval,
                DebugOptions::LocalInterval(..) => Self::DebugOptionsLocalInterval,
            },
            Decorator::Trace(_) => Self::Trace,
        }
    }
}

impl Serializable for EncodedDecoratorVariant {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.discriminant().write_into(target);
    }
}

impl Deserializable for EncodedDecoratorVariant {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let discriminant: u8 = source.read_u8()?;

        Self::from_discriminant(discriminant).ok_or_else(|| {
            DeserializationError::InvalidValue(format!(
                "invalid decorator discriminant: {discriminant}"
            ))
        })
    }
}

// DECORATOR DATA BUILDER
// ===============================================================================================

/// Builds the decorator `data` section of a serialized [`crate::mast::MastForest`].
#[derive(Debug, Default)]
pub struct DecoratorDataBuilder {
    decorator_data: Vec<u8>,
}

/// Constructors
impl DecoratorDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Mutators
impl DecoratorDataBuilder {
    /// If a decorator has extra data to store, encode it in internal data buffer, and return the
    /// offset of the newly added data. If not, return `None`.
    pub fn encode_decorator_data(
        &mut self,
        decorator: &Decorator,
        string_table_builder: &mut StringTableBuilder,
    ) -> Option<DecoratorDataOffset> {
        let data_offset = self.decorator_data.len() as DecoratorDataOffset;

        match decorator {
            Decorator::Advice(_) => None,
            Decorator::AsmOp(assembly_op) => {
                self.decorator_data.push(assembly_op.num_cycles());
                self.decorator_data.write_bool(assembly_op.should_break());

                // source location
                let loc = assembly_op.location();
                self.decorator_data.write_bool(loc.is_some());
                if let Some(loc) = loc {
                    let str_offset = string_table_builder.add_string(loc.path.as_ref());
                    self.decorator_data.write_usize(str_offset);
                    self.decorator_data.write_u32(loc.start.to_u32());
                    self.decorator_data.write_u32(loc.end.to_u32());
                }

                // context name
                {
                    let str_offset = string_table_builder.add_string(assembly_op.context_name());
                    self.decorator_data.write_usize(str_offset);
                }

                // op
                {
                    let str_index_in_table = string_table_builder.add_string(assembly_op.op());
                    self.decorator_data.write_usize(str_index_in_table);
                }

                Some(data_offset)
            },
            Decorator::Debug(debug_options) => match debug_options {
                DebugOptions::StackTop(value) => {
                    self.decorator_data.push(*value);
                    Some(data_offset)
                },
                DebugOptions::MemInterval(start, end) => {
                    self.decorator_data.extend(start.to_le_bytes());
                    self.decorator_data.extend(end.to_le_bytes());

                    Some(data_offset)
                },
                DebugOptions::LocalInterval(start, second, end) => {
                    self.decorator_data.extend(start.to_le_bytes());
                    self.decorator_data.extend(second.to_le_bytes());
                    self.decorator_data.extend(end.to_le_bytes());

                    Some(data_offset)
                },
                DebugOptions::StackAll | DebugOptions::MemAll => None,
            },
            Decorator::Trace(value) => {
                self.decorator_data.extend(value.to_le_bytes());

                Some(data_offset)
            },
        }
    }

    /// Returns the serialized [`crate::mast::MastForest`] decorator data field.
    pub fn finalize(self) -> Vec<u8> {
        self.decorator_data
    }
}
