use super::FieldElement;
use core::ops::Range;
use vm_core::{utils::collections::Vec, utils::range as create_range};

// BASIC CONSTRAINT OPERATORS
// ================================================================================================

/// Returns zero only when a == b.
pub fn are_equal<E: FieldElement>(a: E, b: E) -> E {
    a - b
}

#[inline(always)]
pub fn is_binary<E: FieldElement>(v: E) -> E {
    v.square() - v
}

#[inline(always)]
pub fn binary_not<E: FieldElement>(v: E) -> E {
    E::ONE - v
}

#[inline(always)]
pub fn is_zero<E: FieldElement>(v: E) -> E {
    v
}

// TRAIT TO SIMPLIFY CONSTRAINT AGGREGATION
// ================================================================================================

pub trait EvaluationResult<E: FieldElement> {
    fn agg_constraint(&mut self, index: usize, flag: E, value: E);
}

impl<E: FieldElement> EvaluationResult<E> for [E] {
    fn agg_constraint(&mut self, index: usize, flag: E, value: E) {
        self[index] += flag * value;
    }
}

impl<E: FieldElement> EvaluationResult<E> for Vec<E> {
    fn agg_constraint(&mut self, index: usize, flag: E, value: E) {
        self[index] += flag * value;
    }
}

// TRANSITION CONSTRAINT RANGE
// ================================================================================================

/// Manages the starting index and length of transition constraints for individual processors so
/// indices can be handled easily during transition evaluation.
#[derive(Debug)]
pub struct TransitionConstraintRange {
    pub(super) stack: Range<usize>,
    pub(super) range_checker: Range<usize>,
    pub(super) chiplets: Range<usize>,
}

impl TransitionConstraintRange {
    pub fn new(
        sys: usize,
        stack_len: usize,
        range_checker_len: usize,
        chiplets_len: usize,
    ) -> Self {
        let stack = create_range(sys, stack_len);
        let range_checker = create_range(stack.end, range_checker_len);
        let chiplets = create_range(range_checker.end, chiplets_len);

        Self {
            stack,
            range_checker,
            chiplets,
        }
    }
}

// MACRO TO SIMPLIFY RANGE HANDLING
// ================================================================================================
/// Select an array range from a mutable result array and a specified range.
#[macro_export]
macro_rules! select_result_range {
    ($result:expr, $range:expr) => {
        &mut $result[$range.start..$range.end]
    };
}
