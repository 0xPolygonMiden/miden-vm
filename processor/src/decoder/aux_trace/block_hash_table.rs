use vm_core::Word;

use super::{
    AuxColumnBuilder, Felt, FieldElement, MainTrace, DYN, END, HALT, JOIN, LOOP, ONE, REPEAT, SPLIT,
};

// BLOCK HASH TABLE COLUMN BUILDER
// ================================================================================================

/// Builds the execution trace of the decoder's `p2` column which describes the state of the block
/// hash table via multiset checks.
///
/// At any point in time, the block hash table contains the hashes of the blocks whose parents have
/// been visited, and that remain to be executed. For example, when we encounter the beginning of a
/// JOIN block, we add both children to the table, since both will be executed at some point in the
/// future. However, when we encounter the beginning of a SPLIT block, we only push the left or the
/// right child, depending on the current value on the stack (since only one child gets executed in
/// a SPLIT block). When we encounter an `END` operation, we remove the block from the table that
/// corresponds to the block that just ended. The table is initialized with the root block's hash,
/// since it doesn't have a parent, and so would never be added to the table otherwise.
#[derive(Default)]
pub struct BlockHashTableColumnBuilder {}

impl<E: FieldElement<BaseField = Felt>> AuxColumnBuilder<E> for BlockHashTableColumnBuilder {
    fn init_responses(&self, main_trace: &MainTrace, alphas: &[E]) -> E {
        let row_index = (0..main_trace.num_rows())
            .find(|row| main_trace.get_op_code(*row) == Felt::from(HALT))
            .expect("execution trace must include at least one occurrence of HALT");
        let program_hash = main_trace.decoder_hasher_state_first_half(row_index);

        // Computes the initialization value for the block hash table.
        alphas[0]
            + alphas[2].mul_base(program_hash[0])
            + alphas[3].mul_base(program_hash[1])
            + alphas[4].mul_base(program_hash[2])
            + alphas[5].mul_base(program_hash[3])
    }

    /// Removes a row from the block hash table.
    fn get_requests_at(&self, main_trace: &MainTrace, alphas: &[E], i: usize) -> E {
        let op_code_felt = main_trace.get_op_code(i);
        let op_code = op_code_felt.as_int() as u8;

        let op_code_felt_next = main_trace.get_op_code(i + 1);
        let op_code_next = op_code_felt_next.as_int() as u8;

        match op_code {
            END => get_block_hash_table_removal_multiplicand(main_trace, i, alphas, op_code_next),
            _ => E::ONE,
        }
    }

    /// Adds a row to the block hash table.
    fn get_responses_at(&self, main_trace: &MainTrace, alphas: &[E], i: usize) -> E {
        let op_code_felt = main_trace.get_op_code(i);
        let op_code = op_code_felt.as_int() as u8;

        match op_code {
            JOIN => get_block_hash_table_inclusion_multiplicand_join(main_trace, i, alphas),
            SPLIT => get_block_hash_table_inclusion_multiplicand_split(main_trace, i, alphas),
            LOOP => get_block_hash_table_inclusion_multiplicand_loop(main_trace, i, alphas),
            REPEAT => get_block_hash_table_inclusion_multiplicand_repeat(main_trace, i, alphas),
            DYN => get_block_hash_table_inclusion_multiplicand_dyn(main_trace, i, alphas),
            _ => E::ONE,
        }
    }
}

/// Builds a row for the block hash table. Since the block hash table is a virtual table, a "row" is
/// a single field element representing all the columns (which is achieved by taking a random linear
/// combination of all the columns). The columns are defined as follows:
///
/// - current_block_id: contains the ID of the current block. Note: the current block's ID is the
///   parent block's ID from the perspective of the block being added to the table.
/// - block_hash: these 4 columns hold the hash of the current block's child which will be executed
///   at some point in time in the future
/// - is_first_child: set to true if the table row being added represents the first child of the
///   current block. If the current block has only one child, set to false.
/// - is_loop_body: Set to true when the current block block is a LOOP code block (and hence, the
///   current block's child being added to the table is the body of a loop).
#[inline(always)]
fn block_hash_table_row<E>(
    current_block_id: Felt,
    child_block_hash: Word,
    is_first_child: bool,
    is_loop_body: bool,
    alphas: &[E],
) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    alphas[0]
        + alphas[1].mul_base(current_block_id)
        + alphas[2].mul_base(child_block_hash[0])
        + alphas[3].mul_base(child_block_hash[1])
        + alphas[4].mul_base(child_block_hash[2])
        + alphas[5].mul_base(child_block_hash[3])
        + alphas[6].mul_base(is_first_child.into())
        + alphas[7].mul_base(is_loop_body.into())
}

