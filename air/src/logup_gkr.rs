use alloc::vec::Vec;
use core::marker::PhantomData;

use vm_core::{utils::range, ExtensionOf, Felt, FieldElement, StarkField};
use winter_air::{EvaluationFrame, LogUpGkrEvaluator, LogUpGkrOracle};

use crate::{
    constraints::chiplets::hasher::{HASH_K0_MASK, HASH_K1_MASK, HASH_K2_MASK},
    decoder::{
        DECODER_ADDR_COL_IDX, DECODER_GROUP_COUNT_COL_IDX, DECODER_HASHER_STATE_OFFSET,
        DECODER_IN_SPAN_COL_IDX, DECODER_IS_CALL_FLAG_COL_IDX, DECODER_IS_LOOP_BODY_FLAG_COL_IDX,
        DECODER_IS_LOOP_FLAG_COL_IDX, DECODER_IS_SYSCALL_FLAG_COL_IDX,
        DECODER_OP_BATCH_FLAGS_OFFSET, DECODER_OP_BITS_EXTRA_COLS_OFFSET, DECODER_OP_BITS_OFFSET,
        DECODER_USER_OP_HELPERS_OFFSET,
    },
    trace::{
        chiplets::{
            HASHER_NODE_INDEX_COL_IDX, HASHER_SELECTOR_COL_RANGE, HASHER_STATE_COL_RANGE,
            MEMORY_D0_COL_IDX, MEMORY_D1_COL_IDX,
        },
        range::{M_COL_IDX, V_COL_IDX},
        stack::{B0_COL_IDX, B1_COL_IDX, STACK_TOP_OFFSET},
    },
    PublicInputs, CHIPLETS_OFFSET, CTX_COL_IDX, FMP_COL_IDX, FN_HASH_RANGE, STACK_TRACE_OFFSET,
    TRACE_WIDTH,
};

// CONSTANTS
// ===============================================================================================

const fn const_max(a: usize, b: usize) -> usize {
    // NOTE: `[(a < b) as usize]` evaluates to 0 or 1, selection `a` or `b` accordingly
    [a, b][(a < b) as usize]
}

// Random values

/// The number of random values used as offsets (alpha_0 is our docs)
pub const NUM_OFFSET_RAND_VALUES: usize = 6;

const RANGE_CHECKER_NUM_RAND_LINCOMB_VALUES: usize = 0;
const OP_GROUP_TABLE_NUM_RAND_LINCOMB_VALUES: usize = 3;
const BLOCK_HASH_TABLE_NUM_RAND_LINCOMB_VALUES: usize = 7;
const BLOCK_STACK_TABLE_NUM_RAND_LINCOMB_VALUES: usize = 11;
const HASHER_TABLE_NUM_RAND_LINCOMB_VALUES: usize = 15;
const KERNEL_PROC_TABLE_NUM_RAND_LINCOMB_VALUES: usize = 5;

/// The number of random values to generate to support all random linear combinations.
///
/// All tables are allowed to share the same random linear combination coefficients since each table
/// is offset by a different random value.
pub const MAX_RAND_LINCOMB_VALUES: usize = const_max(
    const_max(
        const_max(
            const_max(
                const_max(
                    RANGE_CHECKER_NUM_RAND_LINCOMB_VALUES,
                    OP_GROUP_TABLE_NUM_RAND_LINCOMB_VALUES,
                ),
                BLOCK_HASH_TABLE_NUM_RAND_LINCOMB_VALUES,
            ),
            BLOCK_STACK_TABLE_NUM_RAND_LINCOMB_VALUES,
        ),
        HASHER_TABLE_NUM_RAND_LINCOMB_VALUES,
    ),
    KERNEL_PROC_TABLE_NUM_RAND_LINCOMB_VALUES,
);

/// The total number of random values to generate
pub const TOTAL_NUM_RAND_VALUES: usize = NUM_OFFSET_RAND_VALUES + MAX_RAND_LINCOMB_VALUES;

// Fractions

pub const RANGE_CHECKER_FRACTIONS_OFFSET: usize = 0;
pub const RANGE_CHECKER_NUM_FRACTIONS: usize = 7;

pub const OP_GROUP_TABLE_FRACTIONS_OFFSET: usize =
    RANGE_CHECKER_FRACTIONS_OFFSET + RANGE_CHECKER_NUM_FRACTIONS;
pub const OP_GROUP_TABLE_NUM_FRACTIONS: usize = 8;

