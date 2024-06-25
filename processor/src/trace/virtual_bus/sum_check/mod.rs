use miden_air::gkr_proof::{RoundProof, SumCheckRoundClaim};
use vm_core::FieldElement;

mod prover;
pub use prover::{Error as SumCheckProverError, FinalClaimBuilder, SumCheckProver};

/// Reduces an old claim to a new claim using the round challenge.
pub fn reduce_claim<E: FieldElement>(
    current_poly: &RoundProof<E>,
    current_round_claim: SumCheckRoundClaim<E>,
    round_challenge: E,
) -> SumCheckRoundClaim<E> {
    // evaluate the round polynomial at the round challenge to obtain the new claim
    let new_claim = current_poly
        .round_poly_coefs
        .evaluate_using_claim(&current_round_claim.claim, &round_challenge);

    // update the evaluation point using the round challenge
    let mut new_partial_eval_point = current_round_claim.eval_point;
    new_partial_eval_point.push(round_challenge);

    SumCheckRoundClaim {
        eval_point: new_partial_eval_point,
        claim: new_claim,
    }
}

#[cfg(test)]
mod test {
    use super::prover::{FinalClaimBuilder, SumCheckProver};
    use alloc::{borrow::ToOwned, vec::Vec};
    use miden_air::{
        gkr_proof::{CompositionPolynomial, FinalOpeningClaim, MultiLinearPoly},
        CompositionPolyQueryBuilder, SumCheckVerifier,
    };
    use test_utils::rand::rand_vector;
    use vm_core::{crypto::random::RpoRandomCoin, Felt, FieldElement, Word, ONE, ZERO};

    #[test]
    fn test_sum_check_sum() {
        let num_variables = 14;
        let values = rand_vector(1 << num_variables);
        let claim = values.iter().fold(ZERO, |acc, &x| x + acc);

        let ml = MultiLinearPoly::from_evaluations(values.to_vec()).expect("should not fail");
        let mls = vec![ml];
        let virtual_poly = ProjectionComposition::new(0);

        // Prover
        let prover = SumCheckProver::new(virtual_poly, PlainClaimBuilder);
        let mut coin = RpoRandomCoin::new(Word::default());
        let proof = prover.prove(claim, mls, &mut coin).unwrap();

        // Verifier
        let plain_query_builder = ProjectionPolyQueryBuilder::default();
        let verifier = SumCheckVerifier::new(virtual_poly, plain_query_builder);
        let mut coin = RpoRandomCoin::new(Word::default());
        let result = verifier.verify(claim, proof, &mut coin);

        assert!(result.is_ok())
    }

    #[test]
    fn test_sum_check_product() {
        let num_variables = 14;
        let values_0 = rand_vector(1 << num_variables);
        let values_1 = rand_vector(1 << num_variables);
        let claim = values_0.iter().zip(values_1.iter()).fold(ZERO, |acc, (x, y)| *x * *y + acc);

        let ml_0 = MultiLinearPoly::from_evaluations(values_0.to_vec()).expect("should not fail");
        let ml_1 = MultiLinearPoly::from_evaluations(values_1.to_vec()).expect("should not fail");
        let mls = vec![ml_0, ml_1];
        let virtual_poly = ProductComposition;

        // Prover
        let prover = SumCheckProver::new(virtual_poly, PlainClaimBuilder);
        let mut coin = RpoRandomCoin::new(Word::default());
        let proof = prover.prove(claim, mls, &mut coin).unwrap();

        // Verifier
        let plain_query_builder = ProjectionPolyQueryBuilder::default();
        let verifier = SumCheckVerifier::new(virtual_poly, plain_query_builder);
        let mut coin = RpoRandomCoin::new(Word::default());
        let result = verifier.verify(claim, proof, &mut coin);

        assert!(result.is_ok())
    }

    #[test]
    fn test_sum_check_product_failure() {
        let num_variables = 14;
        let values_0 = rand_vector(1 << num_variables);
        let values_1 = rand_vector(1 << num_variables);
        let mut claim =
            values_0.iter().zip(values_1.iter()).fold(ZERO, |acc, (x, y)| *x * *y + acc);

        // modifying the claim should make the Verifier reject the proof
        claim += ONE;

        let ml_0 = MultiLinearPoly::from_evaluations(values_0.to_vec()).expect("should not fail");
        let ml_1 = MultiLinearPoly::from_evaluations(values_1.to_vec()).expect("should not fail");
        let mls = vec![ml_0, ml_1];
        let virtual_poly = ProductComposition;

        // Prover
        let prover = SumCheckProver::new(virtual_poly, PlainClaimBuilder);
        let mut coin = RpoRandomCoin::new(Word::default());
        let proof = prover.prove(claim, mls, &mut coin).unwrap();

        // Verifier
        let plain_query_builder = ProjectionPolyQueryBuilder::default();
        let verifier = SumCheckVerifier::new(virtual_poly, plain_query_builder);
        let mut coin = RpoRandomCoin::new(Word::default());
        let result = verifier.verify(claim, proof, &mut coin);

        assert!(result.is_err())
    }

    struct PlainClaimBuilder;

    impl FinalClaimBuilder for PlainClaimBuilder {
        type Field = Felt;

        fn build_claim(
            &self,
            openings: Vec<Self::Field>,
            evaluation_point: &[Self::Field],
        ) -> FinalOpeningClaim<Self::Field> {
            FinalOpeningClaim {
                eval_point: evaluation_point.to_owned(),
                openings,
            }
        }
    }

    #[derive(Default)]
    struct ProjectionPolyQueryBuilder;

    impl<E: FieldElement> CompositionPolyQueryBuilder<E> for ProjectionPolyQueryBuilder {
        fn build_query(
            &self,
            openings_claim: &FinalOpeningClaim<E>,
            _evaluation_point: &[E],
        ) -> Vec<E> {
            openings_claim.openings.to_vec()
        }
    }

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
        fn max_degree(&self) -> u32 {
            1
        }

        fn evaluate(&self, query: &[E]) -> E {
            query[self.coordinate]
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct ProductComposition;

    impl<E> CompositionPolynomial<E> for ProductComposition
    where
        E: FieldElement,
    {
        fn max_degree(&self) -> u32 {
            2
        }

        fn evaluate(&self, query: &[E]) -> E {
            assert_eq!(query.len(), 2);
            query[0] * query[1]
        }
    }
}
