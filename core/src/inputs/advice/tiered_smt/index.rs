use crypto::StarkField;

use super::{Felt, Leaf, RpoDigest, TieredSmt, Word};
use core::{cmp::Ordering, ops::Deref};

// NODE INDEX
// ================================================================================================

/// A Merkle tree address to an arbitrary node.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct NodeIndex {
    depth: u8,
    index: u64,
}

impl NodeIndex {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Creates a new node index.
    pub const fn new(depth: u8, index: u64) -> Self {
        Self { depth, index }
    }

    /// Creates a new node index pointing to the root of the tree.
    pub const fn root() -> Self {
        Self { depth: 0, index: 0 }
    }

    // PROVIDERS
    // --------------------------------------------------------------------------------------------

    /// Builds a node to be used as input of a hash function when computing a Merkle path.
    ///
    /// Will evaluate the parity of the current instance to define the result.
    pub fn build_node(&self, slf: Word, sibling: Word) -> [RpoDigest; 2] {
        if self.is_right_sibling() {
            [sibling.into(), slf.into()]
        } else {
            [slf.into(), sibling.into()]
        }
    }

    /// Returns the depth of the current instance.
    pub const fn depth(&self) -> u8 {
        self.depth
    }

    /// Returns the index of the current depth.
    pub const fn index(&self) -> u64 {
        self.index
    }

    /// Returns true if the current instance is at the maximum depth of `[TieredSmt]`.
    pub const fn is_max_depth(&self) -> bool {
        self.depth >= TieredSmt::MAX_DEPTH
    }

    /// Returns true if the current instance points to a right sibling node.
    pub const fn is_right_sibling(&self) -> bool {
        (self.index & 1) == 1
    }

    /// Returns `true` if the depth is `0`.
    pub const fn is_root(&self) -> bool {
        self.depth == 0
    }

    /// Computes the index of the sibling of the current node.
    pub const fn sibling(mut self) -> Self {
        self.index ^= 1;
        self
    }

    /// Returns `true` if current depth is of a tier.
    pub const fn is_tier(&self) -> bool {
        (self.depth % TieredSmt::TIER_DEPTH) == 0
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Traverse towards the leaves, incrementing the depth by `1`.
    pub fn traverse(&mut self, right: bool) -> &mut Self {
        self.depth += 1;
        self.index <<= 1;
        self.index += right as u64;
        self
    }

    /// Traverse one level towards the root, decrementing the depth by `1`.
    pub fn backtrack(&mut self) -> &mut Self {
        self.depth = self.depth.saturating_sub(1);
        self.index >>= 1;
        self
    }

    /// Traverse `n` levels towards the root.
    pub fn backtrack_by(&mut self, n: u8) -> &mut Self {
        self.depth = self.depth.saturating_sub(n);
        self.index >>= n as u64;
        self
    }

    /// Traverse to the previous tier.
    pub fn backtrack_to_previous_tier(&mut self) -> &mut Self {
        let n = (self.depth / TieredSmt::TIER_DEPTH) * TieredSmt::TIER_DEPTH;
        let n = self.depth - n;
        self.backtrack_by(n)
    }
}

// LEAF INDEX
// ================================================================================================

/// A Merkle tree address to a tier node.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LeafIndex {
    key: CanonicalWord,
    canonical: u64,
    depth: u8,
}

impl PartialOrd for LeafIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.canonical.partial_cmp(&other.canonical)
    }
}

impl Ord for LeafIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.canonical.cmp(&other.canonical)
    }
}



impl From<CanonicalWord> for LeafIndex {
    fn from(key: CanonicalWord) -> Self {
        Self {
            key,
            canonical: key.path(),
            depth: 0,
        }
    }
}

impl From<&Leaf> for LeafIndex {
    fn from(leaf: &Leaf) -> Self {
        Self::from(leaf.key)
    }
}

impl From<&LeafIndex> for NodeIndex {
    fn from(index: &LeafIndex) -> Self {
        let depth = index.depth;
        let index = index.index();
        NodeIndex::new(depth, index)
    }
}

impl From<&mut LeafIndex> for NodeIndex {
    fn from(index: &mut LeafIndex) -> Self {
        (&*index).into()
    }
}

impl From<LeafIndex> for NodeIndex {
    fn from(index: LeafIndex) -> Self {
        Self::from(&index)
    }
}

impl Iterator for LeafIndex {
    type Item = NodeIndex;

    /// Traverse to the next tier of the tree, returning an index.
    ///
    /// Returns `None` if maximum depth is reached.
    fn next(&mut self) -> Option<Self::Item> {
        self.traverse().then(|| self.into())
    }
}

