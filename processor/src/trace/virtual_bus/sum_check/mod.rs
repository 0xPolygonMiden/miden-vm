use vm_core::FieldElement;

use self::domain::EvaluationDomain;

mod domain;
mod prover;
mod verifier;

#[derive(Debug, Clone)]
pub struct RoundProof<E> {
    pub poly_evals: Vec<E>,
}

impl<E: FieldElement> RoundProof<E> {
    pub fn to_evals(&self, claim: E) -> Vec<E> {
        let mut result = vec![];

        // s(0) + s(1) = claim
        let c0 = claim - self.poly_evals[0];

        result.push(c0);
        result.extend_from_slice(&self.poly_evals);
        result
    }

    // TODO: refactor once we move to coefficient form
    pub(crate) fn evaluate(&self, domain: EvaluationDomain<E>, claim: E, r: E) -> E {
        let poly_evals = self.to_evals(claim);
        domain.evaluate(&poly_evals, r)
    }
}

#[derive(Debug, Clone)]
pub struct Proof<E> {
    pub round_proofs: Vec<RoundProof<E>>,
}

#[derive(Debug)]
pub struct RoundClaim<E: FieldElement> {
    pub partial_eval_point: Vec<E>,
    pub current_claim: E,
}

pub fn reduce_claim<E: FieldElement>(
    domain: &EvaluationDomain<E>,
    current_poly: &RoundProof<E>,
    current_round_claim: RoundClaim<E>,
    round_challenge: E,
) -> RoundClaim<E> {
    let poly_evals = current_poly.to_evals(current_round_claim.current_claim);
    let new_claim = domain.evaluate(&poly_evals, round_challenge);

    let mut new_partial_eval_point = current_round_claim.partial_eval_point;
    new_partial_eval_point.push(round_challenge);

    RoundClaim {
        partial_eval_point: new_partial_eval_point,
        current_claim: new_claim,
    }
}

#[derive(Clone)]
pub struct FinalEvaluationClaim<E: FieldElement> {
    pub evaluation_point: Vec<E>,
    pub claimed_evaluation: E,
}

pub fn eval<E>(p: &[E], x: E) -> E
where
    E: FieldElement,
{
    // Horner evaluation
    p.iter().rev().fold(E::ZERO, |acc, &coeff| acc * x + coeff)
}

#[cfg(test)]
mod test {
    use super::{
        domain::EvaluationDomain, eval, prover::SumCheckProver, verifier::SumCheckVerifier,
        RoundClaim,
    };
    use crate::trace::virtual_bus::multilinear::{MultiLinear, ProjectionComposition};
    use test_utils::{
        crypto::Rpo256,
        rand::{rand_array, rand_value, rand_vector},
    };
    use vm_core::{crypto::random::RpoRandomCoin, Felt, Word, ZERO};

    #[test]
    fn test_evaluation_domain() {
        let max_degree = 5;
        let eval_domain = EvaluationDomain::<Felt>::new(max_degree);

        let r = rand_value();
        let coefficients: [Felt; 6] = rand_array();

        let evaluations: Vec<Felt> = (0..=max_degree)
            .into_iter()
            .map(|x| eval(&coefficients, Felt::from(x as u8)))
            .collect();

        assert_eq!(coefficients.len(), evaluations.len());

        let result = eval_domain.evaluate(&evaluations, r);
        let expected = eval(&coefficients, r);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_sum_check() {
        let num_variables = 10;
        let values = rand_vector(1 << num_variables);
        let claim = values.iter().fold(ZERO, |acc, &x| x + acc);
        let ml = MultiLinear::new(values.to_vec());
        let mut mls = vec![ml.clone()];
        let virtual_poly = ProjectionComposition::new(0);
        let prover =
            SumCheckProver::<Felt, Felt, ProjectionComposition, RpoRandomCoin, Rpo256>::new(
                virtual_poly,
                num_variables,
            );

        let mut coin = RpoRandomCoin::new(Word::default());
        let (final_claim, round_proofs) = prover.prove(claim, &mut mls, &mut coin);

        let verifier =
            SumCheckVerifier::<Felt, Felt, ProjectionComposition, RpoRandomCoin, Rpo256>::new(
                virtual_poly,
                num_variables,
            );

        let mut coin = RpoRandomCoin::new(Word::default());
        verifier.verify(claim, round_proofs, &mut coin);

        let RoundClaim {
            partial_eval_point,
            current_claim,
        } = final_claim;

        assert_eq!(ml.evaluate(&partial_eval_point), current_claim);
    }
}
