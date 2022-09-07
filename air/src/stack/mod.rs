use vm_core::{
    utils::collections::Vec, ProgramOutputs, StarkField, FMP_COL_IDX, STACK_AUX_TRACE_OFFSET,
};

use super::{
    Assertion, AuxTraceRandElements, EvaluationFrame, Felt, FieldElement,
    TransitionConstraintDegree, MIN_STACK_DEPTH, STACK_TRACE_OFFSET,
};

mod field_ops;
pub mod op_flags;
mod stack_manipulation;
mod system_ops;

// CONSTANTS
// ================================================================================================

const B0_COL_IDX: usize = STACK_TRACE_OFFSET + MIN_STACK_DEPTH;
const B1_COL_IDX: usize = B0_COL_IDX + 1;

// --- Main constraints ---------------------------------------------------------------------------

/// The number of boundary constraints required by the Stack, which is all stack positions for
/// inputs and outputs as well as the initial values of the bookkeeping columns.
pub const NUM_ASSERTIONS: usize = 2 * MIN_STACK_DEPTH + 2;

// --- Auxiliary column constraints ---------------------------------------------------------------

/// The number of auxiliary assertions.
pub const NUM_AUX_ASSERTIONS: usize = 2;

// STACK OPERATIONS TRANSITION CONSTRAINTS
// ================================================================================================

/// Build the transition constraint degrees for the stack module and all the stack operations.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    // system operations contraints degrees.
    let mut degrees = system_ops::get_transition_constraint_degrees();
    // field operations contraints degrees.
    degrees.append(&mut field_ops::get_transition_constraint_degrees());
    // stack manipulation operations contraints degrees.
    degrees.append(&mut stack_manipulation::get_transition_constraint_degrees());

    degrees
}

/// Returns the number of transition constraints for the stack operations.
pub fn get_transition_constraint_count() -> usize {
    system_ops::get_transition_constraint_count()
        + field_ops::get_transition_constraint_count()
        + stack_manipulation::get_transition_constraint_count()
}

/// Enforces constraints for the stack module and all stack operations.
pub fn enforce_constraints<E: FieldElement<BaseField = Felt>>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
) -> usize {
    let mut index = 0;

    let op_flag = op_flags::OpFlags::new(frame);

    // Enforces stack operations unique constraints.
    index += enforce_unique_constraints(frame, result, &op_flag);

    index
}

/// Enforces unique constraints for all the stack ops.
pub fn enforce_unique_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    op_flag: &op_flags::OpFlags<E>,
) -> usize {
    let mut constraint_offset = 0;

    // system operations transition constraints.
    system_ops::enforce_constraints(frame, result, op_flag);

    constraint_offset += system_ops::get_transition_constraint_count();

    // field operations transition constraints.
    field_ops::enforce_constraints(frame, &mut result[constraint_offset..], op_flag);

    constraint_offset += field_ops::get_transition_constraint_count();

    // stack manipulation operations transition constraints.
    stack_manipulation::enforce_constraints(frame, &mut result[constraint_offset..], op_flag);

    constraint_offset += stack_manipulation::get_transition_constraint_count();

    constraint_offset
}

// BOUNDARY CONSTRAINTS
// ================================================================================================

// --- MAIN TRACE ---------------------------------------------------------------------------------

/// Returns the stack's boundary assertions for the main trace at the first step.
pub fn get_assertions_first_step(result: &mut Vec<Assertion<Felt>>, stack_inputs: &[Felt]) {
    // stack columns at the first step should be set to stack inputs, excluding overflow inputs.
    for (i, &value) in stack_inputs.iter().take(MIN_STACK_DEPTH).enumerate() {
        result.push(Assertion::single(STACK_TRACE_OFFSET + i, 0, value));
    }

    // if there are remaining slots on top of the stack without specified values, set them to ZERO.
    for i in stack_inputs.len()..MIN_STACK_DEPTH {
        result.push(Assertion::single(STACK_TRACE_OFFSET + i, 0, Felt::ZERO));
    }

    // get the initial values for the bookkeeping columns.
    let mut depth = MIN_STACK_DEPTH;
    let mut overflow_addr = Felt::ZERO;
    if stack_inputs.len() > MIN_STACK_DEPTH {
        depth = stack_inputs.len();
        overflow_addr = Felt::new(Felt::MODULUS - 1);
    }

    // b0 should be initialized to the depth of the stack.
    result.push(Assertion::single(B0_COL_IDX, 0, Felt::new(depth as u64)));

    // b1 should be initialized to the address of the last row in the overflow table, which is 0
    // when the overflow table is empty and -1 mod p when the stack is initialized with overflow.
    result.push(Assertion::single(B1_COL_IDX, 0, overflow_addr));
}

