use super::{Felt, StarkField, Vec, ONE, ZERO};

use miden_air::trace::{
    decoder::{OP_BATCH_2_GROUPS, OP_BATCH_4_GROUPS, OP_BATCH_8_GROUPS},
    main_trace::MainTrace,
};

use vm_core::{crypto::hash::RpoDigest, utils::uninit_vector, FieldElement, Operation};
use winter_prover::matrix::ColMatrix;

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
    /// Builds and returns stack auxiliary trace columns. Currently this consists of a single
    /// column p1 describing states of the stack overflow table.
    pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &ColMatrix<Felt>,
        rand_elements: &[E],
        program_hash: &RpoDigest,
    ) -> Vec<Vec<E>> {
        build_aux_columns(main_trace, rand_elements, program_hash)
    }
}

// DECODER AUXILIARY TRACE COLUMNS
// ================================================================================================

/// Builds and returns decoder auxiliary trace columns p1, p2, and p3 describing states of block
/// stack, block hash, and op group tables respectively.
pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
    main_trace: &ColMatrix<Felt>,
    rand_elements: &[E],
    program_hash: &RpoDigest,
) -> Vec<Vec<E>> {
    let p1 = build_aux_col_p1(main_trace, rand_elements);
    let p2 = build_aux_col_p2(main_trace, rand_elements, program_hash);
    let p3 = build_aux_col_p3(main_trace, rand_elements);

    vec![p1, p2, p3]
}

// BLOCK STACK TABLE COLUMN
// ================================================================================================

/// Builds the execution trace of the decoder's `p1` column which describes the state of the block
/// stack table via multiset checks.
fn build_aux_col_p1<E: FieldElement<BaseField = Felt>>(
    main_trace: &ColMatrix<Felt>,
    alphas: &[E],
) -> Vec<E> {
    let main_tr = MainTrace::new(main_trace);
    let mut result_1: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result_2: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    result_1[0] = E::ONE;
    result_2[0] = E::ONE;

    let mut result_2_acc = E::ONE;
    for i in 0..main_trace.num_rows() - 1 {
        result_1[i + 1] = result_1[i] * block_stack_table_inclusions(&main_tr, alphas, i);
        result_2[i + 1] = block_stack_table_removals(&main_tr, alphas, i);
        result_2_acc *= result_2[i + 1];
    }

    let mut acc_inv = result_2_acc.inv();

    for i in (0..main_trace.num_rows()).rev() {
        result_1[i] *= acc_inv;
        acc_inv *= result_2[i];
    }
    result_1
}

/// Adds a row to the block stack table.
fn block_stack_table_inclusions<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_code_felt = main_trace.get_op_code(i);
    let op_code = op_code_felt.as_int() as u8;

    match op_code {
        JOIN | SPLIT | SPAN | DYN | LOOP | RESPAN | CALL | SYSCALL => {
            get_block_stack_table_inclusion_multiplicand(main_trace, i, alphas, op_code)
        }
        _ => E::ONE,
    }
}

/// Removes a row from the block stack table.
fn block_stack_table_removals<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_code_felt = main_trace.get_op_code(i);
    let op_code = op_code_felt.as_int() as u8;

    match op_code {
        RESPAN => get_block_stack_table_removal_multiplicand(main_trace, i, true, alphas),
        END => get_block_stack_table_removal_multiplicand(main_trace, i, false, alphas),
        _ => E::ONE,
    }
}

// BLOCK HASH TABLE COLUMN
// ================================================================================================

/// Builds the execution trace of the decoder's `p2` column which describes the state of the block
/// hash table via multiset checks.
fn build_aux_col_p2<E: FieldElement<BaseField = Felt>>(
    main_trace: &ColMatrix<Felt>,
    alphas: &[E],
    program_hash: &RpoDigest,
) -> Vec<E> {
    let main_tr = MainTrace::new(main_trace);
    let mut result_1: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result_2: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    result_1[0] = block_hash_table_initialize(program_hash, alphas);
    result_2[0] = E::ONE;

    let mut result_2_acc = E::ONE;
    for i in 0..main_trace.num_rows() - 1 {
        result_1[i + 1] = result_1[i] * block_hash_table_inclusions(&main_tr, alphas, i);
        result_2[i + 1] = block_hash_table_removals(&main_tr, alphas, i);
        result_2_acc *= result_2[i + 1];
    }

    let mut acc_inv = result_2_acc.inv();

    for i in (0..main_trace.num_rows()).rev() {
        result_1[i] *= acc_inv;
        acc_inv *= result_2[i];
    }
    result_1
}

