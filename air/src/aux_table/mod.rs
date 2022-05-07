use super::{EvaluationFrame, FieldElement, TransitionConstraintDegree};
use crate::utils::{binary_not, is_binary};
use vm_core::AUX_TRACE_OFFSET;

mod bitwise_pow2;
mod memory;

// EXPORTS
// ================================================================================================
pub use bitwise_pow2::{BITWISE_POW2_K0_MASK, BITWISE_POW2_K1_MASK};

// CONSTANTS
// ================================================================================================

/// The number of constraints on the management of the Auxiliary Table. This does not include
/// constraints for the co-processors.
pub const NUM_CONSTRAINTS: usize = 3;
/// The degrees of constraints on the management of the Auxiliary Table. This does not include
/// constraint degrees for the co-processors
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    2, 3, 4, // Selector flags must be binary.
];

/// The first selector column, used as a selector for the entire auxiliary table.
pub const S0_COL_IDX: usize = AUX_TRACE_OFFSET;
/// The second selector column, used as a selector for the bitwise, memory, and padding segments
/// after the hasher trace ends.
pub const S1_COL_IDX: usize = AUX_TRACE_OFFSET + 1;
/// The third selector column, used as a selector for the memory and padding segments after the
/// bitwise trace ends.
pub const S2_COL_IDX: usize = AUX_TRACE_OFFSET + 2;

/// The first column of the bitwise co-processor.
pub const BITWISE_TRACE_OFFSET: usize = S1_COL_IDX + 1;
/// The first column of the memory co-processor.
pub const MEMORY_TRACE_OFFSET: usize = S2_COL_IDX + 1;

// AUXILIARY TABLE TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the auxiliary table and all of its co-processors.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    let mut degrees: Vec<TransitionConstraintDegree> = CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect();

    degrees.append(&mut bitwise_pow2::get_transition_constraint_degrees());

    degrees.append(&mut memory::get_transition_constraint_degrees());

    degrees
}

/// Returns the number of transition constraints for the auxiliary table and all of its
/// co-processors.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
        + bitwise_pow2::get_transition_constraint_count()
        + memory::get_transition_constraint_count()
}

/// Enforces constraints for the auxiliary table and all of its co-processors.
pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
) {
    // auxiliary table transition constraints
    enforce_selectors(frame, result);

    let mut constraint_offset = NUM_CONSTRAINTS;
    // bitwise transition constraints
    bitwise_pow2::enforce_constraints(
        frame,
        periodic_values,
        &mut result[constraint_offset..],
        frame.bitwise_flag(),
    );
    constraint_offset += bitwise_pow2::get_transition_constraint_count();

    // memory transition constraints
    memory::enforce_constraints(frame, &mut result[constraint_offset..], frame.memory_flag());
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

/// Constraint evaluation function to enforce that the Auxiliary Table selector columns must be
/// binary during the portion of the trace when they're being used as selectors.
fn enforce_selectors<E: FieldElement>(frame: &EvaluationFrame<E>, result: &mut [E]) {
    // Selector flag s0 must be binary for the entire table.
    result[0] = is_binary(frame.s0());

    // Selector s1 is only used as a flag when s0 is set.
    result[1] = frame.s0() * is_binary(frame.s1());

    // Selector s2 is only used as a flag when both s0 and s1 are set.
    result[2] = frame.s0() * frame.s1() * is_binary(frame.s2());
}

// AUXILIARY TABLE FRAME EXTENSION TRAIT
// ================================================================================================

/// Trait to allow easy access to column values and intermediate variables used in constraint
/// calculations for the Auxiliary Table and its Hasher, Bitwise, and Memory co-processors.
trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    /// Current value of the S0 selector column.
    fn s0(&self) -> E;

    /// Current value of the S1 selector column.
    fn s1(&self) -> E;

    /// Current value of the S2 selector column.
    fn s2(&self) -> E;

    // --- Co-processor selector flags ------------------------------------------------------------

    /// Flag to indicate whether the frame is in the hasher portion of the Auxiliary Table trace.
    fn hasher_flag(&self) -> E;

    /// Flag to indicate whether the frame is in the bitwise portion of the Auxiliary Table trace.
    fn bitwise_flag(&self) -> E;

    /// Flag to indicate whether the frame is in the memory portion of the Auxiliary Table trace.
    fn memory_flag(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------
    fn s0(&self) -> E {
        self.current()[S0_COL_IDX]
    }
    fn s1(&self) -> E {
        self.current()[S1_COL_IDX]
    }
    fn s2(&self) -> E {
        self.current()[S2_COL_IDX]
    }

    // --- Co-processor selector flags ------------------------------------------------------------
    fn hasher_flag(&self) -> E {
        binary_not(self.s0())
    }
    fn bitwise_flag(&self) -> E {
        self.s0() * binary_not(self.s1())
    }
    fn memory_flag(&self) -> E {
        self.s0() * self.s1() * binary_not(self.s2())
    }
}
