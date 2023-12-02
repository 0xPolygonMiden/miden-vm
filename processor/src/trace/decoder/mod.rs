
use super::{
    super::decoder::AuxTraceHints, ColMatrix, Felt, FieldElement, Vec, DECODER_TRACE_OFFSET,
};

use miden_air::trace::{
    chiplets::hasher::DIGEST_LEN,
    decoder::{
        GROUP_COUNT_COL_IDX, HASHER_STATE_OFFSET, IN_SPAN_COL_IDX, IS_CALL_FLAG_COL_IDX,
        IS_LOOP_BODY_FLAG_COL_IDX, IS_LOOP_FLAG_COL_IDX, IS_SYSCALL_FLAG_COL_IDX,
        NUM_OP_BATCH_FLAGS, OP_BATCH_2_GROUPS, OP_BATCH_4_GROUPS, OP_BATCH_8_GROUPS,
        OP_BATCH_FLAGS_OFFSET,
    },
    stack::{B0_COL_IDX, B1_COL_IDX},
    CTX_COL_IDX, FMP_COL_IDX, FN_HASH_OFFSET, STACK_TRACE_OFFSET,
};

use vm_core::{
    chiplets::hasher::RATE_LEN, crypto::hash::RpoDigest, utils::uninit_vector, Operation,
    StarkField, ONE, ZERO,
};
use winter_prover::math::batch_inversion;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const ADDR_COL_IDX: usize = DECODER_TRACE_OFFSET + miden_air::trace::decoder::ADDR_COL_IDX;
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

// DECODER AUXILIARY TRACE COLUMNS
// ================================================================================================

/// Builds and returns decoder auxiliary trace columns p1, p2, and p3 describing states of block
/// stack, block hash, and op group tables respectively.
pub fn build_aux_columns<E: FieldElement<BaseField = Felt>>(
    main_trace: &ColMatrix<Felt>,
    aux_trace_hints: &AuxTraceHints,
    rand_elements: &[E],
    program_hash: &RpoDigest,
) -> Vec<Vec<E>> {
    let p1 = build_aux_col_p1(main_trace, aux_trace_hints, rand_elements);
    let p2 = build_aux_col_p2(main_trace, aux_trace_hints, rand_elements, program_hash);
    let p3 = build_aux_col_p3(main_trace, rand_elements);

    vec![p1, p2, p3]
}

// BLOCK STACK TABLE COLUMN
// ================================================================================================

/// Builds the execution trace of the decoder's `p1` column which describes the state of the block
/// stack table via multiset checks.
fn build_aux_col_p1<E: FieldElement<BaseField = Felt>>(
    main_trace: &ColMatrix<Felt>,
    _aux_trace_hints: &AuxTraceHints,
    alphas: &[E],
) -> Vec<E> {
    let mut result_1: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result_2: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };

    result_1[0] = E::ONE;
    result_2[0] = E::ONE;
    result[0] = E::ONE;

    let main_tr = MainTrace::new(main_trace);
    for i in 0..main_trace.num_rows() - 1 {
        result_1[i] = block_stack_table_inclusions(&main_tr, alphas, i);
        result_2[i] = block_stack_table_removals(&main_tr, alphas, i);
    }

    let result_2 = batch_inversion(&result_2);

    for i in 0..main_trace.num_rows() - 1 {
        result[i + 1] = result[i] * result_1[i] * result_2[i];
    }

    result
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
            main_trace.get_block_stack_table_inclusion_multiplicand(i, alphas, op_code)
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
        RESPAN => main_trace.get_block_stack_table_removal_multiplicand(i, true, alphas),
        END => main_trace.get_block_stack_table_removal_multiplicand(i, false, alphas),
        _ => E::ONE,
    }
}

// BLOCK HASH TABLE COLUMN
// ================================================================================================

/// Builds the execution trace of the decoder's `p2` column which describes the state of the block
/// hash table via multiset checks.
fn build_aux_col_p2<E: FieldElement<BaseField = Felt>>(
    main_trace: &ColMatrix<Felt>,
    _aux_trace_hints: &AuxTraceHints,
    alphas: &[E],
    program_hash: &RpoDigest,
) -> Vec<E> {
    let mut result_1: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result_2: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };

    let main_tr = MainTrace::new(main_trace);

    result_1[0] = E::ONE;
    result_2[0] = E::ONE;
    result[0] = main_tr.block_hash_table_initialize(program_hash, alphas);

    for i in 0..main_trace.num_rows() - 1 {
        result_1[i] = block_hash_table_inclusions(&main_tr, alphas, i);
        result_2[i] = block_hash_table_removals(&main_tr, alphas, i);
    }

    let result_2 = batch_inversion(&result_2);

    for i in 0..main_trace.num_rows() - 1 {
        result[i + 1] = result[i] * result_1[i] * result_2[i];
    }

    result
}