pub const BLOCK_HASH_TABLE_FRACTIONS_OFFSET: usize =
    OP_GROUP_TABLE_FRACTIONS_OFFSET + OP_GROUP_TABLE_NUM_FRACTIONS;
pub const BLOCK_HASH_TABLE_NUM_FRACTIONS: usize = 8;

pub const BLOCK_STACK_TABLE_FRACTIONS_OFFSET: usize =
    BLOCK_HASH_TABLE_FRACTIONS_OFFSET + BLOCK_HASH_TABLE_NUM_FRACTIONS;
pub const BLOCK_STACK_TABLE_NUM_FRACTIONS: usize = 7;

pub const HASHER_TABLE_FRACTIONS_OFFSET: usize =
    BLOCK_STACK_TABLE_FRACTIONS_OFFSET + BLOCK_STACK_TABLE_NUM_FRACTIONS;
pub const HASHER_TABLE_NUM_FRACTIONS: usize = 4;

pub const KERNEL_PROC_TABLE_FRACTIONS_OFFSET: usize =
    HASHER_TABLE_FRACTIONS_OFFSET + HASHER_TABLE_NUM_FRACTIONS;
pub const KERNEL_PROC_TABLE_NUM_FRACTIONS: usize = 1;

pub const PADDING_FRACTIONS_OFFSET: usize =
    KERNEL_PROC_TABLE_FRACTIONS_OFFSET + KERNEL_PROC_TABLE_NUM_FRACTIONS;
pub const PADDING_NUM_FRACTIONS: usize = TOTAL_NUM_FRACTIONS - PADDING_FRACTIONS_OFFSET;

pub const TOTAL_NUM_FRACTIONS: usize = 64;

// LogUp GKR Evaluator
// ===============================================================================================

#[derive(Clone, Default)]
pub struct MidenLogUpGkrEval<B: FieldElement + StarkField> {
    oracles: Vec<LogUpGkrOracle>,
    _field: PhantomData<B>,
}

impl<B: FieldElement + StarkField> MidenLogUpGkrEval<B> {
    pub fn new() -> Self {
        let oracles = {
            let oracles_current_row = (0..TRACE_WIDTH).map(LogUpGkrOracle::CurrentRow);
            let oracles_next_row = (0..TRACE_WIDTH).map(LogUpGkrOracle::NextRow);

            oracles_current_row.chain(oracles_next_row).collect()
        };

        Self { oracles, _field: PhantomData }
    }
}

impl LogUpGkrEvaluator for MidenLogUpGkrEval<Felt> {
    type BaseField = Felt;

    type PublicInputs = PublicInputs;

    fn get_oracles(&self) -> &[LogUpGkrOracle] {
        &self.oracles
    }

    fn get_periodic_column_values(&self) -> Vec<Vec<Self::BaseField>> {
        vec![HASH_K0_MASK.to_vec(), HASH_K1_MASK.to_vec(), HASH_K2_MASK.to_vec()]
    }

    fn get_num_rand_values(&self) -> usize {
        TOTAL_NUM_RAND_VALUES
    }

    fn get_num_fractions(&self) -> usize {
        TOTAL_NUM_FRACTIONS
    }

    fn max_degree(&self) -> usize {
        10
    }

    fn build_query<E>(&self, frame: &EvaluationFrame<E>, query: &mut [E])
    where
        E: FieldElement<BaseField = Self::BaseField>,
    {
        let frame_current_then_next = frame.current().iter().chain(frame.next().iter());

        query.iter_mut().zip(frame_current_then_next).for_each(|(q, f)| *q = *f);
    }

