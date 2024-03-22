use super::{hasher, CodeBlock, Digest, Felt, Operation};
use alloc::boxed::Box;
use core::fmt;

// JOIN BLOCKS
// ================================================================================================
/// Block for sequential execution of two sub-blocks.
///
/// Executes left sub-block then the right sub-block. Fails if either of the sub-block execution
/// fails.
///
/// The hash of a join block is computed as:
///
/// > hash(left_block_hash || right_block_hash, domain=JOIN_DOMAIN)
///
/// Where `left_block_hash` and `right_block_hash` are 4 field elements (256 bits) each.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Join {
    body: Box<[CodeBlock; 2]>,
    hash: Digest,
}

impl Join {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the join block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Join.op_code() as u64);

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Join] block instantiated with the specified code blocks.
    pub fn new(body: [CodeBlock; 2]) -> Self {
        let hash = hasher::merge_in_domain(&[body[0].hash(), body[1].hash()], Self::DOMAIN);
        Self {
            body: Box::new(body),
            hash,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a hash of this code block.
    pub fn hash(&self) -> Digest {
        self.hash
    }

    /// Returns a reference to the code block which is to be executed first when this join block
    /// is executed.
    pub fn first(&self) -> &CodeBlock {
        &self.body[0]
    }

    /// Returns a reference to the code block which is to be executed second when this join block
    /// is executed.
    pub fn second(&self) -> &CodeBlock {
        &self.body[1]
    }
}

#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for Join {
    #[rustfmt::skip]
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        indent(
            4,
            const_text("join")
            + nl()
            + self.body[0].render()
            + nl()
            + self.body[1].render(),
        ) + nl() + const_text("end")
    }
}

#[cfg(feature = "formatter")]
impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}

#[cfg(not(feature = "formatter"))]
impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join {} {} end", self.body[0], self.body[1])
    }
}
