use super::{EvaluationFrame, FieldElement, Vec, BITWISE_TRACE_OFFSET, OP_CYCLE_LEN};
use crate::utils::{binary_not, is_binary, EvaluationResult};
use core::ops::Range;
use vm_core::{
    bitwise::{
        BITWISE_A_COL_IDX, BITWISE_B_COL_IDX, BITWISE_NUM_DECOMP_BITS as NUM_DECOMP_BITS,
        BITWISE_OUTPUT_COL_IDX, NUM_SELECTORS,
    },
    utils::range as create_range,
};
use winter_air::TransitionConstraintDegree;

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ================================================================================================

/// The number of transition constraints on the bitwise co-processor.
pub const NUM_CONSTRAINTS: usize = 14;
/// The degrees of constraints on the bitwise co-processor. The degree of all
/// constraints is increased by 4 due to the co-processor selector flag from the auxiliary table
/// (degree 2) and the selector flag specifying the bitwise operation (degree 2).
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    6, 6, 6, 6, 6, 6, 6, 6, // Input decomposition values should be binary.
    6, 6, // Enforce correct initial values of a and b columns.
    6, 6, // Enforce correct aggregation of a and b columns during transitions.
    7, 7, // Ensure correct output aggregation
];

/// Index of CONSTRAINT_DEGREES array after which all constraints use periodic columns.
const PERIODIC_CONSTRAINTS_START: usize = 8;

/// The range of the selector columns in the trace.
const SELECTOR_COL_RANGE: Range<usize> = create_range(BITWISE_TRACE_OFFSET, NUM_SELECTORS);
/// The index of the column holding the aggregated value of input `a`.
const A_COL_IDX: usize = BITWISE_TRACE_OFFSET + BITWISE_A_COL_IDX;
/// The index of the column holding the aggregated value of input `b`.
const B_COL_IDX: usize = BITWISE_TRACE_OFFSET + BITWISE_B_COL_IDX;
/// The index range for the bit decomposition of `a`.
const A_COL_RANGE: Range<usize> = create_range(B_COL_IDX + 1, NUM_DECOMP_BITS);
/// The index range for the bit decomposition of `b`.
const B_COL_RANGE: Range<usize> = create_range(A_COL_RANGE.end, NUM_DECOMP_BITS);
/// The index of the column containing the aggregated output value.
const OUTPUT_COL_IDX: usize = BITWISE_TRACE_OFFSET + BITWISE_OUTPUT_COL_IDX;

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
            .map(|&degree| TransitionConstraintDegree::with_cycles(degree - 1, vec![OP_CYCLE_LEN]))
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

/// Enforces correct decomposition of the input values `a` and `b` in each row. This requires the
/// following constraints:
/// - All values in decomposition columns must be binary.
/// - In the first row, the values in `a` and `b` must be the aggregation of their respective bit
///   columns.
/// - For every row except the last, the aggregated input value in the next row must be 16 times the
///   the value in the current row plus the aggregation of the bit decomposition in the next row.
fn enforce_input_decomposition<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let mut constraint_offset = 0;
    // Flag that enforces these constraints when this co-processor segment is selected in the
    // auxiliary table and the co-processor's selectors specify a bitwise operation.
    let bitwise_op_flag = processor_flag * frame.bitwise_op_flag();

    // Values in bit decomposition columns a0..a3 should be binary.
    for (idx, result) in result.iter_mut().take(NUM_DECOMP_BITS).enumerate() {
        *result = bitwise_op_flag * is_binary(frame.a_bit(idx));
    }
    constraint_offset += NUM_DECOMP_BITS;

    // Values in bit decomposition columns b0..b3 should be binary.
    for (idx, result) in result[constraint_offset..]
        .iter_mut()
        .take(NUM_DECOMP_BITS)
        .enumerate()
    {
        *result = bitwise_op_flag * is_binary(frame.b_bit(idx));
    }
    constraint_offset += NUM_DECOMP_BITS;

    // The values in column a in the first row should be the aggregation of the decomposed bit
    // columns a0..a3.
    let first_row_flag = bitwise_op_flag * periodic_values[0];
    result[constraint_offset] = first_row_flag * (frame.a() - frame.a_agg_bits());
    constraint_offset += 1;

    // The values in column b in the first row should be the aggregation of the decomposed bit
    // columns b0..b3.
    result[constraint_offset] = first_row_flag * (frame.b() - frame.b_agg_bits());
    constraint_offset += 1;

    // During a transition between rows, the next value in the a column should be 16 times the
    // previous value plus the aggregation of the next row's bit values.
    let transition_flag = bitwise_op_flag * periodic_values[1];
    result[constraint_offset] =
        transition_flag * (frame.a_next() - (E::from(16_u8) * frame.a() + frame.a_agg_bits_next()));
    constraint_offset += 1;

    // During a transition between rows, the next value in the b column should be 16 times the
    // previous value plus the aggregation of the next row's bit values.
    result[constraint_offset] =
        transition_flag * (frame.b_next() - (E::from(16_u8) * frame.b() + frame.b_agg_bits_next()));
    constraint_offset += 1;

    constraint_offset
}