/// Adds a row to the block hash table.
fn block_hash_table_inclusions<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_code_felt = main_trace.get_op_code(i);
    let op_code = op_code_felt.as_int() as u8;

    match op_code {
        JOIN => main_trace.get_block_hash_table_inclusion_multiplicand_join(i, alphas),
        SPLIT => main_trace.get_block_hash_table_inclusion_multiplicand_split(i, alphas),
        LOOP => main_trace.get_block_hash_table_inclusion_multiplicand_loop(i, alphas),
        REPEAT => main_trace.get_block_hash_table_inclusion_multiplicand_repeat(i, alphas),
        DYN => main_trace.get_block_hash_table_inclusion_multiplicand_dyn(i, alphas),
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
        END => main_trace.get_block_hash_table_removal_multiplicand(i, alphas, op_code_next),
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
    let mut result_1: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result_2: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };
    let mut result: Vec<E> = unsafe { uninit_vector(main_trace.num_rows()) };

    let main_tr = MainTrace::new(main_trace);

    result_1[0] = E::ONE;
    result_2[0] = E::ONE;
    result[0] = E::ONE;

    for i in 0..main_trace.num_rows() - 1 {
        result_1[i] = op_group_table_inclusions(&main_tr, alphas, i);
        result_2[i] = op_group_table_removals(&main_tr, alphas, i);
    }

    let result_2 = batch_inversion(&result_2);

    for i in 0..main_trace.num_rows() - 1 {
        result[i + 1] = result[i] * result_1[i] * result_2[i];
    }

    result
}

/// Adds a row to the block hash table.
fn op_group_table_inclusions<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let op_code_felt = main_trace.get_op_code(i);
    let op_code = op_code_felt.as_int() as u8;

    match op_code {
        SPAN | RESPAN => main_trace.get_op_group_table_inclusion_multiplicand(i, alphas),
        _ => E::ONE,
    }
}

/// Removes a row from the block hash table.
fn op_group_table_removals<E>(main_trace: &MainTrace, alphas: &[E], i: usize) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let delete_group_flag = main_trace.get_delta_group_count(i) * main_trace.get_is_in_span(i);

    if delete_group_flag == ONE {
        main_trace.get_op_group_table_removal_multiplicand(i, alphas)
    } else {
        E::ONE
    }
}

// HELPER FUNCTIONS
// ================================================================================================

struct MainTrace<'a> {
    columns: &'a ColMatrix<Felt>,
}

impl<'a> MainTrace<'a> {
    pub fn new(main_trace: &'a ColMatrix<Felt>) -> Self {
        Self {
            columns: main_trace,
        }
    }

    /// Constructs the i-th op code value from its individual bits.
    pub fn get_op_code(&self, i: usize) -> Felt {
        let col_b0 = self.columns.get_column(DECODER_TRACE_OFFSET + 1);
        let col_b1 = self.columns.get_column(DECODER_TRACE_OFFSET + 2);
        let col_b2 = self.columns.get_column(DECODER_TRACE_OFFSET + 3);
        let col_b3 = self.columns.get_column(DECODER_TRACE_OFFSET + 4);
        let col_b4 = self.columns.get_column(DECODER_TRACE_OFFSET + 5);
        let col_b5 = self.columns.get_column(DECODER_TRACE_OFFSET + 6);
        let col_b6 = self.columns.get_column(DECODER_TRACE_OFFSET + 7);
        let [b0, b1, b2, b3, b4, b5, b6] =
            [col_b0[i], col_b1[i], col_b2[i], col_b3[i], col_b4[i], col_b5[i], col_b6[i]];
        b0 + b1.mul_small(2)
            + b2.mul_small(4)
            + b3.mul_small(8)
            + b4.mul_small(16)
            + b5.mul_small(32)
            + b6.mul_small(64)
    }

    /// Returns the value in the block address column at the row i.
    fn get_block_addr(&self, i: usize) -> Felt {
        self.columns.get(ADDR_COL_IDX, i)
    }

    /// Returns the hasher state at row i.
    pub fn get_decoder_hasher_state(&self, i: usize) -> [Felt; RATE_LEN] {
        let mut state = [ZERO; RATE_LEN];
        for (col, s) in state.iter_mut().enumerate() {
            *s = self.columns.get_column(DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + col)[i];
        }
        state
    }

