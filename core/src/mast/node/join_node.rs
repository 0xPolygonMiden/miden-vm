use core::fmt;

use miden_crypto::{hash::rpo::RpoDigest, Felt};
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use crate::{chiplets::hasher, prettier::PrettyPrint, Operation};

use crate::mast::{MastForest, MastNodeId, MerkleTreeNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinNode {
    children: [MastNodeId; 2],
    digest: RpoDigest,
}

/// Constants
impl JoinNode {
    /// The domain of the join block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Join.op_code() as u64);
}

/// Constructors
impl JoinNode {
    /// Returns a new [`JoinNode`] instantiated with the specified children nodes.
    pub fn new(children: [MastNodeId; 2], mast_forest: &MastForest) -> Self {
        let digest = {
            let left_child_hash = mast_forest[children[0]].digest();
            let right_child_hash = mast_forest[children[1]].digest();

            hasher::merge_in_domain(&[left_child_hash, right_child_hash], Self::DOMAIN)
        };

        Self { children, digest }
    }

    #[cfg(test)]
    pub fn new_test(children: [MastNodeId; 2], digest: RpoDigest) -> Self {
        Self { children, digest }
    }
}

/// Accessors
impl JoinNode {
    pub fn first(&self) -> MastNodeId {
        self.children[0]
    }

    pub fn second(&self) -> MastNodeId {
        self.children[1]
    }
}

impl JoinNode {
    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        JoinNodePrettyPrint {
            join_node: self,
            mast_forest,
        }
    }
}

impl MerkleTreeNode for JoinNode {
    fn digest(&self) -> RpoDigest {
        self.digest
    }

    fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        JoinNodePrettyPrint {
            join_node: self,
            mast_forest,
        }
    }
}

impl Serializable for JoinNode {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        let Self { children, digest } = self;

        children.write_into(target);
        digest.write_into(target);
    }
}

impl Deserializable for JoinNode {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let children = Deserializable::read_from(source)?;
        let digest = Deserializable::read_from(source)?;

        Ok(Self { children, digest })
    }
}

struct JoinNodePrettyPrint<'a> {
    join_node: &'a JoinNode,
    mast_forest: &'a MastForest,
}

impl<'a> PrettyPrint for JoinNodePrettyPrint<'a> {
    #[rustfmt::skip]
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;

        let first_child = self.mast_forest[self.join_node.first()].to_pretty_print(self.mast_forest);
        let second_child = self.mast_forest[self.join_node.second()].to_pretty_print(self.mast_forest);

        indent(
            4,
            const_text("join")
            + nl()
            + first_child.render()
            + nl()
            + second_child.render(),
        ) + nl() + const_text("end")
    }
}

impl<'a> fmt::Display for JoinNodePrettyPrint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
