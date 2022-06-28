use super::super::bitwise_pow2;
use super::bitwise::tests::get_test_frame_with_two_selectors;
use super::{bitwise, pow2};
use crate::{Felt, FieldElement};
use vm_core::bitwise::{BITWISE_OR, BITWISE_XOR};

// UNIT TESTS
// ================================================================================================

pub const NUM_CONSTRAINTS: usize =
    bitwise_pow2::NUM_CONSTRAINTS + bitwise::NUM_CONSTRAINTS + pow2::NUM_CONSTRAINTS;

#[test]
/// Tests that the bitwise constraints do not all evaluate to zero if the operation selectors change
/// within a cycle even when the output column will be same for both of them.
fn test_bitwise_pow2_selectors_fail() {
    let current_bitwise = vec![
        Felt::ONE,
        Felt::ONE,
        Felt::ZERO,
        Felt::ZERO,
        Felt::ONE,
        Felt::ZERO,
        Felt::ONE,
        Felt::ONE,
    ];

    let next_bitwise = vec![
        Felt::ZERO,
        Felt::ZERO,
        Felt::ZERO,
        Felt::ZERO,
        Felt::ONE,
        Felt::ONE,
        Felt::ONE,
        Felt::ONE,
    ];
    let cycle = 1;
    let expected = [Felt::ZERO; NUM_CONSTRAINTS];

    let frame =
        get_test_frame_with_two_selectors(&current_bitwise, &next_bitwise, BITWISE_OR, BITWISE_XOR);

    let periodic_values = bitwise_pow2::get_periodic_values(cycle);
    let mut result = [Felt::ZERO; NUM_CONSTRAINTS];

    bitwise_pow2::enforce_constraints(&frame, &periodic_values, &mut result, Felt::ONE);

    assert_ne!(result, expected);
}
