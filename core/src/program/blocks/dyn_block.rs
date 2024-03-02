use super::{fmt, Digest, Felt, Operation};

// CONSTANTS
// ================================================================================================

/// The Dyn block is represented by a constant, which is set to be the hash of two empty words
/// ([ZERO, ZERO, ZERO, ZERO]) with a domain value of `DYN_DOMAIN`, i.e.
/// hasher::merge_in_domain(&[Digest::default(), Digest::default()], Dyn::DOMAIN)
const DYN_CONSTANT: Digest = Digest::new([
    Felt::new(8115106948140260551),
    Felt::new(13491227816952616836),
    Felt::new(15015806788322198710),
    Felt::new(16575543461540527115),
]);

// Dyn BLOCK
// ================================================================================================
/// Block for dynamic code where the target is specified by the stack.
///
/// Executes the code block referenced by the hash on top of the stack. Fails if the body is
/// unavailable to the VM, or if the execution of the dynamically-specified code block fails.
///
/// The child of a Dyn block (the target specified by the stack) is always dynamic and does not
/// affect the representation of the Dyn block. Therefore all Dyn blocks are represented by the same
/// constant (rather than by unique hashes), which is computed as an RPO hash of two empty words
/// ([ZERO, ZERO, ZERO, ZERO]) with a domain value of `DYN_DOMAIN`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dyn {}

impl Dyn {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the Dyn block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Dyn.op_code() as u64);

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Dyn] block instantiated with the specified function body hash.
    pub fn new() -> Self {
        Self {}
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a hash of this code block.
    pub fn hash(&self) -> Digest {
        Self::dyn_hash()
    }

    /// Returns a hash of this code block.
    pub fn dyn_hash() -> Digest {
        DYN_CONSTANT
    }
}

impl Default for Dyn {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "formatter")]
impl crate::prettier::PrettyPrint for Dyn {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        const_text("dyn")
    }
}
impl fmt::Display for Dyn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dyn")?;

        Ok(())
    }
}
