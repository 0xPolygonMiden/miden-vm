use crate::mast::{MastForest, MerkleTreeNode};
use core::fmt;
use miden_crypto::hash::rpo::RpoDigest;
/// Block for a unknown function call.
///
/// Proxy blocks are used to verify the integrity of a program's hash while keeping parts
/// of the program secret. Fails if executed.
///
/// Hash of a proxy block is not computed but is rather defined at instantiation time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalNode {
    digest: RpoDigest,
}

impl ExternalNode {
    /// Returns a new [Proxy] block instantiated with the specified code hash.
    pub fn new(code_hash: RpoDigest) -> Self {
        Self { digest: code_hash }
    }
}

impl MerkleTreeNode for ExternalNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }
    fn to_display<'a>(&'a self, _mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
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