    #[inline(always)]
    fn evaluate_query<F, E>(
        &self,
        query: &[F],
        periodic_values: &[F],
        rand_values: &[E],
        numerator: &mut [E],
        denominator: &mut [E],
    ) where
        F: FieldElement<BaseField = Self::BaseField>,
        E: FieldElement<BaseField = Self::BaseField> + ExtensionOf<F>,
    {
        debug_assert_eq!(numerator.len(), TOTAL_NUM_FRACTIONS);
        debug_assert_eq!(denominator.len(), TOTAL_NUM_FRACTIONS);
        debug_assert_eq!(query.len(), TRACE_WIDTH * 2);

        let query_current = &query[0..TRACE_WIDTH];
        let query_next = &query[TRACE_WIDTH..];

        let op_flags = LogUpOpFlags::new(query_current, query_next);

        let offset_rand_values = &rand_values[0..NUM_OFFSET_RAND_VALUES];
        let mut alphas = {
            let lin_comb_rand_values = &rand_values[NUM_OFFSET_RAND_VALUES..];
            let mut alphas = [E::ZERO; 1 + MAX_RAND_LINCOMB_VALUES];
            // `alphas[0]` will be reassigned before each table
            alphas[0] = offset_rand_values[0];
            alphas[1..].copy_from_slice(lin_comb_rand_values);

            alphas
        };

        range_checker(
            query_current,
            &op_flags,
            alphas[0],
            &mut numerator[range(RANGE_CHECKER_FRACTIONS_OFFSET, RANGE_CHECKER_NUM_FRACTIONS)],
            &mut denominator[range(RANGE_CHECKER_FRACTIONS_OFFSET, RANGE_CHECKER_NUM_FRACTIONS)],
        );
        {
            alphas[0] = offset_rand_values[1];
            op_group_table(
                query_current,
                query_next,
                &op_flags,
                &alphas,
                &mut numerator
                    [range(OP_GROUP_TABLE_FRACTIONS_OFFSET, OP_GROUP_TABLE_NUM_FRACTIONS)],
                &mut denominator
                    [range(OP_GROUP_TABLE_FRACTIONS_OFFSET, OP_GROUP_TABLE_NUM_FRACTIONS)],
            );
        }
        {
            alphas[0] = offset_rand_values[2];
            block_hash_table(
                query_current,
                query_next,
                &op_flags,
                &alphas,
                &mut numerator
                    [range(BLOCK_HASH_TABLE_FRACTIONS_OFFSET, BLOCK_HASH_TABLE_NUM_FRACTIONS)],
                &mut denominator
                    [range(BLOCK_HASH_TABLE_FRACTIONS_OFFSET, BLOCK_HASH_TABLE_NUM_FRACTIONS)],
            );
        }
        {
            alphas[0] = offset_rand_values[3];
            block_stack_table(
                query_current,
                query_next,
                &op_flags,
                &alphas,
                &mut numerator
                    [range(BLOCK_STACK_TABLE_FRACTIONS_OFFSET, BLOCK_STACK_TABLE_NUM_FRACTIONS)],
                &mut denominator
                    [range(BLOCK_STACK_TABLE_FRACTIONS_OFFSET, BLOCK_STACK_TABLE_NUM_FRACTIONS)],
            );
        }
        {
            alphas[0] = offset_rand_values[4];
            hasher_table(
                query_current,
                query_next,
                periodic_values,
                &alphas,
                &mut numerator[range(HASHER_TABLE_FRACTIONS_OFFSET, HASHER_TABLE_NUM_FRACTIONS)],
                &mut denominator[range(HASHER_TABLE_FRACTIONS_OFFSET, HASHER_TABLE_NUM_FRACTIONS)],
            );
        }
        {
            alphas[0] = offset_rand_values[5];
            kernel_proc_table(
                query_current,
                query_next,
                &alphas,
                &mut numerator
                    [range(KERNEL_PROC_TABLE_FRACTIONS_OFFSET, KERNEL_PROC_TABLE_NUM_FRACTIONS)],
                &mut denominator
                    [range(KERNEL_PROC_TABLE_FRACTIONS_OFFSET, KERNEL_PROC_TABLE_NUM_FRACTIONS)],
            );
        }
        padding(
            &mut numerator[range(PADDING_FRACTIONS_OFFSET, PADDING_NUM_FRACTIONS)],
            &mut denominator[range(PADDING_FRACTIONS_OFFSET, PADDING_NUM_FRACTIONS)],
        );
    }

    fn compute_claim<E>(&self, inputs: &Self::PublicInputs, rand_values: &[E]) -> E
    where
        E: FieldElement<BaseField = Self::BaseField>,
    {
        let offset_rand_values = &rand_values[0..NUM_OFFSET_RAND_VALUES];
        let mut alphas = {
            let lin_comb_rand_values = &rand_values[NUM_OFFSET_RAND_VALUES..];
            let mut alphas = [E::ZERO; 1 + MAX_RAND_LINCOMB_VALUES];
            // `alphas[0]` will be reassigned before each table
            alphas[0] = offset_rand_values[0];
            alphas[1..].copy_from_slice(lin_comb_rand_values);

            alphas
        };
        // block hash table
        let block_hash_table_claim = {
            alphas[0] = offset_rand_values[2];
            let program_hash = inputs.program_info.program_hash();

            -(alphas[0] + inner_product(&alphas[2..6], program_hash.as_elements()))
        };

        block_hash_table_claim.inv()
    }
}

