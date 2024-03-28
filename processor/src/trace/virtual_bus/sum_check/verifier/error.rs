#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the final evaluation check of sum-check failed")]
    FinalEvaluationCheckFailed,
    #[error("the proof doesn't contain openings of the multi-linears")]
    NoOpeningsProvided,
    #[error("failed to generate round challenge")]
    FailedToGenerateChallenge,
}
