use super::Word;
use crate::utils::collections::Vec;
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
                write!(f, "{key:02x?} is a duplicate of the current advice set")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InputError {}

// ADVICE SET ERROR
// ================================================================================================

#[derive(Clone, Debug)]
pub enum AdviceSetError {
    DepthTooSmall,
    DepthTooBig(u32),
    NumLeavesNotPowerOfTwo(usize),
    InvalidKey(u64),
    InvalidIndex(u32, u64),
    InvalidDepth(u32, u32),
    InvalidPath(Vec<Word>),
    NodeNotInSet(u64),
}

impl fmt::Display for AdviceSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AdviceSetError::*;
        match self {
            DepthTooSmall => write!(f, "the provided depth is too small"),
            DepthTooBig(depth) => write!(f, "the provided depth {depth} is too big"),
            NumLeavesNotPowerOfTwo(num) => {
                write!(f, "the number of leaves {num} must be a power of 2")
            }
            InvalidKey(key) => write!(f, "the provided key {key} is invalid"),
            InvalidIndex(depth, key) => {
                write!(f, "the provided index with depth {depth} and key {key} is invalid")
            }
            InvalidDepth(expected, provided) => {
                write!(f, "expected depth {expected}, but provided {provided}")
            }
            InvalidPath(_) => write!(f, "the provided merkle path isn't valid"),
            NodeNotInSet(node) => write!(f, "the node {node} doesn't exist in the set"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AdviceSetError {}
