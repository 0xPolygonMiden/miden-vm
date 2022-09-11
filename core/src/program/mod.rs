use super::{
    chiplets::hasher::{self, Digest},
    utils::{
        collections::{BTreeMap, Vec},
        Box,
    },
    Felt, FieldElement, Operation,
};
use core::fmt;

pub mod blocks;
use blocks::CodeBlock;

mod library;
pub use library::Library;

// PROGRAM
// ================================================================================================
/// A program which can be executed by the VM.
///
/// A program is described by a Merkelized Abstract Syntax Tree (MAST), where each node is a
/// [CodeBlock]. Internal nodes describe control flow semantics of the program, while leaf nodes
/// contain linear sequences of instructions which contain no control flow.
#[derive(Clone, Debug)]
pub struct Program {
    root: CodeBlock,
    cb_table: CodeBlockTable,
}

impl Program {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Instantiates a new [Program] from the specified code block.
    pub fn new(root: CodeBlock) -> Self {
        Self::with_table(root, CodeBlockTable::default())
    }

    /// Instantiates a new [Program] from the specified code block and associated code block table.
    pub fn with_table(root: CodeBlock, cb_table: CodeBlockTable) -> Self {
        Self { root, cb_table }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the root code block of this program.
    pub fn root(&self) -> &CodeBlock {
        &self.root
    }

    /// Returns a hash of this program.
    pub fn hash(&self) -> Digest {
        self.root.hash()
    }

    /// Returns code block table for this program.
    pub fn cb_table(&self) -> &CodeBlockTable {
        &self.cb_table
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "begin {} end", self.root)
    }
}

// CODE BLOCK TABLE
// ================================================================================================

/// A map of code block hashes to their underlying code blocks.
///
/// This table is used to hold code blocks which are referenced from the program MAST but are
/// actually not a part of the MAST itself. Thus, for example, multiple nodes in the MAST can
/// reference the same code block in the table.
#[derive(Clone, Debug, Default)]
pub struct CodeBlockTable(BTreeMap<[u8; 32], CodeBlock>);

impl CodeBlockTable {
    /// Returns a code block for the specified hash, or None if the code block is not present
    /// in this table.
    pub fn get(&self, hash: Digest) -> Option<&CodeBlock> {
        let key: [u8; 32] = hash.into();
        self.0.get(&key)
    }

    /// Returns true if a code block with the specified hash is present in this table.
    pub fn has(&self, hash: Digest) -> bool {
        let key: [u8; 32] = hash.into();
        self.0.contains_key(&key)
    }

    /// Inserts the provided code block into this table.
    pub fn insert(&mut self, block: CodeBlock) {
        let key: [u8; 32] = block.hash().into();
        self.0.insert(key, block);
    }
}
