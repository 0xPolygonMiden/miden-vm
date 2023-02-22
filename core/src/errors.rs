use core::fmt;

// INPUT ERROR
// ================================================================================================

#[derive(Clone, Debug)]
pub enum InputError {
    NotFieldElement(u64, &'static str),
    DuplicateAdviceRoot([u8; 32]),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InputError::*;
        match self {
            NotFieldElement(num, description) => {
                write!(f, "{num} is not a valid field element: {description}")
            }
            DuplicateAdviceRoot(key) => {
                write!(f, "{key:02x?} is a duplicate of the current merkle set")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InputError {}
