#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("number of rounds for sum-check must be greater than zero")]
    NumRoundsZero,
    #[error("sumcheck polynomial degree must be greater than zero")]
    PolynomialDegreeIsZero,
    #[error("the input was not well formed: {0}")]
    ImproperInput(String),
    #[error("the evaluation domain does not match the expected size")]
    EvaluationDomainMismatch,
    #[error("the number of rounds is greater than the number of variables")]
    TooManyRounds,
    #[error("should provide at least one multi-linear polynomial as input")]
    NoMlsProvided,
    #[error("failed to generate round challenge")]
    FailedToGenerateChallenge,
    #[error("the provided multi-linears have different arities")]
    MlesDifferentArities,
    #[error("multi-linears should have at least one variable")]
    AtLeastOneVariable,
}
