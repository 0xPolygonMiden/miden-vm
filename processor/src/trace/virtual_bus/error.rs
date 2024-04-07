#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("number of numerators and denominators is different")]
    NumeratorDenominatorLengthMismatch,
    #[error("number of numerators and denominators should be at least two")]
    NumeratorDenominatorLessThanTwo,
}

#[derive(Debug, thiserror::Error)]
pub enum ProverError {
    #[error("failed to generate a proof for the virtual bus relation")]
    FailedToGenerateProof,
}

#[derive(Debug, thiserror::Error)]
pub enum VerifierError {
    #[error("failed to generate a proof for the virtual bus relation")]
    FailedToVerifyProof,
}
