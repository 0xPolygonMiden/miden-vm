use super::{fmt, hasher, Box, CodeBlock, Digest, Felt, Operation};

// SPLIT BLOCK
// ================================================================================================
/// Block for conditional execution.
///
/// Executes the first branch if the top of the stack is `1` or the second branch if `0`. Fails if
/// the top of the stack is neither `1` or `0` or if the branch execution fails.
///
/// The hash of a split block is:
///
/// > hash(true_branch_hash || false_branch_hash, domain=SPLIT_DOMAIN)
///
/// Where `true_branch_hash` and `false_branch_hash` are 4 field elements (256 bits) each.
#[derive(Clone, Debug)]
pub struct Split {
    branches: Box<[CodeBlock; 2]>,
    hash: Digest,
}

impl Split {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the split block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Split.op_code() as u64);

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Split] block instantiated with the specified true and false branches.
    pub fn new(t_branch: CodeBlock, f_branch: CodeBlock) -> Self {
        let hash = hasher::merge_in_domain(&[t_branch.hash(), f_branch.hash()], Self::DOMAIN);
        Self {
            branches: Box::new([t_branch, f_branch]),
            hash,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a hash of this code block.
    pub fn hash(&self) -> Digest {
        self.hash
    }

    /// Returns a reference to the code block which is to be executed when the top of the stack
    /// is `1`.
    pub fn on_true(&self) -> &CodeBlock {
        &self.branches[0]
    }

    /// Returns a reference to the code block which is to be executed when the top of the stack
    /// is `0`.
    pub fn on_false(&self) -> &CodeBlock {
        &self.branches[1]
    }
}

impl fmt::Display for Split {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if.true {} else {} end", self.branches[0], self.branches[1])
    }
}
