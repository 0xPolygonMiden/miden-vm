use super::{Felt, ONE, ZERO};
use crate::trace::AuxColumnBuilder;
use alloc::vec::Vec;
use miden_air::trace::main_trace::MainTrace;
use vm_core::{FieldElement, Operation};

mod block_hash_table;
use block_hash_table::BlockHashTableColumnBuilder;

mod block_stack_table;
use block_stack_table::BlockStackColumnBuilder;

mod op_group_table;
use op_group_table::OpGroupTableColumnBuilder;

// CONSTANTS
// ================================================================================================

const JOIN: u8 = Operation::Join.op_code();
const SPLIT: u8 = Operation::Split.op_code();
const LOOP: u8 = Operation::Loop.op_code();
const REPEAT: u8 = Operation::Repeat.op_code();
const DYN: u8 = Operation::Dyn.op_code();
const CALL: u8 = Operation::Call.op_code();
const SYSCALL: u8 = Operation::SysCall.op_code();
const SPAN: u8 = Operation::Span.op_code();
const RESPAN: u8 = Operation::Respan.op_code();
const PUSH: u8 = Operation::Push(ZERO).op_code();
const END: u8 = Operation::End.op_code();
const HALT: u8 = Operation::Halt.op_code();

// AUXILIARY TRACE BUILDER
// ================================================================================================

/// Constructs the execution traces of stack-related auxiliary trace segment columns
/// (used in multiset checks).
#[derive(Default, Clone, Copy)]
pub struct AuxTraceBuilder {}

impl AuxTraceBuilder {
    /// Builds and returns decoder auxiliary trace columns p1, p2, and p3 describing states of block
    /// stack, block hash, and op group tables respectively.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &MainTrace,
        rand_elements: &[E],
    ) -> Vec<Vec<E>> {
        let block_stack_column_builder = BlockStackColumnBuilder::default();
        let block_hash_column_builder = BlockHashTableColumnBuilder::default();
        let op_group_table_column_builder = OpGroupTableColumnBuilder::default();

        let p1 = block_stack_column_builder.build_aux_column(main_trace, rand_elements);
        let p2 = block_hash_column_builder.build_aux_column(main_trace, rand_elements);
        let p3 = op_group_table_column_builder.build_aux_column(main_trace, rand_elements);

        vec![p1, p2, p3]
    }
}