/// Adds a row to the block hash table.
fn block_hash_table_inclusions<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
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

/// Removes a row from the block hash table.
fn block_hash_table_removals<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_code_felt = main_trace.get_op_code(i);
    let op_code = op_code_felt.as_int() as u8;

    let op_code_felt_next = main_trace.get_op_code(i + 1);
    let op_code_next = op_code_felt_next.as_int() as u8;

    match op_code {
        END => get_block_hash_table_removal_multiplicand(main_trace, i, alphas, op_code_next),
        _ => E::ONE,
    }
}

// OP GROUP TABLE COLUMN
// ================================================================================================

/// Builds the execution trace of the decoder's `p3` column which describes the state of the op
/// group table via multiset checks.
fn build_aux_col_p3<E: FieldElement<BaseField = Felt>>(
    main_trace: &ColMatrix<Felt>,
    alphas: &[E],
) -> Vec<E> {
    let main_tr = MainTrace::new(main_trace);
    let mut result_1: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result_2: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    result_1[0] = E::ONE;
    result_2[0] = E::ONE;

    let mut result_2_acc = E::ONE;
    for i in 0..main_trace.num_rows() - 1 {
        result_1[i + 1] = result_1[i] * op_group_table_inclusions(&main_tr, alphas, i);
        result_2[i + 1] = op_group_table_removals(&main_tr, alphas, i);
        result_2_acc *= result_2[i + 1];
    }

    let mut acc_inv = result_2_acc.inv();

    for i in (0..main_trace.num_rows()).rev() {
        result_1[i] *= acc_inv;
        acc_inv *= result_2[i];
    }
    result_1
}

/// Adds a row to the block hash table.
fn op_group_table_inclusions<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_code_felt = main_trace.get_op_code(i);
    let op_code = op_code_felt.as_int() as u8;

    match op_code {
        SPAN | RESPAN => get_op_group_table_inclusion_multiplicand(main_trace, i, alphas),
        _ => E::ONE,
    }
}

/// Removes a row from the block hash table.
fn op_group_table_removals<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let delete_group_flag = main_trace.delta_group_count(i) * main_trace.is_in_span(i);

    if delete_group_flag == ONE {
        get_op_group_table_removal_multiplicand(main_trace, i, alphas)
    } else {
        E::ONE
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Computes the multiplicand representing the inclusion of a new row to the block stack table.
pub fn get_block_stack_table_inclusion_multiplicand<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
    op_code: u8,
) -> E {
    let block_id = main_trace.addr(i + 1);
    let parent_id = if op_code == RESPAN {
        main_trace.decoder_hasher_state_element(1, i + 1)
    } else {
        main_trace.addr(i)
    };
    let is_loop = if op_code == LOOP {
        main_trace.stack_element(0, i)
    } else {
        ZERO
    };
    let elements = if op_code == CALL || op_code == SYSCALL {
        let parent_ctx = main_trace.ctx(i);
        let parent_fmp = main_trace.fmp(i);
        let parent_stack_depth = main_trace.stack_depth(i);
        let parent_next_overflow_addr = main_trace.parent_overflow_address(i);
        let parent_fn_hash = main_trace.decoder_hasher_state_first_half(i);
        [
            ONE,
            block_id,
            parent_id,
            is_loop,
            parent_ctx,
            parent_fmp,
            parent_stack_depth,
            parent_next_overflow_addr,
            parent_fn_hash[0],
            parent_fn_hash[1],
            parent_fn_hash[2],
            parent_fn_hash[3],
        ]
    } else {
        let mut result = [ZERO; 12];
        result[0] = ONE;
        result[1] = block_id;
        result[2] = parent_id;
        result[3] = is_loop;
        result
    };

    let mut value = E::ZERO;

    for (&alpha, &element) in alphas.iter().zip(elements.iter()) {
        value += alpha.mul_base(element);
    }
    value
}

