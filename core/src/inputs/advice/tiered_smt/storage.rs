use std::collections::BTreeMap;

use crate::errors::AdviceSetError;

use super::{CanonicalWord, NodeIndex, NodeType, Word, Leaf};



// TIERED SPARSE MERKLE TREE STORAGE
// ================================================================================================

/// A storage implementation for the tiered sparse merkle tree.
#[derive(Clone, Debug, Default)]
pub struct Storage {
    types: BTreeMap<NodeIndex, NodeType>,
    nodes: BTreeMap<NodeIndex, Word>,
    keys: BTreeMap<CanonicalWord, NodeIndex>,
    upper_leaf_keys: BTreeMap<NodeIndex, CanonicalWord>,
    leaf_values: BTreeMap<CanonicalWord, Word>,
    ordered_leaves: BTreeMap<u64, Vec<CanonicalWord>>,
    pub pre_inserted: BTreeMap<u64, Leaf>
}

impl Storage {
    // PROVIDERS
    // --------------------------------------------------------------------------------------------

    /// Returns the type of a node.
    pub fn get_type(&self, index: &NodeIndex) -> Result<Option<NodeType>, AdviceSetError> {
        Ok(self.types.get(index).copied())
    }

    /// Returns the value of a node.
    pub fn get_node(&self, index: &NodeIndex) -> Result<Option<Word>, AdviceSetError> {
        Ok(self.nodes.get(index).copied())
    }

    /// Returns the index of a leaf key.
    pub fn get_leaf_index(&self, key: &CanonicalWord) -> Result<Option<NodeIndex>, AdviceSetError> {
        Ok(self.keys.get(key).copied())
    }

    /// Returns the leaf key of an index.
    pub fn get_leaf_key(&self, index: &NodeIndex) -> Result<Option<CanonicalWord>, AdviceSetError> {
        Ok(self.upper_leaf_keys.get(index).copied())
    }

    /// Returns the leaf value of its key.
    pub fn get_leaf_value(&self, key: &CanonicalWord) -> Result<Option<Word>, AdviceSetError> {
        Ok(self.leaf_values.get(key).copied())
    }

    /// Returns a list of leaves for a given index of the lowest depth of the tree.
    pub fn get_ordered_leaves(
        &self,
        index: u64,
    ) -> Result<Option<Vec<CanonicalWord>>, AdviceSetError> {
        Ok(self.ordered_leaves.get(&index).cloned())
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Overwrites the node type of a given index.
    pub fn replace_type(&mut self, index: NodeIndex, r#type: NodeType) -> Result<(), AdviceSetError> {
        self.types.insert(index, r#type);
        Ok(())
    }

    /// Overwrites the node value of a given index.
    pub fn replace_node(&mut self, index: NodeIndex, node: Word) -> Result<(), AdviceSetError> {
        self.nodes.insert(index, node);
        Ok(())
    }

    /// Overwrites the index of a given leaf key.
    pub fn replace_key(
        &mut self,
        key: CanonicalWord,
        index: NodeIndex,
    ) -> Result<(), AdviceSetError> {
        self.keys.insert(key, index);
        Ok(())
    }

    /// Overwrites the leaf key of a given index.
    pub fn replace_leaf_key(
        &mut self,
        index: NodeIndex,
        key: CanonicalWord,
    ) -> Result<(), AdviceSetError> {
        self.upper_leaf_keys.insert(index, key);
        Ok(())
    }

    /// Overwrites the leaf value given its key.
    pub fn replace_leaf_value(
        &mut self,
        key: CanonicalWord,
        value: Word,
    ) -> Result<(), AdviceSetError> {
        self.leaf_values.insert(key, value);
        Ok(())
    }

    /// Overwrites the list of ordered leaves of the given index.
    ///
    /// Note: This will remove any previous instance, they will not be merged.
    pub fn replace_ordered_leaves(
        &mut self,
        index: u64,
        leaves: Vec<CanonicalWord>,
    ) -> Result<(), AdviceSetError> {
        self.ordered_leaves.insert(index, leaves);
        Ok(())
    }

    /// Removes a type from a given index, returning it.
    pub fn take_type(&mut self, index: &NodeIndex) -> Result<Option<NodeType>, AdviceSetError> {
        Ok(self.types.remove(index))
    }

    /// Removes a node value from a given index, returning it.
    pub fn take_node(&mut self, index: &NodeIndex) -> Result<Option<Word>, AdviceSetError> {
        Ok(self.nodes.remove(index))
    }

    /// Removes a leaf key mapping index, returning it.
    pub fn take_key(&mut self, key: &CanonicalWord) -> Result<Option<NodeIndex>, AdviceSetError> {
        Ok(self.keys.remove(key))
    }

    /// Removes a leaf key from a given index, returning it.
    pub fn take_leaf_key(
        &mut self,
        index: &NodeIndex,
    ) -> Result<Option<CanonicalWord>, AdviceSetError> {
        Ok(self.upper_leaf_keys.remove(index))
    }

    /// Removes a leaf value mapping, returning it.
    pub fn take_leaf_value(&mut self, key: &CanonicalWord) -> Result<Option<Word>, AdviceSetError> {
        Ok(self.leaf_values.remove(key))
    }

    /// Removes an ordered leaves list for the given bottom index, returning it.
    pub fn take_ordered_leaves(
        &mut self,
        index: u64,
    ) -> Result<Option<Vec<CanonicalWord>>, AdviceSetError> {
        Ok(self.ordered_leaves.remove(&index))
    }
}
