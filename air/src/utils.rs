use super::{EvaluationFrame, FieldElement};

// BASIC CONSTRAINT OPERATORS
// ================================================================================================

#[inline(always)]
pub fn is_binary<E: FieldElement>(v: E) -> E {
    v.square() - v
}

#[inline(always)]
pub fn binary_not<E: FieldElement>(v: E) -> E {
    E::ONE - v
}

pub trait ColumnTransition<E: FieldElement> {
    fn change(&self, column: usize) -> E;
}

impl<E: FieldElement> ColumnTransition<E> for EvaluationFrame<E> {
    fn change(&self, column: usize) -> E {
        let current = self.current();
        let next = self.next();
        next[column] - current[column]
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