/// TODO(plafer): docs
#[inline(always)]
fn range_checker<F, E>(
    query_current: &[F],
    op_flags: &LogUpOpFlags<F>,
    alpha: E,
    numerator: &mut [E],
    denominator: &mut [E],
) where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    // numerators
    let multiplicity = query_current[M_COL_IDX];
    let f_m = {
        let mem_selec0 = query_current[CHIPLETS_OFFSET];
        let mem_selec1 = query_current[CHIPLETS_OFFSET + 1];
        let mem_selec2 = query_current[CHIPLETS_OFFSET + 2];

        E::from(mem_selec0 * mem_selec1 * (F::ONE - mem_selec2))
    };

    let f_rc: E = op_flags.f_range_check().into();
    numerator[0] = E::from(multiplicity);
    numerator[1] = f_m;
    numerator[2] = f_m;
    numerator[3] = f_rc;
    numerator[4] = f_rc;
    numerator[5] = f_rc;
    numerator[6] = f_rc;

    // denominators
    let table_denom = alpha - E::from(query_current[V_COL_IDX]);
    let memory_denom_0 = -(alpha - E::from(query_current[MEMORY_D0_COL_IDX]));
    let memory_denom_1 = -(alpha - E::from(query_current[MEMORY_D1_COL_IDX]));
    let stack_value_denom_0 = -(alpha - E::from(query_current[DECODER_USER_OP_HELPERS_OFFSET]));
    let stack_value_denom_1 = -(alpha - E::from(query_current[DECODER_USER_OP_HELPERS_OFFSET + 1]));
    let stack_value_denom_2 = -(alpha - E::from(query_current[DECODER_USER_OP_HELPERS_OFFSET + 2]));
    let stack_value_denom_3 = -(alpha - E::from(query_current[DECODER_USER_OP_HELPERS_OFFSET + 3]));

    denominator[0] = table_denom;
    denominator[1] = memory_denom_0;
    denominator[2] = memory_denom_1;
    denominator[3] = stack_value_denom_0;
    denominator[4] = stack_value_denom_1;
    denominator[5] = stack_value_denom_2;
    denominator[6] = stack_value_denom_3;
}

/// TODO(plafer): docs
#[inline(always)]
fn op_group_table<F, E>(
    query_current: &[F],
    query_next: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
    numerator: &mut [E],
    denominator: &mut [E],
) where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    // numerators
    let f_delete_group = E::from(
        query_current[DECODER_IN_SPAN_COL_IDX]
            * (query_current[DECODER_GROUP_COUNT_COL_IDX]
                - query_next[DECODER_GROUP_COUNT_COL_IDX]),
    );

    let (f_g2, f_g4, f_g8) = {
        let bc0 = query_current[DECODER_OP_BATCH_FLAGS_OFFSET];
        let bc1 = query_current[DECODER_OP_BATCH_FLAGS_OFFSET + 1];
        let bc2 = query_current[DECODER_OP_BATCH_FLAGS_OFFSET + 2];

        (
            E::from((F::ONE - bc0) * (F::ONE - bc1) * bc2),
            E::from((F::ONE - bc0) * bc1 * (F::ONE - bc2)),
            E::from(bc0),
        )
    };

    numerator[0] = f_delete_group;
    numerator[1] = f_g2 + f_g4 + f_g8; // v1
    numerator[2] = f_g4 + f_g8; // v2
    numerator[3] = f_g4 + f_g8; // v3
    numerator[4] = f_g8; // v4
    numerator[5] = f_g8; // v5
    numerator[6] = f_g8; // v6
    numerator[7] = f_g8; // v7

    // denominators
    let addr = query_current[DECODER_ADDR_COL_IDX];
    let addr_next = query_next[DECODER_ADDR_COL_IDX];
    let group_count = query_current[DECODER_GROUP_COUNT_COL_IDX];
    let h0_next = query_next[DECODER_HASHER_STATE_OFFSET];
    let op_next = op_flags.op_value_next();
    let h2 = query_current[DECODER_HASHER_STATE_OFFSET + 2];
    let s0_next = query_next[STACK_TRACE_OFFSET + STACK_TOP_OFFSET];
    let (v1, v2, v3, v4, v5, v6, v7) = {
        let v = |idx: u8| {
            alphas[0]
                + alphas[1].mul_base(addr_next)
                + alphas[2].mul_base(group_count - idx.into())
                + alphas[3].mul_base(query_current[DECODER_HASHER_STATE_OFFSET + idx as usize])
        };

        (v(1), v(2), v(3), v(4), v(5), v(6), v(7))
    };

    let f_push = op_flags.f_push();
    let f_emit = op_flags.f_emit();
    let f_imm = op_flags.f_imm();

    denominator[0] = -(alphas[0]
        + alphas[1].mul_base(addr)
        + alphas[2].mul_base(group_count)
        + alphas[3].mul_base(
            (F::from(128_u32) * h0_next + op_next) * (F::ONE - f_imm)
                + s0_next * f_push
                + h2 * f_emit,
        ));
    denominator[1] = v1;
    denominator[2] = v2;
    denominator[3] = v3;
    denominator[4] = v4;
    denominator[5] = v5;
    denominator[6] = v6;
    denominator[7] = v7;
}

