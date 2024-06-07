use core::{fmt, ops::Index};

use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use miden_crypto::hash::rpo::RpoDigest;
use miden_formatting::prettier::{Document, PrettyPrint};

use crate::{DecoratorList, Kernel, Operation};

mod basic_block_node;
pub use basic_block_node::BasicBlockNode;

mod call_node;
pub use call_node::CallNode;

mod dyn_node;
pub use dyn_node::DynNode;

mod external_node;
pub use external_node::ExternalNode;

mod join_node;
pub use join_node::JoinNode;

mod split_node;
pub use split_node::SplitNode;

mod loop_node;
pub use loop_node::LoopNode;

pub trait MerkleTreeNode {
    fn digest(&self) -> RpoDigest;
    fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a;
}

/// An opaque handle to a [`MastNode`] in some [`MastForest`]. It is the responsibility of the user
/// to use a given [`MastNodeId`] with the corresponding [`MastForest`].
///
/// Note that since a [`MastForest`] enforces the invariant that equal [`MastNode`]s MUST have equal
/// [`MastNodeId`]s, [`MastNodeId`] equality can be used to determine equality of the underlying
/// [`MastNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MastNodeId(u32);

#[derive(Clone, Debug, Default)]
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

/// Constructors
impl MastForest {
    /// Creates a new empty [`MastForest`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// Mutators
impl MastForest {
    // TODOP: Rename `ensure_node()` to be clear that it doesn't always add?
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
            let new_node_id =
                MastNodeId(self.nodes.len().try_into().expect("u32 expected to fit in usize"));

            self.node_id_by_hash.insert(node.digest(), new_node_id);
            self.nodes.push(node);

            new_node_id
        }
    }

    /// Sets the kernel for this forest.
    ///
    /// The kernel MUST have been compiled using this [`MastForest`]; that is, all kernel procedures
    /// must be present in this forest.
    pub fn set_kernel(&mut self, kernel: Kernel) {
        #[cfg(debug_assertions)]
        for proc_hash in kernel.proc_hashes() {
            assert!(self.node_id_by_hash.contains_key(proc_hash));
        }

        self.kernel = kernel;
    }

    /// Sets the entrypoint for this forest.
    pub fn set_entrypoint(&mut self, entrypoint: MastNodeId) {
        self.entrypoint = Some(entrypoint);
    }
}

/// Public accessors
impl MastForest {
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

    #[inline(always)]
    pub fn get_node_by_id(&self, node_id: MastNodeId) -> &MastNode {
        let idx: usize = node_id.0.try_into().expect("u32 expected to fit in usize");

        &self.nodes[idx]
    }

    #[inline(always)]
    pub fn get_node_id_by_digest(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.node_id_by_hash.get(&digest).copied()
    }
}

impl Index<MastNodeId> for MastForest {
    type Output = MastNode;

    fn index(&self, node_id: MastNodeId) -> &Self::Output {
        self.get_node_by_id(node_id)
    }
}

impl crate::prettier::PrettyPrint for MastForest {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        // TODOP: How to render MAST forests without an entrypoint?
        let entrypoint = self
            .get_node_by_id(
                self.entrypoint.expect("can only render MAST forests with an entrypoint"),
            )
            .to_pretty_print(self);

        indent(4, const_text("begin") + nl() + entrypoint.render()) + nl() + const_text("end")
    }
}

impl fmt::Display for MastForest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}

// TODOP: Implement `Eq` only as a hash check on all nodes?
// As a blanket impl over `MerkleTreeNode::digest()`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MastNode {
    Block(BasicBlockNode),
    Join(JoinNode),
    Split(SplitNode),
    Loop(LoopNode),
    Call(CallNode),
    Dyn,
    External(ExternalNode),
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

    pub fn new_join(
        left_child: MastNodeId,
        right_child: MastNodeId,
        mast_forest: &MastForest,
    ) -> Self {
        Self::Join(JoinNode::new([left_child, right_child], mast_forest))
    }

    pub fn new_split(
        if_branch: MastNodeId,
        else_branch: MastNodeId,
        mast_forest: &MastForest,
    ) -> Self {
        Self::Split(SplitNode::new([if_branch, else_branch], mast_forest))
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

    pub fn new_dynexec() -> Self {
        Self::Dyn
    }

    pub fn new_dyncall(dyn_node_id: MastNodeId, mast_forest: &MastForest) -> Self {
        Self::Call(CallNode::new(dyn_node_id, mast_forest))
    }

    pub fn new_external(code_hash: RpoDigest) -> Self {
        Self::External(ExternalNode::new(code_hash))
    }
}

/// Public accessors
impl MastNode {
    pub fn is_basic_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    // TODOP: Cleanup
    pub(super) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        match self {
            MastNode::Block(basic_block_node) => MastNodePrettyPrint::new(basic_block_node),
            MastNode::Join(join_node) => {
                MastNodePrettyPrint::new_box(Box::new(join_node.to_pretty_print(mast_forest)))
            }
            MastNode::Split(split_node) => {
                MastNodePrettyPrint::new_box(Box::new(split_node.to_pretty_print(mast_forest)))
            }
            MastNode::Loop(loop_node) => {
                MastNodePrettyPrint::new_box(Box::new(loop_node.to_pretty_print(mast_forest)))
            }
            MastNode::Call(call_node) => {
                MastNodePrettyPrint::new_box(Box::new(call_node.to_pretty_print(mast_forest)))
            }
            MastNode::Dyn => MastNodePrettyPrint::new(&DynNode),
            MastNode::External(external_node) => MastNodePrettyPrint::new(external_node),
        }
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
            MastNode::External(node) => node.digest(),
        }
    }

    fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        match self {
            MastNode::Block(node) => MastNodeDisplay::new(node.to_display(mast_forest)),
            MastNode::Join(node) => MastNodeDisplay::new(node.to_display(mast_forest)),
            MastNode::Split(node) => MastNodeDisplay::new(node.to_display(mast_forest)),
            MastNode::Loop(node) => MastNodeDisplay::new(node.to_display(mast_forest)),
            MastNode::Call(node) => MastNodeDisplay::new(node.to_display(mast_forest)),
            MastNode::Dyn => MastNodeDisplay::new(DynNode.to_display(mast_forest)),
            MastNode::External(node) => MastNodeDisplay::new(node.to_display(mast_forest)),
        }
    }
}

struct MastNodePrettyPrint<'a> {
    node_pretty_print: Box<dyn PrettyPrint + 'a>,
}

impl<'a> MastNodePrettyPrint<'a> {
    pub fn new(node: &'a dyn PrettyPrint) -> Self {
        Self {
            node_pretty_print: Box::new(node),
        }
    }

    pub fn new_box(node_pretty_print: Box<dyn PrettyPrint + 'a>) -> Self {
        Self { node_pretty_print }
    }
}

impl<'a> PrettyPrint for MastNodePrettyPrint<'a> {
    fn render(&self) -> Document {
        self.node_pretty_print.render()
    }
}

struct MastNodeDisplay<'a> {
    node_display: Box<dyn fmt::Display + 'a>,
}

impl<'a> MastNodeDisplay<'a> {
    pub fn new(node: impl fmt::Display + 'a) -> Self {
        Self {
            node_display: Box::new(node),
        }
    }
}

impl<'a> fmt::Display for MastNodeDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.node_display.fmt(f)
    }
}
