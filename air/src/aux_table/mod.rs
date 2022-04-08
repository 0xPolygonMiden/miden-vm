use super::{EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree};
use crate::utils::{binary_not, is_binary};
use vm_core::{utils::range as create_range, AUX_TRACE_OFFSET};

mod memory;

// CONSTANTS
// ================================================================================================
const NUM_CONSTRAINTS: usize = 3;
const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    2, 3, 4, // Selector flags must be binary.
];

pub const S0_COL_IDX: usize = AUX_TRACE_OFFSET;
pub const S1_COL_IDX: usize = AUX_TRACE_OFFSET + 1;
pub const S2_COL_IDX: usize = AUX_TRACE_OFFSET + 2;
pub const MEMORY_TRACE_OFFSET: usize = S2_COL_IDX + 1;

// CONSTRAINT DEGREES
// ================================================================================================

pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    let mut degrees = Vec::new();

    // --- auxiliary table management -------------------------------------------------------------
    for &degree in CONSTRAINT_DEGREES.iter() {
        degrees.push(TransitionConstraintDegree::new(degree));
    }

    // --- memory -----------------------------------------------------------------------------
    degrees.append(&mut memory::get_transition_constraint_degrees());

    degrees
}

// TRANSITION CONSTRAINTS
// ================================================================================================

pub fn enforce_constraints<E: FieldElement<BaseField = Felt>>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
) {
    let current = frame.current();

    // Define the selector values.
    let s0 = current[S0_COL_IDX];
    let s1 = current[S1_COL_IDX];
    let s2 = current[S2_COL_IDX];

    // Define the processor flags.
    let _hasher_flag = binary_not(s0);
    let _bitwise_flag = s0 * binary_not(s1);
    let memory_flag = s0 * s1 * binary_not(s2);

    // --- auxiliary table management -------------------------------------------------------------
    enforce_selectors(frame, result);

    // --- memory ---------------------------------------------------------------------------------
    let memory_range = create_range(3, memory::get_transition_constraint_degrees().len());
    memory::enforce_constraints::<E>(frame, &mut result[memory_range.start..], memory_flag);
}

fn enforce_selectors<E: FieldElement<BaseField = Felt>>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
) {
    let current = frame.current();

    // Define the selector values.
    let s0 = current[S0_COL_IDX];
    let s1 = current[S1_COL_IDX];
    let s2 = current[S2_COL_IDX];

    // Selector flag s0 must be binary for the entire table.
    result[0] = is_binary(s0);

    // Selector s1 is only used as a flag when s0 is set.
    result[1] = s0 * is_binary(s1);

    // Selector s2 is only used as a flag when both s0 and s1 are set.
    result[2] = s0 * s1 * is_binary(s2);
}
