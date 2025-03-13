use alloc::vec::Vec;

use miden_crypto::hash::rpo::RpoDigest;

use crate::{
    errors::KernelError,
    utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable},
};

// KERNEL
// ================================================================================================

/// A list of procedure hashes defining a VM kernel.
///
/// The internally-stored list always has a consistent order, regardless of the order of procedure
/// list used to instantiate a kernel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Kernel(Vec<RpoDigest>);

impl Kernel {
    /// The maximum number of procedures which can be exported from a Kernel.
    pub const MAX_NUM_PROCEDURES: usize = u8::MAX as usize;

    /// Returns a new [Kernel] instantiated with the specified procedure hashes.
    pub fn new(proc_hashes: &[RpoDigest]) -> Result<Self, KernelError> {
        if proc_hashes.len() > Self::MAX_NUM_PROCEDURES {
            Err(KernelError::TooManyProcedures(Self::MAX_NUM_PROCEDURES, proc_hashes.len()))
        } else {
            let mut hashes = proc_hashes.to_vec();
            hashes.sort();

            let duplicated = hashes.windows(2).any(|data| data[0] == data[1]);

            if duplicated {
                Err(KernelError::DuplicatedProcedures)
            } else {
                Ok(Self(hashes))
            }
        }
    }

    /// Returns true if this kernel does not contain any procedures.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if a procedure with the specified hash belongs to this kernel.
    pub fn contains_proc(&self, proc_hash: RpoDigest) -> bool {
        self.0.binary_search(&proc_hash).is_ok()
    }

    /// Returns a list of procedure hashes contained in this kernel.
    pub fn proc_hashes(&self) -> &[RpoDigest] {
        &self.0
    }
}

// this is required by AIR as public inputs will be serialized with the proof
impl Serializable for Kernel {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        // expect is OK here because the number of procedures is enforced by the constructor
        target.write_u8(self.0.len().try_into().expect("too many kernel procedures"));
        target.write_many(&self.0)
    }
}

impl Deserializable for Kernel {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let len = source.read_u8()? as usize;
        let kernel = source.read_many::<RpoDigest>(len)?;
        Ok(Self(kernel))
    }
}
