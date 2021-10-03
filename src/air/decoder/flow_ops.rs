use winterfell::math::FieldElement;

use super::{
    are_equal, enforce_left_shift, enforce_right_shift, enforce_stack_copy, is_zero, BaseElement,
    EvaluationResult, TraceState, SPONGE_WIDTH,
};

// CONSTRAINT EVALUATORS
// ================================================================================================

pub fn enforce_begin<E>(result: &mut [E], current: &TraceState<E>, next: &TraceState<E>, op_flag: E)
where
    E: FieldElement<BaseField = BaseElement>,
{
    // make sure sponge state has been cleared
    let new_sponge = next.sponge();
    result.agg_constraint(0, op_flag, is_zero(new_sponge[0]));
    result.agg_constraint(1, op_flag, is_zero(new_sponge[1]));
    result.agg_constraint(2, op_flag, is_zero(new_sponge[2]));
    result.agg_constraint(3, op_flag, is_zero(new_sponge[3]));

    // make sure hash of parent block was pushed onto the context stack
    let parent_hash = current.sponge()[0];
    let ctx_stack_start = SPONGE_WIDTH + 1; // 1 is for loop image constraint
    let ctx_stack_end = ctx_stack_start + current.ctx_stack().len();
    let ctx_result = &mut result[ctx_stack_start..ctx_stack_end];
    ctx_result.agg_constraint(0, op_flag, are_equal(parent_hash, next.ctx_stack()[0]));
    enforce_right_shift(
        ctx_result,
        current.ctx_stack(),
        next.ctx_stack(),
        1,
        op_flag,
    );

    // make sure loop stack didn't change
    let loop_result = &mut result[ctx_stack_end..ctx_stack_end + current.loop_stack().len()];
    enforce_stack_copy(
        loop_result,
        current.loop_stack(),
        next.loop_stack(),
        0,
        op_flag,
    );
}

pub fn enforce_tend<E>(result: &mut [E], current: &TraceState<E>, next: &TraceState<E>, op_flag: E)
where
    E: FieldElement<BaseField = BaseElement>,
{
    let parent_hash = current.ctx_stack()[0];
    let block_hash = current.sponge()[0];

    let new_sponge = next.sponge();
    result.agg_constraint(0, op_flag, are_equal(parent_hash, new_sponge[0]));
    result.agg_constraint(1, op_flag, are_equal(block_hash, new_sponge[1]));
    // no constraint on the 3rd element of the sponge
    result.agg_constraint(3, op_flag, is_zero(new_sponge[3]));

    // make parent hash was popped from context stack
    let ctx_stack_start = SPONGE_WIDTH + 1; // 1 is for loop image constraint
    let ctx_stack_end = ctx_stack_start + current.ctx_stack().len();
    let ctx_result = &mut result[ctx_stack_start..ctx_stack_end];
    enforce_left_shift(
        ctx_result,
        current.ctx_stack(),
        next.ctx_stack(),
        1,
        1,
        op_flag,
    );

    // make sure loop stack didn't change
    let loop_result = &mut result[ctx_stack_end..ctx_stack_end + current.loop_stack().len()];
    enforce_stack_copy(
        loop_result,
        current.loop_stack(),
        next.loop_stack(),
        0,
        op_flag,
    );
}

pub fn enforce_fend<E>(result: &mut [E], current: &TraceState<E>, next: &TraceState<E>, op_flag: E)
where
    E: FieldElement<BaseField = BaseElement>,
{
    let parent_hash = current.ctx_stack()[0];
    let block_hash = current.sponge()[0];

    let new_sponge = next.sponge();
    result.agg_constraint(0, op_flag, are_equal(parent_hash, new_sponge[0]));
    // no constraint on the 2nd element of the sponge
    result.agg_constraint(2, op_flag, are_equal(block_hash, new_sponge[2]));
    result.agg_constraint(3, op_flag, is_zero(new_sponge[3]));

    // make sure parent hash was popped from context stack
    let ctx_stack_start = SPONGE_WIDTH + 1; // 1 is for loop image constraint
    let ctx_stack_end = ctx_stack_start + current.ctx_stack().len();
    let ctx_result = &mut result[ctx_stack_start..ctx_stack_end];
    enforce_left_shift(
        ctx_result,
        current.ctx_stack(),
        next.ctx_stack(),
        1,
        1,
        op_flag,
    );

    // make sure loop stack didn't change
    let loop_result = &mut result[ctx_stack_end..ctx_stack_end + current.loop_stack().len()];
    enforce_stack_copy(
        loop_result,
        current.loop_stack(),
        next.loop_stack(),
        0,
        op_flag,
    );
}

