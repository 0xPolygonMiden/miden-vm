use super::String;
use core::fmt::{Display, Formatter};

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionOptionsError {
    ExpectedCyclesTooBig(u32, u32),
    OtherErrors(String),
}

impl Display for ExecutionOptionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        use ExecutionOptionsError::*;

        match self {
            ExpectedCyclesTooBig(max, expected) => {
                write!(f, "The expected number of cycles must be smaller than the maximum number of cycles: maximum is {max}, but expectd is {expected}")
            }
            OtherErrors(error) => write!(f, "{error}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ExecutionOptionsError {}