impl DoubleEndedIterator for LeafIndex {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.depth <= TieredSmt::TIER_DEPTH {
            self.depth = 0;
            return None;
        }
        self.depth -= TieredSmt::TIER_DEPTH;
        let index = self.index();
        Some(NodeIndex::new(self.depth, index))
    }
}

impl LeafIndex {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the depth of the current instance.
    pub const fn depth(&self) -> u8 {
        self.depth
    }

    // PROVIDERS
    // --------------------------------------------------------------------------------------------

    /// Computes the index for the current depth.
    pub const fn index(&self) -> u64 {
        self.canonical >> (64 - self.depth)
    }

    /// Returns true if the current instance is at the maximum depth of `[TieredSmt]`.
    pub const fn is_max_depth(&self) -> bool {
        self.depth == TieredSmt::MAX_DEPTH
    }
    /// Computes the remaining path of a leaf for the current depth.
    ///
    /// Will mutate only the last element of the key as it is the only element used to traverse the
    /// tree.
    pub fn remaining_path(&self) -> CanonicalWord {
        self.remaining_path_with_depth(self.depth)
    }

    /// Computes the remainig path of a leaf for an arbitrary depth.
    pub fn remaining_path_with_depth(&self, depth: u8) -> CanonicalWord {
        // clippy incorrectly assumes the function when `None` is a constant/copy, so it is cheap
        // enough to be executed by default. however, `64` is a valid input for this function, and
        // it would cause overflow.
        //
        // satisfying clippy here isn't worthy as we might greatly increase the complexity of this
        // function to do so.
        #[allow(clippy::unnecessary_lazy_evaluations)]
        let value = (depth == TieredSmt::MAX_DEPTH)
            .then_some(0)
            .unwrap_or_else(|| (self.canonical << depth) >> depth);
        self.key.with_path(value)
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Traverse the leaf to the next tier. Returns `false` if the current tear is the maximum and
    /// no mutation was performed.
    pub fn traverse(&mut self) -> bool {
        if self.depth >= TieredSmt::MAX_DEPTH {
            self.depth = TieredSmt::MAX_DEPTH + TieredSmt::TIER_DEPTH;
            return false;
        }
        self.depth += TieredSmt::TIER_DEPTH;
        true
    }

    /// Traverse the index to the first tier, returning it's correspondent node index.
    pub fn traverse_to_first_tier(&mut self) -> NodeIndex {
        self.depth = TieredSmt::TIER_DEPTH;
        self.into()
    }

    /// Traverse to a given depth.
    ///
    /// This function will not evaluate if the provided depth is greater than the maximum allowed
    /// for `[TieredSmt]`.
    pub fn traverse_to(&mut self, depth: u8) -> &mut Self {
        // normalize the depth into a tier, truncating any intermediate level.
        let tier = depth / TieredSmt::TIER_DEPTH;
        self.depth = tier * TieredSmt::TIER_DEPTH;
        self
    }
}

// CANONICAL WORD
// ================================================================================================

/// A `[Word]` in canonical representation.
#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct CanonicalWord([u64; 4]);

impl AsRef<[u64; 4]> for CanonicalWord {
    fn as_ref(&self) -> &[u64; 4] {
        &self.0
    }
}

impl Deref for CanonicalWord {
    type Target = [u64; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Word> for CanonicalWord {
    fn from(value: &Word) -> Self {
        Self([
            value[0].as_int(),
            value[1].as_int(),
            value[2].as_int(),
            value[3].as_int(),
        ])
    }
}

impl From<Word> for CanonicalWord {
    fn from(value: Word) -> Self {
        Self::from(&value)
    }
}

impl From<CanonicalWord> for Word {
    fn from(value: CanonicalWord) -> Self {
        [
            Felt::new(value.0[0]),
            Felt::new(value.0[1]),
            Felt::new(value.0[2]),
            Felt::new(value.0[3]),
        ]
    }
}

impl From<CanonicalWord> for RpoDigest {
    fn from(value: CanonicalWord) -> Self {
        Word::from(value).into()
    }
}

impl CanonicalWord {
    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the limb used to compute the merkle path of a key.
    pub const fn path(&self) -> u64 {
        self.0[3]
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Updates the limb used to compute the merkle path of a key.
    pub const fn with_path(mut self, path: u64) -> Self {
        self.0[3] = path;
        self
    }
}

#[test]
fn leaf_index_assumptions_match_tiered_parameters() {
    // we traverse only the last u64 element of the key, so we are constrained by 64 bits
    assert!(TieredSmt::MAX_DEPTH <= 64);
}