/// Enforces correct output aggregation for the operation. This requires the following 2 constraints
/// for each operation:
/// - In the first row, the output value must equal the aggregated result of the operation applied
///   to the row.
/// - For every row except the last, the output value in the next row must equal the output in the
///   current row plus the aggregated result of the specified operation applied to the next row.
///
/// Because the selectors for the AND, OR, and XOR operations are mutually exclusive, the
/// constraints for different operations can be aggregated into the same result indices.
fn enforce_output_aggregation<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    periodic_values: &[E],
    result: &mut [E],
    processor_flag: E,
) -> usize {
    let mut constraint_offset = 0;
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
        constraint_offset,
        k0_flag * bitwise_and_flag,
        frame.output() - bitwise_and(frame.bit_decomp()),
    );
    result.agg_constraint(
        constraint_offset,
        k0_flag * bitwise_or_flag,
        frame.output() - bitwise_or(frame.bit_decomp()),
    );
    result.agg_constraint(
        constraint_offset,
        k0_flag * bitwise_xor_flag,
        frame.output() - bitwise_xor(frame.bit_decomp()),
    );
    constraint_offset += 1;

    // During a transition between rows, the next value in the output column should be 16 times the
    // previous value plus the aggregation of the next row's operation output.
    let shifted_output = frame.output() * E::from(16_u8);
    result.agg_constraint(
        constraint_offset,
        k1_flag * bitwise_and_flag,
        frame.output_next() - (shifted_output + bitwise_and(frame.bit_decomp_next())),
    );
    result.agg_constraint(
        constraint_offset,
        k1_flag * bitwise_or_flag,
        frame.output_next() - (shifted_output + bitwise_or(frame.bit_decomp_next())),
    );
    result.agg_constraint(
        constraint_offset,
        k1_flag * bitwise_xor_flag,
        frame.output_next() - (shifted_output + bitwise_xor(frame.bit_decomp_next())),
    );
    constraint_offset += 1;

    constraint_offset
}

/// Calculates the result of bitwise AND applied to the decomposed values provided as a bit array.
/// The result will be the AND of the first 4 bits in the provided array with the latter 4 bits.
pub fn bitwise_and<E: FieldElement>(decomposed_values: &[E]) -> E {
    let mut result = E::ZERO;
    // Aggregate the result of the bitwise AND over the decomposed bits in the row.
    for idx in 0..NUM_DECOMP_BITS {
        let a = decomposed_values[idx];
        let b = decomposed_values[idx + NUM_DECOMP_BITS];
        result += E::from(2_u64.pow(idx as u32)) * a * b
    }
    result
}

/// Calculates the result of bitwise OR applied to the decomposed values provided as a bit array.
/// The result will be the OR of the first 4 bits in the provided array with the latter 4 bits.
pub fn bitwise_or<E: FieldElement>(decomposed_values: &[E]) -> E {
    let mut result = E::ZERO;
    // Aggregate the result of the bitwise OR over the decomposed bits in the row.
    for idx in 0..NUM_DECOMP_BITS {
        let a = decomposed_values[idx];
        let b = decomposed_values[idx + NUM_DECOMP_BITS];
        result += E::from(2_u64.pow(idx as u32)) * (a + b - a * b)
    }
    result
}

