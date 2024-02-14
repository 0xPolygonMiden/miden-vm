use super::{
    AuxColumnBuilder, Felt, FieldElement, MainTrace, DYN, END, HALT, JOIN, LOOP, ONE, REPEAT, SPLIT,
};

// BLOCK HASH TABLE COLUMN BUILDER
// ================================================================================================

/// Builds the execution trace of the decoder's `p2` column which describes the state of the block
/// hash table via multiset checks.
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

// HELPER FUNCTIONS
// ================================================================================================

/// Computes the multiplicand representing the removal of a row from the block hash table.
fn get_block_hash_table_removal_multiplicand<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
    op_code_next: u8,
) -> E {
    let a = main_trace.addr(i + 1);
    let digest = main_trace.decoder_hasher_state_first_half(i);
    let is_loop_body = main_trace.is_loop_body_flag(i);
    let next_end_or_repeat =
        if op_code_next == END || op_code_next == REPEAT || op_code_next == HALT {
            E::ZERO
        } else {
            alphas[6]
        };

    alphas[0]
        + alphas[1].mul_base(a)
        + alphas[2].mul_base(digest[0])
        + alphas[3].mul_base(digest[1])
        + alphas[4].mul_base(digest[2])
        + alphas[5].mul_base(digest[3])
        + alphas[7].mul_base(is_loop_body)
        + next_end_or_repeat
}

/// Computes the multiplicand representing the inclusion of a new row representing a JOIN block
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_join<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
) -> E {
    let a_prime = main_trace.addr(i + 1);
    let state = main_trace.decoder_hasher_state(i);
    let ch1 = alphas[0]
        + alphas[1].mul_base(a_prime)
        + alphas[2].mul_base(state[0])
        + alphas[3].mul_base(state[1])
        + alphas[4].mul_base(state[2])
        + alphas[5].mul_base(state[3]);
    let ch2 = alphas[0]
        + alphas[1].mul_base(a_prime)
        + alphas[2].mul_base(state[4])
        + alphas[3].mul_base(state[5])
        + alphas[4].mul_base(state[6])
        + alphas[5].mul_base(state[7]);

    (ch1 + alphas[6]) * ch2
}

/// Computes the multiplicand representing the inclusion of a new row representing a SPLIT block
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_split<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
) -> E {
    let s0 = main_trace.stack_element(0, i);
    let a_prime = main_trace.addr(i + 1);
    let state = main_trace.decoder_hasher_state(i);

    if s0 == ONE {
        alphas[0]
            + alphas[1].mul_base(a_prime)
            + alphas[2].mul_base(state[0])
            + alphas[3].mul_base(state[1])
            + alphas[4].mul_base(state[2])
            + alphas[5].mul_base(state[3])
    } else {
        alphas[0]
            + alphas[1].mul_base(a_prime)
            + alphas[2].mul_base(state[4])
            + alphas[3].mul_base(state[5])
            + alphas[4].mul_base(state[6])
            + alphas[5].mul_base(state[7])
    }
}

/// Computes the multiplicand representing the inclusion of a new row representing a LOOP block
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_loop<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
) -> E {
    let s0 = main_trace.stack_element(0, i);

    if s0 == ONE {
        let a_prime = main_trace.addr(i + 1);
        let state = main_trace.decoder_hasher_state(i);
        alphas[0]
            + alphas[1].mul_base(a_prime)
            + alphas[2].mul_base(state[0])
            + alphas[3].mul_base(state[1])
            + alphas[4].mul_base(state[2])
            + alphas[5].mul_base(state[3])
            + alphas[7]
    } else {
        E::ONE
    }
}

/// Computes the multiplicand representing the inclusion of a new row representing a REPEAT
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_repeat<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
) -> E {
    let a_prime = main_trace.addr(i + 1);
    let state = main_trace.decoder_hasher_state_first_half(i);

    alphas[0]
        + alphas[1].mul_base(a_prime)
        + alphas[2].mul_base(state[0])
        + alphas[3].mul_base(state[1])
        + alphas[4].mul_base(state[2])
        + alphas[5].mul_base(state[3])
        + alphas[7]
}

/// Computes the multiplicand representing the inclusion of a new row representing a DYN block
/// to the block hash table.
fn get_block_hash_table_inclusion_multiplicand_dyn<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
) -> E {
    let a_prime = main_trace.addr(i + 1);
    let s0 = main_trace.stack_element(0, i);
    let s1 = main_trace.stack_element(1, i);
    let s2 = main_trace.stack_element(2, i);
    let s3 = main_trace.stack_element(3, i);

    alphas[0]
        + alphas[1].mul_base(a_prime)
        + alphas[2].mul_base(s3)
        + alphas[3].mul_base(s2)
        + alphas[4].mul_base(s1)
        + alphas[5].mul_base(s0)
}
