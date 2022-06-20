use super::{BTreeMap, Felt, FieldElement, StarkField, Vec, OP_BATCH_SIZE};

// AUXILIARY TRACE HINTS
// ================================================================================================

pub struct AuxTraceHints {
    op_group_hints: BTreeMap<usize, OpGroupTableUpdate>,
    op_group_rows: Vec<OpGroupTableRow>,
}

impl AuxTraceHints {
    pub fn new() -> Self {
        Self {
            op_group_hints: BTreeMap::new(),
            op_group_rows: Vec::new(),
        }
    }

    pub fn op_group_table_hints(&self) -> &BTreeMap<usize, OpGroupTableUpdate> {
        &self.op_group_hints
    }

    pub fn op_group_table_rows(&self) -> &[OpGroupTableRow] {
        &self.op_group_rows
    }

    /// TODO: add docs
    pub fn insert_op_batch(&mut self, clk: usize, num_groups_left: Felt) {
        // compute number of op groups in a batch; to do this we take the min of number of groups
        // left and max batch size. thus, if the number of groups left is > 8, the number of
        // groups will be 8; otherwise, it will be equal to the number of groups left to process.
        let num_batch_groups = core::cmp::min(num_groups_left.as_int() as usize, OP_BATCH_SIZE);
        debug_assert!(num_batch_groups > 0, "op batch is empty");

        // the first op group in a batch is not added to the op_group table, so, we subtract 1 here
        let num_inserted_groups = num_batch_groups - 1;

        // if at least one group was inserted, mark the current clock cycle with the number of op
        // groups added to the op group table
        if num_inserted_groups > 0 {
            let update = OpGroupTableUpdate::InsertRows(num_inserted_groups as u32);
            self.op_group_hints.insert(clk, update);
        }
    }

    pub fn remove_op_group(
        &mut self,
        clk: usize,
        batch_id: Felt,
        group_pos: Felt,
        group_value: Felt,
    ) {
        self.op_group_hints
            .insert(clk, OpGroupTableUpdate::RemoveRow);
        self.op_group_rows.push(OpGroupTableRow {
            batch_id,
            group_pos,
            group_value,
        });
    }
}

impl Default for AuxTraceHints {
    fn default() -> Self {
        Self::new()
    }
}

// OP GROUP TABLE UPDATE HINTS
// ================================================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpGroupTableUpdate {
    InsertRows(u32),
    RemoveRow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OpGroupTableRow {
    batch_id: Felt,
    group_pos: Felt,
    group_value: Felt,
}

impl OpGroupTableRow {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        alphas[0]
            + alphas[1].mul_base(self.batch_id)
            + alphas[2].mul_base(self.group_pos)
            + alphas[3].mul_base(self.group_value)
    }

    #[cfg(test)]
    pub fn new_test(batch_id: Felt, group_pos: Felt, group_value: Felt) -> Self {
        Self {
            batch_id,
            group_pos,
            group_value,
        }
    }
}
