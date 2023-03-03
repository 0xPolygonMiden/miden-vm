use super::{
    chiplets::hasher::{self, Digest},
    utils::{
        collections::{BTreeMap, Vec},
        Box,
    },
    Felt, FieldElement, Operation,
};
use core::fmt;
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

pub mod blocks;
use blocks::CodeBlock;

mod info;
pub use info::ProgramInfo;

#[cfg(test)]
mod tests;

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
    kernel: Kernel,
    cb_table: CodeBlockTable,
}

impl Program {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Instantiates a new [Program] from the specified code block.
    pub fn new(root: CodeBlock) -> Self {
        Self::with_kernel(root, Kernel::default(), CodeBlockTable::default())
    }

    /// Instantiates a new [Program] from the specified code block and associated code block table.
    pub fn with_kernel(root: CodeBlock, kernel: Kernel, cb_table: CodeBlockTable) -> Self {
        Self {
            root,
            kernel,
            cb_table,
        }
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

    /// Returns a kernel for this program.
    pub fn kernel(&self) -> &Kernel {
        &self.kernel
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

    /// Returns true if this code block table is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// KERNEL
// ================================================================================================

/// A list of procedure hashes defining a VM kernel.
///
/// The internally-stored list always has a consistent order, regardless of the order of procedure
/// list used to instantiate a kernel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Kernel(Vec<Digest>);

impl Kernel {
    /// Returns a new [Kernel] instantiated with the specified procedure hashes.
    pub fn new(proc_hashes: &[Digest]) -> Self {
        // make sure procedure roots are ordered consistently
        let mut hash_map: BTreeMap<[u8; 32], Digest> = BTreeMap::new();
        proc_hashes.iter().cloned().for_each(|r| {
            hash_map.insert(r.into(), r);
        });
        Self(hash_map.values().copied().collect())
    }

    /// Returns true if this kernel does not contain any procedures.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if a procedure with the specified hash belongs to this kernel.
    pub fn contains_proc(&self, proc_hash: Digest) -> bool {
        // linear search here is OK because we expect the kernels to have a relatively small number
        // of procedures (e.g., under 100)
        self.0.iter().any(|&h| h == proc_hash)
    }

    /// Returns a list of procedure hashes contained in this kernel.
    pub fn proc_hashes(&self) -> &[Digest] {
        &self.0
    }
}

// this is required by AIR as public inputs will be serialized with the proof
impl Serializable for Kernel {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // TODO the serialization of MAST will not support values greater than `u16::MAX`, so we
        // reflect the same restriction here. however, this should be tweaked in the future. This
        // value will likely be capped to `u8::MAX`.

        debug_assert!(self.0.len() <= u16::MAX as usize);
        target.write_u16(self.0.len() as u16);
        Digest::write_batch_into(&self.0, target)
    }
}

impl Deserializable for Kernel {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let len = source.read_u16()?;
        let kernel = (0..len).map(|_| source.read::<Digest>()).collect::<Result<_, _>>()?;
        Ok(Self(kernel))
    }
}
