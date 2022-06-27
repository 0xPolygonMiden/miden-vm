use super::{
    hasher::{self, Digest},
    AdviceSetError, Word,
};
use crate::utils::collections::BTreeMap;
use crate::utils::collections::Vec;

#[derive(Clone, Debug)]
pub struct SparseMerkleTree {
    root: Word,
    depth: u32,
    store: Store,
}

#[derive(Clone, Debug)]
pub struct Store {
    branches: BTreeMap<(u64, u32), BranchNode>,
    leaves: BTreeMap<u64, Word>,
}

impl Store {
    fn new() -> Self {
        let branches = BTreeMap::new();
        let leaves = BTreeMap::new();
        Self { branches, leaves }
    }

    fn get_leaf_node(&self, key: u64) -> Result<Word, AdviceSetError> {
        self.leaves
            .get(&key)
            .cloned()
            .ok_or(AdviceSetError::InvalidKey(key))
    }

    fn insert_leaf_node(&mut self, key: u64, node: Word) {
        self.leaves.insert(key, node);
    }

    fn get_branch_node(&self, key: u64, depth: u32) -> Result<BranchNode, AdviceSetError> {
        self.branches
            .get(&(key, depth))
            .cloned()
            .ok_or(AdviceSetError::InvalidKey(key))
    }

    fn insert_branch_node(&mut self, key: u64, depth: u32, left: Digest, right: Digest) {
        let node = BranchNode { left, right };
        self.branches.insert((key, depth), node);
    }
}

#[derive(Clone, Debug, Default)]
struct BranchNode {
    left: Digest,
    right: Digest,
}

impl SparseMerkleTree {
    pub fn new(keys: Vec<u64>, values: Vec<Word>, depth: u32) -> Result<Self, AdviceSetError> {
        let root = Word::default();
        let store = Store::new();
        let mut tree = Self { root, depth, store };
        for (key, val) in keys.into_iter().zip(values) {
            tree.insert_leaf(key, val)
                .expect("Failed to insert leaf value");
        }
        Ok(tree)
    }

    /// Returns the root of this Merkle tree.
    pub fn root(&self) -> Word {
        self.root
    }

    /// Returns the depth of this Merkle tree.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Returns a node at the specified key
    ///
    /// # Errors
    /// Returns an error if:
    /// * The specified depth is greater than the depth of the tree.
    /// * The specified key does not exist
    pub fn get_node(&self, depth: u32, key: u64) -> Result<Word, AdviceSetError> {
        if depth == 0 {
            Err(AdviceSetError::DepthTooSmall)
        } else if depth > self.depth() {
            Err(AdviceSetError::DepthTooBig(depth))
        } else if depth == self.depth() {
            self.store.get_leaf_node(key)
        } else {
            let branch_node = self.store.get_branch_node(key, depth)?;
            Ok(hasher::merge(&[branch_node.left, branch_node.right]).into())
        }
    }

    /// Returns a Merkle path to the node at the specified key. The node itself is
    /// not included in the path.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The specified key does not exist as a branch or leaf node
    /// * The specified depth is greater than the depth of the tree.
    pub fn get_path(&self, depth: u32, key: u64) -> Result<Vec<Word>, AdviceSetError> {
        if self.store.get_leaf_node(key).is_err() {
            return Err(AdviceSetError::InvalidKey(key));
        }
        if depth == 0 {
            return Err(AdviceSetError::DepthTooSmall);
        } else if depth > self.depth() {
            return Err(AdviceSetError::DepthTooBig(depth));
        }

        let mut path = Vec::with_capacity(depth as usize);
        let mut curr_key = key;
        for n in (0..depth).rev() {
            let parent_key = curr_key >> 1;
            let parent_node = self.store.get_branch_node(parent_key, n)?;
            let sibling_node = if curr_key & 1 == 1 {
                parent_node.left
            } else {
                parent_node.right
            };
            path.push(sibling_node.into());
            curr_key >>= 1;
        }
        Ok(path)
    }

    /// Replaces the leaf located at the specified key, and recomputes hashes by walking up the tree
    ///
    /// # Errors
    /// Returns an error if the specified key is not a valid leaf index for this tree.
    pub fn update_leaf(&mut self, key: u64, value: Word) -> Result<(), AdviceSetError> {
        if self.store.get_leaf_node(key).is_err() {
            return Err(AdviceSetError::InvalidKey(key));
        }
        self.insert_leaf(key, value)?;

        Ok(())
    }

