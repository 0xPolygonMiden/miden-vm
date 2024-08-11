use alloc::vec::Vec;
use core::{fmt, ops::Index};

use miden_crypto::{hash::rpo::RpoDigest, Felt, WORD_SIZE};
use winter_utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};

use super::Kernel;
use crate::{
    mast::{MastForest, MastNode, MastNodeId},
    utils::ToElements,
};

// PROGRAM
// ===============================================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    mast_forest: MastForest,
    /// The "entrypoint" is the node where execution of the program begins.
    entrypoint: MastNodeId,
    kernel: Kernel,
}

/// Constructors
impl Program {
    /// Construct a new [`Program`] from the given MAST forest and entrypoint. The kernel is assumed
    /// to be empty.
    ///
    /// # Panics:
    /// - if `mast_forest` doesn't have an entrypoint
    pub fn new(mast_forest: MastForest, entrypoint: MastNodeId) -> Self {
        assert!(mast_forest.get_node_by_id(entrypoint).is_some());

        Self {
            mast_forest,
            entrypoint,
            kernel: Kernel::default(),
        }
    }

    /// Construct a new [`Program`] from the given MAST forest, entrypoint, and kernel.
    ///
    /// # Panics:
    /// - if `mast_forest` doesn't have an entrypoint
    pub fn with_kernel(mast_forest: MastForest, entrypoint: MastNodeId, kernel: Kernel) -> Self {
        assert!(mast_forest.get_node_by_id(entrypoint).is_some());

        Self { mast_forest, entrypoint, kernel }
    }
}

/// Public accessors
impl Program {
    /// Returns the underlying [`MastForest`].
    pub fn mast_forest(&self) -> &MastForest {
        &self.mast_forest
    }

    /// Returns the kernel associated with this program.
    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// Returns the entrypoint associated with this program.
    pub fn entrypoint(&self) -> MastNodeId {
        self.entrypoint
    }

    /// Returns the hash of the program's entrypoint.
    ///
    /// Equivalently, returns the hash of the root of the entrypoint procedure.
    pub fn hash(&self) -> RpoDigest {
        self.mast_forest[self.entrypoint].digest()
    }

    /// Returns the [`MastNode`] associated with the provided [`MastNodeId`] if valid, or else
    /// `None`.
    ///
    /// This is the faillible version of indexing (e.g. `program[node_id]`).
    #[inline(always)]
    pub fn get_node_by_id(&self, node_id: MastNodeId) -> Option<&MastNode> {
        self.mast_forest.get_node_by_id(node_id)
    }

    /// Returns the [`MastNodeId`] of the procedure root associated with a given digest, if any.
    #[inline(always)]
    pub fn find_procedure_root(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.mast_forest.find_procedure_root(digest)
    }

    /// Returns the number of procedures in this program.
    pub fn num_procedures(&self) -> u32 {
        self.mast_forest.num_procedures()
    }
}

/// Serialization
impl Program {
    /// Writes this [Program] to the provided file path.
    #[cfg(feature = "std")]
    pub fn write_to_file<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }

        // NOTE: We're protecting against unwinds here due to i/o errors that will get turned into
        // panics if writing to the underlying file fails. This is because ByteWriter does not have
        // fallible APIs, thus WriteAdapter has to panic if writes fail. This could be fixed, but
        // that has to happen upstream in winterfell
        std::panic::catch_unwind(|| match std::fs::File::create(path) {
            Ok(ref mut file) => {
                self.write_into(file);
                Ok(())
            },
            Err(err) => Err(err),
        })
        .map_err(|p| {
            match p.downcast::<std::io::Error>() {
                // SAFETY: It is guaranteed to be safe to read Box<std::io::Error>
                Ok(err) => unsafe { core::ptr::read(&*err) },
                // Propagate unknown panics
                Err(err) => std::panic::resume_unwind(err),
            }
        })?
    }
}

impl Serializable for Program {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.mast_forest.write_into(target);
        self.kernel.write_into(target);
        target.write_u32(self.entrypoint.as_u32());
    }
}

impl Deserializable for Program {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let mast_forest = source.read()?;
        let kernel = source.read()?;
        let entrypoint = MastNodeId::from_u32_safe(source.read_u32()?, &mast_forest)?;

        Ok(Self { mast_forest, kernel, entrypoint })
    }
}

impl Index<MastNodeId> for Program {
    type Output = MastNode;

    fn index(&self, node_id: MastNodeId) -> &Self::Output {
        &self.mast_forest[node_id]
    }
}

impl crate::prettier::PrettyPrint for Program {
    fn render(&self) -> crate::prettier::Document {
        use crate::prettier::*;
        let entrypoint = self[self.entrypoint()].to_pretty_print(&self.mast_forest);

        indent(4, const_text("begin") + nl() + entrypoint.render()) + nl() + const_text("end")
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::prettier::PrettyPrint;
        self.pretty_print(f)
    }
}

impl From<Program> for MastForest {
    fn from(program: Program) -> Self {
        program.mast_forest
    }
}

// PROGRAM INFO
// ===============================================================================================

/// A program information set consisting of its MAST root and set of kernel procedure roots used
/// for its compilation.
///
/// This will be used as public inputs of the proof so we bind its verification to the kernel and
/// root used to execute the program. This way, we extend the correctness of the proof to the
/// security guarantees provided by the kernel. We also allow the user to easily prove the
/// membership of a given kernel procedure for a given proof, without compromising its
/// zero-knowledge properties.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramInfo {
    program_hash: RpoDigest,
    kernel: Kernel,
}

impl ProgramInfo {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of a program info.
    pub const fn new(program_hash: RpoDigest, kernel: Kernel) -> Self {
        Self { program_hash, kernel }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the program hash computed from its code block root.
    pub const fn program_hash(&self) -> &RpoDigest {
        &self.program_hash
    }

    /// Returns the program kernel used during the compilation.
    pub const fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// Returns the list of procedures of the kernel used during the compilation.
    pub fn kernel_procedures(&self) -> &[RpoDigest] {
        self.kernel.proc_hashes()
    }
}

impl From<Program> for ProgramInfo {
    fn from(program: Program) -> Self {
        let program_hash = program.hash();
        let kernel = program.kernel().clone();

        Self { program_hash, kernel }
    }
}

// SERIALIZATION
// ------------------------------------------------------------------------------------------------

impl Serializable for ProgramInfo {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.program_hash.write_into(target);
        self.kernel.write_into(target);
    }
}

impl Deserializable for ProgramInfo {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let program_hash = source.read()?;
        let kernel = source.read()?;
        Ok(Self { program_hash, kernel })
    }
}

// TO ELEMENTS
// ------------------------------------------------------------------------------------------------

impl ToElements for ProgramInfo {
    fn to_elements(&self) -> Vec<Felt> {
        let num_kernel_proc_elements = self.kernel.proc_hashes().len() * WORD_SIZE;
        let mut result = Vec::with_capacity(WORD_SIZE + num_kernel_proc_elements);

        // append program hash elements
        result.extend_from_slice(self.program_hash.as_elements());

        // append kernel procedure hash elements
        for proc_hash in self.kernel.proc_hashes() {
            result.extend_from_slice(proc_hash.as_elements());
        }
        result
    }
}
