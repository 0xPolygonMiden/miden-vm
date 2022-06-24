use super::{
    super::trace::LookupTableRow, get_num_groups_in_next_batch, BTreeMap, BlockInfo, Felt,
    FieldElement, StarkField, Vec, Word, ONE, ZERO,
};

// AUXILIARY TRACE HINTS
// ================================================================================================

/// Contains information which can be used to simplify construction of execution traces of
/// decoder-related auxiliary trace segment columns (used in multiset checks).
pub struct AuxTraceHints {
    /// A list of updates made to the block stack and block hash tables. Each entry contains a
    /// clock cycle at which the update was made, as well as the description of the update.
    block_exec_hints: Vec<(usize, BlockTableUpdate)>,
    /// Contains a list of rows which were added and then removed from the block stack table. The
    /// rows are sorted by `block_id` in ascending order.
    block_stack_rows: Vec<BlockStackTableRow>,
    /// Contains a list of rows which were added and then removed form the block hash table. The
    /// rows are sorted first by `parent_id` and then by `is_first_child` with the entry where
    /// `is_first_child` = true coming first.
    block_hash_rows: Vec<BlockHashTableRow>,
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
            block_exec_hints: Vec::new(),
            block_stack_rows: Vec::new(),
            block_hash_rows: Vec::new(),
            op_group_hints: BTreeMap::new(),
            op_group_rows: Vec::new(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn block_exec_hints(&self) -> &[(usize, BlockTableUpdate)] {
        &self.block_exec_hints
    }

    /// TODO: add docs
    pub fn block_stack_table_rows(&self) -> &[BlockStackTableRow] {
        &self.block_stack_rows
    }

    /// TODO: add docs
    pub fn block_hash_table_rows(&self) -> &[BlockHashTableRow] {
        &self.block_hash_rows
    }

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

    /// TODO: add comments
    pub fn get_block_stack_row_idx(&self, block_id: Felt) -> Option<usize> {
        let block_id = block_id.as_int();
        self.block_stack_rows
            .binary_search_by_key(&block_id, |row| row.block_id.as_int())
            .ok()
    }

    /// TODO: add comments
    pub fn get_block_hash_row_idx(&self, parent_id: Felt, is_first_child: bool) -> Option<usize> {
        let parent_id = parent_id.as_int();
        match self
            .block_hash_rows
            .binary_search_by_key(&parent_id, |row| row.parent_id.as_int())
        {
            Ok(idx) => {
                // check if the row for the found index is the right one; we need to do this
                // because binary search may return an index for either of the two entries for
                // the specified parent_id
                if self.block_hash_rows[idx].is_first_child == is_first_child {
                    Some(idx)
                } else if is_first_child {
                    // if we got here, it means that is_first_child for the row at the found index
                    // is false. thus, the row with is_first_child = true should be right before it
                    let row = &self.block_hash_rows[idx - 1];
                    debug_assert_eq!(row.parent_id.as_int(), parent_id);
                    debug_assert_eq!(row.is_first_child, is_first_child);
                    Some(idx - 1)
                } else {
                    // similarly, if we got here, is_first_child for the row at the found index
                    // must be true. thus, the row with is_first_child = false should be right
                    // after it
                    let row = &self.block_hash_rows[idx + 1];
                    debug_assert_eq!(row.parent_id.as_int(), parent_id);
                    debug_assert_eq!(row.is_first_child, is_first_child);
                    Some(idx + 1)
                }
            }
            Err(_) => None,
        }
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Specifies that a new code block started executing at the specified clock cycle. This also
    /// records the relevant rows for both, block stack and block hash tables.
    pub fn start_block(
        &mut self,
        clk: usize,
        block_info: &BlockInfo,
        child1_hash: Option<Word>,
        child2_hash: Option<Word>,
    ) {
        // insert the hint with the relevant update
        let hint = BlockTableUpdate::BlockStarted(block_info.block_type.num_children());
        self.block_exec_hints.push((clk, hint));

        // create a row which would be inserted into the block stack table
        let bst_row = BlockStackTableRow::new(block_info);
        self.block_stack_rows.push(bst_row);

        // crete rows for the block hash table. this may result in creation of 0, 1, or 2 rows:
        // - no rows are created for SPAN blocks (both child hashes are None).
        // - one row is created with is_first_child=false for SPLIT and LOOP blocks.
        // - two rows are created for JOIN blocks with first row having is_first_child=true, and
        //   the second row having is_first_child=false
        if let Some(child1_hash) = child1_hash {
            let is_first_child = child2_hash.is_some();
            let bsh_row1 = BlockHashTableRow::from_parent(block_info, child1_hash, is_first_child);
            self.block_hash_rows.push(bsh_row1);

            if let Some(child2_hash) = child2_hash {
                let bsh_row2 = BlockHashTableRow::from_parent(block_info, child2_hash, false);
                self.block_hash_rows.push(bsh_row2);
            }
        }
    }

    /// Specifies that a code block execution was completed at the specified clock cycle. We also
    /// need to specify whether the block was the first child of a JOIN block so that we can find
    /// correct block hash table row.
    pub fn end_block(&mut self, clk: usize, is_first_child: bool) {
        self.block_exec_hints
            .push((clk, BlockTableUpdate::BlockEnded(is_first_child)));
    }

    /// TODO: add comments
    pub fn repeat_loop_body(&mut self, clk: usize) {
        self.block_exec_hints
            .push((clk, BlockTableUpdate::LoopRepeated));
    }

    /// TODO: add comments
    pub fn continue_span(&mut self, clk: usize, block_info: &BlockInfo) {
        let row = BlockStackTableRow::new(block_info);
        self.block_stack_rows.push(row);
        self.block_exec_hints
            .push((clk, BlockTableUpdate::SpanExtended))
    }

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

    /// TODO: add docs
    pub fn set_program_hash(&mut self, program_hash: Word) {
        let first_row = BlockHashTableRow::from_program_hash(program_hash);
        self.block_hash_rows.insert(0, first_row);
    }
}

impl Default for AuxTraceHints {
    fn default() -> Self {
        Self::new()
    }
}

// UPDATE HINTS
// ================================================================================================

/// Describes updates to both, block stack and block hash tables as follows:
/// - `BlockStarted` and `BlockEnded` are relevant for both tables.
/// - `SpanExtended` is relevant only for the block stack table.
/// - `LoopRepeated` is relevant only for the block hash table.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlockTableUpdate {
    BlockStarted(u32), // inner value contains the number of children for the block: 0, 1, or 2.
    SpanExtended,
    LoopRepeated,
    BlockEnded(bool), // true indicates that the block was the first child of a JOIN block
}

/// Describes an update to the op group table. There could be two types of updates:
/// - Some number of rows could be added to the table. In this case, the associated value specifies
///   how many rows were added.
/// - A single row could be removed from the table.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpGroupTableUpdate {
    InsertRows(u32),
    RemoveRow,
}

