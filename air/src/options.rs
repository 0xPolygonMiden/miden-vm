use super::{HashFunction, ProvingError};
use winter_air::{FieldExtension, ProofOptions as WinterProofOptions};

// PROOF OPTIONS
// ================================================================================================

/// A set of parameters specifying how Miden VM execution proofs are to be generated.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProvingOptions {
    pub exec_options: ExecutionOptions,
    pub proof_options: WinterProofOptions,
    pub hash_fn: HashFunction,
}

impl ProvingOptions {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of [ProvingOptions] from the specified parameters.
    pub fn new(
        num_queries: usize,
        blowup_factor: usize,
        grinding_factor: u32,
        field_extension: FieldExtension,
        fri_folding_factor: usize,
        fri_max_remainder_size: usize,
        hash_fn: HashFunction,
    ) -> Self {
        let proof_options = WinterProofOptions::new(
            num_queries,
            blowup_factor,
            grinding_factor,
            field_extension,
            fri_folding_factor,
            fri_max_remainder_size,
        );
        let exec_options = ExecutionOptions::default();
        Self {
            exec_options,
            proof_options,
            hash_fn,
        }
    }

    /// Creates a new preset instance of [ProvingOptions] targeting 96-bit security level.
    ///
    /// If `recursive` flag is set to true, proofs will be generated using an arithmetization-
    /// friendly hash function (RPO). Such proofs are well-suited for recursive proof verification,
    /// but may take significantly longer to generate.
    pub fn with_96_bit_security(recursive: bool) -> Self {
        if recursive {
            let proof_options = WinterProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 4, 7);
            let exec_options = ExecutionOptions::default();
            Self {
                exec_options,
                proof_options,
                hash_fn: HashFunction::Rpo256,
            }
        } else {
            let proof_options =
                WinterProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 8, 255);
            let exec_options = ExecutionOptions::default();
            Self {
                exec_options,
                proof_options,
                hash_fn: HashFunction::Blake3_192,
            }
        }
    }

    /// Creates a new preset instance of [ProvingOptions] targeting 128-bit security level.
    ///
    /// If `recursive` flag is set to true, proofs will be generated using an arithmetization-
    /// friendly hash function (RPO). Such proofs are well-suited for recursive proof verification,
    /// but may take significantly longer to generate.
    pub fn with_128_bit_security(recursive: bool) -> Self {
        if recursive {
            let proof_options = WinterProofOptions::new(27, 16, 21, FieldExtension::Cubic, 4, 7);
            let exec_options = ExecutionOptions::default();
            Self {
                exec_options,
                proof_options,
                hash_fn: HashFunction::Rpo256,
            }
        } else {
            let proof_options = WinterProofOptions::new(27, 16, 21, FieldExtension::Cubic, 8, 255);
            let exec_options = ExecutionOptions::default();
            Self {
                exec_options,
                proof_options,
                hash_fn: HashFunction::Blake3_256,
            }
        }
    }

    /// Adds the [ExecutionOptions] to the [ProvingOptions] to provide maximum cycles limit data
    /// and average cycles data.
    pub fn with_execution_options(mut self, exec_options: ExecutionOptions) -> Self {
        self.exec_options = exec_options;
        self
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the hash function to be used in STARK proof generation.
    pub const fn hash_fn(&self) -> HashFunction {
        self.hash_fn
    }

    /// Returns the execution options specified for this [ProvingOptions]
    pub const fn execution_options(&self) -> &ExecutionOptions {
        &self.exec_options
    }
}

impl Default for ProvingOptions {
    fn default() -> Self {
        Self::with_96_bit_security(false)
    }
}

impl From<ProvingOptions> for WinterProofOptions {
    fn from(options: ProvingOptions) -> Self {
        options.proof_options
    }
}

// EXECUTION OPTIONS
// ================================================================================================

/// A set of parameters specifying execution parameters of the VM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionOptions {
    max_cycles: Option<u32>,
    expected_cycles: u32,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        ExecutionOptions {
            max_cycles: None,
            expected_cycles: 64,
        }
    }
}

impl ExecutionOptions {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of [ExecutionOptions] from the specified parameters.
    pub fn new(max_cycles: Option<u32>, expected_cycles: u32) -> Result<Self, ProvingError> {
        if max_cycles.is_some_and(|max_cycles| max_cycles < expected_cycles) {
            return Err(ProvingError::ContradictingCycleNumbers(
                max_cycles.unwrap(),
                expected_cycles,
            ));
        }
        Ok(ExecutionOptions {
            max_cycles,
            expected_cycles,
        })
    }
}
