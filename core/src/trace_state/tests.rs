use super::TraceState;
use crate::{utils::ToElements, BaseElement, FieldElement, StarkField};

#[test]
fn from_vec() {
    // empty context and loop stacks
    let state = TraceState::from_u128_slice(
        0,
        0,
        2,
        &[101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    );

    assert_eq!(BaseElement::new(101), state.op_counter());
    assert_eq!([1, 2, 3, 4].to_elements(), state.op_sponge());
    assert_eq!([5, 6, 7].to_elements(), state.cf_op_bits());
    assert_eq!([8, 9, 10, 11, 12].to_elements(), state.ld_op_bits());
    assert_eq!([13, 14].to_elements(), state.hd_op_bits());
    assert_eq!([0].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!([15, 16, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
    assert_eq!(17, state.width());
    assert_eq!(2, state.stack_depth());
    assert_eq!(
        [101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].to_elements(),
        state.to_vec()
    );

    // 1 item on context stack, empty loop stack
    let state = TraceState::from_u128_slice(
        1,
        0,
        2,
        &[
            101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
        ],
    );

    assert_eq!(BaseElement::new(101), state.op_counter());
    assert_eq!([1, 2, 3, 4].to_elements(), state.op_sponge());
    assert_eq!([5, 6, 7].to_elements(), state.cf_op_bits());
    assert_eq!([8, 9, 10, 11, 12].to_elements(), state.ld_op_bits());
    assert_eq!([13, 14].to_elements(), state.hd_op_bits());
    assert_eq!([15].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!([16, 17, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
    assert_eq!(18, state.width());
    assert_eq!(2, state.stack_depth());
    assert_eq!(
        [101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17].to_elements(),
        state.to_vec()
    );

    // non-empty loop stack
    let state = TraceState::from_u128_slice(
        2,
        1,
        9,
        &[
            101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26,
        ],
    );

    assert_eq!(BaseElement::new(101), state.op_counter());
    assert_eq!([1, 2, 3, 4].to_elements(), state.op_sponge());
    assert_eq!([5, 6, 7].to_elements(), state.cf_op_bits());
    assert_eq!([8, 9, 10, 11, 12].to_elements(), state.ld_op_bits());
    assert_eq!([13, 14].to_elements(), state.hd_op_bits());
    assert_eq!([15, 16].to_elements(), state.ctx_stack());
    assert_eq!([17].to_elements(), state.loop_stack());
    assert_eq!(
        [18, 19, 20, 21, 22, 23, 24, 25, 26].to_elements(),
        state.user_stack()
    );
    assert_eq!(27, state.width());
    assert_eq!(9, state.stack_depth());
    assert_eq!(
        [
            101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26,
        ]
        .to_elements(),
        state.to_vec()
    );
}

#[test]
fn update() {
    let row_data = vec![
        101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ]
    .to_elements();

    // first row
    let mut state = TraceState::new(2, 1, 3);
    state.update(&vec![BaseElement::ZERO; row_data.len()]);

    assert_eq!(0, state.op_counter().as_int());
    assert_eq!([0, 0, 0, 0].to_elements(), state.op_sponge());
    assert_eq!([0, 0, 0].to_elements(), state.cf_op_bits());
    assert_eq!([0, 0, 0, 0, 0].to_elements(), state.ld_op_bits());
    assert_eq!([0, 0].to_elements(), state.hd_op_bits());
    assert_eq!([0, 0].to_elements(), state.ctx_stack());
    assert_eq!([0].to_elements(), state.loop_stack());
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
    assert_eq!(21, state.width());
    assert_eq!(3, state.stack_depth());

    // second row
    state.update(&row_data);

    assert_eq!(101, state.op_counter().as_int());
    assert_eq!([1, 2, 3, 4].to_elements(), state.op_sponge());
    assert_eq!([5, 6, 7].to_elements(), state.cf_op_bits());
    assert_eq!([8, 9, 10, 11, 12].to_elements(), state.ld_op_bits());
    assert_eq!([13, 14].to_elements(), state.hd_op_bits());
    assert_eq!([15, 16].to_elements(), state.ctx_stack());
    assert_eq!([17].to_elements(), state.loop_stack());
    assert_eq!(
        [18, 19, 20, 0, 0, 0, 0, 0].to_elements(),
        state.user_stack()
    );
    assert_eq!(21, state.width());
    assert_eq!(3, state.stack_depth());
}

#[test]
fn op_code() {
    let state = TraceState::from_u128_slice(
        1,
        0,
        2,
        &[101, 1, 2, 3, 4, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 15, 16, 17],
    );
    assert_eq!(BaseElement::ZERO, state.op_code());

    let state = TraceState::from_u128_slice(
        1,
        0,
        2,
        &[101, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 15, 16, 17],
    );
    assert_eq!(BaseElement::new(127), state.op_code());

    let state = TraceState::from_u128_slice(
        1,
        0,
        2,
        &[101, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 15, 16, 17],
    );
    assert_eq!(BaseElement::new(63), state.op_code());

    let state = TraceState::from_u128_slice(
        1,
        0,
        2,
        &[101, 1, 2, 3, 4, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 15, 16, 17],
    );
    assert_eq!(BaseElement::new(97), state.op_code());
}
