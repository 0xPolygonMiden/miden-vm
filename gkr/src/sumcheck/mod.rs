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

#[derive(Clone, Debug)]
pub struct SumCheckProof<E: FieldElement, H: ElementHasher> {
    pub round_proofs: Vec<RoundProof<E, H>>,
    pub final_round: FinalRound<E, H>,
}

#[derive(Clone, Debug)]
pub struct RoundProof<E: FieldElement, H: ElementHasher> {
    pub s_commit: PolynomialCommitment<H>,
    pub s_at_0: CommittedValue<E>,
    pub s_at_1: CommittedValue<E>,

    // Used by next round
    pub s_at_r: CommittedValue<E>,
}

#[derive(Clone, Debug)]
pub struct FinalRound<E: FieldElement, H: ElementHasher> {
    pub poly_commits: Vec<PolynomialCommitment<H>>,
    pub poly_openings_at_r: Vec<CommittedValue<E>>,
}

pub struct VerificationError;

/// An instance of a sum check problem
pub trait SumCheckInstance<E: FieldElement> {
    const FINAL_CLAIMED_VALUE: E;

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
    let mut challenges: Vec<E::BaseField> = Vec::new();

    // check first round
    {
        let round_0_proof = proof.round_proofs.first().unwrap();
        let round_0_challenge = verify_round(round_0_proof, I::FINAL_CLAIMED_VALUE, transcript)?;
        challenges.push(round_0_challenge);
    }

    // check all other rounds
    for (round_current, round_prev) in
        zip(proof.round_proofs.iter().skip(1), proof.round_proofs.iter())
    {
        let round_claim = &round_prev.s_at_r;
        let round_challenge = verify_round(round_current, round_claim.value, transcript)?;
        challenges.push(round_challenge);
    }

    // final check
    {
        let last_round_claim = proof.round_proofs.last().unwrap().s_at_r.value;
        let poly_evals: Vec<E> = proof
            .final_round
            .poly_openings_at_r
            .into_iter()
            .map(|opening| opening.value)
            .collect();

        if last_round_claim != instance.g(poly_evals) {
            return Err(VerificationError);
        }
    }

    Ok(())
}

pub fn verify_round<E, H, R>(
    current_round_proof: &RoundProof<E, H>,
    round_claim: E,
    transcript: &mut R,
) -> Result<E::BaseField, VerificationError>
where
    E: FieldElement,
    H: ElementHasher<BaseField = E>,
    R: RandomCoin<BaseField = E::BaseField, Hasher = H>,
{
    let RoundProof {
        s_commit,
        s_at_0,
        s_at_1,
        s_at_r: _,
    } = current_round_proof;

    transcript.reseed(s_commit.commitment);

    // the actual check
    if s_at_0.value + s_at_1.value != round_claim {
        return Err(VerificationError);
    }

    Ok(transcript.draw().unwrap())
}
