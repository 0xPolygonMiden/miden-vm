use alloc::vec::Vec;

use winter_utils::{ByteWriter, Serializable};

use super::{DecoratorDataOffset, NodeDataOffset};
use crate::mast::{BasicBlockNode, OperationOrDecorator};

// BASIC BLOCK DATA BUILDER
// ================================================================================================

/// Builds the node `data` section of a serialized [`crate::mast::MastForest`].
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
    ) -> (NodeDataOffset, Option<DecoratorDataOffset>) {
        let ops_offset = self.node_data.len() as NodeDataOffset;

        // TODO(plafer): implement an `Operation` iterator
        // TODO(plafer): Store `Vec<Operation>` instead of explicitly storing length
        self.node_data.write_usize(basic_block.num_operations() as usize);
        for op_or_decorator in basic_block.iter() {
            match op_or_decorator {
                OperationOrDecorator::Operation(operation) => {
                    operation.write_into(&mut self.node_data)
                },
                OperationOrDecorator::Decorator(_) => {
                    // do nothing
                },
            }
        }

        if basic_block.decorators().is_empty() {
            (ops_offset, None)
        } else {
            let decorator_data_offset = self.node_data.len() as DecoratorDataOffset;
            basic_block.decorators().write_into(&mut self.node_data);

            (ops_offset, Some(decorator_data_offset))
        }
    }

    /// Returns the serialized [`crate::mast::MastForest`] nod data field.
    pub fn finalize(self) -> Vec<u8> {
        self.node_data
    }
}
