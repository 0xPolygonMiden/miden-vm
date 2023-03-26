use super::{
    super::{ToElements, WORD_SIZE},
    ByteReader, ByteWriter, Deserializable, DeserializationError, Digest, Felt, Kernel, Program,
    Serializable, Vec,
};

// PROGRAM INFO
// ================================================================================================

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
    program_hash: Digest,
    kernel: Kernel,
}

impl ProgramInfo {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of a program info.
    pub const fn new(program_hash: Digest, kernel: Kernel) -> Self {
        Self {
            program_hash,
            kernel,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the program hash computed from its code block root.
    pub const fn program_hash(&self) -> &Digest {
        &self.program_hash
    }

    /// Returns the program kernel used during the compilation.
    pub const fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// Returns the list of procedures of the kernel used during the compilation.
    pub fn kernel_procedures(&self) -> &[Digest] {
        self.kernel.proc_hashes()
    }
}

impl From<Program> for ProgramInfo {
    fn from(program: Program) -> Self {
        let Program { root, kernel, .. } = program;
        let program_hash = root.hash();

        Self {
            program_hash,
            kernel,
        }
    }
}

// SERIALIZATION
// ------------------------------------------------------------------------------------------------

impl Serializable for ProgramInfo {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.program_hash.write_into(target);
        <Kernel as Serializable>::write_into(&self.kernel, target);
    }
}

impl Deserializable for ProgramInfo {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let program_hash = source.read()?;
        let kernel = source.read()?;
        Ok(Self {
            program_hash,
            kernel,
        })
    }
}

// TO ELEMENTS
// ------------------------------------------------------------------------------------------------

impl ToElements<Felt> for ProgramInfo {
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
