use super::{chiplets::hasher::Digest, errors, Felt};
use crate::utils::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};
use alloc::vec::Vec;

pub mod blocks;

mod info;
pub use info::ProgramInfo;

#[cfg(test)]
mod tests;

// KERNEL
// ================================================================================================

/// A list of procedure hashes defining a VM kernel.
///
/// The internally-stored list always has a consistent order, regardless of the order of procedure
/// list used to instantiate a kernel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Kernel(Vec<Digest>);

pub const MAX_KERNEL_PROCEDURES: usize = u8::MAX as usize;

impl Kernel {
    /// Returns a new [Kernel] instantiated with the specified procedure hashes.
    pub fn new(proc_hashes: &[Digest]) -> Result<Self, errors::KernelError> {
        if proc_hashes.len() > MAX_KERNEL_PROCEDURES {
            Err(errors::KernelError::TooManyProcedures(MAX_KERNEL_PROCEDURES, proc_hashes.len()))
        } else {
            let mut hashes = proc_hashes.to_vec();
            hashes.sort_by_key(|v| v.as_bytes()); // ensure consistent order

            let duplicated = hashes.windows(2).any(|data| data[0] == data[1]);

            if duplicated {
                Err(errors::KernelError::DuplicatedProcedures)
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
        debug_assert!(self.0.len() <= MAX_KERNEL_PROCEDURES);
        target.write_usize(self.0.len());
        target.write_many(&self.0)
    }
}

impl Deserializable for Kernel {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let len = source.read_usize()?;
        if len > MAX_KERNEL_PROCEDURES {
            return Err(DeserializationError::InvalidValue(format!(
                "Number of kernel procedures can not be more than {}, but {} was provided",
                MAX_KERNEL_PROCEDURES, len
            )));
        }
        let kernel = source.read_many::<Digest>(len)?;
        Ok(Self(kernel))
    }
}
