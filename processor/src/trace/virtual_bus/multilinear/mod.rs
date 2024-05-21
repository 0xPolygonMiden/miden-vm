use alloc::{borrow::ToOwned, vec::Vec};
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

    /// Computes f(r_0, y_1, ..., y_{ν - 1}) using the linear interpolation formula
    /// (1 - r_0) * f(0, y_1, ..., y_{ν - 1}) + r_0 * f(1, y_1, ..., y_{ν - 1}) and assigns
    /// the resulting multi-linear, defined over a domain of half the size, to `self`.
    pub fn bind(&mut self, round_challenge: E) {
        let mut result = vec![E::ZERO; 1 << (self.num_variables() - 1)];
        for (i, res) in result.iter_mut().enumerate() {
            *res = self.evaluations[i << 1]
                + round_challenge * (self.evaluations[(i << 1) + 1] - self.evaluations[i << 1]);
        }
        *self = Self::from_evaluations(result.to_owned())
            .expect("should not fail given that it is a multi-linear");
    }

    /// Given two instances of [`MultiLinearPoly`], f(x_0, x_1, ..., x_{ν - 1}) and
    /// g(x_0, x_1, ..., x_{ν - 1}), constructs the following polynomial defined by
    ///
    /// merge(f, g)(x_0, x_1, ..., x_{ν - 1}, z) := (1 - z) * f(x_0, x_1, ..., x_{ν - 1})
    ///                                                      + z * g(x_0, x_1, ..., x_{ν - 1})
    /// Notice that:
    ///
    /// 1. merge(f, g)(x_0, x_1, ..., x_{ν - 1}, 0) = f(x_0, x_1, ..., x_{ν - 1})
    /// 2. merge(f, g)(x_0, x_1, ..., x_{ν - 1}, 1) = g(x_0, x_1, ..., x_{ν - 1})
    pub fn extend(&mut self, other: &MultiLinearPoly<E>) {
        let other_vec = other.evaluations.to_vec();
        assert_eq!(other_vec.len(), self.evaluations().len());
        self.evaluations.extend(other_vec);
        self.num_variables += 1;
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
    /// The number of variables when interpreted as a multi-variate polynomial.
    fn num_variables(&self) -> u32;

    /// Maximum degree in all variables.
    fn max_degree(&self) -> u32;

    /// Given a query, of length equal the number of variables, evaluates [Self] at this query.
    fn evaluate(&self, query: &[E]) -> E;
}
