use super::String;
use core::fmt::{Display, Formatter};

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ProvingError {
    ContradictingCycleNumbers(u32, u32),
    InvalidSecuritySetting(String),
    OtherErrors(String),
}

impl Display for ProvingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        use ProvingError::*;

        match self {
            ContradictingCycleNumbers(max, expected) => {
                write!(f, "The maximum allowed number of cycles is less than expected: maximum is {max}, but expectd is {expected}")
            }
            InvalidSecuritySetting(security_setting) => {
                write!(f, "{security_setting} is not a valid security setting")
            }
            OtherErrors(error) => write!(f, "{error}"),
        }
    }
}
