use super::{
    hasher::{self, Digest},
    Felt, FieldElement, Operation,
};
use core::fmt;
use winter_utils::collections::BTreeMap;

pub mod blocks;
use blocks::CodeBlock;

mod library;
pub use library::Library;

// SCRIPT
// ================================================================================================
/// A program which can be executed by the VM.
///
/// A script is a self-contained program which can be executed by the VM. A script has its own
/// read-write memory, but has no storage and cannot define functions. When executed against a
/// [Module], a script can call the module's functions, and via these functions modify the module's
/// storage.
#[derive(Clone, Debug)]
pub struct Script {
    root: CodeBlock,
    hash: [u8; 32],
}

impl Script {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Constructs a new program from the specified code block.
    pub fn new(root: CodeBlock) -> Self {
        let hash = hasher::merge(&[root.hash(), Digest::default()]).into();
        Self { root, hash }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the root code block of this script.
    pub fn root(&self) -> &CodeBlock {
        &self.root
    }

    /// Returns a hash of this script.
    pub fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "begin {} end", self.root)
    }
}

// MODULE
// ================================================================================================
/// TODO: add comments
#[derive(Clone)]
#[allow(dead_code)]
pub struct Module {
    functions: BTreeMap<Digest, CodeBlock>,
    storage: Vec<u64>, // TODO: this should be a sparse Merkle tree
}

#[allow(dead_code)]
impl Module {
    pub fn new(_functions: BTreeMap<Digest, CodeBlock>) -> Self {
        unimplemented!()
    }

    pub fn code_root(&self) -> Digest {
        unimplemented!()
    }

    pub fn storage_root(&self) -> Digest {
        unimplemented!()
    }

    pub fn get_function(&self, _hash: Digest) -> Option<&CodeBlock> {
        //self.functions.get(&hash)
        unimplemented!()
    }

    pub fn load(&self, _index: Felt) -> [Felt; 4] {
        unimplemented!()
    }

    pub fn store(&self, _index: usize, _value: [Felt; 4]) {}
}