/// Calculates the result of bitwise XOR applied to the decomposed values provided as a bit array.
/// The result will be the XOR of the first 4 bits in the provided array with the latter 4 bits.
pub fn bitwise_xor<E: FieldElement>(decomposed_values: &[E]) -> E {
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

    /// Gets the current value of the specified selector column.
    fn selector(&self, index: usize) -> E;
    /// Gets the current value of the aggregated `a` input.
    fn a(&self) -> E;
    /// Gets the value of the aggregated `a` input in the next row.
    fn a_next(&self) -> E;
    /// Gets the value of the decomposed bit of `a` at the specified index in the current row.
    fn a_bit(&self, index: usize) -> E;
    /// Gets the current value of the aggregated `b` input.
    fn b(&self) -> E;
    /// Gets the value of the aggregated `b` input in the next row.
    fn b_next(&self) -> E;
    /// Gets the value of the decomposed bit of `b` at the specified index in the current row.
    fn b_bit(&self, index: usize) -> E;
    /// Gets the entire range of decomposed input values for `a` and `b` in the current row.
    fn bit_decomp(&self) -> &[E];
    /// Gets the entire range of decomposed input values for `a` and `b` in the next row.
    fn bit_decomp_next(&self) -> &[E];
    /// Gets the value of the aggregated output in the current row.
    fn output(&self) -> E;
    /// Gets the value of the aggregated output in the next row.
    fn output_next(&self) -> E;

    // --- Intermediate variables & helpers -------------------------------------------------------
    /// The aggregated value of the decomposed bits from `a` in the current row.
    fn a_agg_bits(&self) -> E;
    /// The aggregated value of the decomposed bits from `a` in the next row.
    fn a_agg_bits_next(&self) -> E;
    /// The aggregated value of the decomposed bits from `b` in the current row.
    fn b_agg_bits(&self) -> E;
    /// The aggregated value of the decomposed bits from `b` in the next row.
    fn b_agg_bits_next(&self) -> E;

    // --- Flags ----------------------------------------------------------------------------------

    /// A selector flag that specifies the operation is any one of bitwise AND, OR or XOR.
    fn bitwise_op_flag(&self) -> E;
    /// The selector flag for the bitwise AND operation.
    fn bitwise_and_flag(&self) -> E;
    /// The selector flag for the bitwise OR operation.
    fn bitwise_or_flag(&self) -> E;
    /// The selector flag for the bitwise XOR operation.
    fn bitwise_xor_flag(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    #[inline(always)]
    fn selector(&self, index: usize) -> E {
        self.current()[SELECTOR_COL_RANGE.start + index]
    }
    #[inline(always)]
    fn a(&self) -> E {
        self.current()[A_COL_IDX]
    }
    #[inline(always)]
    fn a_next(&self) -> E {
        self.next()[A_COL_IDX]
    }
    #[inline(always)]
    fn a_bit(&self, index: usize) -> E {
        self.current()[A_COL_RANGE.start + index]
    }
    #[inline(always)]
    fn b(&self) -> E {
        self.current()[B_COL_IDX]
    }
    #[inline(always)]
    fn b_next(&self) -> E {
        self.next()[B_COL_IDX]
    }
    #[inline(always)]
    fn b_bit(&self, index: usize) -> E {
        self.current()[B_COL_RANGE.start + index]
    }
    #[inline(always)]
    fn bit_decomp(&self) -> &[E] {
        &self.current()[A_COL_RANGE.start..B_COL_RANGE.end]
    }
    #[inline(always)]
    fn bit_decomp_next(&self) -> &[E] {
        &self.next()[A_COL_RANGE.start..B_COL_RANGE.end]
    }
    #[inline(always)]
    fn output(&self) -> E {
        self.current()[OUTPUT_COL_IDX]
    }
    #[inline(always)]
    fn output_next(&self) -> E {
        self.next()[OUTPUT_COL_IDX]
    }

    // --- Intermediate variables & helpers -------------------------------------------------------
    #[inline(always)]
    fn a_agg_bits(&self) -> E {
        agg_bits(self.current(), A_COL_RANGE.start)
    }
    #[inline(always)]
    fn a_agg_bits_next(&self) -> E {
        agg_bits(self.next(), A_COL_RANGE.start)
    }
    #[inline(always)]
    fn b_agg_bits(&self) -> E {
        agg_bits(self.current(), B_COL_RANGE.start)
    }
    #[inline(always)]
    fn b_agg_bits_next(&self) -> E {
        agg_bits(self.next(), B_COL_RANGE.start)
    }

    // --- Flags ----------------------------------------------------------------------------------

    #[inline(always)]
    fn bitwise_op_flag(&self) -> E {
        binary_not(self.selector(0) * self.selector(1))
    }
    #[inline(always)]
    fn bitwise_and_flag(&self) -> E {
        binary_not(self.selector(0)) * binary_not(self.selector(1))
    }
    #[inline(always)]
    fn bitwise_or_flag(&self) -> E {
        binary_not(self.selector(0)) * self.selector(1)
    }
    #[inline(always)]
    fn bitwise_xor_flag(&self) -> E {
        self.selector(0) * binary_not(self.selector(1))
    }
}

// HELPER FUNCTIONS
// ================================================================================================
/// Aggregate 4 decomposed bits representing a 4-bit binary value into a decimal value, starting
/// from `start_idx` in the provided row.
pub fn agg_bits<E: FieldElement>(row: &[E], start_idx: usize) -> E {
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