pub fn enforce_loop<E>(result: &mut [E], current: &TraceState<E>, next: &TraceState<E>, op_flag: E)
where
    E: FieldElement<BaseField = BaseElement>,
{
    // make sure sponge state has been cleared
    let new_sponge = next.sponge();
    result.agg_constraint(0, op_flag, is_zero(new_sponge[0]));
    result.agg_constraint(1, op_flag, is_zero(new_sponge[1]));
    result.agg_constraint(2, op_flag, is_zero(new_sponge[2]));
    result.agg_constraint(3, op_flag, is_zero(new_sponge[3]));

    // make sure hash of parent block was pushed onto the context stack
    let parent_hash = current.sponge()[0];
    let ctx_stack_start = SPONGE_WIDTH + 1; // 1 is for loop image constraint
    let ctx_stack_end = ctx_stack_start + current.ctx_stack().len();
    let ctx_result = &mut result[ctx_stack_start..ctx_stack_end];
    ctx_result.agg_constraint(0, op_flag, are_equal(parent_hash, next.ctx_stack()[0]));
    enforce_right_shift(
        ctx_result,
        current.ctx_stack(),
        next.ctx_stack(),
        1,
        op_flag,
    );

    // make sure loop stack was shifted by 1 item to the right, but don't enforce constraints
    // on the first item of the stack (which will contain loop image)
    let loop_result = &mut result[ctx_stack_end..ctx_stack_end + current.loop_stack().len()];
    enforce_right_shift(
        loop_result,
        current.loop_stack(),
        next.loop_stack(),
        1,
        op_flag,
    );
}

pub fn enforce_wrap<E>(result: &mut [E], current: &TraceState<E>, next: &TraceState<E>, op_flag: E)
where
    E: FieldElement<BaseField = BaseElement>,
{
    // make sure sponge state has been cleared
    let new_sponge = next.sponge();
    result.agg_constraint(0, op_flag, is_zero(new_sponge[0]));
    result.agg_constraint(1, op_flag, is_zero(new_sponge[1]));
    result.agg_constraint(2, op_flag, is_zero(new_sponge[2]));
    result.agg_constraint(3, op_flag, is_zero(new_sponge[3]));

    // make sure item at the top of loop stack is equal to loop image
    let loop_image = current.sponge()[0];
    result.agg_constraint(
        SPONGE_WIDTH,
        op_flag,
        are_equal(loop_image, current.loop_stack()[0]),
    );

    // make sure context stack didn't change
    let ctx_stack_start = SPONGE_WIDTH + 1; // 1 is for loop image constraint
    let ctx_stack_end = ctx_stack_start + current.ctx_stack().len();
    let ctx_result = &mut result[ctx_stack_start..ctx_stack_end];
    enforce_stack_copy(
        ctx_result,
        current.ctx_stack(),
        next.ctx_stack(),
        0,
        op_flag,
    );

    // make sure loop stack didn't change
    let loop_result = &mut result[ctx_stack_end..ctx_stack_end + current.loop_stack().len()];
    enforce_stack_copy(
        loop_result,
        current.loop_stack(),
        next.loop_stack(),
        0,
        op_flag,
    );
}

