#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("A multi-linear polynomial should have a power of 2 number of evaluations over the Boolean hyper-cube")]
    EvaluationsNotPowerOfTwo,
}
