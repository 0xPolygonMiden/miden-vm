use crate::mast::MastForest;
use core::fmt;
use miden_crypto::hash::rpo::RpoDigest;

/// Node for referencing procedures not present in a given [`MastForest`] (hence "external").
///
/// External nodes can be used to verify the integrity of a program's hash while keeping parts of
/// the program secret. They also allow a program to refer to a well-known procedure that was not
/// compiled with the program (e.g. a procedure in the standard library).
///
/// The hash of an external node is the hash of the procedure it represents, such that an external
/// node can be swapped with the actual subtree that it represents without changing the MAST root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalNode {
    digest: RpoDigest,
}

impl ExternalNode {
    /// Returns a new [`ExternalNode`] instantiated with the specified procedure hash.
    pub fn new(procedure_hash: RpoDigest) -> Self {
        Self {
            digest: procedure_hash,
        }
    }
}

impl ExternalNode {
    pub fn digest(&self) -> RpoDigest {
        self.digest
    }
    pub fn to_display<'a>(&'a self, _mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        self
    }
}

impl crate::prettier::PrettyPrint for ExternalNode {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        use miden_formatting::hex::ToHex;
        const_text("external") + const_text(".") + text(self.digest.as_bytes().to_hex_with_prefix())
    }
}

impl fmt::Display for ExternalNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