pub fn enforce_break<E>(result: &mut [E], current: &TraceState<E>, next: &TraceState<E>, op_flag: E)
where
    E: FieldElement<BaseField = BaseElement>,
{
    // make sure sponge state didn't change
    let old_sponge = current.sponge();
    let new_sponge = next.sponge();
    for i in 0..SPONGE_WIDTH {
        result.agg_constraint(i, op_flag, are_equal(old_sponge[i], new_sponge[i]));
    }

    // make sure item at the top of loop stack is equal to loop image
    let loop_image = old_sponge[0];
    result.agg_constraint(
        SPONGE_WIDTH,
        op_flag,
        are_equal(loop_image, current.loop_stack()[0]),
    );

    // make sure context stack didn't change
    let ctx_stack_start = SPONGE_WIDTH + 1; // 1 is for loop image constraint
    let ctx_stack_end = ctx_stack_start + current.ctx_stack().len();
    let ctx_result = &mut result[ctx_stack_start..ctx_stack_end];
    enforce_stack_copy(
        ctx_result,
        current.ctx_stack(),
        next.ctx_stack(),
        0,
        op_flag,
    );

    // make loop image was popped from loop stack
    let loop_result = &mut result[ctx_stack_end..ctx_stack_end + current.loop_stack().len()];
    enforce_left_shift(
        loop_result,
        current.loop_stack(),
        next.loop_stack(),
        1,
        1,
        op_flag,
    );
}

