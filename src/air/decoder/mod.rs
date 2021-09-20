use super::{
    utils::{
        are_equal, binary_not, enforce_left_shift, enforce_right_shift, enforce_stack_copy,
        is_binary, is_zero, EvaluationResult,
    },
    TransitionConstraintDegree,
};
use crate::{
    air::{TraceState, VmTransition},
    processor::opcodes::{FlowOps, UserOps},
    utils::sponge::ARK,
    BASE_CYCLE_LENGTH, MIN_CONTEXT_DEPTH, MIN_LOOP_DEPTH, SPONGE_WIDTH,
};

use winterfell::math::{fields::f128::BaseElement, FieldElement};

mod op_bits;
use op_bits::enforce_op_bits;

mod op_sponge;
use op_sponge::enforce_hacc;

mod flow_ops;
use flow_ops::{
    enforce_begin, enforce_break, enforce_fend, enforce_loop, enforce_tend, enforce_void,
    enforce_wrap,
};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================
const NUM_OP_CONSTRAINTS: usize = 15;
const OP_CONSTRAINT_DEGREES: [usize; NUM_OP_CONSTRAINTS] = [
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // all op bits are binary
    3, // op_counter should be incremented for HACC operations
    8, // ld_ops and hd_ops cannot be all 0s
    8, // when cf_ops are not all 0s, ld_ops and hd_ops must be all 1s
    6, // VOID can be followed only by VOID
    4, // operations happen on allowed step multiples
];

const NUM_SPONGE_CONSTRAINTS: usize = 4;
const SPONGE_CONSTRAINT_DEGREES: [usize; NUM_SPONGE_CONSTRAINTS] = [
    6, 7, 6, 6, // sponge transition constraints
];

const LOOP_IMAGE_CONSTRAINT_DEGREE: usize = 4;
const STACK_CONSTRAINT_DEGREE: usize = 4;

const CYCLE_MASK_IDX: usize = 0;
const PREFIX_MASK_IDX: usize = 1;
const PUSH_MASK_IDX: usize = 2;

pub const NUM_STATIC_DECODER_CONSTRAINTS: usize = NUM_OP_CONSTRAINTS + NUM_SPONGE_CONSTRAINTS + 1; // for loop image constraint

// CONSTRAINT DEGREES
// ================================================================================================

pub fn get_transition_constraint_degrees(
    _ctx_depth: usize,
    _loop_depth: usize,
) -> Vec<TransitionConstraintDegree> {
    unimplemented!()
}

// CONSTRAINT EVALUATOR
// ================================================================================================

pub fn enforce_constraints<E: FieldElement<BaseField = BaseElement>>(
    transition: &VmTransition<E>,
    masks: &[E],
    ark: &[E],
    result: &mut [E],
) {
    let op_flags = transition.cf_op_flags();

    let current = transition.current();
    let next = transition.next();

    // evaluate constraints for decoding op codes
    enforce_op_bits(&mut result[..NUM_OP_CONSTRAINTS], transition, &masks);

    enforce_hacc(result, transition, &ark, op_flags[FlowOps::Hacc as usize]);
    enforce_begin(result, current, next, op_flags[FlowOps::Begin as usize]);
    enforce_tend(result, current, next, op_flags[FlowOps::Tend as usize]);
    enforce_fend(result, current, next, op_flags[FlowOps::Fend as usize]);
    enforce_loop(result, current, next, op_flags[FlowOps::Loop as usize]);
    enforce_wrap(result, current, next, op_flags[FlowOps::Wrap as usize]);
    enforce_break(result, current, next, op_flags[FlowOps::Break as usize]);
    enforce_void(result, current, next, op_flags[FlowOps::Void as usize]);
}

// CYCLE MASKS
// ================================================================================================
const MASKS: [[u128; BASE_CYCLE_LENGTH]; 3] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], // multiples of 16
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0], // one less than multiple of 16
    [0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1], // multiples of 8
];
