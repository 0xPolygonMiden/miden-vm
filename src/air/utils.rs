use winterfell::math::{fields::f128::BaseElement, FieldElement};

// BASIC CONSTRAINTS OPERATORS
// ================================================================================================

#[inline(always)]
pub fn is_zero<E: FieldElement>(v: E) -> E {
    v
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
pub fn are_equal<E: FieldElement>(v1: E, v2: E) -> E {
    v1 - v2
}

// COMMON STACK CONSTRAINTS
// ================================================================================================

/// Enforces that stack values starting from `from_slot` haven't changed. All constraints in the
/// `result` slice are filled in.
pub fn enforce_stack_copy<E: FieldElement>(
    result: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    from_slot: usize,
    op_flag: E,
) {
    for i in from_slot..result.len() {
        result.agg_constraint(i, op_flag, are_equal(old_stack[i], new_stack[i]));
    }
}

/// Enforces that values in the stack were shifted to the right by `num_slots`. Constraints in
/// the `result` slice are filled in starting from `num_slots` index.
pub fn enforce_right_shift<E: FieldElement>(
    result: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    num_slots: usize,
    op_flag: E,
) {
    for i in num_slots..result.len() {
        result.agg_constraint(
            i,
            op_flag,
            are_equal(old_stack[i - num_slots], new_stack[i]),
        );
    }
}

/// Enforces that values in the stack were shifted to the left by `num_slots` starting from
/// `from_slots`. All constraints in the `result` slice are filled in.
pub fn enforce_left_shift<E: FieldElement>(
    result: &mut [E],
    old_stack: &[E],
    new_stack: &[E],
    from_slot: usize,
    num_slots: usize,
    op_flag: E,
) {
    // make sure values in the stack were shifted by `num_slots` to the left
    let start_idx = from_slot - num_slots;
    let remainder_idx = result.len() - num_slots;
    for i in start_idx..remainder_idx {
        result.agg_constraint(
            i,
            op_flag,
            are_equal(old_stack[i + num_slots], new_stack[i]),
        );
    }

    // also make sure that remaining slots were filled in with 0s
    for i in remainder_idx..result.len() {
        result.agg_constraint(i, op_flag, is_zero(new_stack[i]));
    }
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

// TYPE CONVERSIONS
// ================================================================================================

pub trait ToElements {
    fn to_elements(&self) -> Vec<BaseElement>;
}

impl<const N: usize> ToElements for [u128; N] {
    fn to_elements(&self) -> Vec<BaseElement> {
        self.iter().map(|&v| BaseElement::new(v)).collect()
    }
}

impl ToElements for Vec<u128> {
    fn to_elements(&self) -> Vec<BaseElement> {
        self.iter().map(|&v| BaseElement::new(v)).collect()
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::ToElements;
    use winterfell::math::{fields::f128::BaseElement, FieldElement};

    #[test]
    fn enforce_left_shift() {
        let op_flag = BaseElement::ONE;

        // sift left by 1 starting from 1
        let mut result = vec![BaseElement::ZERO; 8];
        super::enforce_left_shift(
            &mut result,
            &[1, 2, 3, 4, 5, 6, 7, 8].to_elements(),
            &[1, 2, 3, 4, 5, 6, 7, 8].to_elements(),
            1,
            1,
            op_flag,
        );
        assert_eq!([1, 1, 1, 1, 1, 1, 1, 8].to_elements(), result);

        // sift left by 2 starting from 2
        let mut result = vec![BaseElement::ZERO; 8];
        super::enforce_left_shift(
            &mut result,
            &[1, 2, 3, 4, 5, 6, 7, 8].to_elements(),
            &[1, 2, 3, 4, 5, 6, 7, 8].to_elements(),
            2,
            2,
            op_flag,
        );
        assert_eq!([2, 2, 2, 2, 2, 2, 7, 8].to_elements(), result);

        // sift left by 1 starting from 2
        let mut result = vec![BaseElement::ZERO; 8];
        super::enforce_left_shift(
            &mut result,
            &[1, 2, 3, 4, 5, 6, 7, 8].to_elements(),
            &[1, 2, 3, 4, 5, 6, 7, 8].to_elements(),
            2,
            1,
            op_flag,
        );
        assert_eq!([0, 1, 1, 1, 1, 1, 1, 8].to_elements(), result);

        // sift left by 4 starting from 6
        let mut result = vec![BaseElement::ZERO; 8];
        super::enforce_left_shift(
            &mut result,
            &[1, 2, 3, 4, 5, 6, 7, 8].to_elements(),
            &[1, 2, 3, 4, 5, 6, 7, 8].to_elements(),
            6,
            4,
            op_flag,
        );
        assert_eq!([0, 0, 4, 4, 5, 6, 7, 8].to_elements(), result);
    }
}
