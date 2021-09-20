use super::{
    are_equal, binary_not, enforce_left_shift, enforce_stack_copy, is_binary, is_zero,
    EvaluationResult, FieldElement,
};

// CONSTANTS
// ================================================================================================

const POW2_IDX: usize = 0;
const X_BIT_IDX: usize = 1;
const Y_BIT_IDX: usize = 2;
const NOT_SET_IDX: usize = 3;
const GT_IDX: usize = 4;
const LT_IDX: usize = 5;
const Y_ACC_IDX: usize = 6;
const X_ACC_IDX: usize = 7;

// ASSERTIONS
// ================================================================================================

/// Enforces constraints for ASSERT operation. The constraints are similar to DROP operation, but
/// have an auxiliary constraint which enforces that 1 - x = 0, where x is the top of the stack.
pub fn enforce_assert<E: FieldElement>(
    result: &mut [E],
    aux: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    op_flag: E,
) {
    enforce_left_shift(result, old_stack, new_stack, 1, 1, op_flag);
    aux.agg_constraint(0, op_flag, are_equal(E::ONE, old_stack[0]));
}

/// Enforces constraints for ASSERTEQ operation. The stack is shifted by 2 registers the left and
/// an auxiliary constraint enforces that the first element of the stack is equal to the second.
pub fn enforce_asserteq<E: FieldElement>(
    result: &mut [E],
    aux: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    op_flag: E,
) {
    enforce_left_shift(result, old_stack, new_stack, 2, 2, op_flag);
    aux.agg_constraint(0, op_flag, are_equal(old_stack[0], old_stack[1]));
}

// EQUALITY
// ================================================================================================

/// Evaluates constraints for EQ operation. These enforce that when x == y, top of the stack at
/// the next step is set to 1, otherwise top of the stack at the next step is set to 0.
pub fn enforce_eq<E: FieldElement>(
    result: &mut [E],
    aux: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    op_flag: E,
) {
    // compute difference between top two values of the stack
    let x = old_stack[1];
    let y = old_stack[2];
    let diff = x - y;

    // when x == y, the first stack register contains inverse of the difference
    let inv_diff = old_stack[0];

    // the operation is defined as 1 - diff * inv(diff)
    let op_result = binary_not(diff * inv_diff);
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], op_result));

    // stack items beyond 3nd item are shifted the the left by 2
    enforce_left_shift(result, old_stack, new_stack, 3, 2, op_flag);

    // we also need to make sure that result * diff = 0; this ensures that when diff != 0
    // the result must be set to 0
    aux.agg_constraint(0, op_flag, new_stack[0] * diff);
}

// INEQUALITY
// ================================================================================================

/// Evaluates constraints for CMP operation.
pub fn enforce_cmp<E: FieldElement>(
    result: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    op_flag: E,
) {
    let two = E::ONE + E::ONE;

    // layout of first 8 registers
    // [pow, bit_a, bit_b, not_set, gt, lt, acc_b, acc_a]

    // x and y bits are binary
    let x_bit = new_stack[X_BIT_IDX];
    let y_bit = new_stack[Y_BIT_IDX];
    result.agg_constraint(0, op_flag, is_binary(x_bit));
    result.agg_constraint(1, op_flag, is_binary(y_bit));

    // comparison trackers were updated correctly
    let not_set = new_stack[NOT_SET_IDX];
    let bit_gt = x_bit * binary_not(y_bit);
    let bit_lt = y_bit * binary_not(x_bit);

    let gt = old_stack[GT_IDX] + bit_gt * not_set;
    let lt = old_stack[LT_IDX] + bit_lt * not_set;
    result.agg_constraint(2, op_flag, are_equal(new_stack[GT_IDX], gt));
    result.agg_constraint(3, op_flag, are_equal(new_stack[LT_IDX], lt));

    // binary representation accumulators were updated correctly
    let power_of_two = old_stack[POW2_IDX];
    let x_acc = old_stack[X_ACC_IDX] + x_bit * power_of_two;
    let y_acc = old_stack[Y_ACC_IDX] + y_bit * power_of_two;
    result.agg_constraint(4, op_flag, are_equal(new_stack[Y_ACC_IDX], y_acc));
    result.agg_constraint(5, op_flag, are_equal(new_stack[X_ACC_IDX], x_acc));

    // when GT or LT register is set to 1, not_set flag is cleared
    let not_set_check = binary_not(old_stack[LT_IDX]) * binary_not(old_stack[GT_IDX]);
    result.agg_constraint(6, op_flag, are_equal(not_set, not_set_check));

    // power of 2 register was updated correctly
    let power_of_two_constraint = are_equal(new_stack[POW2_IDX] * two, power_of_two);
    result.agg_constraint(7, op_flag, power_of_two_constraint);

    // registers beyond the 7th register were not affected
    enforce_stack_copy(result, old_stack, new_stack, 8, op_flag);
}

/// Evaluates constraints for BINACC operation.
pub fn enforce_binacc<E: FieldElement>(
    result: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    op_flag: E,
) {
    let two = E::ONE + E::ONE;

    // layout of first 4 registers:
    // [value bit, 0, power of two, accumulated value]
    // value bit is located in the next state (not current state)

    // the bit was a binary value
    let bit = new_stack[0];
    result.agg_constraint(0, op_flag, is_binary(bit));

    // register after bit register was empty
    result.agg_constraint(1, op_flag, is_zero(new_stack[1]));

    // power of 2 register was updated correctly
    let power_of_two = old_stack[2];
    let power_of_two_constraint = are_equal(new_stack[2], power_of_two * two);
    result.agg_constraint(2, op_flag, power_of_two_constraint);

    // binary representation accumulator was updated correctly
    let acc = old_stack[3] + bit * power_of_two;
    result.agg_constraint(3, op_flag, are_equal(new_stack[3], acc));

    // registers beyond 2nd register remained the same
    enforce_stack_copy(result, old_stack, new_stack, 4, op_flag);
}
