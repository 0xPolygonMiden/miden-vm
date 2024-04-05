use super::multilinear::CompositionPolynomial;
use alloc::{sync::Arc, vec::Vec};
use vm_core::FieldElement;

mod range_checker;
pub use range_checker::*;

/// Defines a sub-bus of the global virtual bus
pub trait BusBuilder<E: FieldElement> {
    fn compute_initial_claim(&self) -> E;

    fn build_numerators(&self) -> Vec<Arc<dyn CompositionPolynomial<E>>>;

    fn build_denominators(&self) -> Vec<Arc<dyn CompositionPolynomial<E>>>;
}
