use super::{EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree, Vec};
use crate::utils::{binary_not, is_binary};
use vm_core::AUX_TRACE_OFFSET;

mod bitwise_pow2;
mod hasher;
mod memory;

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
/// The first column of the hasher co-processor.
pub const HASHER_TRACE_OFFSET: usize = S0_COL_IDX + 1;
/// The first column of the memory co-processor.
pub const MEMORY_TRACE_OFFSET: usize = S2_COL_IDX + 1;

// PERIODIC COLUMNS
// ================================================================================================

/// Returns the set of periodic columns required by the co-processors in the Auxiliary Table.
pub fn get_periodic_column_values() -> Vec<Vec<Felt>> {
    let mut result = hasher::get_periodic_column_values();
    result.append(&mut bitwise_pow2::get_periodic_column_values());
    result
}

// AUXILIARY TABLE TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the auxiliary table and all of its co-processors.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    let mut degrees: Vec<TransitionConstraintDegree> = CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect();

    degrees.append(&mut hasher::get_transition_constraint_degrees());

    degrees.append(&mut bitwise_pow2::get_transition_constraint_degrees());

    degrees.append(&mut memory::get_transition_constraint_degrees());

    degrees
}

/// Returns the number of transition constraints for the auxiliary table and all of its
/// co-processors.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
        + hasher::get_transition_constraint_count()
        + bitwise_pow2::get_transition_constraint_count()
        + memory::get_transition_constraint_count()
}

/// Enforces constraints for the auxiliary table and all of its co-processors.
pub fn enforce_constraints<E: FieldElement<BaseField = Felt>>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
) {
    // auxiliary table transition constraints
    enforce_selectors(frame, result);
    let mut constraint_offset = NUM_CONSTRAINTS;

    // hasher transition constraints
    hasher::enforce_constraints(
        frame,
        &periodic_values[..hasher::NUM_PERIODIC_COLUMNS],
        &mut result[constraint_offset..],
        frame.hasher_flag(),
        binary_not(frame.s0_next()),
    );
    constraint_offset += hasher::get_transition_constraint_count();

    // bitwise transition constraints
    bitwise_pow2::enforce_constraints(
        frame,
        &periodic_values[hasher::NUM_PERIODIC_COLUMNS..],
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

    /// Value of the S0 selector column in the next row.
    fn s0_next(&self) -> E;

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

    #[inline(always)]
    fn s0(&self) -> E {
        self.current()[S0_COL_IDX]
    }
    #[inline(always)]
    fn s0_next(&self) -> E {
        self.next()[S0_COL_IDX]
    }
    #[inline(always)]
    fn s1(&self) -> E {
        self.current()[S1_COL_IDX]
    }
    #[inline(always)]
    fn s2(&self) -> E {
        self.current()[S2_COL_IDX]
    }

    // --- Co-processor selector flags ------------------------------------------------------------

    #[inline(always)]
    fn hasher_flag(&self) -> E {
        binary_not(self.s0())
    }
    #[inline(always)]
    fn bitwise_flag(&self) -> E {
        self.s0() * binary_not(self.s1())
    }
    #[inline(always)]
    fn memory_flag(&self) -> E {
        self.s0() * self.s1() * self.s2()
    }
}
