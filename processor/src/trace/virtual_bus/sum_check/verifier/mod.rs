use crate::trace::virtual_bus::multilinear::CompositionPolynomial;

use super::{domain::EvaluationDomain, FinalEvaluationClaim, Proof};
use core::marker::PhantomData;
use vm_core::{FieldElement, StarkField};
use winter_prover::crypto::{ElementHasher, RandomCoin};

pub struct SumCheckVerifier<B, E, P, C, H>
where
    B: StarkField,
    E: FieldElement<BaseField = B>,
    C: RandomCoin<Hasher = H, BaseField = B>,
    P: CompositionPolynomial<E>,
    H: ElementHasher<BaseField = B>,
{
    pub virtual_poly: P,
    pub eval_domain: EvaluationDomain<E>,
    pub num_rounds: usize,
    _channel: PhantomData<C>,
}

impl<B, E, P, C, H> SumCheckVerifier<B, E, P, C, H>
where
    B: StarkField,
    E: FieldElement<BaseField = B>,
    C: RandomCoin<Hasher = H, BaseField = B>,
    P: CompositionPolynomial<E>,
    H: ElementHasher<BaseField = B>,
{
    pub fn new(virtual_poly: P, num_rounds: usize) -> Self {
        let max_degree = virtual_poly.max_degree();
        let eval_domain = EvaluationDomain::new(max_degree);

        Self {
            virtual_poly,
            eval_domain,
            num_rounds,
            _channel: PhantomData,
        }
    }

    pub fn verify(
        &self,
        claim: E,
        round_proofs: Proof<E>,
        coin: &mut C,
    ) -> FinalEvaluationClaim<E> {
        let mut claimed_evaluation = claim;
        let mut evaluation_point = vec![];
        for proof in round_proofs.round_proofs {
            let partial_evals = proof.poly_evals.clone();
            coin.reseed(H::hash_elements(&partial_evals));
            let evals = proof.to_evals(claimed_evaluation);

            let r = coin.draw().unwrap();
            let reduced_evaluation = self.eval_domain.evaluate(&evals, r);

            claimed_evaluation = reduced_evaluation;
            evaluation_point.push(r);
        }

        FinalEvaluationClaim {
            evaluation_point,
            claimed_evaluation,
        }
    }
}
