use alloc::{collections::BTreeMap, vec::Vec};

use miden_crypto::hash::blake::{Blake3Digest, Blake3_256};
use winter_utils::{ByteWriter, Serializable};

use super::{decorator::EncodedDecoratorVariant, NodeDataOffset, StringDataOffset};
use crate::{
    mast::{BasicBlockNode, MastForest, OperationOrDecorator},
    AdviceInjector, DebugOptions, Decorator, SignatureKind,
};

// BASIC BLOCK DATA BUILDER
// ================================================================================================

/// Builds the `data` section of a serialized [`crate::mast::MastForest`].
#[derive(Debug, Default)]
pub struct BasicBlockDataBuilder {
    node_data: Vec<u8>,
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
    pub fn get_offset(&self) -> NodeDataOffset {
        self.node_data.len() as NodeDataOffset
    }
}

/// Mutators
impl BasicBlockDataBuilder {
    /// Encodes a [`BasicBlockNode`] into the serialized [`crate::mast::MastForest`] data field.
    pub fn encode_basic_block(&mut self, basic_block: &BasicBlockNode, mast_forest: &MastForest) {
        // 2nd part of `mast_node_to_info()` (inside the match)
        for op_or_decorator in basic_block.iter() {
            match op_or_decorator {
                OperationOrDecorator::Operation(operation) => {
                    operation.write_into(&mut self.node_data)
                },
                OperationOrDecorator::Decorator(&decorator_id) => {
                    self.encode_decorator(&mast_forest[decorator_id])
                },
            }
        }
    }

    // TODO(plafer): Make nice types for `Vec<u8>`
    /// Returns the serialized [`crate::mast::MastForest`] data field, as well as the string table.
    pub fn into_parts(self) -> (Vec<u8>, Vec<u8>) {
        let string_table = self.string_table_builder.into_table();
        (self.node_data, string_table)
    }
}

/// Helpers
impl BasicBlockDataBuilder {
    fn encode_decorator(&mut self, decorator: &Decorator) {
        // Set the first byte to the decorator discriminant.
        {
            let decorator_variant: EncodedDecoratorVariant = decorator.into();
            self.node_data.push(decorator_variant.discriminant());
        }

        // For decorators that have extra data, encode it in `data` and `strings`.
        match decorator {
            Decorator::Advice(advice_injector) => match advice_injector {
                AdviceInjector::MapValueToStack { include_len, key_offset } => {
                    self.node_data.write_bool(*include_len);
                    self.node_data.write_usize(*key_offset);
                },
                AdviceInjector::HdwordToMap { domain } => {
                    self.node_data.extend(domain.as_int().to_le_bytes())
                },

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
                self.node_data.push(assembly_op.num_cycles());
                self.node_data.write_bool(assembly_op.should_break());

                // source location
                let loc = assembly_op.location();
                self.node_data.write_bool(loc.is_some());
                if let Some(loc) = loc {
                    let str_offset = self.string_table_builder.add_string(loc.path.as_ref());
                    self.node_data.write_usize(str_offset);
                    self.node_data.write_u32(loc.start.to_u32());
                    self.node_data.write_u32(loc.end.to_u32());
                }

                // context name
                {
                    let str_offset =
                        self.string_table_builder.add_string(assembly_op.context_name());
                    self.node_data.write_usize(str_offset);
                }

                // op
                {
                    let str_index_in_table = self.string_table_builder.add_string(assembly_op.op());
                    self.node_data.write_usize(str_index_in_table);
                }
            },
            Decorator::Debug(debug_options) => match debug_options {
                DebugOptions::StackTop(value) => self.node_data.push(*value),
                DebugOptions::MemInterval(start, end) => {
                    self.node_data.extend(start.to_le_bytes());
                    self.node_data.extend(end.to_le_bytes());
                },
                DebugOptions::LocalInterval(start, second, end) => {
                    self.node_data.extend(start.to_le_bytes());
                    self.node_data.extend(second.to_le_bytes());
                    self.node_data.extend(end.to_le_bytes());
                },
                DebugOptions::StackAll | DebugOptions::MemAll => (),
            },
            Decorator::Event(value) | Decorator::Trace(value) => {
                self.node_data.extend(value.to_le_bytes())
            },
        }
    }
}

// STRING TABLE BUILDER
// ================================================================================================

#[derive(Debug, Default)]
struct StringTableBuilder {
    str_to_offset: BTreeMap<Blake3Digest<32>, StringDataOffset>,
    strings_data: Vec<u8>,
}

impl StringTableBuilder {
    pub fn add_string(&mut self, string: &str) -> StringDataOffset {
        if let Some(str_idx) = self.str_to_offset.get(&Blake3_256::hash(string.as_bytes())) {
            // return already interned string
            *str_idx
        } else {
            // add new string to table
            let str_offset = self.strings_data.len();
            assert!(str_offset <= u32::MAX as usize, "strings table larger than 2^32 bytes");

            string.write_into(&mut self.strings_data);
            self.str_to_offset.insert(Blake3_256::hash(string.as_bytes()), str_offset);

            str_offset
        }
    }

    pub fn into_table(self) -> Vec<u8> {
        self.strings_data
    }
}
