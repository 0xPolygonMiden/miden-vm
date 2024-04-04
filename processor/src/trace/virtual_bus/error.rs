#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("number of numerators and denominators is different")]
    NumeratorDenominatorLengthMismatch,
    #[error("number of numerators and denominators should be at least two")]
    NumeratorDenominatorLessThanTwo,
}
