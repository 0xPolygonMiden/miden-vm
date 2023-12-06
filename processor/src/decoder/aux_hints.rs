use vm_core::FieldElement;
use winter_prover::matrix::ColMatrix;

use crate::system::ContextId;

use super::{
    super::trace::LookupTableRow, Felt,
    Word, ONE, ZERO,
};

// BLOCK STACK TABLE ROW
// ================================================================================================

/// Describes a single entry in the block stack table.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BlockStackTableRow {
    block_id: Felt,
    parent_id: Felt,
    is_loop: bool,
    parent_ctx: ContextId,
    parent_fn_hash: Word,
    parent_fmp: Felt,
    parent_stack_depth: u32,
    parent_next_overflow_addr: Felt,
}

impl BlockStackTableRow {

    /// Returns a new [BlockStackTableRow] instantiated with the specified parameters. This is
    /// used for test purpose only.
    #[cfg(test)]
    pub fn new_test(block_id: Felt, parent_id: Felt, is_loop: bool) -> Self {

        Self {
            block_id,
            parent_id,
            is_loop,
            parent_ctx: ContextId::root(),
            parent_fn_hash: vm_core::EMPTY_WORD,
            parent_fmp: ZERO,
            parent_stack_depth: 0,
            parent_next_overflow_addr: ZERO,
        }
    }
}

impl LookupTableRow for BlockStackTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 12 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(
        &self,
        _main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> E {
        let is_loop = if self.is_loop { ONE } else { ZERO };
        alphas[0]
            + alphas[1].mul_base(self.block_id)
            + alphas[2].mul_base(self.parent_id)
            + alphas[3].mul_base(is_loop)
            + alphas[4].mul_base(Felt::from(self.parent_ctx))
            + alphas[5].mul_base(self.parent_fmp)
            + alphas[6].mul_base(Felt::from(self.parent_stack_depth))
            + alphas[7].mul_base(self.parent_next_overflow_addr)
            + alphas[8].mul_base(self.parent_fn_hash[0])
            + alphas[9].mul_base(self.parent_fn_hash[1])
            + alphas[10].mul_base(self.parent_fn_hash[2])
            + alphas[11].mul_base(self.parent_fn_hash[3])
    }
}

// BLOCK HASH TABLE ROW
// ================================================================================================

/// Describes a single entry in the block hash table. An entry in the block hash table is a tuple
/// (parent_id, block_hash, is_first_child, is_loop_body).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BlockHashTableRow {
    parent_id: Felt,
    block_hash: Word,
    is_first_child: bool,
    is_loop_body: bool,
}

impl BlockHashTableRow {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
   
    /// Returns a new [BlockHashTableRow] instantiated with the specified parameters. This is
    /// used for test purpose only.
    pub fn new_test(
        parent_id: Felt,
        block_hash: Word,
        is_first_child: bool,
        is_loop_body: bool,
    ) -> Self {
        Self {
            parent_id,
            block_hash,
            is_first_child,
            is_loop_body,
        }
    }
}

impl LookupTableRow for BlockHashTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 8 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(
        &self,
        _main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> E {
        let is_first_child = if self.is_first_child { ONE } else { ZERO };
        let is_loop_body = if self.is_loop_body { ONE } else { ZERO };
        alphas[0]
            + alphas[1].mul_base(self.parent_id)
            + alphas[2].mul_base(self.block_hash[0])
            + alphas[3].mul_base(self.block_hash[1])
            + alphas[4].mul_base(self.block_hash[2])
            + alphas[5].mul_base(self.block_hash[3])
            + alphas[6].mul_base(is_first_child)
            + alphas[7].mul_base(is_loop_body)
    }
}

// OP GROUP TABLE ROW
// ================================================================================================

/// Describes a single entry in the op group table. An entry in the op group table is a tuple
/// (batch_id, group_pos, group_value).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OpGroupTableRow {
    batch_id: Felt,
    group_pos: Felt,
    group_value: Felt,
}

impl OpGroupTableRow {
    /// Returns a new [OpGroupTableRow] instantiated with the specified parameters.
    pub fn new(batch_id: Felt, group_pos: Felt, group_value: Felt) -> Self {
        Self {
            batch_id,
            group_pos,
            group_value,
        }
    }
}

impl LookupTableRow for OpGroupTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 4 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(
        &self,
        _main_trace: &ColMatrix<Felt>,
        alphas: &[E],
    ) -> E {
        alphas[0]
            + alphas[1].mul_base(self.batch_id)
            + alphas[2].mul_base(self.group_pos)
            + alphas[3].mul_base(self.group_value)
    }
}