#[inline(always)]
fn block_hash_table<F, E>(
    query_current: &[F],
    query_next: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
    numerator: &mut [E],
    denominator: &mut [E],
) where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    let stack_0 = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET];

    // numerators
    let f_join: E = op_flags.f_join().into();

    numerator[0] = op_flags.f_end().into();
    numerator[1] = f_join;
    numerator[2] = f_join;
    numerator[3] = op_flags.f_split().into();
    numerator[4] = (op_flags.f_loop() * stack_0).into();
    numerator[5] = op_flags.f_repeat().into();
    numerator[6] = op_flags.f_dyn().into();
    // TODO(plafer): update docs (no mention of call or syscall)
    numerator[7] = (op_flags.f_call() + op_flags.f_syscall()).into();

    // denominators
    let addr_next = query_next[DECODER_ADDR_COL_IDX];
    let h0_to_3 = &query_current[range(DECODER_HASHER_STATE_OFFSET, 4)];
    let h4_to_7 = &query_current[range(DECODER_HASHER_STATE_OFFSET + 4, 4)];
    let stack_1 = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 1];
    let stack_2 = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 2];
    let stack_3 = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET + 3];
    // TODO(plafer): update docs (this is h4 in docs)
    let f_is_loop_body = query_current[DECODER_IS_LOOP_BODY_FLAG_COL_IDX];
    let child1 = alphas[0] + alphas[1].mul_base(addr_next) + inner_product(&alphas[2..6], h0_to_3);
    let child2 = alphas[0] + alphas[1].mul_base(addr_next) + inner_product(&alphas[2..6], h4_to_7);

    let u_end = {
        // TODO(plafer): update docs (f_halt missing)
        let is_first_child =
            F::ONE - (op_flags.f_end_next() + op_flags.f_repeat_next() + op_flags.f_halt_next());

        // TODO(plafer): Double check addr_next; docs inconsistent with BlockHashTableRow
        alphas[0]
            + alphas[1].mul_base(addr_next)
            + inner_product(&alphas[2..6], h0_to_3)
            + alphas[6].mul_base(is_first_child)
            + alphas[7].mul_base(f_is_loop_body)
    };

    let v_join_1 = child1 + alphas[6];
    let v_join_2 = child2;
    let v_split = child1.mul_base(stack_0) + child2.mul_base(F::ONE - stack_0);
    let v_loop = child1 + alphas[7];
    let v_repeat = child1 + alphas[7];
    let v_dyn = alphas[0]
        + alphas[1].mul_base(addr_next)
        + inner_product(&alphas[2..6], &[stack_3, stack_2, stack_1, stack_0]);

    denominator[0] = -u_end;
    denominator[1] = v_join_1;
    denominator[2] = v_join_2;
    denominator[3] = v_split;
    denominator[4] = v_loop;
    denominator[5] = v_repeat;
    denominator[6] = v_dyn;
    denominator[7] = child1;
}

