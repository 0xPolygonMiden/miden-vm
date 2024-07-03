use alloc::{string::String, vec::Vec};
use miden_crypto::{Felt, ZERO};
use winter_utils::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SliceReader,
};

use crate::{
    mast::MerkleTreeNode, AdviceInjector, AssemblyOp, DebugOptions, Decorator, DecoratorList,
    Operation, OperationData, SignatureKind,
};

use super::{MastForest, MastNode, MastNodeId};

mod decorator;
use decorator::EncodedDecoratorVariant;

mod info;
use info::{MastNodeInfo, MastNodeType};

mod basic_block_data_builder;
use basic_block_data_builder::BasicBlockDataBuilder;

mod basic_block_data_decoder;
use basic_block_data_decoder::BasicBlockDataDecoder;

#[cfg(test)]
mod tests;

/// Specifies an offset into the `data` section of an encoded [`MastForest`].
type DataOffset = u32;

/// Specifies an offset into the `strings` table of an encoded [`MastForest`]
type StringIndex = usize;

/// Magic string for detecting that a file is binary-encoded MAST.
const MAGIC: &[u8; 5] = b"MAST\0";

/// The format version.
///
/// If future modifications are made to this format, the version should be incremented by 1. A
/// version of `[255, 255, 255]` is reserved for future extensions that require extending the
/// version field itself, but should be considered invalid for now.
const VERSION: [u8; 3] = [0, 0, 0];

/// An entry in the `strings` table of an encoded [`MastForest`].
///
/// Strings are UTF8-encoded.
#[derive(Debug)]
pub struct StringRef {
    /// Offset into the `data` section.
    offset: DataOffset,

    /// Length of the utf-8 string.
    len: u32,
}

impl Serializable for StringRef {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.offset.write_into(target);
        self.len.write_into(target);
    }
}

impl Deserializable for StringRef {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let offset = DataOffset::read_from(source)?;
        let len = source.read_u32()?;

        Ok(Self { offset, len })
    }
}

impl Serializable for MastForest {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let mut basic_block_data_builder = BasicBlockDataBuilder::new();

        // magic & version
        target.write_bytes(MAGIC);
        target.write_bytes(&VERSION);

        // node count
        target.write_usize(self.nodes.len());

        // roots
        self.roots.write_into(target);

        // MAST node infos
        for mast_node in &self.nodes {
            let mast_node_info =
                MastNodeInfo::new(mast_node, basic_block_data_builder.current_data_offset());

            if let MastNode::Block(basic_block) = mast_node {
                basic_block_data_builder.encode_basic_block(basic_block);
            }

            mast_node_info.write_into(target);
        }

        let (data, string_table) = basic_block_data_builder.into_parts();

        string_table.write_into(target);
        data.write_into(target);
    }
}

impl Deserializable for MastForest {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let magic: [u8; 5] = source.read_array()?;
        if magic != *MAGIC {
            return Err(DeserializationError::InvalidValue(format!(
                "Invalid magic bytes. Expected '{:?}', got '{:?}'",
                *MAGIC, magic
            )));
        }

        let version: [u8; 3] = source.read_array()?;
        if version != VERSION {
            return Err(DeserializationError::InvalidValue(format!(
                "Unsupported version. Got '{version:?}', but only '{VERSION:?}' is supported",
            )));
        }

        let node_count = source.read_usize()?;

        let roots: Vec<MastNodeId> = Deserializable::read_from(source)?;

        let mast_node_infos = {
            let mut mast_node_infos = Vec::with_capacity(node_count);
            for _ in 0..node_count {
                let mast_node_info = MastNodeInfo::read_from(source)?;
                mast_node_infos.push(mast_node_info);
            }

            mast_node_infos
        };

        let strings: Vec<StringRef> = Deserializable::read_from(source)?;

        let data: Vec<u8> = Deserializable::read_from(source)?;
        let mut data_reader = SliceReader::new(&data);

        let mast_forest = {
            let mut mast_forest = MastForest::new();

            for mast_node_info in mast_node_infos {
                let node = try_info_to_mast_node(
                    mast_node_info,
                    &mast_forest,
                    &mut data_reader,
                    &data,
                    &strings,
                )?;
                mast_forest.add_node(node);
            }

            for root in roots {
                mast_forest.make_root(root);
            }

            mast_forest
        };

        Ok(mast_forest)
    }
}

// TODOP: Make `MastNodeInfo` method
// TODOP: Can we not have both `data` and `data_reader`?
fn try_info_to_mast_node(
    mast_node_info: MastNodeInfo,
    mast_forest: &MastForest,
    data_reader: &mut SliceReader,
    data: &[u8],
    strings: &[StringRef],
) -> Result<MastNode, DeserializationError> {
    let mast_node = match mast_node_info.ty {
        MastNodeType::Block {
            len: num_operations_and_decorators,
        } => {
            let (operations, decorators) = decode_operations_and_decorators(
                num_operations_and_decorators,
                data_reader,
                data,
                strings,
            )?;

            Ok(MastNode::new_basic_block_with_decorators(operations, decorators))
        }
        MastNodeType::Join {
            left_child_id,
            right_child_id,
        } => {
            let left_child = MastNodeId::from_u32_safe(left_child_id, mast_forest)?;
            let right_child = MastNodeId::from_u32_safe(right_child_id, mast_forest)?;

            Ok(MastNode::new_join(left_child, right_child, mast_forest))
        }
        MastNodeType::Split {
            if_branch_id,
            else_branch_id,
        } => {
            let if_branch = MastNodeId::from_u32_safe(if_branch_id, mast_forest)?;
            let else_branch = MastNodeId::from_u32_safe(else_branch_id, mast_forest)?;

            Ok(MastNode::new_split(if_branch, else_branch, mast_forest))
        }
        MastNodeType::Loop { body_id } => {
            let body_id = MastNodeId::from_u32_safe(body_id, mast_forest)?;

            Ok(MastNode::new_loop(body_id, mast_forest))
        }
        MastNodeType::Call { callee_id } => {
            let callee_id = MastNodeId::from_u32_safe(callee_id, mast_forest)?;

            Ok(MastNode::new_call(callee_id, mast_forest))
        }
        MastNodeType::SysCall { callee_id } => {
            let callee_id = MastNodeId::from_u32_safe(callee_id, mast_forest)?;

            Ok(MastNode::new_syscall(callee_id, mast_forest))
        }
        MastNodeType::Dyn => Ok(MastNode::new_dynexec()),
        MastNodeType::External => Ok(MastNode::new_external(mast_node_info.digest)),
    }?;

    if mast_node.digest() == mast_node_info.digest {
        Ok(mast_node)
    } else {
        Err(DeserializationError::InvalidValue(format!(
            "MastNodeInfo's digest '{}' doesn't match deserialized MastNode's digest '{}'",
            mast_node_info.digest,
            mast_node.digest()
        )))
    }
}

