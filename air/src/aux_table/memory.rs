use super::{EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree, MEMORY_TRACE_OFFSET};
use crate::utils::{binary_not, is_binary, ColumnTransition, EvaluationResult};
use std::ops::Range;
use vm_core::utils::range as create_range;

// CONSTANTS
// ================================================================================================
const NUM_CONSTRAINTS: usize = 13;
const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    7, 6, 9, 8, // Constrain the values in the d inverse column.
    8, // Enforce values in ctx, addr, clk transition correctly.
    8, 8, 8, 8, // Enforce memory is initialized to zero.
    8, 8, 8,
    8, // Ensure next old values equal current new values when ctx and addr don't change.
];

// The number of elements accessible in one read or write memory access.
const NUM_ELEMENTS: usize = 4;
const CTX_COL_IDX: usize = MEMORY_TRACE_OFFSET;
const ADDR_COL_IDX: usize = CTX_COL_IDX + 1;
const CLK_COL_IDX: usize = ADDR_COL_IDX + 1;
const U_COL_RANGE: Range<usize> = create_range(CLK_COL_IDX + 1, NUM_ELEMENTS);
const V_COL_RANGE: Range<usize> = create_range(U_COL_RANGE.end, NUM_ELEMENTS);
const D0_COL_IDX: usize = V_COL_RANGE.end;
const D1_COL_IDX: usize = D0_COL_IDX + 1;
const D_INV_COL_IDX: usize = D1_COL_IDX + 1;

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
    processor_flag: E,
) {
    let current = frame.current();
    let next = frame.next();

    // --- Helper variables -----------------------------------------------------------------------
    let ctx_diff = frame.change(CTX_COL_IDX);
    let addr_diff = frame.change(ADDR_COL_IDX);
    let n0 = ctx_diff * current[D_INV_COL_IDX];
    let n1 = addr_diff * current[D_INV_COL_IDX];
    let same_ctx_flag = binary_not(n0);
    let same_addr_flag = binary_not(n1);

    // --- Constrain the values in the d inverse column. ------------------------------------------
    result.agg_constraint(0, processor_flag, is_binary(n0));
    result.agg_constraint(1, processor_flag * same_ctx_flag, ctx_diff);
    result.agg_constraint(2, processor_flag * same_ctx_flag, is_binary(n1));
    result.agg_constraint(
        3,
        processor_flag * same_ctx_flag * same_addr_flag,
        addr_diff,
    );

    // --- Enforce values in ctx, addr, clk transition correctly. ---------------------------------
    let clk_change = next[CLK_COL_IDX] - current[CLK_COL_IDX] - E::ONE;
    let delta = E::from(2_u32.pow(16)) * next[D1_COL_IDX] + next[D0_COL_IDX];

    // If the context changed, include the difference.
    result.agg_constraint(4, processor_flag * n0, ctx_diff);
    // If the context is the same, include the address difference if it changed or else include the
    // clock change.
    result.agg_constraint(
        4,
        processor_flag * same_ctx_flag,
        n1 * addr_diff + same_addr_flag * clk_change,
    );
    // Always subtract the delta. It should offset the other changes.
    result.agg_constraint(4, E::ONE, -delta);

    // --- Constrain the memory values. -----------------------------------------------------------
    let val_constraint_idx = 5;
    for i in 0..NUM_ELEMENTS {
        // Memory must be initialized to zero.
        result.agg_constraint(
            val_constraint_idx + i,
            processor_flag * n0 + same_ctx_flag * n1,
            next[U_COL_RANGE.start + i],
        );
        // The next old values must equal the current new values when ctx and addr don't change.
        result.agg_constraint(
            val_constraint_idx + NUM_ELEMENTS + i,
            processor_flag * same_ctx_flag * same_addr_flag,
            next[U_COL_RANGE.start + i] - current[V_COL_RANGE.start + i],
        );
    }
}
