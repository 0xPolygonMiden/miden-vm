use super::{Assertion, EvaluationFrame, Felt, FieldElement, TransitionConstraintDegree};

// PROCESSOR CONSTRAINTS
// ================================================================================================

/// Information about a single boundary constraint, specifying the column index and the boundary
/// value. It currently only supports single assertions, but it can be used for either first step or
/// last step assertions.
pub struct ColumnBoundary {
    column: usize,
    value: Felt,
}

impl ColumnBoundary {
    pub fn new(column: usize, value: Felt) -> Self {
        Self { column, value }
    }

    pub fn get_constraint(&self, step: usize) -> Assertion<Felt> {
        Assertion::single(self.column, step, self.value)
    }
}

/// The basic transition constraint information for a processor.
pub struct TransitionConstraints {
    count: usize,
    degrees: Vec<usize>,
}

impl TransitionConstraints {
    pub fn new(count: usize, degrees: Vec<usize>) -> Self {
        Self { count, degrees }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn degrees(&self) -> &Vec<usize> {
        &self.degrees
    }
}

/// Trait to manage interactions with constraint information for individual processing sections of
/// the trace so that boundary and transition constraint information can be accessed easily for each
/// processor.
pub trait ProcessorConstraints {
    fn first_step(&self) -> &[ColumnBoundary];
    fn last_step(&self) -> &[ColumnBoundary];
    fn transitions(&self) -> &TransitionConstraints;
    fn enforce_constraints<E: FieldElement>(&self, frame: &EvaluationFrame<E>, result: &mut [E]);

    // --- BOUNDARY CONSTRAINTS -------------------------------------------------------------------
    fn get_assertions_first_step(&self, result: &mut Vec<Assertion<Felt>>) {
        self.first_step()
            .iter()
            .for_each(|boundary| result.push(boundary.get_constraint(0)));
    }

    fn get_assertions_last_step(&self, result: &mut Vec<Assertion<Felt>>, step: usize) {
        self.last_step()
            .iter()
            .for_each(|boundary| result.push(boundary.get_constraint(step)));
    }

    // --- TRANSITION CONSTRAINTS -----------------------------------------------------------------
    fn get_transition_constraint_count(&self) -> usize {
        self.transitions().count
    }

    fn get_transition_constraint_degrees(&self) -> Vec<TransitionConstraintDegree> {
        self.transitions()
            .degrees
            .iter()
            .map(|&degree| TransitionConstraintDegree::new(degree))
            .collect()
    }
}

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
