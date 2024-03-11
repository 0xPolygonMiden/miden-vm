use vm_core::FieldElement;

#[derive(Clone, Debug)]
pub struct PolynomialCommitment;

#[derive(Clone, Debug)]
pub struct CommitmentProof;

#[derive(Clone, Debug)]
pub struct CommittedValue<E: FieldElement> {
    pub value: E,
    pub commitment_proof: CommitmentProof,
}

#[derive(Clone, Debug)]
pub struct SumCheckProof<E: FieldElement> {
    pub round_proofs: Vec<RoundProof<E>>,
    pub final_round: FinalRound<E>,
}

#[derive(Clone, Debug)]
pub struct RoundProof<E: FieldElement> {
    pub s_commit: PolynomialCommitment,
    pub s_at_0: CommittedValue<E>,
    pub s_at_1: CommittedValue<E>,
}

#[derive(Clone, Debug)]
pub struct FinalRound<E: FieldElement> {
    pub poly_commits: Vec<PolynomialCommitment>,
    pub last_s_at_r: CommittedValue<E>,
    pub poly_openings_at_r: Vec<CommittedValue<E>>,
}
