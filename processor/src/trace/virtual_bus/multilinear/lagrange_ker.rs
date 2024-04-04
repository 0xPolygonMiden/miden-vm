use super::{tensorize, FieldElement, MultiLinear};
use alloc::vec::Vec;

/// The EQ (equality) function is the binary function defined by
///
/// EQ:    {0 , 1}^ν ⛌ {0 , 1}^ν ⇾ {0 , 1}
///   ((x_0, ..., x_{ν - 1}), (y_0, ..., y_{ν - 1})) ↦ \prod_{i = 0}^{ν - 1} (x_i * y_i + (1 - x_i) * (1 - y_i))
///
/// Taking It's multi-linear extension EQ^{~}, we can define a basis for the set of multi-linear
/// polynomials in ν variables by
///         {EQ^{~}(., (y_0, ..., y_{ν - 1})): (y_0, ..., y_{ν - 1}) ∈ {0 , 1}^ν}
/// where each basis function is a function of its first argument. This is called the Lagrange or
/// evaluation basis with evaluation set {0 , 1}^ν.
///
/// Given a function (f: {0 , 1}^ν ⇾ 𝔽), its multi-linear extension (i.e., the unique
/// mult-linear polynomial extending f to (f^{~}: 𝔽^ν ⇾ 𝔽) and agrees with it on {0 , 1}^ν) is
/// defined as the summation of the evaluations of f against the Lagrange basis.
/// More specifically, given (r_0, ..., r_{ν - 1}) ∈ 𝔽^ν, then:
///
///     f^{~}(r_0, ..., r_{ν - 1}) = \sum_{(y_0, ..., y_{ν - 1}) ∈ {0 , 1}^ν}
///                  f(y_0, ..., y_{ν - 1}) EQ^{~}((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1}))
///
/// We call the Lagrange kernel the evaluation of the EQ^{~} function at
/// ((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1})) for all (y_0, ..., y_{ν - 1}) ∈ {0 , 1}^ν for
/// a fixed (r_0, ..., r_{ν - 1}) ∈ 𝔽^ν.
///
/// [EqFunction] represents EQ^{~} the mult-linear extension of
///
/// ((y_0, ..., y_{ν - 1}) ↦ EQ((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1})))
///
/// and contains a method to generate the Lagrange kernel for defining evaluations of multi-linear
/// extensions of arbitrary functions (f: {0 , 1}^ν ⇾ 𝔽) at a given point (r_0, ..., r_{ν - 1})
/// as well as a method to evaluate EQ^{~}((r_0, ..., r_{ν - 1}), (t_0, ..., t_{ν - 1}))) for
/// (t_0, ..., t_{ν - 1}) ∈ 𝔽^ν.
pub struct EqFunction<E> {
    r: Vec<E>,
}

impl<E: FieldElement> EqFunction<E> {
    /// Creates a new [EqFunction].
    pub fn new(r: Vec<E>) -> Self {
        let tmp = r.clone();
        EqFunction { r: tmp }
    }

    /// Computes EQ((r_0, ..., r_{ν - 1}), (t_0, ..., t_{ν - 1}))).
    pub fn evaluate(&self, t: &[E]) -> E {
        assert_eq!(self.r.len(), t.len());

        (0..self.r.len())
            .map(|i| self.r[i] * t[i] + (E::ONE - self.r[i]) * (E::ONE - t[i]))
            .fold(E::ONE, |acc, term| acc * term)
    }

    /// Computes EQ((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1})) for all
    /// (y_0, ..., y_{ν - 1}) ∈ {0 , 1}^ν i.e., the Lagrange kernel at r = (r_0, ..., r_{ν - 1}).
    pub fn evaluations(&self) -> Vec<E> {
        tensorize(&self.r)
    }

    /// Returns the evaluations of
    /// ((y_0, ..., y_{ν - 1}) ↦ EQ^{~}((r_0, ..., r_{ν - 1}), (y_0, ..., y_{ν - 1})))
    /// over {0 , 1}^ν.
    pub fn ml_at(evaluation_point: Vec<E>) -> MultiLinear<E> {
        let eq_evals = EqFunction::new(evaluation_point.clone()).evaluations();
        MultiLinear::from_values(&eq_evals)
            .expect("should not fail because evaluations is a power of two")
    }
}
