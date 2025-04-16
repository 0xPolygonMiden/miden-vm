use alloc::vec::Vec;

use crate::{Felt, crypto::ElementHasher};

/// Arithmetic operation applied to two nodes in the evaluation graph.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Op {
    Sub = 0,
    Mul = 1,
    Add = 2,
}

impl TryFrom<u64> for Op {
    type Error = u64;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let op = match value {
            0 => Self::Sub,
            1 => Self::Mul,
            2 => Self::Add,
            _ => return Err(value),
        };
        Ok(op)
    }
}

/// An `EncodedCircuit` represents a `Circuit` as a list of field elements, containing both
/// constants and instructions.
#[derive(Clone, Debug)]
pub struct EncodedCircuit {
    pub num_vars: usize,
    pub num_eval: usize,
    pub encoded_circuit: Vec<Felt>,
}

impl EncodedCircuit {
    /// Number of bits used to represent the ID of a node in the evaluation graph.
    /// Define as 30 bits to ensure two indices and the operation can be encoded in a single `Felt`
    pub const ID_BITS: u64 = 30;

    /// Maximum allowed ID, also equal to the mask extracting an ID from the lower 30 bits of a
    /// `uint`.
    pub const MAX_ID: u32 = (1 << Self::ID_BITS) - 1;

    /// Given a `Felt`, try to recover the components `id_l, id_r, op`.
    pub fn decode_instruction(instruction: Felt) -> Option<(u32, u32, Op)> {
        let mut remaining = instruction.as_int();
        let id_l = (remaining & Self::MAX_ID as u64) as u32;
        remaining >>= Self::ID_BITS;
        let id_r = (remaining & Self::MAX_ID as u64) as u32;
        remaining >>= Self::ID_BITS;

        // Ensure the ID did not overflow
        if id_l > Self::MAX_ID || id_r > Self::MAX_ID {
            return None;
        }

        let op = Op::try_from(remaining).ok()?;
        Some((id_l, id_r, op))
    }

    // HASHING

    /// Compute the hash of all circuit constants and instructions.
    fn raw_circuit_hash<H: ElementHasher<BaseField = Felt>>(&self) -> H::Digest {
        debug_assert_eq!(self.encoded_circuit.len() % 8, 0);
        H::hash_elements(&self.encoded_circuit)
    }

    /// Returns the digest of the circuit including a header
    pub fn circuit_hash<H: ElementHasher<BaseField = Felt>>(&self) -> H::Digest {
        todo!()
    }

    // HELPERS

    pub fn num_constants(&self) -> usize {
        (self.encoded_circuit.len() - self.num_eval) / 2
    }

    pub fn num_inputs(&self) -> usize {
        self.num_vars - self.num_constants()
    }
}
