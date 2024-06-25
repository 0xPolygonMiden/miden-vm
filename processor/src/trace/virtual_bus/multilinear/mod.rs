use vm_core::FieldElement;

mod lagrange_ker;
pub use lagrange_ker::EqFunction;

mod error;

// COMPOSITION POLYNOMIAL
// ================================================================================================

/// A multi-variate polynomial for composing individual multi-linear polynomials.
pub trait CompositionPolynomial<E: FieldElement> {
    /// Maximum degree in all variables.
    fn max_degree(&self) -> u32;

    /// Given a query, of length equal the number of variables, evaluates [Self] at this query.
    fn evaluate(&self, query: &[E]) -> E;
}
