#[derive(Debug, thiserror::Error)]
pub enum ProverError {
    #[error("failed to generate multi-linear from the given evaluations")]
    FailedToGenerateML,
    #[error("failed to generate the sum-check proof")]
    FailedToProveSumCheck,
    #[error("failed to generate the random challenge")]
    FailedToGenerateChallenge,
}

#[derive(Debug, thiserror::Error)]
pub enum VerifierError {
    #[error("one of the claimed circuit denominators is zero")]
    ZeroOutputDenominator,
    #[error("the output of the fraction circuit is not equal to the expected value")]
    MismatchingCircuitOutput,
    #[error("failed to generate the random challenge")]
    FailedToGenerateChallenge,
    #[error("failed to verify the sum-check proof")]
    FailedToVerifySumCheck,
}
