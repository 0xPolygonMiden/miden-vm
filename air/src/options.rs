use super::{
    trace::MIN_TRACE_LEN, ExecutionOptionsError, FieldExtension, HashFunction, WinterProofOptions,
};

// PROVING OPTIONS
// ================================================================================================

/// A set of parameters specifying how Miden VM execution proofs are to be generated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvingOptions {
    exec_options: ExecutionOptions,
    proof_options: WinterProofOptions,
    hash_fn: HashFunction,
}

impl ProvingOptions {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------

    /// Standard proof parameters for 96-bit conjectured security in non-recursive context.
    pub const REGULAR_96_BITS: WinterProofOptions =
        WinterProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 8, 255);

    /// Standard proof parameters for 128-bit conjectured security in non-recursive context.
    pub const REGULAR_128_BITS: WinterProofOptions =
        WinterProofOptions::new(27, 16, 21, FieldExtension::Cubic, 8, 255);

    /// Standard proof parameters for 96-bit conjectured security in recursive context.
    pub const RECURSIVE_96_BITS: WinterProofOptions =
        WinterProofOptions::new(27, 8, 16, FieldExtension::Quadratic, 4, 7);

    /// Standard proof parameters for 128-bit conjectured security in recursive context.
    pub const RECURSIVE_128_BITS: WinterProofOptions =
        WinterProofOptions::new(27, 16, 21, FieldExtension::Cubic, 4, 7);

    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of [ProvingOptions] from the specified parameters.
    pub fn new(
        num_queries: usize,
        blowup_factor: usize,
        grinding_factor: u32,
        field_extension: FieldExtension,
        fri_folding_factor: usize,
        fri_remainder_max_degree: usize,
        hash_fn: HashFunction,
    ) -> Self {
        let proof_options = WinterProofOptions::new(
            num_queries,
            blowup_factor,
            grinding_factor,
            field_extension,
            fri_folding_factor,
            fri_remainder_max_degree,
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
            Self {
                exec_options: ExecutionOptions::default(),
                proof_options: Self::RECURSIVE_96_BITS,
                hash_fn: HashFunction::Rpo256,
            }
        } else {
            Self {
                exec_options: ExecutionOptions::default(),
                proof_options: Self::REGULAR_96_BITS,
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
            Self {
                exec_options: ExecutionOptions::default(),
                proof_options: Self::RECURSIVE_128_BITS,
                hash_fn: HashFunction::Rpo256,
            }
        } else {
            Self {
                exec_options: ExecutionOptions::default(),
                proof_options: Self::REGULAR_128_BITS,
                hash_fn: HashFunction::Blake3_256,
            }
        }
    }

    /// Sets [ExecutionOptions] for this [ProvingOptions].
    ///
    /// This sets the maximum number of cycles a program is allowed to execute as well as
    /// the number of cycles the program is expected to execute.
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
///
/// - `max_cycles` specifies the maximum number of cycles a program is allowed to execute.
/// - `expected_cycles` specifies the number of cycles a program is expected to execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionOptions {
    max_cycles: u32,
    expected_cycles: u32,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        ExecutionOptions {
            max_cycles: u32::MAX,
            expected_cycles: MIN_TRACE_LEN as u32,
        }
    }
}

impl ExecutionOptions {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of [ExecutionOptions] from the specified parameters.
    ///
    /// If the `max_cycles` is `None` the maximum number of cycles will be set to `u32::MAX`
    pub fn new(
        max_cycles: Option<u32>,
        expected_cycles: u32,
    ) -> Result<Self, ExecutionOptionsError> {
        let max_cycles = max_cycles.unwrap_or(u32::MAX);
        if max_cycles < MIN_TRACE_LEN as u32 {
            return Err(ExecutionOptionsError::MaxCycleNumTooSmall(expected_cycles));
        }
        if max_cycles < expected_cycles {
            return Err(ExecutionOptionsError::ExpectedCyclesTooBig(max_cycles, expected_cycles));
        }

        // Round up the expected number of cycles to the next power of two. If it is smaller than
        // MIN_TRACE_LEN -- pad expected number to it.
        let expected_cycles = expected_cycles.next_power_of_two().max(MIN_TRACE_LEN as u32);

        Ok(ExecutionOptions {
            max_cycles,
            expected_cycles,
        })
    }

    /// Returns maximum number of cycles
    pub fn max_cycles(&self) -> u32 {
        self.max_cycles
    }

    /// Returns number of the expected cycles
    pub fn expected_cycles(&self) -> u32 {
        self.expected_cycles
    }
}
