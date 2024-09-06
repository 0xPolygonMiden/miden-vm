use alloc::vec::Vec;

use winter_utils::{ByteWriter, Serializable};

use super::{decorator::EncodedDecoratorVariant, string_table::StringTableBuilder, NodeDataOffset};
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
    pub fn encode_basic_block(
        &mut self,
        basic_block: &BasicBlockNode,
        mast_forest: &MastForest,
        string_table_builder: &mut StringTableBuilder,
    ) {
        // 2nd part of `mast_node_to_info()` (inside the match)
        for op_or_decorator in basic_block.iter() {
            match op_or_decorator {
                OperationOrDecorator::Operation(operation) => {
                    operation.write_into(&mut self.node_data)
                },
                OperationOrDecorator::Decorator(&decorator_id) => {
                    self.encode_decorator(&mast_forest[decorator_id], string_table_builder)
                },
            }
        }
    }

    /// Returns the serialized [`crate::mast::MastForest`] nod data field.
    pub fn finalize(self) -> Vec<u8> {
        self.node_data
    }
}

/// Helpers
impl BasicBlockDataBuilder {
    fn encode_decorator(
        &mut self,
        decorator: &Decorator,
        string_table_builder: &mut StringTableBuilder,
    ) {
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
                    let str_offset = string_table_builder.add_string(loc.path.as_ref());
                    self.node_data.write_usize(str_offset);
                    self.node_data.write_u32(loc.start.to_u32());
                    self.node_data.write_u32(loc.end.to_u32());
                }

                // context name
                {
                    let str_offset = string_table_builder.add_string(assembly_op.context_name());
                    self.node_data.write_usize(str_offset);
                }

                // op
                {
                    let str_index_in_table = string_table_builder.add_string(assembly_op.op());
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
