use std::vec;

use super::{Rpo256,Word};
use core::ops::Deref;

// MERKLE PATH
// ================================================================================================

/// A merkle path container, composed of a sequence of nodes of a Merkle tree.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MerklePath {
    pub nodes: Vec<Word>,
}

impl MerklePath {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Creates a new Merkle path from a list of nodes.
    pub fn new(nodes: Vec<Word>) -> Self {
        Self { nodes }
    }

    // PROVIDERS
    // --------------------------------------------------------------------------------------------

    /// Returns the depth in which this Merkle path proof is valid.
    pub fn depth(&self) -> u8 {
        self.nodes.len() as u8
    }

    /// Verify the Merkle opening proof towards the provided root.
    ///
    /// It will assert `input` exists on a Merkle tree, indexed by `index` for an arbitrary depth.
    pub fn verify(&self, mut index: u64, input: Word, root: &Word) -> bool {
        let computed = self.nodes.iter().copied().fold(input, |node, sibling| {
            // build the input node, considering the parity of the current index.
            let is_right_sibling = (index & 1) == 1;
            let input = if is_right_sibling {
                [sibling.into(), node.into()]
            } else {
                [node.into(), sibling.into()]
            };
            // compute the node and move to the next iteration.
            index >>= 1;
            Rpo256::merge(&input).into()
        });
        root == &computed
    }
}

impl Deref for MerklePath {
    type Target = [Word];

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl FromIterator<Word> for MerklePath {
    fn from_iter<T: IntoIterator<Item = Word>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl IntoIterator for MerklePath {
    type Item = Word;
    type IntoIter = vec::IntoIter<Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}