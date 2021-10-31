use core::fmt;
use crypto::{hashers::Rp62_248, Digest as HasherDigest, ElementHasher, Hasher};
use math::fields::f128::BaseElement;
use std::collections::BTreeMap;

pub mod blocks;
use blocks::CodeBlock;

mod operations;
pub use operations::Operation;

// TYPES ALIASES
// ================================================================================================

type Digest = <Rp62_248 as Hasher>::Digest;

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
        let hash = Rp62_248::merge(&[root.hash(), Digest::default()]);
        Self {
            root,
            hash: hash.as_bytes(),
        }
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

    pub fn load(&self, _index: BaseElement) -> [BaseElement; 4] {
        unimplemented!()
    }

    pub fn store(&self, _index: usize, _value: [BaseElement; 4]) {}
}
