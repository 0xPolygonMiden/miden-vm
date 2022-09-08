use crate::stack::op_flags::get_op_index;

use super::{
    generate_evaluation_frame, OpFlags, DEGREE_4_OPCODE_ENDS, DEGREE_4_OPCODE_STARTS,
    DEGREE_6_OPCODE_ENDS, DEGREE_6_OPCODE_STARTS, DEGREE_7_OPCODE_ENDS, DEGREE_7_OPCODE_STARTS,
    NUM_DEGREE_4_OPS, NUM_DEGREE_6_OPS, NUM_DEGREE_7_OPS,
};
use vm_core::{ONE, ZERO};

/// Asserts the op flag to ONE for degree 7 operation which is being executed in the current
/// frame; assert all the other operation flags to ZERO as they are not present in the current
/// trace.
#[test]
fn degree_7_op_flags() {
    for i in DEGREE_7_OPCODE_STARTS..=DEGREE_7_OPCODE_ENDS {
        // frame initialised with a degree 7 operation using it's unique opcode.
        let frame = generate_evaluation_frame(i);

        // All the operation flags are generated for the given frame.
        let op_flags = OpFlags::new(&frame);

        // index of the operation flag in the op_flag's degree seven array.
        let idx_in_degree7_flags = get_op_index(i as u8);

        // Asserts operation flag of degree 7 operation being executed to ONE.
        assert_eq!(op_flags.degree7_op_flags[idx_in_degree7_flags], ONE);

        // Asserts operation flags not present in the trace to ZERO.
        for i in 0..NUM_DEGREE_7_OPS {
            if i != idx_in_degree7_flags {
                assert_eq!(op_flags.degree7_op_flags[i], ZERO)
            }
        }

        // Asserts all degree 6 operation flags to ZERO as the input operation is of degree 7.
        for i in 0..NUM_DEGREE_6_OPS {
            assert_eq!(op_flags.degree6_op_flags[i], ZERO)
        }

        // Asserts all degree 4 operation flags to ZERO as the input operation is of degree 7.
        for i in 0..NUM_DEGREE_4_OPS {
            assert_eq!(op_flags.degree4_op_flags[i], ZERO)
        }
    }
}

/// Asserts the op flag to ONE for degree 6 operation which is being executed in the current
/// frame; assert all the other operation flags to ZERO as they are not present in the current
/// trace.
#[test]
fn degree_6_op_flags() {
    for i in (DEGREE_6_OPCODE_STARTS..=DEGREE_6_OPCODE_ENDS).step_by(2) {
        // frame initialised with a degree 6 operation using it's unique opcode.
        let frame = generate_evaluation_frame(i);

        // All the operation flags are generated for the given frame.
        let op_flags = OpFlags::new(&frame);

        // index of the operation flag in the op_flag's degree six array.
        let idx_in_degree6_flags = get_op_index(i as u8);

        // Asserts operation flag of degree 6 operation being executed to ONE.
        assert_eq!(op_flags.degree6_op_flags[idx_in_degree6_flags], ONE);

        // Assert operation flags not present in the trace to ZERO as the operation being executed
        // is of degree 6.
        for i in 0..NUM_DEGREE_7_OPS {
            assert_eq!(op_flags.degree7_op_flags[i], ZERO)
        }

        // Except the operation being executed, all the degree 6 operation flag should be ZERO.
        for i in 0..NUM_DEGREE_6_OPS {
            if i != idx_in_degree6_flags {
                assert_eq!(op_flags.degree6_op_flags[i], ZERO)
            }
        }

        // Assert operation flags not present in the trace to ZERO as the operation being executed
        // is of degree 6.
        for i in 0..NUM_DEGREE_4_OPS {
            assert_eq!(op_flags.degree4_op_flags[i], ZERO)
        }
    }
}

/// Asserts the op flag to ONE for degree 4 operation which is being executed in the current
/// frame; assert all the other operation flags to ZERO as they are not present in the current
/// trace.
#[test]
fn degree_4_op_flags() {
    for i in (DEGREE_4_OPCODE_STARTS..=DEGREE_4_OPCODE_ENDS).step_by(4) {
        // frame initialised with a degree 4 operation using it's unique opcode.
        let frame = generate_evaluation_frame(i);

        // All the operation flags are generated for the given frame.
        let op_flags = OpFlags::new(&frame);

        // index of the operation flag in the op_flag's degree four array.
        let idx_in_degree4_flags = get_op_index(i as u8);

        // Asserts operation flag of degree 4 operation being executed to ONE.
        assert_eq!(op_flags.degree4_op_flags[idx_in_degree4_flags], ONE);

        // Assert operation flags not present in the trace to ZERO as the operation being executed
        // is of degree 4.
        for i in 0..NUM_DEGREE_7_OPS {
            assert_eq!(op_flags.degree7_op_flags[i], ZERO)
        }

        // Assert operation flags not present in the trace to ZERO as the operation being executed
        // is of degree 4.
        for i in 0..NUM_DEGREE_6_OPS {
            assert_eq!(op_flags.degree6_op_flags[i], ZERO)
        }

        // Except the operation being executed, all the degree 4 operation flags should be ZERO.
        for i in 0..NUM_DEGREE_4_OPS {
            if i != idx_in_degree4_flags {
                assert_eq!(op_flags.degree4_op_flags[i], ZERO)
            }
        }
    }
}
