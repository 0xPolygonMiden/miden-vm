use super::{
    error::VerifierError, gkr_merge_composition_from_composition_polys, FinalLayerProof,
    GkrCircuitProof, GkrComposition,
};
use crate::trace::virtual_bus::{
    multilinear::{CompositionPolynomial, EqFunction},
    sum_check::{
        CompositionPolyQueryBuilder, FinalOpeningClaim, Proof as SumCheckFullProof, RoundClaim,
    },
    SumCheckVerifier,
};
use alloc::{borrow::ToOwned, sync::Arc, vec::Vec};
use vm_core::{Felt, FieldElement};
use winter_prover::crypto::{ElementHasher, RandomCoin};

/// Verifies the validity of a GKR proof for the correct evaluation of a fractional sum circuit.
pub fn verify<
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    claim: E,
    proof: GkrCircuitProof<E>,
    composition_polys: Vec<Vec<Arc<dyn CompositionPolynomial<E>>>>,
    transcript: &mut C,
) -> Result<FinalOpeningClaim<E>, VerifierError> {
    let GkrCircuitProof {
        circuit_outputs,
        before_final_layer_proofs,
        final_layer_proof,
    } = proof;

    let p0 = circuit_outputs[0];
    let p1 = circuit_outputs[1];
    let q0 = circuit_outputs[2];
    let q1 = circuit_outputs[3];

    // make sure that both denominators are not equal to E::ZERO
    if q0 == E::ZERO || q1 == E::ZERO {
        return Err(VerifierError::ZeroOutputDenominator);
    }

    // check that the output matches the expected `claim`
    if (p0 * q1 + p1 * q0) / (q0 * q1) != claim {
        return Err(VerifierError::MismatchingCircuitOutput);
    }

    // generate the random challenge to reduce two claims into a single claim
    transcript.reseed(H::hash_elements(&circuit_outputs));
    let r = transcript.draw().map_err(|_| VerifierError::FailedGenerateRandomness)?;

    // reduce the claim
    let p_r = p0 + r * (p1 - p0);
    let q_r = q0 + r * (q1 - q0);
    let mut reduced_claim = (p_r, q_r);

    // verify all GKR layers but for the last one
    let num_layers = before_final_layer_proofs.proof.len();
    let mut rand = vec![r];
    for i in 0..num_layers {
        let FinalOpeningClaim {
            eval_point,
            openings,
        } = verify_sum_check_proof_before_last(
            &before_final_layer_proofs.proof[i],
            &rand,
            reduced_claim,
            transcript,
        )?;

        // generate the random challenge to reduce two claims into a single claim
        transcript.reseed(H::hash_elements(&openings));
        let r_layer = transcript.draw().unwrap();

        let p0 = openings[0];
        let p1 = openings[1];
        let q0 = openings[2];
        let q1 = openings[3];
        reduced_claim = (p0 + r_layer * (p1 - p0), q0 + r_layer * (q1 - q0));

        // collect the randomness used for the current layer
        let rand_sumcheck = eval_point;
        let mut ext = rand_sumcheck;
        ext.push(r_layer);
        rand = ext;
    }

    // verify the proof of the final GKR layer and pass final opening claim for verification
    // to the STARK
    verify_sum_check_proof_last(
        composition_polys,
        final_layer_proof,
        &rand,
        reduced_claim,
        transcript,
    )
}

/// Verifies sum-check proofs, as part of the GKR proof, for all GKR layers except for the last one
/// i.e., the circuit input layer.
pub fn verify_sum_check_proof_before_last<
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    proof: &SumCheckFullProof<E>,
    gkr_eval_point: &[E],
    claim: (E, E),
    transcript: &mut C,
) -> Result<FinalOpeningClaim<E>, VerifierError> {
    // generate challenge to batch sum-checks
    transcript.reseed(H::hash_elements(&[claim.0, claim.1]));
    let r_batch: E = transcript.draw().unwrap();

    // compute the claim for the batched sum-check
    let reduced_claim = claim.0 + claim.1 * r_batch;

    // verify the sum-check protocol
    let composition_poly = GkrComposition::new(r_batch);
    let verifier =
        SumCheckVerifier::new(composition_poly, GkrQueryBuilder::new(gkr_eval_point.to_owned()));
    verifier
        .verify(reduced_claim, proof.clone(), transcript)
        .map_err(|_| VerifierError::FailedToVerifySumCheck)
}

