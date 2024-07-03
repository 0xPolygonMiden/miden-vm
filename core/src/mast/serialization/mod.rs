use alloc::{string::String, vec::Vec};
use miden_crypto::{Felt, ZERO};
use num_traits::ToBytes;
use winter_utils::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SliceReader,
};

use crate::{
    mast::{MerkleTreeNode, OperationOrDecorator},
    AdviceInjector, AssemblyOp, DebugOptions, Decorator, DecoratorList, Operation, OperationData,
    SignatureKind,
};

use super::{MastForest, MastNode, MastNodeId};

mod decorator;
use decorator::EncodedDecoratorVariant;

mod info;
use info::{EncodedMastNodeType, MastNodeInfo, MastNodeTypeVariant};

mod string_table_builder;
use string_table_builder::StringTableBuilder;

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
        // TODOP: make sure padding is in accordance with Paul's docs
        let mut string_table_builder = StringTableBuilder::new();
        let mut data: Vec<u8> = Vec::new();

        // magic & version
        target.write_bytes(MAGIC);
        target.write_bytes(&VERSION);

        // node count
        target.write_usize(self.nodes.len());

        // roots
        self.roots.write_into(target);

        // MAST node infos
        for mast_node in &self.nodes {
            let mast_node_info = mast_node_to_info(mast_node, &mut data, &mut string_table_builder);

            mast_node_info.write_into(target);
        }

        // strings table
        let strings = string_table_builder.into_table(&mut data);
        strings.write_into(target);

        // data blob
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

fn mast_node_to_info(
    mast_node: &MastNode,
    data: &mut Vec<u8>,
    string_table_builder: &mut StringTableBuilder,
) -> MastNodeInfo {
    use MastNode::*;

    let ty = EncodedMastNodeType::new(mast_node);
    let digest = mast_node.digest();

    let offset = match mast_node {
        Block(basic_block) => {
            let offset: u32 = data
                .len()
                .try_into()
                .expect("MastForest serialization: data field larger than 2^32 bytes");

            for op_or_decorator in basic_block.iter() {
                match op_or_decorator {
                    OperationOrDecorator::Operation(operation) => encode_operation(operation, data),
                    OperationOrDecorator::Decorator(decorator) => {
                        encode_decorator(decorator, data, string_table_builder)
                    }
                }
            }

            offset
        }
        Join(_) | Split(_) | Loop(_) | Call(_) | Dyn | External(_) => 0,
    };

    MastNodeInfo { ty, offset, digest }
}

fn encode_operation(operation: &Operation, data: &mut Vec<u8>) {
    data.push(operation.op_code());

    // For operations that have extra data, encode it in `data`.
    match operation {
        Operation::Assert(value) | Operation::MpVerify(value) => {
            data.extend_from_slice(&value.to_le_bytes())
        }
        Operation::U32assert2(value) | Operation::Push(value) => {
            data.extend_from_slice(&value.as_int().to_le_bytes())
        }
        // Note: we explicitly write out all the operations so that whenever we make a modification
        // to the `Operation` enum, we get a compile error here. This should help us remember to
        // properly encode/decode each operation variant.
        Operation::Noop
        | Operation::FmpAdd
        | Operation::FmpUpdate
        | Operation::SDepth
        | Operation::Caller
        | Operation::Clk
        | Operation::Join
        | Operation::Split
        | Operation::Loop
        | Operation::Call
        | Operation::Dyn
        | Operation::SysCall
        | Operation::Span
        | Operation::End
        | Operation::Repeat
        | Operation::Respan
        | Operation::Halt
        | Operation::Add
        | Operation::Neg
        | Operation::Mul
        | Operation::Inv
        | Operation::Incr
        | Operation::And
        | Operation::Or
        | Operation::Not
        | Operation::Eq
        | Operation::Eqz
        | Operation::Expacc
        | Operation::Ext2Mul
        | Operation::U32split
        | Operation::U32add
        | Operation::U32add3
        | Operation::U32sub
        | Operation::U32mul
        | Operation::U32madd
        | Operation::U32div
        | Operation::U32and
        | Operation::U32xor
        | Operation::Pad
        | Operation::Drop
        | Operation::Dup0
        | Operation::Dup1
        | Operation::Dup2
        | Operation::Dup3
        | Operation::Dup4
        | Operation::Dup5
        | Operation::Dup6
        | Operation::Dup7
        | Operation::Dup9
        | Operation::Dup11
        | Operation::Dup13
        | Operation::Dup15
        | Operation::Swap
        | Operation::SwapW
        | Operation::SwapW2
        | Operation::SwapW3
        | Operation::SwapDW
        | Operation::MovUp2
        | Operation::MovUp3
        | Operation::MovUp4
        | Operation::MovUp5
        | Operation::MovUp6
        | Operation::MovUp7
        | Operation::MovUp8
        | Operation::MovDn2
        | Operation::MovDn3
        | Operation::MovDn4
        | Operation::MovDn5
        | Operation::MovDn6
        | Operation::MovDn7
        | Operation::MovDn8
        | Operation::CSwap
        | Operation::CSwapW
        | Operation::AdvPop
        | Operation::AdvPopW
        | Operation::MLoadW
        | Operation::MStoreW
        | Operation::MLoad
        | Operation::MStore
        | Operation::MStream
        | Operation::Pipe
        | Operation::HPerm
        | Operation::MrUpdate
        | Operation::FriE2F4
        | Operation::RCombBase => (),
    }
}

