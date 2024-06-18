mod basic_block_node;
use core::fmt;

use alloc::{boxed::Box, vec::Vec};
pub use basic_block_node::{
    get_span_op_group_count, BasicBlockNode, OpBatch, BATCH_SIZE as OP_BATCH_SIZE,
    GROUP_SIZE as OP_GROUP_SIZE,
};

mod call_node;
pub use call_node::CallNode;

mod dyn_node;
pub use dyn_node::DynNode;

mod join_node;
pub use join_node::JoinNode;

mod split_node;
use miden_crypto::{hash::rpo::RpoDigest, Felt};
use miden_formatting::prettier::{Document, PrettyPrint};
pub use split_node::SplitNode;

mod loop_node;
pub use loop_node::LoopNode;

use crate::{
    mast::{MastForest, MastNodeId, MerkleTreeNode},
    DecoratorList, Operation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MastNode {
    Block(BasicBlockNode),
    Join(JoinNode),
    Split(SplitNode),
    Loop(LoopNode),
    Call(CallNode),
    Dyn,
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
}

/// Public accessors
impl MastNode {
    pub fn is_basic_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub(crate) fn to_pretty_print<'a>(
        &'a self,
        mast_forest: &'a MastForest,
    ) -> impl PrettyPrint + 'a {
        match self {
            MastNode::Block(basic_block_node) => {
                MastNodePrettyPrint::new(Box::new(basic_block_node))
            }
            MastNode::Join(join_node) => {
                MastNodePrettyPrint::new(Box::new(join_node.to_pretty_print(mast_forest)))
            }
            MastNode::Split(split_node) => {
                MastNodePrettyPrint::new(Box::new(split_node.to_pretty_print(mast_forest)))
            }
            MastNode::Loop(loop_node) => {
                MastNodePrettyPrint::new(Box::new(loop_node.to_pretty_print(mast_forest)))
            }
            MastNode::Call(call_node) => {
                MastNodePrettyPrint::new(Box::new(call_node.to_pretty_print(mast_forest)))
            }
            MastNode::Dyn => MastNodePrettyPrint::new(Box::new(DynNode)),
        }
    }

    pub fn domain(&self) -> Felt {
        match self {
            MastNode::Block(_) => BasicBlockNode::DOMAIN,
            MastNode::Join(_) => JoinNode::DOMAIN,
            MastNode::Split(_) => SplitNode::DOMAIN,
            MastNode::Loop(_) => LoopNode::DOMAIN,
            MastNode::Call(call_node) => call_node.domain(),
            MastNode::Dyn => DynNode::DOMAIN,
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
        }
    }
}

struct MastNodePrettyPrint<'a> {
    node_pretty_print: Box<dyn PrettyPrint + 'a>,
}

impl<'a> MastNodePrettyPrint<'a> {
    pub fn new(node_pretty_print: Box<dyn PrettyPrint + 'a>) -> Self {
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
