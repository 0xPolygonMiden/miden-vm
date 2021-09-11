use super::{
    are_equal, enforce_left_shift, enforce_right_shift, enforce_stack_copy, is_zero, BaseElement,
    EvaluationResult, TraceState, SPONGE_WIDTH,
};

// CONSTRAINT EVALUATORS
// ================================================================================================

pub fn enforce_begin(
    result: &mut [BaseElement],
    current: &TraceState,
    next: &TraceState,
    op_flag: BaseElement,
) {
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

pub fn enforce_tend(
    result: &mut [BaseElement],
    current: &TraceState,
    next: &TraceState,
    op_flag: BaseElement,
) {
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

pub fn enforce_fend(
    result: &mut [BaseElement],
    current: &TraceState,
    next: &TraceState,
    op_flag: BaseElement,
) {
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

pub fn enforce_loop(
    result: &mut [BaseElement],
    current: &TraceState,
    next: &TraceState,
    op_flag: BaseElement,
) {
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

pub fn enforce_wrap(
    result: &mut [BaseElement],
    current: &TraceState,
    next: &TraceState,
    op_flag: BaseElement,
) {
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

pub fn enforce_break(
    result: &mut [BaseElement],
    current: &TraceState,
    next: &TraceState,
    op_flag: BaseElement,
) {
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

pub fn enforce_void(
    result: &mut [BaseElement],
    current: &TraceState,
    next: &TraceState,
    op_flag: BaseElement,
) {
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

// TODO: migrate