fn encode_decorator(
    decorator: &Decorator,
    data: &mut Vec<u8>,
    string_table_builder: &mut StringTableBuilder,
) {
    // Set the first byte to the decorator discriminant.
    //
    // Note: the most significant bit is set to 1 (to differentiate decorators from operations).
    {
        let decorator_variant: EncodedDecoratorVariant = decorator.into();
        data.push(decorator_variant.discriminant() | 0b1000_0000);
    }

    // For decorators that have extra data, encode it in `data` and `strings`.
    match decorator {
        Decorator::Advice(advice_injector) => match advice_injector {
            AdviceInjector::MapValueToStack {
                include_len,
                key_offset,
            } => {
                data.write_bool(*include_len);
                data.write_usize(*key_offset);
            }
            AdviceInjector::HdwordToMap { domain } => data.extend(domain.as_int().to_le_bytes()),

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
            data.push(assembly_op.num_cycles());
            data.write_bool(assembly_op.should_break());

            // context name
            {
                let str_index_in_table =
                    string_table_builder.add_string(assembly_op.context_name());
                data.write_usize(str_index_in_table);
            }

            // op
            {
                let str_index_in_table = string_table_builder.add_string(assembly_op.op());
                data.write_usize(str_index_in_table);
            }
        }
        Decorator::Debug(debug_options) => match debug_options {
            DebugOptions::StackTop(value) => data.push(*value),
            DebugOptions::MemInterval(start, end) => {
                data.extend(start.to_le_bytes());
                data.extend(end.to_le_bytes());
            }
            DebugOptions::LocalInterval(start, second, end) => {
                data.extend(start.to_le_bytes());
                data.extend(second.to_le_bytes());
                data.extend(end.to_le_bytes());
            }
            DebugOptions::StackAll | DebugOptions::MemAll => (),
        },
        Decorator::Event(value) | Decorator::Trace(value) => data.extend(value.to_le_bytes()),
    }
}

fn try_info_to_mast_node(
    mast_node_info: MastNodeInfo,
    mast_forest: &MastForest,
    data_reader: &mut SliceReader,
    data: &[u8],
    strings: &[StringRef],
) -> Result<MastNode, DeserializationError> {
    let mast_node_variant = mast_node_info.ty.variant()?;

    // TODOP: Make a faillible version of `MastNode` ctors
    // TODOP: Check digest of resulting `MastNode` matches `MastNodeInfo.digest`?
    match mast_node_variant {
        MastNodeTypeVariant::Block => {
            let num_operations_and_decorators =
                EncodedMastNodeType::decode_u32_payload(&mast_node_info.ty);

            let (operations, decorators) = decode_operations_and_decorators(
                num_operations_and_decorators,
                data_reader,
                data,
                strings,
            )?;

            Ok(MastNode::new_basic_block_with_decorators(operations, decorators))
        }
        MastNodeTypeVariant::Join => {
            let (left_child, right_child) =
                EncodedMastNodeType::decode_join_or_split(&mast_node_info.ty);

            Ok(MastNode::new_join(left_child, right_child, mast_forest))
        }
        MastNodeTypeVariant::Split => {
            let (if_branch, else_branch) =
                EncodedMastNodeType::decode_join_or_split(&mast_node_info.ty);

            Ok(MastNode::new_split(if_branch, else_branch, mast_forest))
        }
        MastNodeTypeVariant::Loop => {
            let body_id = EncodedMastNodeType::decode_u32_payload(&mast_node_info.ty);

            Ok(MastNode::new_loop(MastNodeId(body_id), mast_forest))
        }
        MastNodeTypeVariant::Call => {
            let callee_id = EncodedMastNodeType::decode_u32_payload(&mast_node_info.ty);

            Ok(MastNode::new_call(MastNodeId(callee_id), mast_forest))
        }
        MastNodeTypeVariant::Syscall => {
            let callee_id = EncodedMastNodeType::decode_u32_payload(&mast_node_info.ty);

            Ok(MastNode::new_syscall(MastNodeId(callee_id), mast_forest))
        }
        MastNodeTypeVariant::Dyn => Ok(MastNode::new_dynexec()),
        MastNodeTypeVariant::External => Ok(MastNode::new_external(mast_node_info.digest)),
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
