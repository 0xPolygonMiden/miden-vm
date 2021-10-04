use super::{
    utils::{
        are_equal, binary_not, enforce_left_shift, enforce_right_shift, enforce_stack_copy,
        is_binary, is_zero, EvaluationResult,
    },
    TransitionConstraintDegree,
};
use crate::{air::VmTransition, HASH_STATE_WIDTH};
use winterfell::math::{fields::f128::BaseElement, FieldElement};

use processor::OpCode;

mod input;
use input::{enforce_push, enforce_read, enforce_read2};

mod arithmetic;
use arithmetic::{
    enforce_add, enforce_and, enforce_inv, enforce_mul, enforce_neg, enforce_not, enforce_or,
};

mod manipulation;
use manipulation::{
    enforce_drop, enforce_drop4, enforce_dup, enforce_dup2, enforce_dup4, enforce_pad2,
    enforce_roll4, enforce_roll8, enforce_swap, enforce_swap2, enforce_swap4,
};

mod comparison;
use comparison::{enforce_assert, enforce_asserteq, enforce_binacc, enforce_cmp, enforce_eq};

mod conditional;
use conditional::{enforce_choose, enforce_choose2, enforce_cswap2};

mod hash;
use hash::enforce_rescr;

// CONSTANTS
// ================================================================================================
pub const NUM_AUX_CONSTRAINTS: usize = 2;

// CONSTRAINT DEGREES
// ================================================================================================

pub fn get_transition_constraint_degrees(stack_depth: usize) -> Vec<TransitionConstraintDegree> {
    // all stack transition constraints have degree 7
    let degree = TransitionConstraintDegree::new(7);
    vec![degree; stack_depth + NUM_AUX_CONSTRAINTS]
}

// HELPER FUNCTIONS
// ================================================================================================
pub fn enforce_constraints<E: FieldElement<BaseField = BaseElement>>(
    transition: &VmTransition<E>,
    ark: &[E],
    result: &mut [E],
) {
    // split constraint evaluation result into aux constraints and stack constraints
    let (aux, result) = result.split_at_mut(NUM_AUX_CONSTRAINTS);

    // get user stack registers from current and next steps
    let old_stack = transition.current().user_stack();
    let new_stack = transition.next().user_stack();

    // initialize a vector to hold stack constraint evaluations; this is needed because
    // constraint evaluator functions assume that the stack is at least 8 items deep; while
    // it may actually be smaller than that
    let mut evaluations = vec![E::ZERO; old_stack.len()];

    // 1 ----- enforce constraints for low-degree operations --------------------------------------
    let ld_flags = transition.ld_op_flags();

    // assertion operations
    enforce_assert(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::Assert.ld_index()],
    );
    enforce_asserteq(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::AssertEq.ld_index()],
    );

    // input operations
    enforce_read(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Read.ld_index()],
    );
    enforce_read2(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Read2.ld_index()],
    );

    // stack manipulation operations
    enforce_dup(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Dup.ld_index()],
    );
    enforce_dup2(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Dup2.ld_index()],
    );
    enforce_dup4(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Dup4.ld_index()],
    );
    enforce_pad2(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Pad2.ld_index()],
    );

    enforce_drop(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Drop.ld_index()],
    );
    enforce_drop4(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Drop4.ld_index()],
    );

    enforce_swap(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Swap.ld_index()],
    );
    enforce_swap2(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Swap2.ld_index()],
    );
    enforce_swap4(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Swap4.ld_index()],
    );

    enforce_roll4(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Roll4.ld_index()],
    );
    enforce_roll8(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Roll8.ld_index()],
    );

    // arithmetic and boolean operations
    enforce_add(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Add.ld_index()],
    );
    enforce_mul(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Mul.ld_index()],
    );
    enforce_inv(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Inv.ld_index()],
    );
    enforce_neg(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::Neg.ld_index()],
    );

    enforce_not(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::Not.ld_index()],
    );
    enforce_and(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::And.ld_index()],
    );
    enforce_or(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::Or.ld_index()],
    );

    // comparison operations
    enforce_eq(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::Eq.ld_index()],
    );
    enforce_binacc(
        &mut evaluations,
        old_stack,
        new_stack,
        ld_flags[OpCode::BinAcc.ld_index()],
    );

    // conditional selection operations
    enforce_choose(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::Choose.ld_index()],
    );
    enforce_choose2(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::Choose2.ld_index()],
    );
    enforce_cswap2(
        &mut evaluations,
        aux,
        old_stack,
        new_stack,
        ld_flags[OpCode::CSwap2.ld_index()],
    );

    // 2 ----- enforce constraints for high-degree operations --------------------------------------
    let hd_flags = transition.hd_op_flags();

    enforce_push(
        &mut evaluations,
        old_stack,
        new_stack,
        hd_flags[OpCode::Push.hd_index()],
    );
    enforce_cmp(
        &mut evaluations,
        old_stack,
        new_stack,
        hd_flags[OpCode::Cmp.hd_index()],
    );
    enforce_rescr(
        &mut evaluations,
        old_stack,
        new_stack,
        ark,
        hd_flags[OpCode::RescR.hd_index()],
    );

    // 3 ----- enforce constraints for composite operations ---------------------------------------

    // BEGIN and NOOP have "composite" opcodes where all 7 opcode bits are set to either 1s or 0s;
    // thus, the flags for these operations are computed separately by multiplying all opcodes;
    // this results in flag degree of 7 for each operation, but since both operations enforce the
    // same constraints (the stack doesn't change), higher degree terms cancel out, and we
    // end up with overall constraint degree of (6 + 1 = 7) for both operations.
    enforce_stack_copy(
        &mut evaluations,
        old_stack,
        new_stack,
        0,
        transition.begin_flag(),
    );
    enforce_stack_copy(
        &mut evaluations,
        old_stack,
        new_stack,
        0,
        transition.noop_flag(),
    );

    // 4 ----- copy evaluations into the result ---------------------------------------------------
    result.copy_from_slice(&evaluations[..result.len()]);
}
