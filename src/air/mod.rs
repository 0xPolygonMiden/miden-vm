use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions, Serializable, TraceInfo,
    TransitionConstraintDegree,
};

mod trace_state;
pub use trace_state::TraceState;

mod decoder;
mod stack;
mod utils;

// PROCESSOR AIR
// ================================================================================================

pub struct ProcessorAir {
    context: AirContext<BaseElement>,
}

impl Air for ProcessorAir {
    type BaseElement = BaseElement;
    type PublicInputs = PublicInputs;

    fn new(trace_info: TraceInfo, _pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        let meta = parse_trace_meta(&trace_info);

        let mut tcd = stack::get_transition_constraint_degrees(meta.stack_depth);
        tcd.append(&mut decoder::get_transition_constraint_degrees(
            meta.ctx_depth,
            meta.loop_depth,
        ));

        Self {
            context: AirContext::new(trace_info, tcd, options),
        }
    }

    fn get_assertions(&self) -> Vec<Assertion<BaseElement>> {
        unimplemented!()
    }

    fn evaluate_transition<E: FieldElement<BaseField = BaseElement>>(
        &self,
        _frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        _result: &mut [E],
    ) {
        unimplemented!()
    }

    fn context(&self) -> &AirContext<BaseElement> {
        &self.context
    }
}

// PUBLIC INPUTS
// ================================================================================================

pub struct PublicInputs {
    program_hash: [u8; 32],
    inputs: Vec<BaseElement>,
    outputs: Vec<BaseElement>,
}

impl Serializable for PublicInputs {
    fn write_into<W: winterfell::ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.program_hash);
        target.write(&self.inputs);
        target.write(&self.outputs);
    }
}

// TRACE METADATA
// ================================================================================================

pub struct TraceMetadata {
    ctx_depth: usize,
    loop_depth: usize,
    stack_depth: usize,
}

fn parse_trace_meta(trace_info: &TraceInfo) -> TraceMetadata {
    let ctx_depth = trace_info.meta()[0] as usize;
    let loop_depth = trace_info.meta()[1] as usize;
    TraceMetadata {
        ctx_depth,
        loop_depth,
        stack_depth: 0, // TODO
    }
}
