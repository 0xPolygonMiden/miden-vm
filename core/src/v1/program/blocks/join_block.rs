use super::{fmt, CodeBlock, Digest, Hasher, Rp62_248};

// JOIN BLOCKS
// ================================================================================================
/// A code block used to combine two other code blocks.
///
/// When the VM executes a Join block, it executes joined blocks in sequence one after the other.
///
/// Hash of a Join block is computed by hashing a concatenation of the hashes of joined blocks.
/// TODO: update hashing methodology to make it different from Split block.
#[derive(Clone, Debug)]
pub struct Join {
    body: Box<[CodeBlock; 2]>,
    hash: Digest,
}

impl Join {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Join] block instantiated with the specified code blocks.
    pub fn new(body: [CodeBlock; 2]) -> Self {
        let hash = Rp62_248::merge(&[body[0].hash(), body[1].hash()]);
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
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join {} {} end", self.body[0], self.body[1])
    }
}
