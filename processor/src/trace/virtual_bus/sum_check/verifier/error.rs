#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the final evaluation check of sum-check failed")]
    FinalEvaluationCheckFailed,
    #[error("failed to generate round challenge")]
    FailedToGenerateChallenge,
    #[error("wrong opening point for the oracles")]
    WrongOpeningPoint,
}
