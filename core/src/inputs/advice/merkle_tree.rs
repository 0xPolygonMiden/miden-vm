use super::Node;
use core::slice;
use crypto::{hashers::Rp64_256, Hasher};
use math::log2;
use std::convert::TryInto;
use winter_utils::uninit_vector;

// TYPE ALIASES
// ================================================================================================

type Digest = <Rp64_256 as Hasher>::Digest;

// MERKLE TREE
// ================================================================================================

/// A fully-balanced binary Merkle tree (i.e., a tree where the number of leaves is a power of two).
///
/// This struct is intended to be used as one of the variants of the MerkleSet enum.
#[derive(Clone, Debug)]
pub struct MerkleTree {
    nodes: Vec<Node>,
}

impl MerkleTree {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a Merkle tree instantiated from the provided leaves.
    ///
    /// # Errors
    /// TODO: Returns an error if the number of leaves is smaller than two or is not a power of two.
    pub fn new(leaves: Vec<Node>) -> Self {
        let n = leaves.len();

        assert!(n > 1, "not greater than 1"); // convert to error
        assert!(n.is_power_of_two(), "not power of two"); // convert to error

        // create un-initialized vector to hold all tree nodes
        let mut nodes = unsafe { uninit_vector(2 * n) };
        nodes[0] = digest_into_node(Digest::default());

        // copy leaves into the second part of the nodes vector
        nodes[n..].copy_from_slice(&leaves);

        // re-interpret nodes as an array of two nodes fused together
        let two_nodes = unsafe { slice::from_raw_parts(nodes.as_ptr() as *const [Digest; 2], n) };

        // calculate all internal tree nodes
        for i in (1..n).rev() {
            nodes[i] = digest_into_node(Rp64_256::merge(&two_nodes[i]));
        }

        Self { nodes }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the root of this Merkle tree.
    pub fn root(&self) -> Node {
        self.nodes[1]
    }

    /// Returns the depth of this Merkle tree.
    ///
    /// Merkle tree of depth 1 has two leaves, depth 2 has four leaves etc.
    pub fn depth(&self) -> u32 {
        log2(self.nodes.len() / 2)
    }

    /// Returns a node at the specified depth and index.
    ///
    /// # Errors
    /// Returns an error if:
    /// * TODO: The specified depth is greater than the depth of the tree.
    /// * TODO: The specified index not valid for the specified depth.
    pub fn get_node(&self, depth: u32, index: u64) -> Node {
        assert!(depth <= self.depth(), "invalid depth");
        assert!(index < 2u64.pow(depth), "invalid index");

        let pos = 2usize.pow(depth as u32) + (index as usize);
        self.nodes[pos]
    }

    /// Returns a Merkle path to the node at the specified depth and index. The note itself is
    /// not included in the path.
    ///
    /// # Errors
    /// Returns an error if:
    /// * TODO: The specified depth is greater than the depth of the tree.
    /// * TODO: The specified index not valid for the specified depth.
    pub fn get_path(&self, depth: u32, index: u64) -> Vec<Node> {
        assert!(depth <= self.depth(), "invalid depth");
        assert!(index < 2u64.pow(depth), "invalid index");

        let mut path = Vec::with_capacity(depth as usize);
        let mut pos = 2usize.pow(depth as u32) + (index as usize);

        while pos > 1 {
            path.push(self.nodes[pos ^ 1]);
            pos >>= 1;
        }

        path
    }
}

// HELPER FUNCTIONS
// ================================================================================================

// TODO: should be part of ElementDigest
fn digest_into_node(digest: Digest) -> Node {
    digest.as_elements().try_into().unwrap()
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{digest_into_node, Node};
    use crypto::{hashers::Rp64_256, ElementHasher, Hasher};
    use math::{fields::f64::BaseElement, FieldElement};

    const LEAVES4: [Node; 4] = [
        int_to_node(1),
        int_to_node(2),
        int_to_node(3),
        int_to_node(4),
    ];

    #[test]
    fn build_merkle_tree() {
        let tree = super::MerkleTree::new(LEAVES4.to_vec());
        assert_eq!(8, tree.nodes.len());

        // leaves were copied correctly
        for (a, b) in tree.nodes.iter().skip(4).zip(LEAVES4.iter()) {
            assert_eq!(a, b);
        }

        let (root, node2, node3) = compute_internal_nodes();

        assert_eq!(root, tree.nodes[1]);
        assert_eq!(node2, tree.nodes[2]);
        assert_eq!(node3, tree.nodes[3]);

        assert_eq!(root, tree.root());
    }

    #[test]
    fn get_leaf() {
        let tree = super::MerkleTree::new(LEAVES4.to_vec());

        // check depth 2
        assert_eq!(LEAVES4[0], tree.get_node(2, 0));
        assert_eq!(LEAVES4[1], tree.get_node(2, 1));
        assert_eq!(LEAVES4[2], tree.get_node(2, 2));
        assert_eq!(LEAVES4[3], tree.get_node(2, 3));

        // check depth 1
        let (root, node2, node3) = compute_internal_nodes();

        assert_eq!(node2, tree.get_node(1, 0));
        assert_eq!(node3, tree.get_node(1, 1));

        // check depth 0
        assert_eq!(root, tree.get_node(0, 0));
    }

    #[test]
    fn get_path() {
        let tree = super::MerkleTree::new(LEAVES4.to_vec());

        let (_, node2, node3) = compute_internal_nodes();

        // check depth 2
        assert_eq!(vec![LEAVES4[1], node3], tree.get_path(2, 0));
        assert_eq!(vec![LEAVES4[0], node3], tree.get_path(2, 1));
        assert_eq!(vec![LEAVES4[3], node2], tree.get_path(2, 2));
        assert_eq!(vec![LEAVES4[2], node2], tree.get_path(2, 3));

        // check depth 1
        assert_eq!(vec![node3], tree.get_path(1, 0));
        assert_eq!(vec![node2], tree.get_path(1, 1));
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn compute_internal_nodes() -> (Node, Node, Node) {
        let node2 = Rp64_256::hash_elements(&[LEAVES4[0], LEAVES4[1]].concat());
        let node3 = Rp64_256::hash_elements(&[LEAVES4[2], LEAVES4[3]].concat());
        let root = Rp64_256::merge(&[node2, node3]);

        (
            digest_into_node(root),
            digest_into_node(node2),
            digest_into_node(node3),
        )
    }

    const fn int_to_node(value: u64) -> Node {
        [
            BaseElement::new(value),
            BaseElement::ZERO,
            BaseElement::ZERO,
            BaseElement::ZERO,
        ]
    }
}