fn decode_operations_and_decorators(
    num_to_decode: u32,
    data_reader: &mut SliceReader,
    data: &[u8],
    strings: &[StringRef],
) -> Result<(Vec<Operation>, DecoratorList), DeserializationError> {
    let mut operations: Vec<Operation> = Vec::new();
    let mut decorators: DecoratorList = Vec::new();

    for _ in 0..num_to_decode {
        let first_byte = data_reader.read_u8()?;

        if first_byte & 0b1000_0000 == 0 {
            // operation.
            let op_code = first_byte;

            let maybe_operation = if op_code == Operation::Assert(0_u32).op_code()
                || op_code == Operation::MpVerify(0_u32).op_code()
            {
                let value_le_bytes: [u8; 4] = data_reader.read_array()?;
                let value = u32::from_le_bytes(value_le_bytes);

                Operation::with_opcode_and_data(op_code, OperationData::U32(value))
            } else if op_code == Operation::U32assert2(ZERO).op_code()
                || op_code == Operation::Push(ZERO).op_code()
            {
                // Felt operation data
                let value_le_bytes: [u8; 8] = data_reader.read_array()?;
                let value_u64 = u64::from_le_bytes(value_le_bytes);
                let value_felt = Felt::try_from(value_u64).map_err(|_| {
                    DeserializationError::InvalidValue(format!(
                        "Operation associated data doesn't fit in a field element: {value_u64}"
                    ))
                })?;

                Operation::with_opcode_and_data(op_code, OperationData::Felt(value_felt))
            } else {
                // No operation data
                Operation::with_opcode_and_data(op_code, OperationData::None)
            };

            let operation = maybe_operation.ok_or_else(|| {
                DeserializationError::InvalidValue(format!("invalid op code: {op_code}"))
            })?;

            operations.push(operation);
        } else {
            // decorator.
            let discriminant = first_byte & 0b0111_1111;
            let decorator = decode_decorator(discriminant, data_reader, data, strings)?;

            decorators.push((operations.len(), decorator));
        }
    }

    Ok((operations, decorators))
}

fn decode_decorator(
    discriminant: u8,
    data_reader: &mut SliceReader,
    data: &[u8],
    strings: &[StringRef],
) -> Result<Decorator, DeserializationError> {
    let decorator_variant =
        EncodedDecoratorVariant::from_discriminant(discriminant).ok_or_else(|| {
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
                read_string(str_index_in_table, data, strings)?
            };

            let op = {
                let str_index_in_table = data_reader.read_usize()?;
                read_string(str_index_in_table, data, strings)?
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
        EncodedDecoratorVariant::DebugOptionsMemAll => Ok(Decorator::Debug(DebugOptions::MemAll)),
        EncodedDecoratorVariant::DebugOptionsMemInterval => {
            let start = u32::from_le_bytes(data_reader.read_array::<4>()?);
            let end = u32::from_le_bytes(data_reader.read_array::<4>()?);

            Ok(Decorator::Debug(DebugOptions::MemInterval(start, end)))
        }
        EncodedDecoratorVariant::DebugOptionsLocalInterval => {
            let start = u16::from_le_bytes(data_reader.read_array::<2>()?);
            let second = u16::from_le_bytes(data_reader.read_array::<2>()?);
            let end = u16::from_le_bytes(data_reader.read_array::<2>()?);

            Ok(Decorator::Debug(DebugOptions::LocalInterval(start, second, end)))
        }
        EncodedDecoratorVariant::Event => {
            let value = u32::from_le_bytes(data_reader.read_array::<4>()?);

            Ok(Decorator::Event(value))
        }
        EncodedDecoratorVariant::Trace => {
            let value = u32::from_le_bytes(data_reader.read_array::<4>()?);

            Ok(Decorator::Trace(value))
        }
    }
}

// TODOP: Rename and/or move to some struct?
fn read_string(
    str_index_in_table: usize,
    data: &[u8],
    strings: &[StringRef],
) -> Result<String, DeserializationError> {
    let str_ref = strings.get(str_index_in_table).ok_or_else(|| {
        DeserializationError::InvalidValue(format!(
            "invalid index in strings table: {str_index_in_table}"
        ))
    })?;

    let str_bytes = {
        let start = str_ref.offset as usize;
        let end = (str_ref.offset + str_ref.len) as usize;

        data.get(start..end).ok_or_else(|| {
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
