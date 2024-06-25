use crate::trace::virtual_bus::sum_check::SumCheckProverError;

#[derive(Debug, thiserror::Error)]
pub enum ProverError {
    #[error("failed to generate multi-linear from the given evaluations")]
    FailedToGenerateML,
    #[error("failed to generate the sum-check proof")]
    FailedToProveSumCheck(#[from] SumCheckProverError),
    #[error("failed to generate the random challenge")]
    FailedToGenerateChallenge,
}
