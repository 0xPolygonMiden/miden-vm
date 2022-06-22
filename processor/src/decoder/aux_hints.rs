use super::{get_num_groups_in_next_batch, BTreeMap, Felt, FieldElement, Vec};

// AUXILIARY TRACE HINTS
// ================================================================================================

/// Contains information which can be used to simplify construction of execution traces of
/// decoder-related auxiliary trace segment columns (used in multiset checks).
pub struct AuxTraceHints {
    /// A map where keys are clock cycles and values describes how the op group table is
    /// updated at these clock cycles.
    op_group_hints: BTreeMap<usize, OpGroupTableUpdate>,
    /// Contains a list of rows which were added to and then removed from the op group table.
    op_group_rows: Vec<OpGroupTableRow>,
}

impl AuxTraceHints {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns an empty [AuxTraceHints] struct.
    pub fn new() -> Self {
        Self {
            op_group_hints: BTreeMap::new(),
            op_group_rows: Vec::new(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns hints which describe how the op group was updated during program execution.
    pub fn op_group_table_hints(&self) -> &BTreeMap<usize, OpGroupTableUpdate> {
        &self.op_group_hints
    }

    /// Returns a list of table rows which were added to and then removed from the op group table.
    /// We don't need to specify which cycles these rows were added/removed at because this info
    /// can be inferred from the op group table hints.
    pub fn op_group_table_rows(&self) -> &[OpGroupTableRow] {
        &self.op_group_rows
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Specifies that an operation batch may have been inserted into the op group table at the
    /// specified cycle. Operation groups are inserted into the table only if the number of groups
    /// left is greater than 1.
    pub fn insert_op_batch(&mut self, clk: usize, num_groups_left: Felt) {
        // compute number of op groups in this batch
        let num_batch_groups = get_num_groups_in_next_batch(num_groups_left);
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

    /// Specifies that an entry for an operation group was removed from the op group table at the
    /// specified clock cycle.
    pub fn remove_op_group(
        &mut self,
        clk: usize,
        batch_id: Felt,
        group_pos: Felt,
        group_value: Felt,
    ) {
        self.op_group_hints
            .insert(clk, OpGroupTableUpdate::RemoveRow);
        // we record a row only when it is deleted because rows are added and deleted in the same
        // order. thus, a sequence of deleted rows is exactly the same as the sequence of added
        // rows.
        self.op_group_rows
            .push(OpGroupTableRow::new(batch_id, group_pos, group_value));
    }
}

impl Default for AuxTraceHints {
    fn default() -> Self {
        Self::new()
    }
}

// OP GROUP TABLE UPDATE HINTS
// ================================================================================================

/// Describes an update to the op group table. There could be two types of updates:
/// - Some number of rows could be added to the table. In this case, the associated value specifies
///   how many rows were added.
/// - A single row could be removed from the table.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpGroupTableUpdate {
    InsertRows(u32),
    RemoveRow,
}

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

    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 4 alpha values.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        alphas[0]
            + alphas[1].mul_base(self.batch_id)
            + alphas[2].mul_base(self.group_pos)
            + alphas[3].mul_base(self.group_value)
    }
}
