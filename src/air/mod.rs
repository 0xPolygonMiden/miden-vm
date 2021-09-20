use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions, Serializable, TraceInfo,
    TransitionConstraintDegree,
};

mod trace_state;
pub use trace_state::TraceState;

mod transition;
pub use transition::VmTransition;

mod decoder;
mod stack;
mod utils;

// PROCESSOR AIR
// ================================================================================================

pub struct ProcessorAir {
    context: AirContext<BaseElement>,
    ctx_depth: usize,
    loop_depth: usize,
    stack_depth: usize,
}

impl Air for ProcessorAir {
    type BaseElement = BaseElement;
    type PublicInputs = PublicInputs;

    fn new(trace_info: TraceInfo, _pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        let meta = TraceMetadata::from_trace_info(&trace_info);

        let mut tcd = stack::get_transition_constraint_degrees(meta.stack_depth);
        tcd.append(&mut decoder::get_transition_constraint_degrees(
            meta.ctx_depth,
            meta.loop_depth,
        ));

        Self {
            context: AirContext::new(trace_info, tcd, options),
            ctx_depth: meta.ctx_depth,
            loop_depth: meta.loop_depth,
            stack_depth: meta.stack_depth,
        }
    }

    fn get_periodic_column_values(&self) -> Vec<Vec<Self::BaseElement>> {
        // TODO
        unimplemented!()
    }

    fn get_assertions(&self) -> Vec<Assertion<BaseElement>> {
        // TODO
        unimplemented!()
    }

    fn evaluate_transition<E: FieldElement<BaseField = BaseElement>>(
        &self,
        frame: &EvaluationFrame<E>,
        periodic_values: &[E],
        result: &mut [E],
    ) {
        let mut transition = VmTransition::new(self.ctx_depth, self.loop_depth, self.stack_depth);
        transition.update(frame);

        // TODO
        let (masks, ark) = periodic_values.split_at(1);

        decoder::enforce_constraints(&transition, masks, ark, result);
        stack::enforce_constraints(&transition, ark, result);
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

impl PublicInputs {
    pub fn new(_program_hash: [u8; 32], _inputs: &[u128], _outputs: &[u128]) -> Self {
        // TODO
        unimplemented!()
    }
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
    pub ctx_depth: usize,
    pub loop_depth: usize,
    pub stack_depth: usize,
}

impl TraceMetadata {
    pub fn from_trace_info(trace_info: &TraceInfo) -> Self {
        let ctx_depth = trace_info.meta()[0] as usize;
        let loop_depth = trace_info.meta()[1] as usize;
        TraceMetadata {
            ctx_depth,
            loop_depth,
            stack_depth: 0, // TODO
        }
    }
}