/// Verifies the final sum-check proof as part of the GKR proof.
pub fn verify_sum_check_proof_last<
    E: FieldElement<BaseField = Felt> + 'static,
    C: RandomCoin<Hasher = H, BaseField = Felt>,
    H: ElementHasher<BaseField = Felt>,
>(
    composition_polys: Vec<Vec<Arc<dyn CompositionPolynomial<E>>>>,
    proof: FinalLayerProof<E>,
    gkr_eval_point: &[E],
    claim: (E, E),
    transcript: &mut C,
) -> Result<FinalOpeningClaim<E>, VerifierError> {
    let FinalLayerProof {
        before_merge_proof,
        after_merge_proof,
    } = proof;

    // generate challenge to batch sum-checks
    transcript.reseed(H::hash_elements(&[claim.0, claim.1]));
    let r_sum_check: E = transcript.draw().unwrap();

    // compute the claim for the batched sum-check
    let reduced_claim = claim.0 + claim.1 * r_sum_check;

    // verify the first part of the sum-check protocol
    let composition_poly = GkrComposition::new(r_sum_check);
    let verifier =
        SumCheckVerifier::new(composition_poly, GkrQueryBuilder::new(gkr_eval_point.to_owned()));
    let RoundClaim {
        eval_point: rand_merge,
        claim,
    } = verifier
        .verify_rounds(reduced_claim, before_merge_proof, transcript)
        .map_err(|_| VerifierError::FailedToVerifySumCheck)?;

    // verify the second part of the sum-check protocol
    let gkr_composition = gkr_merge_composition_from_composition_polys(
        &composition_polys,
        r_sum_check,
        rand_merge.clone(),
    );
    let verifier = SumCheckVerifier::new(
        gkr_composition,
        GkrMergeQueryBuilder::new(gkr_eval_point.to_owned(), rand_merge),
    );
    verifier
        .verify(claim, after_merge_proof, transcript)
        .map_err(|_| VerifierError::FailedToVerifySumCheck)
}

/// A [FinalQueryBuilder] for the sum-check verifier used for all sum-checks but for the final one.
#[derive(Default)]
struct GkrQueryBuilder<E> {
    gkr_eval_point: Vec<E>,
}

impl<E> GkrQueryBuilder<E> {
    fn new(gkr_eval_point: Vec<E>) -> Self {
        Self { gkr_eval_point }
    }
}

impl<E: FieldElement> CompositionPolyQueryBuilder<E> for GkrQueryBuilder<E> {
    fn build_query(&self, openings_claim: &FinalOpeningClaim<E>, evaluation_point: &[E]) -> Vec<E> {
        let rand_sumcheck = evaluation_point;
        let eq_at_gkr_eval_point = EqFunction::new(self.gkr_eval_point.clone());
        let eq = eq_at_gkr_eval_point.evaluate(rand_sumcheck);

        let mut query = openings_claim.openings.clone();
        query.push(eq);
        query
    }
}

/// A [FinalQueryBuilder] for the sum-check verifier used for the final sum-check.
#[derive(Default)]
struct GkrMergeQueryBuilder<E> {
    gkr_eval_point: Vec<E>,
    merge_rand: Vec<E>,
}

impl<E> GkrMergeQueryBuilder<E> {
    fn new(gkr_eval_point: Vec<E>, merge_rand: Vec<E>) -> Self {
        Self {
            gkr_eval_point,
            merge_rand,
        }
    }
}

impl<E: FieldElement> CompositionPolyQueryBuilder<E> for GkrMergeQueryBuilder<E> {
    fn build_query(&self, openings_claim: &FinalOpeningClaim<E>, evaluation_point: &[E]) -> Vec<E> {
        let eq_at_gkr_eval_point = EqFunction::new(self.gkr_eval_point.clone());
        let mut rand_sumcheck = self.merge_rand.clone();
        rand_sumcheck.extend_from_slice(evaluation_point);
        let eq = eq_at_gkr_eval_point.evaluate(&rand_sumcheck);
        let mut query = openings_claim.openings.clone();
        query.push(eq);
        query
    }
}
