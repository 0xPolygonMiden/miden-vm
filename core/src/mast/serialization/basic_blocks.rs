use alloc::vec::Vec;

use winter_utils::{ByteReader, DeserializationError, Serializable, SliceReader};

use super::{DecoratorDataOffset, NodeDataOffset};
use crate::{
    mast::{BasicBlockNode, DecoratorId, MastForest},
    DecoratorList, Operation,
};

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

/// Mutators
impl BasicBlockDataBuilder {
    /// Encodes a [`BasicBlockNode`] into the serialized [`crate::mast::MastForest`] data field.
    pub fn encode_basic_block(
        &mut self,
        basic_block: &BasicBlockNode,
    ) -> (NodeDataOffset, Option<DecoratorDataOffset>) {
        let ops_offset = self.node_data.len() as NodeDataOffset;

        let operations: Vec<Operation> = basic_block.operations().copied().collect();
        operations.write_into(&mut self.node_data);

        if basic_block.decorators().is_empty() {
            (ops_offset, None)
        } else {
            let decorator_data_offset = self.node_data.len() as DecoratorDataOffset;
            basic_block.decorators().write_into(&mut self.node_data);

            (ops_offset, Some(decorator_data_offset))
        }
    }

    /// Returns the serialized [`crate::mast::MastForest`] node data field.
    pub fn finalize(self) -> Vec<u8> {
        self.node_data
    }
}

// BASIC BLOCK DATA DECODER
// ================================================================================================

pub struct BasicBlockDataDecoder<'a> {
    node_data: &'a [u8],
}

/// Constructors
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn new(node_data: &'a [u8]) -> Self {
        Self { node_data }
    }
}

/// Decoding methods
impl BasicBlockDataDecoder<'_> {
    pub fn decode_operations_and_decorators(
        &self,
        ops_offset: NodeDataOffset,
        decorator_list_offset: NodeDataOffset,
        mast_forest: &MastForest,
    ) -> Result<(Vec<Operation>, DecoratorList), DeserializationError> {
        // Read ops
        let mut ops_data_reader = SliceReader::new(&self.node_data[ops_offset as usize..]);
        let operations: Vec<Operation> = ops_data_reader.read()?;

        // read decorators only if there are some
        let decorators = if decorator_list_offset == MastForest::MAX_DECORATORS as u32 {
            Vec::new()
        } else {
            let mut decorators_data_reader =
                SliceReader::new(&self.node_data[decorator_list_offset as usize..]);

            let num_decorators: usize = decorators_data_reader.read()?;
            (0..num_decorators)
                .map(|_| {
                    let decorator_loc: usize = decorators_data_reader.read()?;
                    let decorator_id =
                        DecoratorId::from_u32_safe(decorators_data_reader.read()?, mast_forest)?;

                    Ok((decorator_loc, decorator_id))
                })
                .collect::<Result<DecoratorList, _>>()?
        };

        Ok((operations, decorators))
    }
}
