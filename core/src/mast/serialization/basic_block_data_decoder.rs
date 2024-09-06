use alloc::vec::Vec;

use winter_utils::{ByteReader, DeserializationError, SliceReader};

use super::NodeDataOffset;
use crate::{mast::MastForest, DecoratorList, Operation};

pub struct BasicBlockDataDecoder<'a> {
    node_data: &'a [u8],
}

/// Constructors
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn new(node_data: &'a [u8]) -> Self {
        Self { node_data }
    }
}

/// Mutators
impl<'a> BasicBlockDataDecoder<'a> {
    pub fn decode_operations_and_decorators(
        &self,
        ops_offset: NodeDataOffset,
        decorator_list_offset: NodeDataOffset,
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

            decorators_data_reader.read()?
        };

        Ok((operations, decorators))
    }
}