// BLOCK STACK TABLE ROW
// ================================================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BlockStackTableRow {
    block_id: Felt,
    parent_id: Felt,
    is_loop: bool,
}

impl BlockStackTableRow {
    /// Returns a new [BlockStackTableRow] instantiated from the specified block info.
    pub fn new(block_info: &BlockInfo) -> Self {
        Self {
            block_id: block_info.addr,
            parent_id: block_info.parent_addr,
            is_loop: block_info.is_entered_loop() == ONE,
        }
    }

    /// Returns a new [BlockStackTableRow] instantiated with the specified parameters. This is
    /// used for test purpose only.
    #[cfg(test)]
    pub fn new_test(block_id: Felt, parent_id: Felt, is_loop: bool) -> Self {
        Self {
            block_id,
            parent_id,
            is_loop,
        }
    }
}

impl LookupTableRow for BlockStackTableRow {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 4 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        let is_loop = if self.is_loop { ONE } else { ZERO };
        alphas[0]
            + alphas[1].mul_base(self.block_id)
            + alphas[2].mul_base(self.parent_id)
            + alphas[3].mul_base(is_loop)
    }
}

// BLOCK HASH TABLE ROW
// ================================================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BlockHashTableRow {
    parent_id: Felt,
    block_hash: Word,
    is_first_child: bool,
    is_loop_body: bool,
}

impl BlockHashTableRow {
    /// Returns a new [BlockHashTableRow] instantiated with the specified parameters.
    pub fn from_parent(parent_info: &BlockInfo, block_hash: Word, is_first_child: bool) -> Self {
        Self {
            parent_id: parent_info.addr,
            block_hash,
            is_first_child,
            is_loop_body: parent_info.is_entered_loop() == ONE,
        }
    }

    /// TODO: add comments
    pub fn from_program_hash(program_hash: Word) -> Self {
        Self {
            parent_id: ZERO,
            block_hash: program_hash,
            is_first_child: false,
            is_loop_body: false,
        }
    }

    pub fn is_first_child(&self) -> bool {
        self.is_first_child
    }

    /// Returns a new [BlockHashTableRow] instantiated with the specified parameters. This is
    /// used for test purpose only.
    #[cfg(test)]
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
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
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
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        alphas[0]
            + alphas[1].mul_base(self.batch_id)
            + alphas[2].mul_base(self.group_pos)
            + alphas[3].mul_base(self.group_value)
    }
}