#[inline(always)]
fn block_stack_table<F, E>(
    query_current: &[F],
    query_next: &[F],
    op_flags: &LogUpOpFlags<F>,
    alphas: &[E],
    numerator: &mut [E],
    denominator: &mut [E],
) where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    // numerators
    let f_respan: E = op_flags.f_respan().into();
    let f_end: E = op_flags.f_end().into();
    let f_call_or_syscall_flags: E = (query_current[DECODER_IS_CALL_FLAG_COL_IDX]
        + query_current[DECODER_IS_SYSCALL_FLAG_COL_IDX])
        .into();

    numerator[0] = f_respan;
    numerator[1] = f_end * (E::ONE - f_call_or_syscall_flags);
    numerator[2] = f_end * f_call_or_syscall_flags;

    numerator[3] = (op_flags.f_call() + op_flags.f_syscall()).into();
    numerator[4] = op_flags.f_loop().into();
    numerator[5] = f_respan;
    numerator[6] =
        (op_flags.f_join() + op_flags.f_split() + op_flags.f_span() + op_flags.f_dyn()).into();

    // removal denominators
    {
        let block_id = query_current[DECODER_ADDR_COL_IDX];
        let parent_id_respan = query_next[DECODER_HASHER_STATE_OFFSET + 1];
        let parent_id_end = query_next[DECODER_ADDR_COL_IDX];
        let f_is_loop = query_current[DECODER_IS_LOOP_FLAG_COL_IDX];
        let parent_ctx = query_next[CTX_COL_IDX];
        let parent_fmp = query_next[FMP_COL_IDX];
        let parent_stack_depth = query_next[STACK_TRACE_OFFSET + B0_COL_IDX];
        let parent_next_overflow_addr = query_next[STACK_TRACE_OFFSET + B1_COL_IDX];
        let parent_fn_hash = &query_next[FN_HASH_RANGE];

        let call_or_syscall_inner_product = alphas[4].mul_base(parent_ctx)
            + alphas[5].mul_base(parent_fmp)
            + alphas[6].mul_base(parent_stack_depth)
            + alphas[7].mul_base(parent_next_overflow_addr)
            + inner_product(&alphas[8..12], parent_fn_hash);
        let v_respan =
            alphas[0] + alphas[1].mul_base(block_id) + alphas[2].mul_base(parent_id_respan);
        let v_end = alphas[0]
            + alphas[1].mul_base(block_id)
            + alphas[2].mul_base(parent_id_end)
            + alphas[3].mul_base(f_is_loop);
        let v_end_call_or_syscall = v_end + call_or_syscall_inner_product;

        denominator[0] = -v_respan;
        denominator[1] = -v_end;
        denominator[2] = -v_end_call_or_syscall;
    }

    // insertion denominators
    {
        let block_id = query_next[DECODER_ADDR_COL_IDX];
        let parent_id_respan = query_next[DECODER_HASHER_STATE_OFFSET + 1];
        let parent_id_not_respan = query_current[DECODER_ADDR_COL_IDX];
        let stack_element_0 = query_current[STACK_TRACE_OFFSET + STACK_TOP_OFFSET];
        let parent_ctx = query_current[CTX_COL_IDX];
        let parent_fmp = query_current[FMP_COL_IDX];
        let parent_stack_depth = query_current[STACK_TRACE_OFFSET + B0_COL_IDX];
        let parent_next_overflow_addr = query_current[STACK_TRACE_OFFSET + B1_COL_IDX];
        let parent_fn_hash = &query_current[FN_HASH_RANGE];

        let v_call_or_syscall = alphas[0]
            + alphas[1].mul_base(block_id)
            + alphas[2].mul_base(parent_id_not_respan)
            + alphas[4].mul_base(parent_ctx)
            + alphas[5].mul_base(parent_fmp)
            + alphas[6].mul_base(parent_stack_depth)
            + alphas[7].mul_base(parent_next_overflow_addr)
            + inner_product(&alphas[8..12], parent_fn_hash);
        let v_loop = alphas[0]
            + alphas[1].mul_base(block_id)
            + alphas[2].mul_base(parent_id_not_respan)
            + alphas[3].mul_base(stack_element_0);
        let v_respan =
            alphas[0] + alphas[1].mul_base(block_id) + alphas[2].mul_base(parent_id_respan);
        let v_join_split_span_dyn =
            alphas[0] + alphas[1].mul_base(block_id) + alphas[2].mul_base(parent_id_not_respan);

        denominator[3] = v_call_or_syscall;
        denominator[4] = v_loop;
        denominator[5] = v_respan;
        denominator[6] = v_join_split_span_dyn;
    }
}