/// Returns the stack's boundary assertions for the main trace at the last step.
pub fn get_assertions_last_step(
    result: &mut Vec<Assertion<Felt>>,
    step: usize,
    outputs: &ProgramOutputs,
) {
    // stack columns at the last step should be set to stack outputs, excluding overflow outputs
    for (i, value) in outputs.stack_top().iter().enumerate() {
        result.push(Assertion::single(STACK_TRACE_OFFSET + i, step, *value));
    }
}

// --- AUXILIARY COLUMNS --------------------------------------------------------------------------

/// Returns the stack's boundary assertions for auxiliary columns at the first step.
pub fn get_aux_assertions_first_step<E: FieldElement>(
    result: &mut Vec<Assertion<E>>,
    alphas: &AuxTraceRandElements<E>,
    stack_inputs: &[Felt],
) where
    E: FieldElement<BaseField = Felt>,
{
    let step = 0;
    let value = if stack_inputs.len() > MIN_STACK_DEPTH {
        get_overflow_table_init(
            alphas.get_segment_elements(0),
            &stack_inputs[MIN_STACK_DEPTH..],
        )
    } else {
        E::ONE
    };

    result.push(Assertion::single(STACK_AUX_TRACE_OFFSET, step, value));
}

/// Returns the stack's boundary assertions for auxiliary columns at the last step.
pub fn get_aux_assertions_last_step<E: FieldElement>(
    result: &mut Vec<Assertion<E>>,
    alphas: &AuxTraceRandElements<E>,
    outputs: &ProgramOutputs,
    step: usize,
) where
    E: FieldElement<BaseField = Felt>,
{
    let value = if outputs.has_overflow() {
        get_overflow_table_final(alphas.get_segment_elements(0), outputs)
    } else {
        E::ONE
    };

    result.push(Assertion::single(STACK_AUX_TRACE_OFFSET, step, value));
}

// BOUNDARY CONSTRAINT HELPERS
// ================================================================================================

// --- AUX TRACE ----------------------------------------------------------------------------------

/// Gets the initial value of the overflow table auxiliary column from the provided sets of initial
/// values and random elements.
fn get_overflow_table_init<E: FieldElement>(alphas: &[E], init_values: &[Felt]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut value = E::ONE;
    let mut prev_clk = Felt::ZERO;
    let mut clk = Felt::from(Felt::MODULUS - init_values.len() as u64);

    // the values are in the overflow table in reverse order, since the deepest stack
    // value is added to the overflow table first.
    for &input in init_values.iter().rev() {
        value *= alphas[0]
            + alphas[1].mul_base(clk)
            + alphas[2].mul_base(input)
            + alphas[3].mul_base(prev_clk);
        prev_clk = clk;
        clk += Felt::ONE;
    }

    value
}

/// Gets the final value of the overflow table auxiliary column from the provided program outputs
/// and random elements.
fn get_overflow_table_final<E: FieldElement>(alphas: &[E], outputs: &ProgramOutputs) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut value = E::ONE;

    // When the overflow table is non-empty, we expect at least 2 addresses (the `prev` value of
    // the first row and the address value(s) of the row(s)) and more than MIN_STACK_DEPTH
    // elements in the stack.
    let mut prev = outputs.overflow_prev();
    for (clk, val) in outputs.stack_overflow() {
        value *= alphas[0]
            + alphas[1].mul_base(clk)
            + alphas[2].mul_base(val)
            + alphas[3].mul_base(prev);

        prev = clk;
    }

    value
}

// STACK OPERATION EXTENSION TRAIT
// ================================================================================================

trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    /// Returns the current value at the specified index in the stack.
    fn stack_item(&self, index: usize) -> E;
    /// Returns the next value at the specified index in the stack.
    fn stack_item_next(&self, index: usize) -> E;
    /// Gets the current element of the fmp register in the trace.
    fn fmp(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    #[inline(always)]
    fn stack_item(&self, index: usize) -> E {
        debug_assert!(index < 16, "stack index cannot exceed 15");
        self.current()[STACK_TRACE_OFFSET + index]
    }
    #[inline(always)]
    fn stack_item_next(&self, index: usize) -> E {
        debug_assert!(index < 16, "stack index cannot exceed 15");
        self.next()[STACK_TRACE_OFFSET + index]
    }
    #[inline(always)]
    fn fmp(&self) -> E {
        self.current()[FMP_COL_IDX]
    }
}
