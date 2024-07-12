use alloc::{collections::BTreeMap, vec::Vec};
use miden_crypto::hash::blake::{Blake3Digest, Blake3_256};
use winter_utils::{ByteWriter, Serializable};

use crate::{
    mast::{BasicBlockNode, OperationOrDecorator},
    AdviceInjector, DebugOptions, Decorator, SignatureKind,
};

use super::{decorator::EncodedDecoratorVariant, DataOffset, StringIndex, StringRef};

// BASIC BLOCK DATA BUILDER
// ================================================================================================

/// Builds the `data` section of a serialized [`crate::mast::MastForest`].
#[derive(Debug, Default)]
pub struct BasicBlockDataBuilder {
    data: Vec<u8>,
    string_table_builder: StringTableBuilder,
}

/// Constructors
impl BasicBlockDataBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Accessors
impl BasicBlockDataBuilder {
    /// Returns the current offset into the data buffer.
    pub fn get_offset(&self) -> DataOffset {
        self.data.len() as DataOffset
    }
}

/// Mutators
impl BasicBlockDataBuilder {
    /// Encodes a [`BasicBlockNode`] into the serialized [`crate::mast::MastForest`] data field.
    pub fn encode_basic_block(&mut self, basic_block: &BasicBlockNode) {
        // 2nd part of `mast_node_to_info()` (inside the match)
        for op_or_decorator in basic_block.iter() {
            match op_or_decorator {
                OperationOrDecorator::Operation(operation) => operation.write_into(&mut self.data),
                OperationOrDecorator::Decorator(decorator) => self.encode_decorator(decorator),
            }
        }
    }

    /// Returns the serialized [`crate::mast::MastForest`] data field, as well as the string table.
    pub fn into_parts(mut self) -> (Vec<u8>, Vec<StringRef>) {
        let string_table = self.string_table_builder.into_table(&mut self.data);
        (self.data, string_table)
    }
}

/// Helpers
impl BasicBlockDataBuilder {
    fn encode_decorator(&mut self, decorator: &Decorator) {
        // Set the first byte to the decorator discriminant.
        {
            let decorator_variant: EncodedDecoratorVariant = decorator.into();
            self.data.push(decorator_variant.discriminant());
        }

        // For decorators that have extra data, encode it in `data` and `strings`.
        match decorator {
            Decorator::Advice(advice_injector) => match advice_injector {
                AdviceInjector::MapValueToStack {
                    include_len,
                    key_offset,
                } => {
                    self.data.write_bool(*include_len);
                    self.data.write_usize(*key_offset);
                }
                AdviceInjector::HdwordToMap { domain } => {
                    self.data.extend(domain.as_int().to_le_bytes())
                }

                // Note: Since there is only 1 variant, we don't need to write any extra bytes.
                AdviceInjector::SigToStack { kind } => match kind {
                    SignatureKind::RpoFalcon512 => (),
                },
                AdviceInjector::MerkleNodeMerge
                | AdviceInjector::MerkleNodeToStack
                | AdviceInjector::UpdateMerkleNode
                | AdviceInjector::U64Div
                | AdviceInjector::Ext2Inv
                | AdviceInjector::Ext2Intt
                | AdviceInjector::SmtGet
                | AdviceInjector::SmtSet
                | AdviceInjector::SmtPeek
                | AdviceInjector::U32Clz
                | AdviceInjector::U32Ctz
                | AdviceInjector::U32Clo
                | AdviceInjector::U32Cto
                | AdviceInjector::ILog2
                | AdviceInjector::MemToMap
                | AdviceInjector::HpermToMap => (),
            },
            Decorator::AsmOp(assembly_op) => {
                self.data.push(assembly_op.num_cycles());
                self.data.write_bool(assembly_op.should_break());

                // context name
                {
                    let str_index_in_table =
                        self.string_table_builder.add_string(assembly_op.context_name());
                    self.data.write_usize(str_index_in_table);
                }

                // op
                {
                    let str_index_in_table = self.string_table_builder.add_string(assembly_op.op());
                    self.data.write_usize(str_index_in_table);
                }
            }
            Decorator::Debug(debug_options) => match debug_options {
                DebugOptions::StackTop(value) => self.data.push(*value),
                DebugOptions::MemInterval(start, end) => {
                    self.data.extend(start.to_le_bytes());
                    self.data.extend(end.to_le_bytes());
                }
                DebugOptions::LocalInterval(start, second, end) => {
                    self.data.extend(start.to_le_bytes());
                    self.data.extend(second.to_le_bytes());
                    self.data.extend(end.to_le_bytes());
                }
                DebugOptions::StackAll | DebugOptions::MemAll => (),
            },
            Decorator::Event(value) | Decorator::Trace(value) => {
                self.data.extend(value.to_le_bytes())
            }
        }
    }
}

// STRING TABLE BUILDER
// ================================================================================================

#[derive(Debug, Default)]
struct StringTableBuilder {
    table: Vec<StringRef>,
    str_to_index: BTreeMap<Blake3Digest<32>, StringIndex>,
    strings_data: Vec<u8>,
}

impl StringTableBuilder {
    pub fn add_string(&mut self, string: &str) -> StringIndex {
        if let Some(str_idx) = self.str_to_index.get(&Blake3_256::hash(string.as_bytes())) {
            // return already interned string
            *str_idx
        } else {
            // add new string to table
            // NOTE: these string refs' offset will need to be shifted again in `into_table()`
            let str_ref = StringRef {
                offset: self
                    .strings_data
                    .len()
                    .try_into()
                    .expect("strings table larger than 2^32 bytes"),
            };
            let str_idx = self.table.len();

            string.write_into(&mut self.strings_data);
            self.table.push(str_ref);
            self.str_to_index.insert(Blake3_256::hash(string.as_bytes()), str_idx);

            str_idx
        }
    }

    pub fn into_table(self, data: &mut Vec<u8>) -> Vec<StringRef> {
        let table_offset: u32 = data
            .len()
            .try_into()
            .expect("MAST forest serialization: data field longer than 2^32 bytes");
        data.extend(self.strings_data);

        self.table
            .into_iter()
            .map(|str_ref| StringRef {
                offset: str_ref.offset + table_offset,
            })
            .collect()
    }
}
