use super::{fmt, hasher, Box, CodeBlock, Digest, Felt, Operation};

// JOIN BLOCKS
// ================================================================================================
/// Block for sequential execution of two sub-blocks.
///
/// Executes left sub-block then the right sub-block. Fails if either of the sub-block execution fails.
///
/// The hash of a join block is computed as:
///
/// > hash(left_block_hash || right_block_hash, domain=JOIN_DOMAIN)
///
/// Where `left_block_hash` and `right_block_hash` are 4 field elements (256 bits) each.
#[derive(Clone, Debug)]
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

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join {} {} end", self.body[0], self.body[1])
    }
}
