use super::{select_result_range, TransitionConstraintRange};
use vm_core::utils::range as create_range;

#[test]
fn transition_constraint_ranges() {
    let sys_constraints_len = 1;
    let stack_constraints_len = 2;
    let range_constraints_len = 3;
    let aux_constraints_len = 4;

    let constraint_ranges = TransitionConstraintRange::new(
        sys_constraints_len,
        stack_constraints_len,
        range_constraints_len,
        aux_constraints_len,
    );

    assert_eq!(constraint_ranges.stack.start, sys_constraints_len);
    assert_eq!(constraint_ranges.stack.end, sys_constraints_len + stack_constraints_len);
    assert_eq!(
        constraint_ranges.range_checker.start,
        sys_constraints_len + stack_constraints_len
    );
    assert_eq!(
        constraint_ranges.range_checker.end,
        sys_constraints_len + stack_constraints_len + range_constraints_len
    );
    assert_eq!(
        constraint_ranges.chiplets.start,
        sys_constraints_len + stack_constraints_len + range_constraints_len
    );
    assert_eq!(
        constraint_ranges.chiplets.end,
        sys_constraints_len + stack_constraints_len + range_constraints_len + aux_constraints_len
    );
}

#[test]
fn result_range() {
    let mut result: [u64; 6] = [1, 2, 3, 4, 5, 6];

    // Select the beginning.
    let range = create_range(0, 3);
    let selected_range = select_result_range!(&mut result, range);
    assert_eq!(selected_range, [1, 2, 3]);

    // Select the middle.
    let range = create_range(1, 2);
    let selected_range = select_result_range!(&mut result, range);
    assert_eq!(selected_range, [2, 3]);

    // Select the end.
    let range = create_range(5, 1);
    let selected_range = select_result_range!(&mut result, range);
    assert_eq!(selected_range, [6]);
}
