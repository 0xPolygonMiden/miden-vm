use super::{EvaluationFrame, FieldElement, BITWISE_TRACE_OFFSET};
use crate::utils::{binary_not, is_binary, EvaluationResult};
use core::ops::Range;
use vm_core::{bitwise::NUM_SELECTORS, utils::range as create_range};
use winter_air::TransitionConstraintDegree;

// CONSTANTS
// ================================================================================================

/// The number of transition constraints on the bitwise co-processor.
pub const NUM_CONSTRAINTS: usize = 8;
/// The degrees of constraints on the bitwise co-processor.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    4, 4, // Input decomposition values should be binary.
    4, 4, // Enforce correct initial values of a and b columns.
    4, 4, // Enforce correct aggregation of a and b columns during transitions.
    7, 7, // Ensure correct output aggregation
];

/// Index of CONSTRAINT_DEGREES array after which all constraints use periodic columns.
pub const PERIODIC_CONSTRAINTS_START: usize = 2;
/// The number of bits decomposed per row.
const NUM_DECOMP_BITS: usize = 4;

const SELECTOR_COL_RANGE: Range<usize> = create_range(BITWISE_TRACE_OFFSET, NUM_SELECTORS);
const A_COL_IDX: usize = SELECTOR_COL_RANGE.end;
const B_COL_IDX: usize = A_COL_IDX + 1;
const A_COL_RANGE: Range<usize> = create_range(B_COL_IDX + 1, NUM_DECOMP_BITS);
const B_COL_RANGE: Range<usize> = create_range(A_COL_RANGE.end, NUM_DECOMP_BITS);
const OUTPUT_COL_IDX: usize = B_COL_RANGE.end;

// BITWISE TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the bitwise co-processor.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    // Add the degrees of non-periodic constraints.
    let mut degrees: Vec<TransitionConstraintDegree> = CONSTRAINT_DEGREES
        [..PERIODIC_CONSTRAINTS_START]
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect();

    // Add the degrees of periodic constraints.
    degrees.append(
        &mut CONSTRAINT_DEGREES[PERIODIC_CONSTRAINTS_START..]
            .iter()
            .map(|&degree| TransitionConstraintDegree::with_cycles(degree - 1, vec![8]))
            .collect(),
    );

    degrees
}

/// Returns the number of transition constraints for the bitwise co-processor.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
}

/// Enforces constraints for the bitwise co-processor, which includes the constraints for bitwise
/// operations.
pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    bitwise_flag: E,
) {
    // Enforce correct decomposition of the input values into the a and b columns.
    let index = enforce_input_decomposition(frame, periodic_values, result, bitwise_flag);

    // Enforce that the operation result is aggregated into the output column correctly.
    enforce_output_aggregation(frame, periodic_values, &mut result[index..], bitwise_flag);
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================
fn enforce_input_decomposition<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let constraint_count = 6;
    // Flag that enforces these constraints when this co-processor segment is selected in the
    // auxiliary table and the co-processor's selectors specify a bitwise operation.
    let bitwise_op_flag = processor_flag * frame.bitwise_op_flag();

    // Values in bit decomposition columns a0..a3 and b0..b3 should be binary.
    for idx in 0..NUM_DECOMP_BITS {
        result.agg_constraint(0, bitwise_op_flag, is_binary(frame.a_bit(idx)));
        result.agg_constraint(1, bitwise_op_flag, is_binary(frame.b_bit(idx)));
    }

    // The values in columns a and b in the first row should be the aggregation of the decomposed
    // bit columns a0..a3 and b0..b3.
    let first_row_flag = bitwise_op_flag * periodic_values[0];
    result.agg_constraint(2, first_row_flag, frame.a() - frame.a_agg_bits());
    result.agg_constraint(3, first_row_flag, frame.b() - frame.b_agg_bits());

    // During a transition between rows, the next value in the a or b columns should be 16 times the
    // previous value plus the aggregation of the next row's bit values.
    let transition_flag = bitwise_op_flag * periodic_values[1];
    result.agg_constraint(
        4,
        transition_flag,
        frame.a_next() - (E::from(16_u8) * frame.a() + frame.a_agg_bits_next()),
    );
    result.agg_constraint(
        5,
        transition_flag,
        frame.b_next() - (E::from(16_u8) * frame.b() + frame.b_agg_bits_next()),
    );

    constraint_count
}

fn enforce_output_aggregation<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let constraint_count = 2;
    // Periodic column flags
    let k0_flag = periodic_values[0];
    let k1_flag = periodic_values[1];
    // Operator flags
    let bitwise_and_flag = processor_flag * frame.bitwise_and_flag();
    let bitwise_or_flag = processor_flag * frame.bitwise_or_flag();
    let bitwise_xor_flag = processor_flag * frame.bitwise_xor_flag();

    // The value in the output column in the first row must be exactly equal to the the aggregated
    // value of the operation applied to the bitwise decomposition columns a0..a3 and b0..b3.
    result.agg_constraint(
        0,
        k0_flag * bitwise_and_flag,
        frame.output() - bitwise_and(frame.bit_decomp()),
    );
    result.agg_constraint(
        0,
        k0_flag * bitwise_or_flag,
        frame.output() - bitwise_or(frame.bit_decomp()),
    );
    result.agg_constraint(
        0,
        k0_flag * bitwise_xor_flag,
        frame.output() - bitwise_xor(frame.bit_decomp()),
    );

    // During a transition between rows, the next value in the output column should be 16 times the
    // previous value plus the aggregation of the nevt row's operation output.
    let shifted_output = frame.output() * E::from(16_u8);
    result.agg_constraint(
        1,
        k1_flag * bitwise_and_flag,
        frame.output_next() - (shifted_output + bitwise_and(frame.bit_decomp_next())),
    );
    result.agg_constraint(
        1,
        k1_flag * bitwise_or_flag,
        frame.output_next() - (shifted_output + bitwise_or(frame.bit_decomp_next())),
    );
    result.agg_constraint(
        1,
        k1_flag * bitwise_xor_flag,
        frame.output_next() - (shifted_output + bitwise_xor(frame.bit_decomp_next())),
    );

    constraint_count
}