    /// Inserts a leaf located at the specified key, and recomputes hashes by walking up the tree
    pub fn insert_leaf(&mut self, key: u64, value: Word) -> Result<(), AdviceSetError> {
        self.store.insert_leaf_node(key, value);

        let depth = self.depth();
        let mut curr_key = key;
        let mut curr_node: Digest = value.into();
        for n in (0..depth).rev() {
            let parent_key = curr_key >> 1;
            let parent_node = self
                .store
                .get_branch_node(parent_key, n)
                .unwrap_or_default();

            let (left, right) = if curr_key & 1 == 1 {
                (parent_node.left, curr_node)
            } else {
                (curr_node, parent_node.right)
            };

            self.store.insert_branch_node(parent_key, n, left, right);
            curr_key = parent_key;
            curr_node = hasher::merge(&[left, right]);
        }
        self.root = curr_node.into();

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{Felt, FieldElement},
        Word,
    };
    use crypto::{hashers::Rp64_256, ElementHasher, Hasher};

    const KEYS4: [u64; 4] = [0, 1, 2, 3];
    const KEYS8: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    const VALUES4: [Word; 4] = [
        int_to_node(1),
        int_to_node(2),
        int_to_node(3),
        int_to_node(4),
    ];

    const VALUES8: [Word; 8] = [
        int_to_node(1),
        int_to_node(2),
        int_to_node(3),
        int_to_node(4),
        int_to_node(5),
        int_to_node(6),
        int_to_node(7),
        int_to_node(8),
    ];

    #[test]
    fn build_sparse_merkle_tree() {
        let tree = super::SparseMerkleTree::new(KEYS4.to_vec(), VALUES4.to_vec(), 2).unwrap();

        let (root, node2, node3) = compute_internal_nodes();
        assert_eq!(root, tree.root());
        assert_eq!(node2, tree.get_node(1, 0).unwrap());
        assert_eq!(node3, tree.get_node(1, 1).unwrap());
    }

    #[test]
    fn get_values() {
        let tree = super::SparseMerkleTree::new(KEYS4.to_vec(), VALUES4.to_vec(), 2).unwrap();

        // check depth 2
        assert_eq!(VALUES4[0], tree.get_node(2, 0).unwrap());
        assert_eq!(VALUES4[1], tree.get_node(2, 1).unwrap());
        assert_eq!(VALUES4[2], tree.get_node(2, 2).unwrap());
        assert_eq!(VALUES4[3], tree.get_node(2, 3).unwrap());
    }

    #[test]
    fn get_path() {
        let tree = super::SparseMerkleTree::new(KEYS4.to_vec(), VALUES4.to_vec(), 2).unwrap();

        let (_, node2, node3) = compute_internal_nodes();

        // check depth 2
        assert_eq!(vec![VALUES4[1], node3], tree.get_path(2, 0).unwrap());
        assert_eq!(vec![VALUES4[0], node3], tree.get_path(2, 1).unwrap());
        assert_eq!(vec![VALUES4[3], node2], tree.get_path(2, 2).unwrap());
        assert_eq!(vec![VALUES4[2], node2], tree.get_path(2, 3).unwrap());

        // check depth 1
        assert_eq!(vec![node3], tree.get_path(1, 0).unwrap());
        assert_eq!(vec![node2], tree.get_path(1, 1).unwrap());
    }

    #[test]
    fn update_leaf() {
        let mut tree = super::SparseMerkleTree::new(KEYS8.to_vec(), VALUES8.to_vec(), 3).unwrap();

        // update one value
        let key = 3;
        let new_node = int_to_node(9);
        let mut expected_values = VALUES8.to_vec();
        expected_values[key] = new_node;
        let expected_tree =
            super::SparseMerkleTree::new(KEYS8.to_vec(), expected_values.clone(), 3).unwrap();

        tree.update_leaf(key as u64, new_node).unwrap();
        assert_eq!(expected_tree.root, tree.root);

        // update another value
        let key = 6;
        let new_node = int_to_node(10);
        expected_values[key] = new_node;
        let expected_tree =
            super::SparseMerkleTree::new(KEYS8.to_vec(), expected_values.clone(), 3).unwrap();

        tree.update_leaf(key as u64, new_node).unwrap();
        assert_eq!(expected_tree.root, tree.root);
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn compute_internal_nodes() -> (Word, Word, Word) {
        let node2 = Rp64_256::hash_elements(&[VALUES4[0], VALUES4[1]].concat());
        let node3 = Rp64_256::hash_elements(&[VALUES4[2], VALUES4[3]].concat());
        let root = Rp64_256::merge(&[node2, node3]);

        (root.into(), node2.into(), node3.into())
    }

    const fn int_to_node(value: u64) -> Word {
        [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
    }
}