#[inline(always)]
fn hasher_table<F, E>(
    query_current: &[F],
    query_next: &[F],
    periodic_values: &[F],
    alphas: &[E],
    numerator: &mut [E],
    denominator: &mut [E],
) where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    // numerators
    {
        let (f_mu, f_mua, f_mv, f_mva) = {
            let s = &query_current[HASHER_SELECTOR_COL_RANGE];
            let k = periodic_values;

            (
                E::from(k[2] * s[0] * s[1] * s[2]),
                E::from(k[0] * s[0] * s[1] * s[2]),
                E::from(k[2] * s[0] * s[1] * (F::ONE - s[2])),
                E::from(k[0] * s[0] * s[1] * (F::ONE - s[2])),
            )
        };

        let is_hasher_chiplet = E::ONE - query_current[CHIPLETS_OFFSET].into();

        let index = query_current[HASHER_NODE_INDEX_COL_IDX];
        let index_next = query_next[HASHER_NODE_INDEX_COL_IDX];
        // The value of the bit which is discarded when the node index is shifted by one bit to the
        // right.
        let index_lsb = E::from(index - F::from(2_u32) * index_next);
        let not_index_lsb = E::ONE - index_lsb;

        numerator[0] = is_hasher_chiplet * not_index_lsb * (f_mv - f_mu);
        numerator[1] = is_hasher_chiplet * index_lsb * (f_mv - f_mu);
        numerator[2] = is_hasher_chiplet * not_index_lsb * (f_mva - f_mua);
        numerator[3] = is_hasher_chiplet * index_lsb * (f_mva - f_mua);
    }

    // denominator
    {
        let prefix = {
            let index = query_current[HASHER_NODE_INDEX_COL_IDX];
            alphas[0] + alphas[3].mul_base(index)
        };
        let hasher_state = &query_current[HASHER_STATE_COL_RANGE];
        let hasher_state_next = &query_next[HASHER_STATE_COL_RANGE];

        let sibling = &hasher_state[8..12];
        denominator[0] = prefix + inner_product(&alphas[12..16], sibling);

        let sibling = &hasher_state[4..8];
        denominator[1] = prefix + inner_product(&alphas[8..12], sibling);

        let sibling = &hasher_state_next[8..12];
        denominator[2] = prefix + inner_product(&alphas[12..16], sibling);

        let sibling = &hasher_state_next[4..8];
        denominator[3] = prefix + inner_product(&alphas[8..12], sibling);
    }
}

// Note: the kernel proc implementation is broken (issue #1515), so this is just padding for now.
#[inline(always)]
fn kernel_proc_table<F, E>(
    _query_current: &[F],
    _query_next: &[F],
    _alphas: &[E],
    numerator: &mut [E],
    denominator: &mut [E],
) where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    numerator[0] = E::ZERO;
    denominator[0] = E::ONE;
}

/// TODO(plafer): docs
fn padding<E>(numerator: &mut [E], denominator: &mut [E])
where
    E: FieldElement,
{
    numerator.fill(E::ZERO);
    denominator.fill(E::ONE);
}

// TODO(plafer): save intermediary values between flags instead of recomputing
struct LogUpOpFlags<F: FieldElement> {
    f_push: F,
    f_emit: F,
    f_dyn: F,
    f_split: F,
    f_loop: F,
    f_span: F,
    f_join: F,
    f_imm: F,
    f_repeat: F,
    f_end: F,
    f_syscall: F,
    f_call: F,
    f_respan: F,
    f_range_check: F,
    op_value_next: F,
    f_repeat_next: F,
    f_end_next: F,
    f_halt_next: F,
}

