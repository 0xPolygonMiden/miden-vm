use alloc::{collections::BTreeMap, vec::Vec};
use miden_crypto::{hash::rpo::RpoDigest, Felt};

use crate::{DecoratorList, Kernel, Operation};

mod basic_block_node;
pub use basic_block_node::BasicBlockNode;

mod call_node;
pub use call_node::CallNode;

mod join_node;
pub use join_node::JoinNode;

mod split_node;
pub use split_node::SplitNode;

mod loop_node;
pub use loop_node::LoopNode;

pub trait MerkleTreeNode {
    fn digest(&self) -> RpoDigest;
}

/// An opaque handle to a [`MastNode`] in some [`MastForest`]. It is the responsibility of the user
/// to use a given [`MastNodeId`] with the corresponding [`MastForest`].
///
/// Note that since a [`MastForest`] enforces the invariant that equal [`MastNode`]s MUST have equal
/// [`MastNodeId`]s, [`MastNodeId`] equality can be used to determine equality of the underlying
/// [`MastNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MastNodeId(usize);

#[derive(Debug, Default)]
pub struct MastForest {
    /// All of the blocks local to the trees comprising the MAST forest
    nodes: Vec<MastNode>,
    node_id_by_hash: BTreeMap<RpoDigest, MastNodeId>,

    /// The "entrypoint", when set, is the root of the entire forest, i.e.
    /// a path exists from this node to all other roots in the forest. This
    /// corresponds to the executable entry point. When not set, the forest
    /// may or may not have such a root in `roots`, but is not required.
    /// Whether or not the entrypoint is set distinguishes a MAST which is
    /// executable, versus a MAST which represents a library.
    ///
    /// NOTE: The entrypoint is also present in `roots` if set
    entrypoint: Option<MastNodeId>,
    kernel: Kernel,
}

impl MastForest {
    /// Creates a new empty [`MastForest`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node to the forest, and returns the [`MastNodeId`] associated with it.
    ///
    /// If a [`MastNode`] which is equal to the current node was previously added, the previously
    /// returned [`MastNodeId`] will be returned. This enforces this invariant that equal
    /// [`MastNode`]s have equal [`MastNodeId`]s.
    pub fn add_node(&mut self, node: MastNode) -> MastNodeId {
        let node_digest = node.digest();

        if let Some(node_id) = self.node_id_by_hash.get(&node_digest) {
            // node already exists in the forest; return previously assigned id
            *node_id
        } else {
            let new_node_id = MastNodeId(self.nodes.len());

            self.node_id_by_hash.insert(node.digest(), new_node_id);
            self.nodes.push(node);

            new_node_id
        }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    pub fn entrypoint(&self) -> Option<MastNodeId> {
        self.entrypoint
    }

    /// A convenience method that provides the hash of the entrypoint, if any.
    pub fn entrypoint_digest(&self) -> Option<RpoDigest> {
        self.entrypoint.map(|entrypoint| self.get_node_by_id(entrypoint).digest())
    }

    pub fn get_node_by_id(&self, node_id: MastNodeId) -> &MastNode {
        &self.nodes[node_id.0]
    }

    pub fn get_node_id_by_digest(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.node_id_by_hash.get(&digest).copied()
    }
}

// TODOP: Implement `Eq` only as a hash check on all nodes?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MastNode {
    Block(BasicBlockNode),
    Join(JoinNode),
    Split(SplitNode),
    Loop(LoopNode),
    Call(CallNode),
    Dyn,
    /// A reference to a node whose definition is not
    /// local to the containing `MastForest`.
    External(RpoDigest),
}

/// Constructors
impl MastNode {
    pub fn new_basic_block(operations: Vec<Operation>) -> Self {
        Self::Block(BasicBlockNode::new(operations))
    }

    pub fn new_basic_block_with_decorators(
        operations: Vec<Operation>,
        decorators: DecoratorList,
    ) -> Self {
        Self::Block(BasicBlockNode::with_decorators(operations, decorators))
    }

    pub fn new_join(children: [MastNodeId; 2], mast_forest: &MastForest) -> Self {
        Self::Join(JoinNode::new(children, mast_forest))
    }

    pub fn new_split(branches: [MastNodeId; 2], mast_forest: &MastForest) -> Self {
        Self::Split(SplitNode::new(branches, mast_forest))
    }

    pub fn new_loop(body: MastNodeId, mast_forest: &MastForest) -> Self {
        Self::Loop(LoopNode::new(body, mast_forest))
    }

    pub fn new_call(callee: MastNodeId, mast_forest: &MastForest) -> Self {
        Self::Call(CallNode::new(callee, mast_forest))
    }

    pub fn new_syscall(callee: MastNodeId, mast_forest: &MastForest) -> Self {
        Self::Call(CallNode::new_syscall(callee, mast_forest))
    }
}

impl MerkleTreeNode for MastNode {
    fn digest(&self) -> RpoDigest {
        match self {
            MastNode::Block(node) => node.digest(),
            MastNode::Join(node) => node.digest(),
            MastNode::Split(node) => node.digest(),
            MastNode::Loop(node) => node.digest(),
            MastNode::Call(node) => node.digest(),
            MastNode::Dyn => DynNode.digest(),
            MastNode::External(external_digest) => *external_digest,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynNode;

impl DynNode {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the Dyn block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Dyn.op_code() as u64);
}

impl MerkleTreeNode for DynNode {
    fn digest(&self) -> RpoDigest {
        // The Dyn node is represented by a constant, which is set to be the hash of two empty
        // words ([ZERO, ZERO, ZERO, ZERO]) with a domain value of `DYN_DOMAIN`, i.e.
        // hasher::merge_in_domain(&[Digest::default(), Digest::default()], DynNode::DOMAIN)
        RpoDigest::new([
            Felt::new(8115106948140260551),
            Felt::new(13491227816952616836),
            Felt::new(15015806788322198710),
            Felt::new(16575543461540527115),
        ])
    }
}