/// Computes the multiplicand representing the removal of a row from the block stack table.
pub fn get_block_stack_table_removal_multiplicand<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    is_respan: bool,
    alphas: &[E],
) -> E {
    let block_id = main_trace.addr(i);
    let parent_id = if is_respan {
        main_trace.decoder_hasher_state_element(1, i + 1)
    } else {
        main_trace.addr(i + 1)
    };
    let is_loop = main_trace.is_loop_flag(i);

    let elements = if main_trace.is_call_flag(i) == ONE || main_trace.is_syscall_flag(i) == ONE {
        let parent_ctx = main_trace.ctx(i + 1);
        let parent_fmp = main_trace.fmp(i + 1);
        let parent_stack_depth = main_trace.stack_depth(i + 1);
        let parent_next_overflow_addr = main_trace.parent_overflow_address(i + 1);
        let parent_fn_hash = main_trace.fn_hash(i);

        [
            ONE,
            block_id,
            parent_id,
            is_loop,
            parent_ctx,
            parent_fmp,
            parent_stack_depth,
            parent_next_overflow_addr,
            parent_fn_hash[0],
            parent_fn_hash[1],
            parent_fn_hash[2],
            parent_fn_hash[0],
        ]
    } else {
        let mut result = [ZERO; 12];
        result[0] = ONE;
        result[1] = block_id;
        result[2] = parent_id;
        result[3] = is_loop;
        result
    };

    let mut value = E::ZERO;

    for (&alpha, &element) in alphas.iter().zip(elements.iter()) {
        value += alpha.mul_base(element);
    }
    value
}

/// Computes the intitialization value for the block hash table.
fn block_hash_table_initialize<E>(program_hash: &RpoDigest, alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    alphas[0]
        + alphas[2].mul_base(program_hash[0])
        + alphas[3].mul_base(program_hash[1])
        + alphas[4].mul_base(program_hash[2])
        + alphas[5].mul_base(program_hash[3])
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

/// Computes the multiplicand representing the inclusion of a new row to the op group table.
pub fn get_op_group_table_inclusion_multiplicand<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
) -> E {
    let block_id = main_trace.addr(i + 1);
    let group_count = main_trace.group_count(i);
    let op_batch_flag = main_trace.op_batch_flag(i);

    if op_batch_flag == OP_BATCH_8_GROUPS {
        let h = main_trace.decoder_hasher_state(i);
        (1..8_usize).fold(E::ONE, |acc, k| {
            acc * (alphas[0]
                + alphas[1].mul_base(block_id)
                + alphas[2].mul_base(group_count - Felt::from(k as u64))
                + alphas[3].mul_base(h[k]))
        })
    } else if op_batch_flag == OP_BATCH_4_GROUPS {
        let h = main_trace.decoder_hasher_state_first_half(i);
        (1..4_usize).fold(E::ONE, |acc, k| {
            acc * (alphas[0]
                + alphas[1].mul_base(block_id)
                + alphas[2].mul_base(group_count - Felt::from(k as u64))
                + alphas[3].mul_base(h[k]))
        })
    } else if op_batch_flag == OP_BATCH_2_GROUPS {
        let h = main_trace.decoder_hasher_state_first_half(i);
        alphas[0]
            + alphas[1].mul_base(block_id)
            + alphas[2].mul_base(group_count - ONE)
            + alphas[3].mul_base(h[1])
    } else {
        E::ONE
    }
}

/// Computes the multiplicand representing the removal of a row from the op group table.
pub fn get_op_group_table_removal_multiplicand<E: FieldElement<BaseField = Felt>>(
    main_trace: &MainTrace,
    i: usize,
    alphas: &[E],
) -> E {
    let group_count = main_trace.group_count(i);
    let block_id = main_trace.addr(i);

    let op_code = main_trace.get_op_code(i);
    let tmp = if op_code == Felt::from(PUSH) {
        main_trace.stack_element(0, i + 1)
    } else {
        let h0 = main_trace.decoder_hasher_state_first_half(i + 1)[0];

        let op_prime = main_trace.get_op_code(i + 1);
        h0.mul_small(1 << 7) + op_prime
    };
    alphas[0]
        + alphas[1].mul_base(block_id)
        + alphas[2].mul_base(group_count)
        + alphas[3].mul_base(tmp)
}