impl<F: FieldElement> LogUpOpFlags<F> {
    pub fn new(query_current: &[F], query_next: &[F]) -> Self {
        // current
        // --------------------------------------------------------------------
        let b0 = query_current[DECODER_OP_BITS_OFFSET];
        let b1 = query_current[DECODER_OP_BITS_OFFSET + 1];
        let b2 = query_current[DECODER_OP_BITS_OFFSET + 2];
        let b3 = query_current[DECODER_OP_BITS_OFFSET + 3];
        let b4 = query_current[DECODER_OP_BITS_OFFSET + 4];
        let b5 = query_current[DECODER_OP_BITS_OFFSET + 5];
        let b6 = query_current[DECODER_OP_BITS_OFFSET + 6];
        let e0 = query_current[DECODER_OP_BITS_EXTRA_COLS_OFFSET];
        let e1 = query_current[DECODER_OP_BITS_EXTRA_COLS_OFFSET + 1];

        // degree 5 flags
        let e0_b3_nb2 = e0 * b3 * (F::ONE - b2);
        let e0_b3_nb2_b1 = e0_b3_nb2 * b1;
        let f_push = e0_b3_nb2_b1 * b0;
        let f_emit = e0_b3_nb2_b1 * (F::ONE - b0);

        let e0_nb3_b2 = e0 * (F::ONE - b3) * b2;
        let e0_nb3_b2_b1 = e0_nb3_b2 * b1;
        let e0_nb3_b2_nb1 = e0_nb3_b2 * (F::ONE - b1);

        // degree 4 flags
        let e1_b4 = e1 * b4;
        let e1_b4_nb3 = e1_b4 * (F::ONE - b3);

        let e1_nb4_b3 = e1 * (F::ONE - b4) * b3;

        // next
        // --------------------------------------------------------------------
        let b0_next = query_next[DECODER_OP_BITS_OFFSET];
        let b1_next = query_next[DECODER_OP_BITS_OFFSET + 1];
        let b2_next = query_next[DECODER_OP_BITS_OFFSET + 2];
        let b3_next = query_next[DECODER_OP_BITS_OFFSET + 3];
        let b4_next = query_next[DECODER_OP_BITS_OFFSET + 4];
        let b5_next = query_next[DECODER_OP_BITS_OFFSET + 5];
        let b6_next = query_next[DECODER_OP_BITS_OFFSET + 6];
        let e1_next = query_next[DECODER_OP_BITS_EXTRA_COLS_OFFSET + 1];

        let e1_b4_next = e1_next * b4_next;
        let e1_b4_nb3_next = e1_b4_next * (F::ONE - b3_next);

        Self {
            // degree 5 flags
            f_push,
            f_emit,
            f_imm: f_push + f_emit,
            f_dyn: e0_b3_nb2 * (F::ONE - b1) * (F::ONE - b0),
            f_split: e0_nb3_b2_nb1 * (F::ONE - b0),
            f_loop: e0_nb3_b2_nb1 * b0,
            f_span: e0_nb3_b2_b1 * (F::ONE - b0),
            f_join: e0_nb3_b2_b1 * b0,

            // degree 4 flags
            f_repeat: e1_b4_nb3 * b2,
            f_end: e1_b4_nb3 * (F::ONE - b2),
            f_respan: e1_b4 * b3 * (F::ONE - b2),
            f_syscall: e1_nb4_b3 * (F::ONE - b2),
            f_call: e1_nb4_b3 * b2,

            // misc
            f_range_check: (F::ONE - b4) * (F::ONE - b5) * b6,

            // next
            op_value_next: b0_next
                + F::from(2_u32) * b1_next
                + F::from(4_u32) * b2_next
                + F::from(8_u32) * b3_next
                + F::from(16_u32) * b4_next
                + F::from(32_u32) * b5_next
                + F::from(64_u32) * b6_next,
            f_repeat_next: e1_b4_nb3_next * b2_next,
            f_end_next: e1_b4_nb3_next * (F::ONE - b2_next),
            f_halt_next: e1_b4_next * b3_next * b2_next,
        }
    }

    // flag degree 5

    pub fn f_push(&self) -> F {
        self.f_push
    }

    pub fn f_emit(&self) -> F {
        self.f_emit
    }

    pub fn f_dyn(&self) -> F {
        self.f_dyn
    }

    pub fn f_split(&self) -> F {
        self.f_split
    }

    pub fn f_loop(&self) -> F {
        self.f_loop
    }

    pub fn f_span(&self) -> F {
        self.f_span
    }

    pub fn f_join(&self) -> F {
        self.f_join
    }

    pub fn f_imm(&self) -> F {
        self.f_imm
    }

    // degree 4 flags

    pub fn f_repeat(&self) -> F {
        self.f_repeat
    }

    pub fn f_end(&self) -> F {
        self.f_end
    }

    pub fn f_respan(&self) -> F {
        self.f_respan
    }

    pub fn f_syscall(&self) -> F {
        self.f_syscall
    }

    pub fn f_call(&self) -> F {
        self.f_call
    }

    // misc

    pub fn f_range_check(&self) -> F {
        self.f_range_check
    }

    // next

    pub fn op_value_next(&self) -> F {
        self.op_value_next
    }

    pub fn f_repeat_next(&self) -> F {
        self.f_repeat_next
    }

    pub fn f_end_next(&self) -> F {
        self.f_end_next
    }

    pub fn f_halt_next(&self) -> F {
        self.f_halt_next
    }
}

// HELPERS
// -----------------------------------------------------------------------------------------------

fn inner_product<F, E>(alphas: &[E], eles: &[F]) -> E
where
    F: FieldElement,
    E: FieldElement + ExtensionOf<F>,
{
    alphas
        .iter()
        .zip(eles.iter())
        .fold(E::ZERO, |acc, (alpha, ele)| acc + alpha.mul_base(*ele))
}
