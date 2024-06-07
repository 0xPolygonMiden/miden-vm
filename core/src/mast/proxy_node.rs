use core::fmt;

use miden_crypto::hash::rpo::RpoDigest;

use crate::MerkleTreeNode;

/// Block for a unknown function call.
///
/// Proxy blocks are used to verify the integrity of a program's hash while keeping parts
/// of the program secret. Fails if executed.
///
/// Hash of a proxy block is not computed but is rather defined at instantiation time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProxyNode {
    digest: RpoDigest,
}

impl ProxyNode {
    /// Returns a new [Proxy] block instantiated with the specified code hash.
    pub fn new(code_hash: RpoDigest) -> Self {
        Self { digest: code_hash }
    }
}

impl MerkleTreeNode for ProxyNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }

    fn to_display<'a>(&'a self, _mast_forest: &'a crate::MastForest) -> impl fmt::Display + 'a {
        self
    }
}

impl crate::prettier::PrettyPrint for ProxyNode {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        use miden_formatting::hex::ToHex;

        const_text("proxy") + const_text(".") + text(self.digest.as_bytes().to_hex_with_prefix())
    }
}

impl fmt::Display for ProxyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