fn bitwise_and<E: FieldElement>(decomposed_values: &[E]) -> E {
    let mut result = E::ZERO;
    // Aggregate the result of the bitwise AND over the decomposed bits in the row.
    for idx in 0..NUM_DECOMP_BITS {
        let a = decomposed_values[idx];
        let b = decomposed_values[idx + NUM_DECOMP_BITS];
        result += E::from(2_u64.pow(idx as u32)) * a * b
    }
    result
}

fn bitwise_or<E: FieldElement>(decomposed_values: &[E]) -> E {
    let mut result = E::ZERO;
    // Aggregate the result of the bitwise OR over the decomposed bits in the row.
    for idx in 0..NUM_DECOMP_BITS {
        let a = decomposed_values[idx];
        let b = decomposed_values[idx + NUM_DECOMP_BITS];
        result += E::from(2_u64.pow(idx as u32)) * (a + b - a * b)
    }
    result
}
fn bitwise_xor<E: FieldElement>(decomposed_values: &[E]) -> E {
    let mut result = E::ZERO;
    // Aggregate the result of the bitwise XOR over the decomposed bits in the row.
    for idx in 0..NUM_DECOMP_BITS {
        let a = decomposed_values[idx];
        let b = decomposed_values[idx + NUM_DECOMP_BITS];
        result += E::from(2_u64.pow(idx as u32)) * (a + b - E::from(2_u8) * a * b)
    }
    result
}

// BITWISE FRAME EXTENSION TRAIT
// ================================================================================================
trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    fn selector(&self, index: usize) -> E;
    fn a(&self) -> E;
    fn a_next(&self) -> E;
    fn a_bit(&self, index: usize) -> E;
    fn b(&self) -> E;
    fn b_next(&self) -> E;
    fn b_bit(&self, index: usize) -> E;
    fn bit_decomp(&self) -> &[E];
    fn bit_decomp_next(&self) -> &[E];
    fn output(&self) -> E;
    fn output_next(&self) -> E;

    // --- Intermediate variables & helpers -------------------------------------------------------
    fn a_agg_bits(&self) -> E;
    fn a_agg_bits_next(&self) -> E;
    fn b_agg_bits(&self) -> E;
    fn b_agg_bits_next(&self) -> E;

    // --- Flags ----------------------------------------------------------------------------------

    fn bitwise_op_flag(&self) -> E;
    fn bitwise_and_flag(&self) -> E;
    fn bitwise_or_flag(&self) -> E;
    fn bitwise_xor_flag(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    fn selector(&self, index: usize) -> E {
        self.current()[SELECTOR_COL_RANGE.start + index]
    }
    fn a(&self) -> E {
        self.current()[A_COL_IDX]
    }
    fn a_next(&self) -> E {
        self.next()[A_COL_IDX]
    }
    fn a_bit(&self, index: usize) -> E {
        self.current()[A_COL_RANGE.start + index]
    }
    fn b(&self) -> E {
        self.current()[B_COL_IDX]
    }
    fn b_next(&self) -> E {
        self.next()[B_COL_IDX]
    }
    fn b_bit(&self, index: usize) -> E {
        self.current()[B_COL_RANGE.start + index]
    }
    fn bit_decomp(&self) -> &[E] {
        &self.current()[A_COL_RANGE.start..B_COL_RANGE.end]
    }
    fn bit_decomp_next(&self) -> &[E] {
        &self.next()[A_COL_RANGE.start..B_COL_RANGE.end]
    }
    fn output(&self) -> E {
        self.current()[OUTPUT_COL_IDX]
    }
    fn output_next(&self) -> E {
        self.next()[OUTPUT_COL_IDX]
    }

    // --- Intermediate variables & helpers -------------------------------------------------------
    fn a_agg_bits(&self) -> E {
        agg_bits(self.current(), A_COL_RANGE.start)
    }
    fn a_agg_bits_next(&self) -> E {
        agg_bits(self.next(), A_COL_RANGE.start)
    }
    fn b_agg_bits(&self) -> E {
        agg_bits(self.current(), B_COL_RANGE.start)
    }
    fn b_agg_bits_next(&self) -> E {
        agg_bits(self.next(), B_COL_RANGE.start)
    }

    // --- Flags ----------------------------------------------------------------------------------

    fn bitwise_op_flag(&self) -> E {
        binary_not(self.selector(0) * self.selector(1))
    }
    fn bitwise_and_flag(&self) -> E {
        binary_not(self.selector(0)) * binary_not(self.selector(1))
    }
    fn bitwise_or_flag(&self) -> E {
        binary_not(self.selector(0)) * self.selector(1)
    }
    fn bitwise_xor_flag(&self) -> E {
        self.selector(0) * binary_not(self.selector(1))
    }
}

// HELPER FUNCTIONS
// ================================================================================================
fn agg_bits<E: FieldElement>(row: &[E], start_idx: usize) -> E {
    let mut result = E::ZERO;
    // TODO: this can be optimized.
    // From Bobbin: "we are multiplying by a small power of two and then summing up the results -
    // thus, in theory, we could just aggregate results in a 128-bit integer and perform only a
    // single reduction in the end. This works only when we are in the base field."
    for bit_idx in 0..NUM_DECOMP_BITS {
        result += E::from(2_u64.pow(bit_idx as u32)) * row[start_idx + bit_idx];
    }
    result
}
