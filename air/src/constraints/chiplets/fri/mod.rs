use super::super::{
    EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree,
};
use crate::utils::{are_equal, binary_not, is_binary};
use alloc::vec::Vec;

// CONSTANTS
// ================================================================================================
/// The number of constraints for the FRI chiplet.
pub const NUM_CONSTRAINTS: usize = 6;
/// The degrees of constraints for the FRI chiplet.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [2, 3, 4, 2, 3, 4];

// PERIODIC COLUMNS
// ================================================================================================

/// Returns the set of periodic columns required by the FRI chiplet.
pub fn get_periodic_column_values() -> Vec<Vec<Felt>> {
    // Define and return the periodic columns for FRI chiplet
    vec![
        vec![Felt::one(), Felt::zero(), Felt::zero()], // Example column values
        vec![Felt::zero(), Felt::one(), Felt::zero()],
        vec![Felt::zero(), Felt::zero(), Felt::one()],
    ]
}

// FRI TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the FRI chiplet.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect()
}

/// Returns the number of transition constraints for the FRI chiplet.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
}

/// Enforces constraints for the FRI chiplet.
pub fn enforce_constraints<E: FieldElement<BaseField = Felt>>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    fri_flag: E,
) {
    // --- FRI chiplet constraints --------------------------------------------------------------

    // Constraint 1:
    result[0] = fri_flag * is_binary(frame.s(0));

    // Constraint 2: 
    result[1] = fri_ftlag * frame.s(0) * is_binary(frame.s(1));

    // TODO: Add more constraints
}
