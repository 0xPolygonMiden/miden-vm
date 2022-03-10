use vm_core::{hasher::Digest, MIN_STACK_DEPTH};
use winter_air::{
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions as WinterProofOptions, TraceInfo,
};
use winter_utils::{ByteWriter, Serializable};

mod options;
//mod utils;

// EXPORTS
// ================================================================================================

pub use options::ProofOptions;
pub use vm_core::{utils::ToElements, Felt, FieldElement, StarkField};
pub use winter_air::{FieldExtension, HashFunction};

// PROCESSOR AIR
// ================================================================================================

pub struct ProcessorAir {
    context: AirContext<Felt>,
}

impl Air for ProcessorAir {
    type BaseField = Felt;
    type PublicInputs = PublicInputs;

    fn new(
        _trace_info: TraceInfo,
        _pub_inputs: PublicInputs,
        _options: WinterProofOptions,
    ) -> Self {
        unimplemented!()
    }

    fn get_assertions(&self) -> Vec<Assertion<Felt>> {
        unimplemented!()
    }

    fn evaluate_transition<E: FieldElement<BaseField = Felt>>(
        &self,
        _frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        _result: &mut [E],
    ) {
        unimplemented!()
    }

    fn context(&self) -> &AirContext<Felt> {
        &self.context
    }
}

// PUBLIC INPUTS
// ================================================================================================

pub struct PublicInputs {
    program_hash: Digest,
    init_stack_state: [Felt; MIN_STACK_DEPTH],
    last_stack_state: [Felt; MIN_STACK_DEPTH],
}

impl PublicInputs {
    pub fn new(
        program_hash: Digest,
        init_stack_state: [Felt; MIN_STACK_DEPTH],
        last_stack_state: [Felt; MIN_STACK_DEPTH],
    ) -> Self {
        Self {
            program_hash,
            init_stack_state,
            last_stack_state,
        }
    }
}

impl Serializable for PublicInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write(self.program_hash.as_elements());
        target.write(self.init_stack_state.as_slice());
        target.write(self.last_stack_state.as_slice());
    }
}

// TRACE METADATA
// ================================================================================================

pub struct TraceMetadata {
    pub op_count: usize,
    pub ctx_depth: usize,
    pub loop_depth: usize,
    pub stack_depth: usize,
}

impl TraceMetadata {
    pub fn from_trace_info(_trace_info: &TraceInfo) -> Self {
        unimplemented!()
    }
}
