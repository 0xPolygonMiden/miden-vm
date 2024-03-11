use std::iter::zip;

use vm_core::{
    crypto::{hash::ElementHasher, random::RandomCoin},
    FieldElement,
};

#[derive(Clone, Debug)]
pub struct PolynomialCommitment<H>
where
    H: ElementHasher,
{
    pub commitment: H::Digest,
}

#[derive(Clone, Debug)]
pub struct CommitmentProof;

#[derive(Clone, Debug)]
pub struct CommittedValue<E: FieldElement> {
    pub value: E,
    pub commitment_proof: CommitmentProof,
}

/// A sum-check proof. Consists of multiple rounds, followed by a final check, and the value of the
/// evaluation of the grand sum, as claimed by the prover.
///
/// This implements a generalized version of the traditional sum-check protocol, as described in
/// [this issue](https://github.com/0xPolygonMiden/miden-vm/issues/1182).
#[derive(Clone, Debug)]
pub struct SumCheckProof<E: FieldElement, H: ElementHasher> {
    pub rounds: Vec<SumCheckRound<E, H>>,
    pub final_check: FinalCheck<E, H>,
    pub claimed_sum_evaluation: E,
}

/// In each round, the prover commits to a univariate "round polynomial" `p`, and opens it at 0, 1
/// and a random `r` provided by the verifier. This round polynomial is designed such that `p(0) +
/// p(1) = claim`, where `claim` is `p_prev(r_prev)`.
#[derive(Clone, Debug)]
pub struct SumCheckRound<E: FieldElement, H: ElementHasher> {
    pub round_poly_commit: PolynomialCommitment<H>,
    pub round_poly_at_0: CommittedValue<E>,
    pub round_poly_at_1: CommittedValue<E>,
    pub round_poly_at_r: CommittedValue<E>,
}

/// Provides the data necessary for the verifier to perform the final check of the protocol. We call
/// the multivariate polynomials `p_1(x1, ..., xu), ..., p_v(x1, ..., xu)` the "input polynomials".
/// The prover commits to each, and opens them at a random `(r1, ..., ru)` provided by the verifier.
#[derive(Clone, Debug)]
pub struct FinalCheck<E: FieldElement, H: ElementHasher> {
    pub input_poly_commits: Vec<PolynomialCommitment<H>>,
    pub input_poly_openings_at_r: Vec<CommittedValue<E>>,
}

pub struct VerificationError;

/// An instance of a sum-check problem
pub trait SumCheckInstance<E: FieldElement> {
    /// The public function g(p_1(x), ..., p_v(x))
    fn g(&self, poly_evals: Vec<E>) -> E;
}

pub fn sumcheck_verify<E, H, R, I>(
    proof: SumCheckProof<E, H>,
    transcript: &mut R,
    instance: I,
) -> Result<(), VerificationError>
where
    E: FieldElement,
    H: ElementHasher<BaseField = E>,
    R: RandomCoin<BaseField = E::BaseField, Hasher = H>,
    I: SumCheckInstance<E>,
{
    let SumCheckProof {
        rounds,
        final_check,
        claimed_sum_evaluation,
    } = proof;
    let mut challenges: Vec<E::BaseField> = Vec::new();

    // check first round
    {
        let round_0 = rounds.first().unwrap();
        let round_0_challenge = verify_round(round_0, claimed_sum_evaluation, transcript)?;
        challenges.push(round_0_challenge);
    }

    // check all other rounds
    for (round_current, round_prev) in zip(rounds.iter().skip(1), rounds.iter()) {
        let round_claim = &round_prev.round_poly_at_r;
        let round_challenge = verify_round(round_current, round_claim.value, transcript)?;
        challenges.push(round_challenge);
    }

    // final check
    {
        let last_round_claim = rounds.last().unwrap().round_poly_at_r.value;
        verify_final_check(final_check, last_round_claim, instance)?;
    }

    Ok(())
}

pub fn verify_round<E, H, R>(
    current_round: &SumCheckRound<E, H>,
    round_claim: E,
    transcript: &mut R,
) -> Result<E::BaseField, VerificationError>
where
    E: FieldElement,
    H: ElementHasher<BaseField = E>,
    R: RandomCoin<BaseField = E::BaseField, Hasher = H>,
{
    let SumCheckRound {
        round_poly_commit,
        round_poly_at_0,
        round_poly_at_1,
        round_poly_at_r: _,
    } = current_round;

    // TODO: Verify all the openings here

    transcript.reseed(round_poly_commit.commitment);

    // the actual check
    if round_poly_at_0.value + round_poly_at_1.value != round_claim {
        return Err(VerificationError);
    }

    Ok(transcript.draw().unwrap())
}

pub fn verify_final_check<E, H, I>(
    final_check: FinalCheck<E, H>,
    last_claim: E,
    instance: I,
) -> Result<(), VerificationError>
where
    E: FieldElement,
    H: ElementHasher<BaseField = E>,
    I: SumCheckInstance<E>,
{
    let input_poly_evals: Vec<E> = final_check
        .input_poly_openings_at_r
        .into_iter()
        .map(|opening| opening.value)
        .collect();

    if last_claim != instance.g(input_poly_evals) {
        return Err(VerificationError);
    }

    Ok(())
}