// HELPER FUNCTIONS
// ================================================================================================

/// Computes the multiplicand representing the removal of a row from the block hash table.
fn get_block_hash_table_removal_multiplicand<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    row: usize,
    alphas: &[E],
    op_code_next: u8,
) -> E {
    let current_block_id = main_trace.addr(row + 1);
    let block_hash = main_trace.decoder_hasher_state_first_half(row);
    let is_first_child = op_code_next != END && op_code_next != REPEAT && op_code_next != HALT;
    let is_loop_body = main_trace
        .is_loop_body_flag(row)
        .try_into()
        .expect("expected loop body flag to be a boolean");

    block_hash_table_row(current_block_id, block_hash, is_first_child, is_loop_body, alphas)
}

/// Computes the multiplicand representing the inclusion of a new row representing a JOIN block
/// to the block hash table.
/// TODOP: get_row_add_join() -> (E, E)
fn get_block_hash_table_inclusion_multiplicand_join<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    row: usize,
    alphas: &[E],
) -> E {
    let current_block_id = main_trace.addr(row + 1);
    let is_loop_body = false;

    let left_child_row = {
        let left_child_block_hash = main_trace.decoder_hasher_state(row)[0..4].try_into().unwrap();
        let is_first_child = true;
        block_hash_table_row(
            current_block_id,
            left_child_block_hash,
            is_first_child,
            is_loop_body,
            alphas,
        )
    };
    let right_child_row = {
        let right_child_block_hash = main_trace.decoder_hasher_state(row)[4..8].try_into().unwrap();
        let is_first_child = false;
        block_hash_table_row(
            current_block_id,
            right_child_block_hash,
            is_first_child,
            is_loop_body,
            alphas,
        )
    };

    // Note: multiplying 2 rows has the effect of adding both rows to the table. This is derived
    // from how multiset checks are defined.
    left_child_row * right_child_row
}

/// Computes the multiplicand representing the inclusion of a new row representing a SPLIT block
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_split<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    row: usize,
    alphas: &[E],
) -> E {
    let stack_top = main_trace.stack_element(0, row);
    let current_block_id = main_trace.addr(row + 1);
    // Note: only one child of a split block is executed. Hence, `is_first_child` is always false.
    let is_first_child = false;
    let is_loop_body = false;

    if stack_top == ONE {
        let left_child_block_hash = main_trace.decoder_hasher_state(row)[0..4].try_into().unwrap();
        block_hash_table_row(
            current_block_id,
            left_child_block_hash,
            is_first_child,
            is_loop_body,
            alphas,
        )
    } else {
        let right_child_block_hash = main_trace.decoder_hasher_state(row)[4..8].try_into().unwrap();
        block_hash_table_row(
            current_block_id,
            right_child_block_hash,
            is_first_child,
            is_loop_body,
            alphas,
        )
    }
}

/// Computes the multiplicand representing the inclusion of a new row representing a LOOP block
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_loop<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    row: usize,
    alphas: &[E],
) -> E {
    let stack_top = main_trace.stack_element(0, row);

    if stack_top == ONE {
        let current_block_id = main_trace.addr(row + 1);
        // TODOP: Add method to main_trace instead
        let child_block_hash = main_trace.decoder_hasher_state(row)[0..4].try_into().unwrap();
        let is_first_child = false;
        let is_loop_body = true;

        block_hash_table_row(
            current_block_id,
            child_block_hash,
            is_first_child,
            is_loop_body,
            alphas,
        )
    } else {
        E::ONE
    }
}

/// Computes the multiplicand representing the inclusion of a new row representing a REPEAT
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_repeat<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    row: usize,
    alphas: &[E],
) -> E {
    let current_block_id = main_trace.addr(row + 1);
    let child_block_hash = main_trace.decoder_hasher_state_first_half(row);
    let is_first_child = false;
    let is_loop_body = true;

    block_hash_table_row(current_block_id, child_block_hash, is_first_child, is_loop_body, alphas)
}

/// Computes the multiplicand representing the inclusion of a new row representing a DYN block
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_dyn<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    row: usize,
    alphas: &[E],
) -> E {
    let current_block_id = main_trace.addr(row + 1);
    let is_first_child = false;
    let is_loop_body = false;
    let child_block_hash = {
        // Note: the child block hash is found on the stack, and hence in reverse order.
        let s0 = main_trace.stack_element(0, row);
        let s1 = main_trace.stack_element(1, row);
        let s2 = main_trace.stack_element(2, row);
        let s3 = main_trace.stack_element(3, row);

        [s3, s2, s1, s0]
    };

    block_hash_table_row(current_block_id, child_block_hash, is_first_child, is_loop_body, alphas)
}