pub fn enforce_void<E>(result: &mut [E], current: &TraceState<E>, next: &TraceState<E>, op_flag: E)
where
    E: FieldElement<BaseField = BaseElement>,
{
    // make sure sponge state didn't change
    let old_sponge = current.sponge();
    let new_sponge = next.sponge();
    for i in 0..SPONGE_WIDTH {
        result.agg_constraint(i, op_flag, are_equal(old_sponge[i], new_sponge[i]));
    }

    // make sure context stack didn't change
    let ctx_stack_start = SPONGE_WIDTH + 1; // 1 is for loop image constraint
    let ctx_stack_end = ctx_stack_start + current.ctx_stack().len();
    let ctx_result = &mut result[ctx_stack_start..ctx_stack_end];
    enforce_stack_copy(
        ctx_result,
        current.ctx_stack(),
        next.ctx_stack(),
        0,
        op_flag,
    );

    // make sure loop stack didn't change
    let loop_result = &mut result[ctx_stack_end..ctx_stack_end + current.loop_stack().len()];
    enforce_stack_copy(
        loop_result,
        current.loop_stack(),
        next.loop_stack(),
        0,
        op_flag,
    );
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {

    use super::{are_equal, TraceState};
    use crate::{
        air::ToElements,
        processor::opcodes::{FlowOps, UserOps},
    };
    use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};

    #[test]
    fn op_begin() {
        // correct transition, context depth = 1
        let state1 = new_state(15, FlowOps::Begin, &[3, 5, 7, 9], &[0], &[]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[3], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_begin(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // correct transition, context depth = 2
        let state1 = new_state(15, FlowOps::Begin, &[3, 5, 7, 9], &[2, 0], &[]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[3, 2], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 8];
        super::enforce_begin(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition, context depth = 1
        let state1 = new_state(15, FlowOps::Begin, &[3, 5, 7, 9], &[0], &[]);
        let state2 = new_state(16, FlowOps::Void, &[1, 2, 3, 4], &[5], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_begin(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!(
            [
                1,
                2,
                3,
                4,
                0,
                are_equal(BaseElement::new(3), BaseElement::new(5)).as_int(),
                0
            ]
            .to_elements(),
            evaluations
        );

        // incorrect transition, context depth = 2
        let state1 = new_state(15, FlowOps::Begin, &[3, 5, 7, 9], &[2, 0], &[]);
        let state2 = new_state(16, FlowOps::Void, &[1, 2, 3, 4], &[5, 6], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 8];
        super::enforce_begin(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!(
            [
                1,
                2,
                3,
                4,
                0,
                are_equal(BaseElement::new(3), BaseElement::new(5)).as_int(),
                are_equal(BaseElement::new(2), BaseElement::new(6)).as_int(),
                0
            ]
            .to_elements(),
            evaluations
        );
    }

    #[test]
    fn op_tend() {
        // correct transition, context depth = 1
        let state1 = new_state(15, FlowOps::Tend, &[3, 5, 7, 9], &[8], &[]);
        let state2 = new_state(16, FlowOps::Void, &[8, 3, 4, 0], &[0], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_tend(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // correct transition, context depth = 2
        let state1 = new_state(15, FlowOps::Tend, &[3, 5, 7, 9], &[8, 2], &[]);
        let state2 = new_state(16, FlowOps::Void, &[8, 3, 4, 0], &[2, 0], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 8];
        super::enforce_tend(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition, context depth = 1
        let state1 = new_state(15, FlowOps::Tend, &[3, 5, 7, 9], &[8], &[]);
        let state2 = new_state(16, FlowOps::Void, &[1, 2, 3, 4], &[8], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_tend(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([7, 1, 0, 4, 0, 8, 0].to_elements(), evaluations);

        // incorrect transition, context depth = 2
        let state1 = new_state(15, FlowOps::Tend, &[3, 5, 7, 9], &[4, 6], &[]);
        let state2 = new_state(16, FlowOps::Void, &[1, 2, 3, 4], &[5, 6], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 8];
        super::enforce_tend(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([3, 1, 0, 4, 0, 1, 6, 0].to_elements(), evaluations);
    }

    #[test]
    fn op_fend() {
        // correct transition, context depth = 1
        let state1 = new_state(15, FlowOps::Fend, &[3, 5, 7, 9], &[8], &[]);
        let state2 = new_state(16, FlowOps::Void, &[8, 4, 3, 0], &[0], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_fend(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // correct transition, context depth = 2
        let state1 = new_state(15, FlowOps::Fend, &[3, 5, 7, 9], &[8, 2], &[]);
        let state2 = new_state(16, FlowOps::Void, &[8, 6, 3, 0], &[2, 0], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 8];
        super::enforce_fend(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition, context depth = 1
        let state1 = new_state(15, FlowOps::Fend, &[3, 5, 7, 9], &[8], &[]);
        let state2 = new_state(16, FlowOps::Void, &[1, 3, 2, 4], &[8], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_fend(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([7, 0, 1, 4, 0, 8, 0].to_elements(), evaluations);

        // incorrect transition, context depth = 2
        let state1 = new_state(15, FlowOps::Fend, &[3, 5, 7, 9], &[4, 6], &[]);
        let state2 = new_state(16, FlowOps::Void, &[1, 6, 2, 4], &[5, 6], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 8];
        super::enforce_fend(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([3, 0, 1, 4, 0, 1, 6, 0].to_elements(), evaluations);
    }

    #[test]
    fn op_loop() {
        // correct transition, context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Loop, &[3, 5, 7, 9], &[0], &[0]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[3], &[11]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_loop(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition (state not cleared), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Loop, &[3, 5, 7, 9], &[0], &[0]);
        let state2 = new_state(16, FlowOps::Void, &[1, 2, 3, 4], &[3], &[11]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_loop(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([1, 2, 3, 4, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition (context not copied), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Loop, &[3, 5, 7, 9], &[0], &[0]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[0], &[11]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_loop(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 3, 0].to_elements(), evaluations);

        // correct transition, context depth = 2, loop depth = 2
        let state1 = new_state(15, FlowOps::Loop, &[3, 5, 7, 9], &[6, 0], &[11, 0]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[3, 6], &[13, 11]);

        let mut evaluations = vec![BaseElement::ZERO; 9];
        super::enforce_loop(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition (loop stack not shifted), context depth = 2, loop depth = 2
        let state1 = new_state(15, FlowOps::Loop, &[3, 5, 7, 9], &[6, 0], &[11, 0]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[3, 6], &[11, 0]);

        let mut evaluations = vec![BaseElement::ZERO; 9];
        super::enforce_loop(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 11].to_elements(), evaluations);
    }

    #[test]
    fn op_wrap() {
        // correct transition, context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[3]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[11], &[3]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_wrap(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition (loop image mismatch), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[5]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[11], &[5]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_wrap(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!(
            [
                0,
                0,
                0,
                0,
                are_equal(BaseElement::new(3), BaseElement::new(5)).as_int(),
                0,
                0
            ]
            .to_elements(),
            evaluations
        );

        // incorrect transition (loop stack changed), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[3]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[11], &[4]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_wrap(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!(
            [
                0,
                0,
                0,
                0,
                0,
                0,
                are_equal(BaseElement::new(3), BaseElement::new(4)).as_int()
            ]
            .to_elements(),
            evaluations
        );

        // incorrect transition (context stack changed), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[3]);
        let state2 = new_state(16, FlowOps::Void, &[0, 0, 0, 0], &[10], &[3]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_wrap(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!(
            [
                0,
                0,
                0,
                0,
                0,
                are_equal(BaseElement::new(11), BaseElement::new(10)).as_int(),
                0
            ]
            .to_elements(),
            evaluations
        );

        // incorrect transition (sponge not reset), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[3]);
        let state2 = new_state(16, FlowOps::Void, &[1, 2, 3, 4], &[11], &[3]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_wrap(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([1, 2, 3, 4, 0, 0, 0].to_elements(), evaluations);
    }

    #[test]
    fn op_break() {
        // correct transition, context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Break, &[3, 5, 7, 9], &[11], &[3]);
        let state2 = new_state(16, FlowOps::Void, &[3, 5, 7, 9], &[11], &[0]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_break(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition (loop image mismatch), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[5]);
        let state2 = new_state(16, FlowOps::Void, &[3, 5, 7, 9], &[11], &[0]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_break(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!(
            [
                0,
                0,
                0,
                0,
                are_equal(BaseElement::new(3), BaseElement::new(5)).as_int(),
                0,
                0
            ]
            .to_elements(),
            evaluations
        );

        // incorrect transition (loop stack not popped), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[3]);
        let state2 = new_state(16, FlowOps::Void, &[3, 5, 7, 9], &[11], &[3]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_break(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!(
            [
                0,
                0,
                0,
                0,
                0,
                0,
                are_equal(BaseElement::new(3), BaseElement::ZERO).as_int()
            ]
            .to_elements(),
            evaluations
        );

        // incorrect transition (context stack changed), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[3]);
        let state2 = new_state(16, FlowOps::Void, &[3, 5, 7, 9], &[10], &[0]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_break(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!(
            [
                0,
                0,
                0,
                0,
                0,
                are_equal(BaseElement::new(11), BaseElement::new(10)).as_int(),
                0
            ]
            .to_elements(),
            evaluations
        );

        // incorrect transition (sponge changed), context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Wrap, &[3, 5, 7, 9], &[11], &[3]);
        let state2 = new_state(16, FlowOps::Void, &[1, 3, 5, 7], &[11], &[0]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_break(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([2, 2, 2, 2, 0, 0, 0].to_elements(), evaluations);
    }

    #[test]
    fn op_void() {
        // correct transition, context depth = 1
        let state1 = new_state(15, FlowOps::Void, &[3, 5, 7, 9], &[8], &[]);
        let state2 = new_state(16, FlowOps::Void, &[3, 5, 7, 9], &[8], &[]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_void(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // correct transition, context depth = 2, loop depth = 1
        let state1 = new_state(15, FlowOps::Void, &[3, 5, 7, 9], &[8, 2], &[11]);
        let state2 = new_state(16, FlowOps::Void, &[3, 5, 7, 9], &[8, 2], &[11]);

        let mut evaluations = vec![BaseElement::ZERO; 8];
        super::enforce_void(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0].to_elements(), evaluations);

        // incorrect transition, context depth = 1, loop depth = 1
        let state1 = new_state(15, FlowOps::Void, &[3, 5, 7, 9], &[8], &[11]);
        let state2 = new_state(16, FlowOps::Void, &[2, 4, 6, 8], &[7], &[10]);

        let mut evaluations = vec![BaseElement::ZERO; 7];
        super::enforce_void(&mut evaluations, &state1, &state2, BaseElement::ONE);
        assert_eq!([1, 1, 1, 1, 0, 1, 1].to_elements(), evaluations);
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------
    fn new_state(
        step: usize,
        flow_op: FlowOps,
        sponge: &[u128; 4],
        ctx_stack: &[u128],
        loop_stack: &[u128],
    ) -> TraceState<BaseElement> {
        let ctx_depth = ctx_stack.len();
        let loop_depth = loop_stack.len();

        let mut state = vec![step as u128, sponge[0], sponge[1], sponge[2], sponge[3]];

        for i in 0..3 {
            state.push(((flow_op as u128) >> i) & 1);
        }

        for i in 0..7 {
            state.push(((UserOps::Noop as u128) >> i) & 1);
        }

        state.extend_from_slice(ctx_stack);
        state.extend_from_slice(loop_stack);
        state.push(101); // single value for user stack

        TraceState::from_vec(ctx_depth, loop_depth, 1, &state.to_elements())
    }
}
