use super::{DeserializationError, StarkProof};
use core::{ops::Deref, str::FromStr};
use vm_core::{
    crypto::hash::{Blake3_192, Blake3_256, Hasher, Rpo256},
    utils::{collections::Vec, string::String},
};
use winter_air::{FieldExtension, ProofOptions as WinterProofOptions};

// ExecutionProof
// ================================================================================================

/// An execution proof with its metadata to define the security parameters and hasher primitive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionProof {
    hasher: HashFunction,
    proof: StarkProof,
}

impl ExecutionProof {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of an execution proof from the used hash primitive and the STARK
    /// proof.
    pub const fn new(hasher: HashFunction, proof: StarkProof) -> Self {
        Self { hasher, proof }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the used hash primite to compute the proof.
    pub const fn hasher(&self) -> HashFunction {
        self.hasher
    }

    /// Returns the underlying STARK proof.
    pub const fn proof(&self) -> &StarkProof {
        &self.proof
    }

    // SERIALIZATION / DESERIALIZATION
    // --------------------------------------------------------------------------------------------

    /// Serializes this proof into a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.proof.to_bytes();
        if bytes.is_empty() {
            // TODO maybe error?
            return vec![];
        }
        bytes.insert(0, self.hasher as u8);
        bytes
    }

    /// Reads the source bytes, parsing a new proof instance.
    ///
    /// Will expect the output of `[Self::to_bytes]`.
    pub fn from_bytes(source: &[u8]) -> Result<Self, DeserializationError> {
        if source.len() < 2 {
            return Err(DeserializationError::UnexpectedEOF);
        }
        let hasher = HashFunction::try_from(source[0])?;
        let proof = StarkProof::from_bytes(&source[1..])?;
        Ok(Self { hasher, proof })
    }

    // DESTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Extracts the underlying hasher and`[StarkProof]`.
    pub fn into_inner(self) -> (HashFunction, StarkProof) {
        (self.hasher, self.proof)
    }
}

// PROOF OPTIONS
// ================================================================================================

/// A set of arguments to specify how the Miden proofs should be constructed.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProofOptions {
    hasher: HashFunction,
    options: WinterProofOptions,
}

impl ProofOptions {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance with arbitrary hasher.
    pub fn new(
        hasher: HashFunction,
        num_queries: usize,
        blowup_factor: usize,
        grinding_factor: u32,
        field_extension: FieldExtension,
        fri_folding_factor: usize,
        fri_max_remainder_size: usize,
    ) -> Self {
        let options = WinterProofOptions::new(
            num_queries,
            blowup_factor,
            grinding_factor,
            field_extension,
            fri_folding_factor,
            fri_max_remainder_size,
        );
        Self { hasher, options }
    }

    /// Creates a new preset instance with `[Blake3_192]` as hasher and 96-bits of security.
    pub fn with_96_bit_security() -> Self {
        let options = WinterProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 8, 256);
        Self {
            hasher: HashFunction::Blake3_192,
            options,
        }
    }

    /// Creates a new preset instance with `[Blake3_256]` as hasher and 128-bits of security.
    pub fn with_128_bit_security() -> Self {
        let options = WinterProofOptions::new(27, 16, 21, FieldExtension::Cubic, 8, 256);
        Self {
            hasher: HashFunction::Blake3_256,
            options,
        }
    }

    /// Creates a new preset instance with `[Rpo256]` as hasher and 128-bits of security.
    pub fn with_rpo_128_bit_security() -> Self {
        let options = WinterProofOptions::new(27, 16, 21, FieldExtension::Cubic, 8, 256);
        Self {
            hasher: HashFunction::Rpo256,
            options,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the representation of the underlying hasher.
    pub const fn hasher(&self) -> HashFunction {
        self.hasher
    }

    /// Returns the STARK protocol parameters.
    pub const fn parameters(&self) -> &WinterProofOptions {
        &self.options
    }

    // DESTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Extracts the underlying `[WinterProofOptions]`.
    pub fn into_inner(self) -> WinterProofOptions {
        self.options
    }
}

impl Default for ProofOptions {
    fn default() -> Self {
        Self::with_96_bit_security()
    }
}

impl Deref for ProofOptions {
    type Target = WinterProofOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

// PROOF HASHER
// ================================================================================================

/// A hash function selector to generate Miden proofs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HashFunction {
    Blake3_192 = 0x00,
    Blake3_256 = 0x01,
    Rpo256 = 0x02,
}

impl Default for HashFunction {
    fn default() -> Self {
        Self::Blake3_192
    }
}

impl HashFunction {
    // PROVIDERS
    // --------------------------------------------------------------------------------------------

    /// Returns the security level/collision resistance of the selected primitive.
    pub const fn security_level(&self) -> u32 {
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

impl FromStr for HashFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3-192" => Ok(Self::Blake3_192),
            "blake3-256" => Ok(Self::Blake3_256),
            "rpo-256" => Ok(Self::Rpo256),
            _ => Err(format!("{s} is not a supported hash function!")),
        }
    }
}
