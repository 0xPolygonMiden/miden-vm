use alloc::vec::Vec;
use core::fmt;

use miden_crypto::hash::rpo::RpoDigest;

use crate::mast::{DecoratorId, MastForest};

// EXTERNAL NODE
// ================================================================================================

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
    before_enter: Vec<DecoratorId>,
    after_exit: Vec<DecoratorId>,
}

impl ExternalNode {
    /// Returns a new [`ExternalNode`] instantiated with the specified procedure hash.
    pub fn new(procedure_hash: RpoDigest) -> Self {
        Self {
            digest: procedure_hash,
            before_enter: Vec::new(),
            after_exit: Vec::new(),
        }
    }
}

impl ExternalNode {
    /// Returns the commitment to the MAST node referenced by this external node.
    pub fn digest(&self) -> RpoDigest {
        self.digest
    }

    /// Returns the decorators to be executed before this node is executed.
    pub fn before_enter(&self) -> &[DecoratorId] {
        &self.before_enter
    }

    /// Returns the decorators to be executed after this node is executed.
    pub fn after_exit(&self) -> &[DecoratorId] {
        &self.after_exit
    }
}

// PRETTY PRINTING
// ================================================================================================

impl ExternalNode {
    pub(super) fn to_display<'a>(&'a self, _mast_forest: &'a MastForest) -> impl fmt::Display + 'a {
        self
    }
}

impl crate::prettier::PrettyPrint for ExternalNode {
    fn render(&self) -> crate::prettier::Document {
        use miden_formatting::hex::ToHex;

        use crate::prettier::*;
        const_text("external") + const_text(".") + text(self.digest.as_bytes().to_hex_with_prefix())
    }
}

impl fmt::Display for ExternalNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}
