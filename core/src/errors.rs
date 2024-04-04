use core::fmt;

use alloc::string::String;

// INPUT ERROR
// ================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputError {
    DuplicateAdviceRoot([u8; 32]),
    InputLengthExceeded(usize, usize),
    NotFieldElement(u64, String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InputError::*;
        match self {
            DuplicateAdviceRoot(key) => {
                write!(f, "{key:02x?} is a duplicate of the current merkle set")
            }
            InputLengthExceeded(limit, provided) => {
                write!(
                    f,
                    "Number of input values can not exceed {limit}, but {provided} was provided"
                )
            }
            NotFieldElement(num, description) => {
                write!(f, "{num} is not a valid field element: {description}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InputError {}

// OUTPUT ERROR
// ================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputError {
    InvalidOverflowAddress(String),
    InvalidOverflowAddressLength(usize, usize),
    InvalidStackElement(String),
    OutputSizeTooBig(usize),
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use OutputError::*;
        match self {
            InvalidOverflowAddress(description) => {
                write!(f, "overflow addresses contains invalid field element: {description}")
            }
            InvalidOverflowAddressLength(actual, expected) => {
                write!(f, "overflow addresses length is {actual}, but expected {expected}")
            }
            InvalidStackElement(description) => {
                write!(f, "stack contains an invalid field element: {description}")
            }
            OutputSizeTooBig(size) => {
                write!(f, "too many elements for output stack, {size} elements")
            }
        }
    }
}

// KERNEL ERROR
// ================================================================================================

#[cfg(feature = "std")]
impl std::error::Error for OutputError {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KernelError {
    DuplicatedProcedures,
    TooManyProcedures(usize, usize),
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::DuplicatedProcedures => {
                write!(f, "Kernel can not have duplicated procedures",)
            }
            KernelError::TooManyProcedures(max, count) => {
                write!(f, "Kernel can have at most {} procedures, received {}", max, count)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for KernelError {}
