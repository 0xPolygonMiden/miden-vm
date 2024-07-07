use alloc::{collections::BTreeMap, vec::Vec};
use miden_crypto::hash::rpo::{Rpo256, RpoDigest};
use winter_utils::ByteWriter;

use crate::{
    mast::{BasicBlockNode, OperationOrDecorator},
    AdviceInjector, DebugOptions, Decorator, Operation, SignatureKind,
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
    /// Returns the offset in the serialized [`crate::mast::MastForest`] data field that the next
    /// [`super::MastNodeInfo`] representing a [`BasicBlockNode`] will take.
    pub fn next_data_offset(&self) -> DataOffset {
        self.data
            .len()
            .try_into()
            .expect("MAST forest data segment larger than 2^32 bytes")
    }
}

/// Mutators
impl BasicBlockDataBuilder {
    /// Encodes a [`BasicBlockNode`] into the serialized [`crate::mast::MastForest`] data field.
    pub fn encode_basic_block(&mut self, basic_block: &BasicBlockNode) {
        // 2nd part of `mast_node_to_info()` (inside the match)
        for op_or_decorator in basic_block.iter() {
            match op_or_decorator {
                OperationOrDecorator::Operation(operation) => self.encode_operation(operation),
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
    fn encode_operation(&mut self, operation: &Operation) {
        self.data.push(operation.op_code());

        // For operations that have extra data, encode it in `data`.
        match operation {
            Operation::Assert(err_code) | Operation::MpVerify(err_code) => {
                self.data.extend_from_slice(&err_code.to_le_bytes())
            }
            Operation::U32assert2(value) | Operation::Push(value) => {
                self.data.extend_from_slice(&value.as_int().to_le_bytes())
            }
            // Note: we explicitly write out all the operations so that whenever we make a
            // modification to the `Operation` enum, we get a compile error here. This
            // should help us remember to properly encode/decode each operation variant.
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

    fn encode_decorator(&mut self, decorator: &Decorator) {
        // Set the first byte to the decorator discriminant.
        //
        // Note: the most significant bit is set to 1 (to differentiate decorators from operations).
        {
            let decorator_variant: EncodedDecoratorVariant = decorator.into();
            self.data.push(decorator_variant.discriminant() | 0b1000_0000);
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
    str_to_index: BTreeMap<RpoDigest, StringIndex>,
    strings_data: Vec<u8>,
}

impl StringTableBuilder {
    pub fn add_string(&mut self, string: &str) -> StringIndex {
        if let Some(str_idx) = self.str_to_index.get(&Rpo256::hash(string.as_bytes())) {
            // return already interned string
            *str_idx
        } else {
            // add new string to table
            // NOTE: these string refs' offset will need to be shifted again in `into_buffer()`
            let str_ref = StringRef {
                offset: self
                    .strings_data
                    .len()
                    .try_into()
                    .expect("strings table larger than 2^32 bytes"),
                len: string.len().try_into().expect("string larger than 2^32 bytes"),
            };
            let str_idx = self.table.len();

            self.strings_data.extend(string.as_bytes());
            self.table.push(str_ref);
            self.str_to_index.insert(Rpo256::hash(string.as_bytes()), str_idx);

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
                len: str_ref.len,
            })
            .collect()
    }
}
