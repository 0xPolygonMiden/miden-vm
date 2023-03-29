use super::DeserializationError;
use vm_core::{
    crypto::hash::{Blake3_192, Blake3_256, Hasher, Rpo256},
    utils::collections::Vec,
};
use winter_air::{proof::StarkProof, FieldExtension, ProofOptions as WinterProofOptions};

// EXECUTION PROOF
// ================================================================================================

/// A proof of correct execution of Miden VM.
///
/// The proof encodes the proof itself as well as STARK protocol parameters used to generate the
/// proof. However, the proof does not contain public inputs needed to verify the proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionProof {
    proof: StarkProof,
    hash_fn: HashFunction,
}

impl ExecutionProof {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of [ExecutionProof] from the specified STARK proof and hash
    /// function.
    pub const fn new(proof: StarkProof, hash_fn: HashFunction) -> Self {
        Self { proof, hash_fn }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the underlying STARK proof.
    pub const fn stark_proof(&self) -> &StarkProof {
        &self.proof
    }

    /// Returns the hash function used during proof generation process.
    pub const fn hash_fn(&self) -> HashFunction {
        self.hash_fn
    }

    /// Returns conjectured security level of this proof in bits.
    pub fn security_level(&self) -> u32 {
        match self.hash_fn {
            HashFunction::Blake3_192 => self.proof.security_level::<Blake3_192>(true),
            HashFunction::Blake3_256 => self.proof.security_level::<Blake3_256>(true),
            HashFunction::Rpo256 => self.proof.security_level::<Rpo256>(true),
        }
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Serializes this proof into a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.proof.to_bytes();
        assert!(!bytes.is_empty(), "invalid STARK proof");
        // TODO: ideally we should write hash function into the proof first to avoid reallocations
        bytes.insert(0, self.hash_fn as u8);
        bytes
    }

    /// Reads the source bytes, parsing a new proof instance.
    pub fn from_bytes(source: &[u8]) -> Result<Self, DeserializationError> {
        if source.len() < 2 {
            return Err(DeserializationError::UnexpectedEOF);
        }
        let hash_fn = HashFunction::try_from(source[0])?;
        let proof = StarkProof::from_bytes(&source[1..])?;
        Ok(Self::new(proof, hash_fn))
    }

    // DESTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Returns components of this execution proof.
    pub fn into_parts(self) -> (HashFunction, StarkProof) {
        (self.hash_fn, self.proof)
    }
}

// PROOF OPTIONS
// ================================================================================================

/// A set of parameters specifying how Miden VM execution proofs are to be generated.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProofOptions {
    options: WinterProofOptions,
    hash_fn: HashFunction,
}

impl ProofOptions {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of [ProofOptions] from the specified parameters.
    pub fn new(
        num_queries: usize,
        blowup_factor: usize,
        grinding_factor: u32,
        field_extension: FieldExtension,
        fri_folding_factor: usize,
        fri_max_remainder_size: usize,
        hash_fn: HashFunction,
    ) -> Self {
        let options = WinterProofOptions::new(
            num_queries,
            blowup_factor,
            grinding_factor,
            field_extension,
            fri_folding_factor,
            fri_max_remainder_size,
        );
        Self { options, hash_fn }
    }

    /// Creates a new preset instance of [ProofOptions] targeting 96-bit security level.
    pub fn with_96_bit_security() -> Self {
        let options = WinterProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 8, 255);
        Self {
            hash_fn: HashFunction::Blake3_192,
            options,
        }
    }

    /// Creates a new preset instance of [ProofOptions] targeting 128-bit security level.
    pub fn with_128_bit_security() -> Self {
        let options = WinterProofOptions::new(27, 16, 21, FieldExtension::Cubic, 8, 255);
        Self {
            hash_fn: HashFunction::Blake3_256,
            options,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the hash function to be used in STARK proof generation.
    pub const fn hash_fn(&self) -> HashFunction {
        self.hash_fn
    }
}

impl Default for ProofOptions {
    fn default() -> Self {
        Self::with_96_bit_security()
    }
}

impl From<ProofOptions> for WinterProofOptions {
    fn from(options: ProofOptions) -> Self {
        options.options
    }
}

// HASH FUNCTION
// ================================================================================================

/// A hash function used during STARK proof generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HashFunction {
    /// BLAKE3 hash function with 192-bit output.
    Blake3_192 = 0x00,
    /// BLAKE3 hash function with 256-bit output.
    Blake3_256 = 0x01,
    /// RPO hash function with 256-bit output.
    Rpo256 = 0x02,
}

impl Default for HashFunction {
    fn default() -> Self {
        Self::Blake3_192
    }
}

impl HashFunction {
    /// Returns the collision resistance level (in bits) of this hash function.
    pub const fn collision_resistance(&self) -> u32 {
        match self {
            HashFunction::Blake3_192 => Blake3_192::COLLISION_RESISTANCE,
            HashFunction::Blake3_256 => Blake3_256::COLLISION_RESISTANCE,
            HashFunction::Rpo256 => Rpo256::COLLISION_RESISTANCE,
        }
    }
}

impl TryFrom<u8> for HashFunction {
    type Error = DeserializationError;

    fn try_from(repr: u8) -> Result<Self, Self::Error> {
        match repr {
            0x00 => Ok(Self::Blake3_192),
            0x01 => Ok(Self::Blake3_256),
            0x02 => Ok(Self::Rpo256),
            _ => Err(DeserializationError::InvalidValue(format!(
                "the hash function representation {repr} is not valid!"
            ))),
        }
    }
}
