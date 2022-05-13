use super::{bitwise, pow2, PERIODIC_CYCLE_LEN};
use crate::{Felt, FieldElement};
use vm_core::bitwise::{Selectors, BITWISE_AND, BITWISE_OR, BITWISE_XOR};

use proptest::prelude::*;

// UNIT TESTS
// ================================================================================================

proptest! {
    #[test]
    fn test_bitwise_and(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(PERIODIC_CYCLE_LEN - 1)) {
        test_bitwise_frame(BITWISE_AND, a, b, cycle_row);
    }

    #[test]
    fn test_bitwise_or(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(PERIODIC_CYCLE_LEN - 1)) {
        test_bitwise_frame(BITWISE_OR, a, b, cycle_row);
    }

    #[test]
    fn test_bitwise_xor(a in any::<u32>(), b in any::<u32>(), cycle_row in 0..(PERIODIC_CYCLE_LEN - 1)) {
        test_bitwise_frame(BITWISE_XOR, a, b, cycle_row);
    }

    #[test]
    fn test_pow2(exponent in 0_u32..64, cycle_row in 0..(PERIODIC_CYCLE_LEN - 1)) {
        test_pow2_frame(exponent, cycle_row);
    }
}

// TEST HELPERS
// ================================================================================================

/// Returns the values from the shared bitwise & power of two processor's periodic columns for the
/// specified cycle row.
fn get_test_periodic_values(cycle_row: usize) -> [Felt; 2] {
    match cycle_row {
        0 => [Felt::ONE, Felt::ONE],
        8 => [Felt::ZERO, Felt::ZERO],
        _ => [Felt::ZERO, Felt::ONE],
    }
}

/// Generates the specified trace frame for the specified bitwise operation and inputs, then asserts
/// that applying the constraints to this frame yields valid results (all zeros).
fn test_bitwise_frame(operation: Selectors, a: u32, b: u32, cycle_row: usize) {
    let frame = bitwise::get_test_frame(operation, a, b, cycle_row);
    let periodic_values = get_test_periodic_values(cycle_row);
    let mut result = [Felt::ZERO; bitwise::NUM_CONSTRAINTS];
    let expected = result;

    bitwise::enforce_constraints(&frame, &periodic_values, &mut result, Felt::ONE);

    assert_eq!(expected, result);
}

/// Generates the specified trace frame for the specified power of two operation, then asserts
/// that applying the constraints to this frame yields valid results (all zeros).
fn test_pow2_frame(exponent: u32, cycle_row: usize) {
    let frame = pow2::get_test_frame(exponent, cycle_row);
    let periodic_values = get_test_periodic_values(cycle_row);
    let mut result = [Felt::ZERO; pow2::NUM_CONSTRAINTS];
    let expected = result;

    pow2::enforce_constraints(&frame, &periodic_values, &mut result, Felt::ONE);

    assert_eq!(expected, result);
}
