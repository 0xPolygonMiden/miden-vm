use super::{fmt, hasher, Box, CodeBlock, Digest, Felt, Operation};

// LOOP BLOCK
// ================================================================================================
/// Block for a conditional loop.
///
/// Executes the loop body while the value on the top of the stack is `1`, stops when `0`. Fails if
/// the top of the stack is neither `1` nor `0`, or if the execution of the body fails.
///
/// The hash of a loop block is:
///
/// > hash(body_hash || padding, domain=LOOP_DOMAIN)
///
/// Where `body_hash` is 4 field elements (256 bits), and `padding` is 4 ZERO elements (256 bits).
#[derive(Clone, Debug)]
pub struct Loop {
    body: Box<CodeBlock>,
    hash: Digest,
}

impl Loop {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the loop block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Loop.op_code() as u64);

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Loop] bock instantiated with the specified body.
    pub fn new(body: CodeBlock) -> Self {
        let hash = hasher::merge_in_domain(&[body.hash(), Digest::default()], Self::DOMAIN);
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

    /// Returns a reference to the code block which represents the body of the loop.
    pub fn body(&self) -> &CodeBlock {
        &self.body
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while.true {} end", self.body)
    }
}
