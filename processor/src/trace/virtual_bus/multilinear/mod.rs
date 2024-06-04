use alloc::vec::Vec;
use core::ops::Index;
use vm_core::FieldElement;

mod lagrange_ker;
pub use lagrange_ker::{inner_product, EqFunction};

mod error;
use self::{error::Error, lagrange_ker::compute_lagrange_basis_evals_at};

// MULTI-LINEAR POLYNOMIAL
// ================================================================================================

/// Represents a multi-linear polynomial.
///
/// The representation stores the evaluations of the polynomial over the boolean hyper-cube
/// {0 , 1}^ν.
#[derive(Clone, Debug)]
pub struct MultiLinearPoly<E: FieldElement> {
    num_variables: usize,
    evaluations: Vec<E>,
}

impl<E: FieldElement> MultiLinearPoly<E> {
    /// Constructs a [`MultiLinearPoly`] from its evaluations over the boolean hyper-cube {0 , 1}^ν.
    pub fn from_evaluations(evaluations: Vec<E>) -> Result<Self, Error> {
        if !evaluations.len().is_power_of_two() {
            return Err(Error::EvaluationsNotPowerOfTwo);
        }
        Ok(Self {
            num_variables: (evaluations.len().ilog2()) as usize,
            evaluations,
        })
    }

    /// Returns the number of variables of the multi-linear polynomial.
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
        let tensored_query = compute_lagrange_basis_evals_at(query);
        inner_product(&self.evaluations, &tensored_query)
    }

    /// Similar to [`Self::evaluate`], except that the query was already turned into the Lagrange
    /// kernel (i.e. the [`lagrange_ker::EqFunction`] evaluated at every point in the set
    /// `{0 , 1}^ν`).
    ///
    /// This is more efficient than [`Self::evaluate`] when multiple different [`MultiLinearPoly`]
    /// need to be evaluated at the same query point.
    pub fn evaluate_with_lagrange_kernel(&self, lagrange_kernel: &[E]) -> E {
        inner_product(&self.evaluations, lagrange_kernel)
    }

    /// Computes f(r_0, y_1, ..., y_{ν - 1}) using the linear interpolation formula
    /// (1 - r_0) * f(0, y_1, ..., y_{ν - 1}) + r_0 * f(1, y_1, ..., y_{ν - 1}) and assigns
    /// the resulting multi-linear, defined over a domain of half the size, to `self`.
    pub fn bind_least_significant_variable(&mut self, round_challenge: E) {
        let mut result = vec![E::ZERO; 1 << (self.num_variables() - 1)];
        for (i, res) in result.iter_mut().enumerate() {
            *res = self.evaluations[i << 1]
                + round_challenge * (self.evaluations[(i << 1) + 1] - self.evaluations[i << 1]);
        }
        *self = Self::from_evaluations(result)
            .expect("should not fail given that it is a multi-linear");
    }

    /// Given the multilinear polynomial f(y_0, y_1, ..., y_{ν - 1}), returns two polynomials:
    /// f(0, y_1, ..., y_{ν - 1}) and f(1, y_1, ..., y_{ν - 1}).
    pub fn project_least_significant_variable(&self) -> (Self, Self) {
        let mut p0 = Vec::with_capacity(self.num_evaluations() / 2);
        let mut p1 = Vec::with_capacity(self.num_evaluations() / 2);
        for chunk in self.evaluations.chunks_exact(2) {
            p0.push(chunk[0]);
            p1.push(chunk[1]);
        }

        (
            MultiLinearPoly::from_evaluations(p0).unwrap(),
            MultiLinearPoly::from_evaluations(p1).unwrap(),
        )
    }
}

impl<E: FieldElement> Index<usize> for MultiLinearPoly<E> {
    type Output = E;

    fn index(&self, index: usize) -> &E {
        &(self.evaluations[index])
    }
}

// COMPOSITION POLYNOMIAL
// ================================================================================================

/// A multi-variate polynomial for composing individual multi-linear polynomials.
pub trait CompositionPolynomial<E: FieldElement> {
    /// Maximum degree in all variables.
    fn max_degree(&self) -> u32;

    /// Given a query, of length equal the number of variables, evaluates [Self] at this query.
    fn evaluate(&self, query: &[E]) -> E;
}
