use core::ops::Index;
use alloc::{borrow::ToOwned, vec::Vec};
use vm_core::FieldElement;
use winter_prover::math::log2;

#[allow(dead_code)]
mod lagrange_ker;

mod error;
use self::error::Error;

// MULTI-LINEAR POLYNOMIAL
// ================================================================================================

/// The evaluations of a mult-linear polynomial over the boolean hyper-cube {0 , 1}^ν.
#[derive(Clone, Debug)]
pub struct MultiLinear<E: FieldElement> {
    num_variables: usize,
    evaluations: Vec<E>,
}

impl<E: FieldElement> MultiLinear<E> {
    /// Constructs a [MultiLinear] from an array of values in the field.
    pub fn new(values: Vec<E>) -> Result<Self, Error> {
        if !values.len().is_power_of_two() {
            return Err(Error::EvaluationsNotPowerOfTwo);
        }
        Ok(Self {
            num_variables: log2(values.len()) as usize,
            evaluations: values,
        })
    }

    /// Constructs a [MultiLinear] from a slice of values in the field.
    pub fn from_values(values: &[E]) -> Result<Self, Error> {
        if !values.len().is_power_of_two() {
            return Err(Error::EvaluationsNotPowerOfTwo);
        }
        Ok(Self {
            num_variables: log2(values.len()) as usize,
            evaluations: values.to_owned(),
        })
    }

    /// Returns the number of the multi-linear polynomial.
    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    /// Returns the evaluations over the boolean hyper-cube.
    pub fn evaluations(&self) -> &[E] {
        &self.evaluations
    }

    /// Returns the number of evaluations. This is equal to the size of the boolean hyper-cube.
    pub fn num_evaluations(&self) -> usize {
        self.evaluations.len()
    }

    /// Evaluate the multi-linear at some query (r_0, ..., r_{ν - 1}) ∈ 𝔽^ν.
    ///
    /// It first computes the evaluations of the Lagrange basis polynomials over the interpolating
    /// set {0 , 1}^ν at (r_0, ..., r_{ν - 1}) i.e., the Lagrange kernel at (r_0, ..., r_{ν - 1}).
    /// The evaluation then is the inner product, indexed by {0 , 1}^ν, of the vector of
    /// evaluations times the Lagrange kernel.
    pub fn evaluate(&self, query: &[E]) -> E {
        let tensored_query = tensorize(query);
        inner_product(&self.evaluations, &tensored_query)
    }

    /// Computes f(r_0, y_1, ..., y_{ν - 1}) using the linear interpolation formula
    /// (1 - r_0) * f(0, y_1, ..., y_{ν - 1}) + r_0 * f(1, y_1, ..., y_{ν - 1}). The resulting
    /// returned multi-linear is defined over a domain of half the size.
    pub fn bind(&self, round_challenge: E) -> Self {
        let mut result = vec![E::ZERO; 1 << (self.num_variables() - 1)];
        for (i, res) in result.iter_mut().enumerate() {
            *res = self.evaluations[i << 1]
                + round_challenge * (self.evaluations[(i << 1) + 1] - self.evaluations[i << 1]);
        }
        Self::from_values(&result).expect("should not fail given that it is a multi-linear")
    }

    /// Computes f(r_0, y_1, ..., y_{ν - 1}) using the linear interpolation formula
    /// (1 - r_0) * f(0, y_1, ..., y_{ν - 1}) + r_0 * f(1, y_1, ..., y_{ν - 1}) and assigns
    /// the resulting multi-linear, defined over a domain of half the size, to `self`.
    pub fn bind_assign(&mut self, round_challenge: E) {
        let mut result = vec![E::ZERO; 1 << (self.num_variables() - 1)];
        for (i, res) in result.iter_mut().enumerate() {
            *res = self.evaluations[i << 1]
                + round_challenge * (self.evaluations[(i << 1) + 1] - self.evaluations[i << 1]);
        }
        *self =
            Self::from_values(&result).expect("should not fail given that it is a multi-linear");
    }
}

impl<E: FieldElement> Index<usize> for MultiLinear<E> {
    type Output = E;

    fn index(&self, index: usize) -> &E {
        &(self.evaluations[index])
    }
}

// COMPOSITION POLYNOMIAL
// ================================================================================================

/// A multi-variate polynomial for composing individual multi-linear polynomials.
pub trait CompositionPolynomial<E: FieldElement>: Sync + Send {
    /// The number of variables when interpreted as a multi-variate polynomial.
    fn num_variables(&self) -> usize;

    /// Maximum degree in all variables.
    fn max_degree(&self) -> usize;

    /// Given a query, of length equal the number of variables, evaluates [Self] at this query.
    fn evaluate(&self, query: &[E]) -> E;
}

// COMPOSITION POLYNOMIAL
// ================================================================================================

/// Computes the inner product of two vectors of the same length.
///
/// Panics if the vectors have different lengths.
fn inner_product<E: FieldElement>(evaluations: &[E], tensored_query: &[E]) -> E {
    assert_eq!(evaluations.len(), tensored_query.len());
    evaluations
        .iter()
        .zip(tensored_query.iter())
        .fold(E::ZERO, |acc, (x_i, y_i)| acc + *x_i * *y_i)
}

/// Computes the evaluations of the Lagrange basis polynomials over the interpolating
/// set {0 , 1}^ν at (r_0, ..., r_{ν - 1}) i.e., the Lagrange kernel at (r_0, ..., r_{ν - 1}).
pub fn tensorize<E: FieldElement>(query: &[E]) -> Vec<E> {
    let nu = query.len();
    let n = 1 << nu;

    let mut evals: Vec<E> = vec![E::ONE; n];
    let mut size = 1;
    for query in query.iter() {
        size *= 2;
        for i in (0..size).rev().step_by(2) {
            let scalar = evals[i / 2];
            evals[i] = scalar * *query;
            evals[i - 1] = scalar - evals[i];
        }
    }
    evals
}
