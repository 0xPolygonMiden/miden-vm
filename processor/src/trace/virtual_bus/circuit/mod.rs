use crate::trace::virtual_bus::{
    multilinear::{CompositionPolynomial, EqFunction},
    sum_check::RoundProof,
};
use alloc::{borrow::ToOwned, sync::Arc, vec::Vec};
use miden_air::trace::TRACE_WIDTH;
use vm_core::{Felt, FieldElement};

mod error;
mod prover;
pub use prover::prove;

mod verifier;
pub use verifier::verify;

use super::sum_check::Proof as SumCheckProof;

/// A GKR proof for the correct evaluation of the sum of fractions circuit.
#[derive(Debug)]
pub struct GkrCircuitProof<E: FieldElement + 'static> {
    circuit_outputs: [E; 4],
    before_final_layer_proofs: BeforeFinalLayerProof<E>,
    final_layer_proof: FinalLayerProof<E>,
}

/// A set of sum-check proofs for all GKR layers but for the input circuit layer.
#[derive(Debug)]
pub struct BeforeFinalLayerProof<E: FieldElement + 'static> {
    pub proof: Vec<SumCheckProof<E>>,
}

/// A proof for the input circuit layer i.e., the final layer in the GKR protocol.
#[derive(Debug)]
pub struct FinalLayerProof<E: FieldElement> {
    before_merge_proof: Vec<RoundProof<E>>,
    after_merge_proof: SumCheckProof<E>,
}

/// Represents a claim to be proven by next sum-check protocol.
#[derive(Debug)]
pub struct GkrClaim<E: FieldElement + 'static> {
    pub evaluation_point: Vec<E>,
    pub claimed_evaluation: (E, E),
}

/// A composition polynomial that projects into a specific component.
#[derive(Clone, Copy, Debug)]
pub struct ProjectionComposition {
    coordinate: usize,
}

impl ProjectionComposition {
    pub fn new(coordinate: usize) -> Self {
        Self { coordinate }
    }
}

impl<E> CompositionPolynomial<E> for ProjectionComposition
where
    E: FieldElement,
{
    fn num_variables(&self) -> usize {
        1
    }

    fn max_degree(&self) -> usize {
        1
    }

    fn evaluate(&self, query: &[E]) -> E {
        query[self.coordinate]
    }
}

/// A composition polynomial used in the GKR protocol for all of its sum-checks except the final one.
#[derive(Clone)]
pub struct GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub combining_randomness: E,
}

impl<E> GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub fn new(combining_randomness: E) -> Self {
        Self {
            combining_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrComposition<E>
where
    E: FieldElement<BaseField = Felt>,
{
    fn num_variables(&self) -> usize {
        5
    }

    fn max_degree(&self) -> usize {
        3
    }

    fn evaluate(&self, query: &[E]) -> E {
        let eval_left_numerator = query[0];
        let eval_right_numerator = query[1];
        let eval_left_denominator = query[2];
        let eval_right_denominator = query[3];
        let eq_eval = query[4];
        eq_eval
            * ((eval_left_numerator * eval_right_denominator
                + eval_right_numerator * eval_left_denominator)
                + eval_left_denominator * eval_right_denominator * self.combining_randomness)
    }
}

/// A composition polynomial used in the GKR protocol for its final sum-check.
#[derive(Clone)]
pub struct GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub sum_check_combining_randomness: E,
    pub tensored_merge_randomness: Vec<E>,
    pub degree: usize,

    pub eq_composer: Arc<dyn CompositionPolynomial<E>>,
    pub right_numerator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
    pub left_numerator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
    pub right_denominator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
    pub left_denominator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
}

impl<E> GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    pub fn new(
        combining_randomness: E,
        merge_randomness: Vec<E>,
        eq_composer: Arc<dyn CompositionPolynomial<E>>,
        right_numerator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
        left_numerator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
        right_denominator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
        left_denominator_composer: Vec<Arc<dyn CompositionPolynomial<E>>>,
    ) -> Self {
        let tensored_merge_randomness =
            EqFunction::ml_at(merge_randomness.clone()).evaluations().to_vec();

        let max_left_num = left_numerator_composer.iter().map(|c| c.max_degree()).max().unwrap();
        let max_right_num = right_numerator_composer.iter().map(|c| c.max_degree()).max().unwrap();
        let max_left_denom =
            left_denominator_composer.iter().map(|c| c.max_degree()).max().unwrap();
        let max_right_denom =
            right_denominator_composer.iter().map(|c| c.max_degree()).max().unwrap();
        let degree =
            1 + core::cmp::max(max_left_num + max_right_denom, max_right_num + max_left_denom);

        Self {
            sum_check_combining_randomness: combining_randomness,
            eq_composer,
            degree,
            right_numerator_composer,
            left_numerator_composer,
            right_denominator_composer,
            left_denominator_composer,
            tensored_merge_randomness,
        }
    }
}

impl<E> CompositionPolynomial<E> for GkrCompositionMerge<E>
where
    E: FieldElement<BaseField = Felt>,
{
    fn num_variables(&self) -> usize {
        TRACE_WIDTH
    }

    fn max_degree(&self) -> usize {
        self.degree
    }

    fn evaluate(&self, query: &[E]) -> E {
        let eval_right_numerator =
            self.right_numerator_composer.iter().enumerate().fold(E::ZERO, |acc, (i, ml)| {
                acc + ml.evaluate(query) * self.tensored_merge_randomness[i]
            });
        let eval_left_numerator =
            self.left_numerator_composer.iter().enumerate().fold(E::ZERO, |acc, (i, ml)| {
                acc + ml.evaluate(query) * self.tensored_merge_randomness[i]
            });
        let eval_right_denominator = self
            .right_denominator_composer
            .iter()
            .enumerate()
            .fold(E::ZERO, |acc, (i, ml)| {
                acc + ml.evaluate(query) * self.tensored_merge_randomness[i]
            });
        let eval_left_denominator =
            self.left_denominator_composer.iter().enumerate().fold(E::ZERO, |acc, (i, ml)| {
                acc + ml.evaluate(query) * self.tensored_merge_randomness[i]
            });
        let eq_eval = self.eq_composer.evaluate(query);
        eq_eval
            * ((eval_left_numerator * eval_right_denominator
                + eval_right_numerator * eval_left_denominator)
                + eval_left_denominator
                    * eval_right_denominator
                    * self.sum_check_combining_randomness)
    }
}

/// Generates a [GkrCompositionMerge] given the sum-check randomness so-far and the random value
/// for batching two sum-checks to one.
pub fn gkr_merge_composition_from_composition_polys<E: FieldElement<BaseField = Felt> + 'static>(
    composition_polys: &[Vec<Arc<dyn CompositionPolynomial<E>>>],
    sum_check_batch_randomness: E,
    merge_randomness: Vec<E>,
) -> GkrCompositionMerge<E> {
    let eq_composer = Arc::new(ProjectionComposition::new(TRACE_WIDTH));
    let left_numerator = composition_polys[0].to_owned();
    let right_numerator = composition_polys[1].to_owned();
    let left_denominator = composition_polys[2].to_owned();
    let right_denominator = composition_polys[3].to_owned();
    GkrCompositionMerge::new(
        sum_check_batch_randomness,
        merge_randomness,
        eq_composer,
        right_numerator,
        left_numerator,
        right_denominator,
        left_denominator,
    )
}
