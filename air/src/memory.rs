use std::ops::Range;
use vm_core::{utils::range as create_range, AUX_TRACE_OFFSET};

use crate::utils::{is_binary, EvaluationResult};

use super::{EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree};

// CONSTANTS
// ================================================================================================
const NUM_CONSTRAINTS: usize = 13;
const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    2, 2, 3, 3, // Constrain the values in the d inverse column.
    5, // Enforce values in ctx, addr, clk transition correctly.
    3, 3, 3, 3, // Enforce memory is initialized to zero.
    3, 3, 3,
    3, // Ensure next old values equal current new values when ctx and addr don't change.
];

// 3 columns are used for flags
// The memory trace starts at index 3 of the auxiliary table.
pub const CTX_COL_IDX: usize = AUX_TRACE_OFFSET + 3;
pub const ADDR_COL_IDX: usize = AUX_TRACE_OFFSET + 4;
pub const CLK_COL_IDX: usize = AUX_TRACE_OFFSET + 5;
pub const U_COL_RANGE: Range<usize> = create_range(AUX_TRACE_OFFSET + 6, 4);
pub const V_COL_RANGE: Range<usize> = create_range(AUX_TRACE_OFFSET + 10, 4);
pub const D0_COL_IDX: usize = AUX_TRACE_OFFSET + 14;
pub const D1_COL_IDX: usize = AUX_TRACE_OFFSET + 15;
pub const D_INV_COL_IDX: usize = AUX_TRACE_OFFSET + 16;

// CONSTRAINT DEGREES
// ================================================================================================

pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    let mut result = Vec::new();

    for &degree in CONSTRAINT_DEGREES.iter() {
        result.push(TransitionConstraintDegree::new(degree));
    }

    result
}

// TRANSITION CONSTRAINTS
// ================================================================================================

pub fn enforce_constraints<E: FieldElement<BaseField = Felt>>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
) {
    let current = frame.current();
    let next = frame.next();

    // --- Helper variables -----------------------------------------------------------------------
    let ctx_diff = next[CTX_COL_IDX] - current[CTX_COL_IDX];
    let addr_diff = next[ADDR_COL_IDX] - current[ADDR_COL_IDX];
    let n0 = ctx_diff * current[D_INV_COL_IDX];
    let n1 = addr_diff * current[D_INV_COL_IDX];
    let same_ctx_flag = E::ONE - n0;
    let same_addr_flag = E::ONE - n1;

    // --- Constrain the values in the d inverse column. ------------------------------------------
    result[0] = is_binary(n0);
    result[1] = same_ctx_flag * ctx_diff;
    result[2] = same_ctx_flag * is_binary(n1);
    result[3] = same_ctx_flag * same_addr_flag * addr_diff;

    // --- Enforce values in ctx, addr, clk transition correctly. ---------------------------------
    let clk_change = next[CLK_COL_IDX] - current[CLK_COL_IDX] - E::ONE;
    let delta = E::from(2_u32.pow(16)) * next[D1_COL_IDX] + next[D0_COL_IDX];

    // If the context changed, include the difference.
    result.agg_constraint(4, n0, ctx_diff);
    // If the context is the same, include the address difference if it changed or else include the
    // clock change.
    result.agg_constraint(
        4,
        same_ctx_flag,
        n1 * addr_diff + same_addr_flag * clk_change,
    );
    // Always subtract the delta. It should offset the other changes.
    result.agg_constraint(4, E::ONE, -delta);

    // --- Constrain the memory values. -----------------------------------------------------------
    let mem_zero_start = 5;
    let old_new_start = 9;
    for i in 0..U_COL_RANGE.len() {
        // Memory must be initialized to zero.
        result[mem_zero_start + i] = (n0 + same_ctx_flag * n1) * next[U_COL_RANGE.start + i];
        // The next old values must equal the current new values when ctx and addr don't change.
        result[old_new_start + i] = same_ctx_flag
            * same_addr_flag
            * (next[U_COL_RANGE.start + i] - current[V_COL_RANGE.start + i]);
    }
}