    /// Returns the first half of the hasher state at row i.
    pub fn get_decoder_hasher_state_first_half(&self, i: usize) -> [Felt; DIGEST_LEN] {
        let mut state = [ZERO; DIGEST_LEN];
        for (col, s) in state.iter_mut().enumerate() {
            *s = self.columns.get_column(DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + col)[i];
        }
        state
    }

    /// Returns a specific element from the hasher state at row i.
    pub fn get_decoder_hasher_state_element(&self, element: usize, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + HASHER_STATE_OFFSET + element)[i + 1]
    }

    /// Returns the current function hash (i.e., root) at row i.
    pub fn get_fn_hash(&self, i: usize) -> [Felt; DIGEST_LEN] {
        let mut state = [ZERO; DIGEST_LEN];
        for (col, s) in state.iter_mut().enumerate() {
            *s = self.columns.get_column(FN_HASH_OFFSET + col)[i];
        }
        state
    }

    /// Returns the `is_loop_body` flag at row i.
    pub fn is_loop_body_flag(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + IS_LOOP_BODY_FLAG_COL_IDX)[i]
    }

    /// Returns the `is_loop` flag at row i.
    pub fn is_loop_flag(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + IS_LOOP_FLAG_COL_IDX)[i]
    }

    /// Returns the `is_call` flag at row i.
    pub fn is_call_flag(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + IS_CALL_FLAG_COL_IDX)[i]
    }

    /// Returns the `is_syscall` flag at row i.
    pub fn is_syscall_flag(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + IS_SYSCALL_FLAG_COL_IDX)[i]
    }

    /// Returns the operation batch flags at row i. This indicates the number of op groups in
    /// the current batch that is being processed.
    pub fn get_op_batch_flag(&self, i: usize) -> [Felt; NUM_OP_BATCH_FLAGS] {
        [
            self.columns.get(DECODER_TRACE_OFFSET + OP_BATCH_FLAGS_OFFSET, i),
            self.columns.get(DECODER_TRACE_OFFSET + OP_BATCH_FLAGS_OFFSET + 1, i),
            self.columns.get(DECODER_TRACE_OFFSET + OP_BATCH_FLAGS_OFFSET + 2, i),
        ]
    }

    /// Returns the operation group count. This indicates the number of operation that remain
    /// to be executed in the current span block.
    pub fn get_group_count(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + GROUP_COUNT_COL_IDX)[i]
    }

    /// Returns the `in_span` flag at row i.
    pub fn get_is_in_span(&self, i: usize) -> Felt {
        self.columns.get_column(DECODER_TRACE_OFFSET + IN_SPAN_COL_IDX)[i]
    }

    /// Returns the delta between the current and next group counts.
    pub fn get_delta_group_count(&self, i: usize) -> Felt {
        self.get_group_count(i) - self.get_group_count(i + 1)
    }

    /// Returns the value of the context column at row i.
    pub fn get_ctx(&self, i: usize) -> Felt {
        self.columns.get_column(CTX_COL_IDX)[i]
    }

    /// Returns the value of the fmp column at row i.
    pub fn get_fmp(&self, i: usize) -> Felt {
        self.columns.get_column(FMP_COL_IDX)[i]
    }

    /// Returns the value of the stack depth column at row i.
    pub fn get_stack_depth(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + B0_COL_IDX)[i]
    }

    /// Returns the address of the top element in the stack overflow table at row i.
    pub fn get_parent_next_overflow_address(&self, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + B1_COL_IDX)[i]
    }

    /// Returns the element at row i in a given stack trace column.
    pub fn get_stack_element(&self, column: usize, i: usize) -> Felt {
        self.columns.get_column(STACK_TRACE_OFFSET + column)[i]
    }

    /// Computes the multiplicand representing the inclusion of a new row to the block stack table.
    pub fn get_block_stack_table_inclusion_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        i: usize,
        alphas: &[E],
        op_code: u8,
    ) -> E {
        let block_id = self.get_block_addr(i + 1);
        let parent_id = if op_code == RESPAN {
            self.get_decoder_hasher_state_element(1, i + 1)
        } else {
            self.get_block_addr(i)
        };
        let is_loop = if op_code == LOOP {
            self.get_stack_element(0, i)
        } else {
            ZERO
        };
        let elements = if op_code == CALL || op_code == SYSCALL {
            let parent_ctx = self.get_ctx(i);
            let parent_fmp = self.get_fmp(i);
            let parent_stack_depth = self.get_stack_depth(i);
            let parent_next_overflow_addr = self.get_parent_next_overflow_address(i);
            let parent_fn_hash = self.get_decoder_hasher_state_first_half(i);
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
        &self,
        i: usize,
        is_respan: bool,
        alphas: &[E],
    ) -> E {
        let block_id = self.get_block_addr(i);
        let parent_id = if is_respan {
            self.get_decoder_hasher_state_element(1, i + 1)
        } else {
            self.get_block_addr(i + 1)
        };
        let is_loop = self.is_loop_flag(i);

        let elements = if self.is_call_flag(i) == ONE || self.is_syscall_flag(i) == ONE {
            let parent_ctx = self.get_ctx(i + 1);
            let parent_fmp = self.get_fmp(i + 1);
            let parent_stack_depth = self.get_stack_depth(i + 1);
            let parent_next_overflow_addr = self.get_parent_next_overflow_address(i + 1);
            let parent_fn_hash = self.get_fn_hash(i);

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
    fn block_hash_table_initialize<E>(&self, program_hash: &RpoDigest, alphas: &[E]) -> E
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
        &self,
        i: usize,
        alphas: &[E],
    ) -> E {
        let a_prime = self.get_block_addr(i + 1);
        let state = self.get_decoder_hasher_state(i);
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
        &self,
        i: usize,
        alphas: &[E],
    ) -> E {
        let s0 = self.get_stack_element(0, i);
        let a_prime = self.get_block_addr(i + 1);
        let state = self.get_decoder_hasher_state(i);

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
        &self,
        i: usize,
        alphas: &[E],
    ) -> E {
        let s0 = self.get_stack_element(0, i);

        if s0 == ONE {
            let a_prime = self.get_block_addr(i + 1);
            let state = self.get_decoder_hasher_state(i);
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
        &self,
        i: usize,
        alphas: &[E],
    ) -> E {
        let a_prime = self.get_block_addr(i + 1);
        let state = self.get_decoder_hasher_state_first_half(i);

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
        &self,
        i: usize,
        alphas: &[E],
    ) -> E {
        let a_prime = self.get_block_addr(i + 1);
        let s0 = self.get_stack_element(0, i);
        let s1 = self.get_stack_element(1, i);
        let s2 = self.get_stack_element(2, i);
        let s3 = self.get_stack_element(3, i);

        alphas[0]
            + alphas[1].mul_base(a_prime)
            + alphas[2].mul_base(s3)
            + alphas[3].mul_base(s2)
            + alphas[4].mul_base(s1)
            + alphas[5].mul_base(s0)
    }

    /// Computes the multiplicand representing the removal of a row from the block hash table.
    fn get_block_hash_table_removal_multiplicand<E: FieldElement<BaseField = Felt>>(
        &self,
        i: usize,
        alphas: &[E],
        op_code_next: u8,
    ) -> E {
        let a = self.get_block_addr(i + 1);
        let digest = self.get_decoder_hasher_state_first_half(i);
        let is_loop_body = self.is_loop_body_flag(i);
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
        &self,
        i: usize,
        alphas: &[E],
    ) -> E {
        let block_id = self.get_block_addr(i + 1);
        let group_count = self.get_group_count(i);
        let op_batch_flag = self.get_op_batch_flag(i);

        if op_batch_flag == OP_BATCH_8_GROUPS {
            let h = self.get_decoder_hasher_state(i);
            (1..8_usize).fold(E::ONE, |acc, k| {
                acc * (alphas[0]
                    + alphas[1].mul_base(block_id)
                    + alphas[2].mul_base(group_count - Felt::from(k as u64))
                    + alphas[3].mul_base(h[k]))
            })
        } else if op_batch_flag == OP_BATCH_4_GROUPS {
            let h = self.get_decoder_hasher_state_first_half(i);
            (1..4_usize).fold(E::ONE, |acc, k| {
                acc * (alphas[0]
                    + alphas[1].mul_base(block_id)
                    + alphas[2].mul_base(group_count - Felt::from(k as u64))
                    + alphas[3].mul_base(h[k]))
            })
        } else if op_batch_flag == OP_BATCH_2_GROUPS {
            let h = self.get_decoder_hasher_state_first_half(i);
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
        &self,
        i: usize,
        alphas: &[E],
    ) -> E {
        let group_count = self.get_group_count(i);
        let block_id = self.get_block_addr(i);

        let op_code = self.get_op_code(i);
        let tmp = if op_code == Felt::from(PUSH) {
            self.get_stack_element(0, i + 1)
        } else {
            let h0 = self.get_decoder_hasher_state_first_half(i + 1)[0];

            let op_prime = self.get_op_code(i + 1);
            h0.mul_small(1 << 7) + op_prime
        };
        alphas[0]
            + alphas[1].mul_base(block_id)
            + alphas[2].mul_base(group_count)
            + alphas[3].mul_base(tmp)
    }
}
