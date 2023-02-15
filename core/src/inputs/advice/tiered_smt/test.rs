use crypto::{WORD_SIZE, StarkField};

use crate::ONE;

use super::*;
use core::ops::{Deref, DerefMut};


#[test]
fn append_to_bottom_level_is_consistent() {
    let mut test = TieredTestEngine::default();

    // insert a leaf into the empty tree. should be allocated to the first tier.
    let raw_a = 0b_01101001_01101100_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_a = [Felt::new(raw_a); WORD_SIZE];
    test.insert(key_a, Word::default())
        .expect_path_with_depth(key_a, 16);

    // insert another leaf and diverge on the first bit, causing both `a` and `b` to be on depth
    // `16. should produce an empty path until the last element.
    let raw_b = 0b_11101001_01101100_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_b = [Felt::new(raw_b); WORD_SIZE];
    test.insert(key_b, Word::default())
        .expect_path_with_depth(key_a, 16)
        .expect_path_with_depth(key_b, 16);

    // this new leaf will have the same path of `a` until the first bit of the second tier, where
    // they diverge. both `a` and `c` should be on the depth `32` after this mutation.
    let raw_c = 0b_01101001_01101100_10011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_c = [Felt::new(raw_c); WORD_SIZE];
    // expect the path of `a` to be mutated as well.
    test.insert(key_c, Word::default())
        .expect_path_with_depth(key_a, 32)
        .expect_path_with_depth(key_b, 16)
        .expect_path_with_depth(key_c, 32);

    // this new leaf will have the same path of `c` until the first bit of the third tier, where
    // they diverge. both `c` and `d` should be on the depth `48` after this mutation.
    let raw_d = 0b_01101001_01101100_10011111_11111111_00010110_10010011_11100000_00000000_u64;
    let key_d = [Felt::new(raw_d); WORD_SIZE];
    // expect the path of `c` to be mutated as well.
    test.insert(key_d, Word::default())
        .expect_path_with_depth(key_a, 32)
        .expect_path_with_depth(key_b, 16)
        .expect_path_with_depth(key_c, 48)
        .expect_path_with_depth(key_d, 48);

    // this new leaf will have the same path of `d` ultil the first bit of the fourth tier, where
    // they will diverge. both `d` and `e` should be on the depth `64` after this mutation.
    let raw_e = 0b_01101001_01101100_10011111_11111111_00010110_10010011_01100000_00000000_u64;
    let key_e = [Felt::new(raw_e); WORD_SIZE];
    // expect the path of `d` to be mutated as well.
    test.insert(key_e, Word::default())
        .expect_path_with_depth(key_a, 32)
        .expect_path_with_depth(key_b, 16)
        .expect_path_with_depth(key_c, 48)
        .expect_path_with_depth(key_d, 64)
        .expect_path_with_depth(key_e, 64);

    // this new leaf will collide with `e` until the last tier, that is depth `64`. it means they
    // should be inserted as ordered list of the bottom level, and will have the same path.
    let mut key_f = key_e;
    key_f[0] += ONE;
    // expect both `e` and `f` to have the same path.
    test.insert(key_f, Word::default())
        .expect_path_with_depth(key_a, 32)
        .expect_path_with_depth(key_b, 16)
        .expect_path_with_depth(key_c, 48)
        .expect_path_with_depth(key_d, 64)
        .expect_path_with_depth(key_e, 64)
        .expect_path_with_depth(key_f, 64);
}


// TIERED SPARSE MERKLE TREE TEST ENGINE
// ================================================================================================

pub struct TieredTestEngine {
    tree: TieredSmt,
}

impl Default for TieredTestEngine {
    fn default() -> Self {
        let storage = Storage::default();
        let tree = TieredSmt::with_storage(storage).unwrap();
        let root = RpoDigest::from(*tree.root());
        assert_eq!(root, RpoDigest::new(EMPTY_SUBTREES[0]));
        Self { tree }
    }
}

impl Deref for TieredTestEngine {
    type Target = TieredSmt;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl DerefMut for TieredTestEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}

impl TieredTestEngine {
    pub fn insert<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        Word: From<K>,
        Word: From<V>,
    {
        let key = Word::from(key);
        let value = Word::from(value);

        // assert non-membership
        let root = self.tree.root();
        let path = self.tree.get_leaf_proof(key).unwrap();
        assert!(path.verify_non_membership(key, value, root));

        self.tree.insert(key, value).unwrap();

        // assert membership
        let root = self.tree.root();
        let path = self.tree.get_leaf_proof(key).unwrap();
        assert!(path.verify_membership(key, value, root));

        self
    }

    pub fn expect_path_with_depth<W>(&mut self, key: W, mut depth: u8) -> &mut Self
    where
        Word: From<W>,
    {
        let key = Word::from(key);
        let canonical = key[3].as_int();
        let mut path = Vec::with_capacity(depth as usize);
        while depth > 0 {
            let index = canonical >> (64 - depth).min(63);
            let index = index ^ 1;
            let node = self
                .tree
                .storage
                .get_node(&NodeIndex::new(depth, index))
                .unwrap()
                .unwrap_or_else(|| EMPTY_SUBTREES[depth as usize].into());
            path.push(node);
            depth -= 1;
        }
        let opening = self.tree.get_leaf_path(key).unwrap();
        assert_eq!(opening.deref(), path);
        self
    }
}