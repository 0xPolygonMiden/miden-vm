use distaff_processor::{
    CF_OP_BITS_RANGE, HASH_DIGEST_SIZE, HD_OP_BITS_RANGE, LD_OP_BITS_RANGE, OP_COUNTER_IDX,
    OP_SPONGE_RANGE,
};
use core::convert::TryFrom;
use distaff_utils::hasher::ARK;
use std::convert::TryInto;
use winter_utils::group_slice_elements;
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

pub mod utils;
use utils::ToElements;

// PROCESSOR AIR
// ================================================================================================

pub struct ProcessorAir {
    context: AirContext<BaseElement>,
    op_count: usize,
    inputs: Vec<BaseElement>,
    outputs: Vec<BaseElement>,
    program_hash: [BaseElement; HASH_DIGEST_SIZE],
    ctx_depth: usize,
    loop_depth: usize,
    stack_depth: usize,
    decoder_constraint_count: usize,
}

impl Air for ProcessorAir {
    type BaseElement = BaseElement;
    type PublicInputs = PublicInputs;

    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        let meta = TraceMetadata::from_trace_info(&trace_info);

        let mut tcd = decoder::get_transition_constraint_degrees(meta.ctx_depth, meta.loop_depth);
        let decoder_constraint_count = tcd.len();
        tcd.append(&mut stack::get_transition_constraint_degrees(
            meta.stack_depth,
        ));

        Self {
            context: AirContext::new(trace_info, tcd, options),
            op_count: meta.op_count,
            inputs: pub_inputs.inputs,
            outputs: pub_inputs.outputs,
            program_hash: pub_inputs.program_hash,
            ctx_depth: meta.ctx_depth,
            loop_depth: meta.loop_depth,
            stack_depth: meta.stack_depth,
            decoder_constraint_count,
        }
    }

    fn get_periodic_column_values(&self) -> Vec<Vec<Self::BaseElement>> {
        let mut result = Vec::new();
        for mask in decoder::MASKS.iter() {
            result.push(mask.to_elements());
        }

        for ark in ARK.iter() {
            result.push(ark.to_vec());
        }

        result
    }

    #[allow(clippy::vec_init_then_push)]
    fn get_assertions(&self) -> Vec<Assertion<BaseElement>> {
        let mut result = Vec::new();

        // --- set assertions for the first step --------------------------------------------------

        // make sure op_counter is set to zero
        result.push(Assertion::single(OP_COUNTER_IDX, 0, BaseElement::ZERO));

        // make sure instruction sponge registers are set to zeros
        for i in OP_SPONGE_RANGE {
            result.push(Assertion::single(i, 0, BaseElement::ZERO));
        }

        // make sure cf_bits are set to HACC (000)
        for i in CF_OP_BITS_RANGE {
            result.push(Assertion::single(i, 0, BaseElement::ZERO));
        }

        // make sure low-degree op_bits are set to BEGIN (0000)
        for i in LD_OP_BITS_RANGE {
            result.push(Assertion::single(i, 0, BaseElement::ZERO));
        }

        // make sure high-degree op_bits are set to BEGIN (00)
        for i in HD_OP_BITS_RANGE {
            result.push(Assertion::single(i, 0, BaseElement::ZERO));
        }

        // make sure all context stack registers are zeros
        let ctx_stack_start = HD_OP_BITS_RANGE.end;
        let ctx_stack_end = ctx_stack_start + self.ctx_depth;
        for i in ctx_stack_start..ctx_stack_end {
            result.push(Assertion::single(i, 0, BaseElement::ZERO));
        }

        // make sure all loop stack registers are 0s
        let loop_stack_start = ctx_stack_end;
        let loop_stack_end = loop_stack_start + self.loop_depth;
        for i in loop_stack_start..loop_stack_end {
            result.push(Assertion::single(i, 0, BaseElement::ZERO));
        }

        // make sure user stack registers are set to inputs
        let user_stack_start = loop_stack_end;
        for (i, &input_value) in self.inputs.iter().enumerate() {
            result.push(Assertion::single(user_stack_start + i, 0, input_value));
        }

        // --- set assertions for the last step ---------------------------------------------------
        let last_step = self.trace_length() - 1;

        // make sure op_counter register is set to the claimed value of operations
        result.push(Assertion::single(
            OP_COUNTER_IDX,
            last_step,
            BaseElement::new(self.op_count as u128),
        ));

        // make sure operation sponge contains program hash
        let program_hash_start = OP_SPONGE_RANGE.start;
        for (i, &value) in self.program_hash.iter().enumerate() {
            result.push(Assertion::single(program_hash_start + i, last_step, value));
        }

        // make sure control flow op_bits are set VOID (111)
        for i in CF_OP_BITS_RANGE {
            result.push(Assertion::single(i, last_step, BaseElement::ONE));
        }

        // make sure low-degree op_bits are set to NOOP (11111)
        for i in LD_OP_BITS_RANGE {
            result.push(Assertion::single(i, last_step, BaseElement::ONE));
        }

        // make sure high-degree op_bits are set to NOOP (11)
        for i in HD_OP_BITS_RANGE {
            result.push(Assertion::single(i, last_step, BaseElement::ONE));
        }

        // make sure all context stack registers are zeros
        for i in ctx_stack_start..ctx_stack_end {
            result.push(Assertion::single(i, last_step, BaseElement::ZERO));
        }

        // make sure all loop stack registers are 0s
        for i in loop_stack_start..loop_stack_end {
            result.push(Assertion::single(i, last_step, BaseElement::ZERO));
        }

        // make sure user stack registers are set to outputs
        for (i, &output_value) in self.outputs.iter().enumerate() {
            result.push(Assertion::single(
                user_stack_start + i,
                last_step,
                output_value,
            ));
        }

        result
    }

    fn evaluate_transition<E: FieldElement<BaseField = BaseElement>>(
        &self,
        frame: &EvaluationFrame<E>,
        periodic_values: &[E],
        result: &mut [E],
    ) {
        let mut transition = VmTransition::new(self.ctx_depth, self.loop_depth, self.stack_depth);
        transition.update(frame);

        let (masks, ark) = periodic_values.split_at(decoder::MASKS.len());

        decoder::enforce_constraints(&transition, masks, ark, result);
        stack::enforce_constraints(
            &transition,
            ark,
            &mut result[self.decoder_constraint_count..],
        );
    }

    fn context(&self) -> &AirContext<BaseElement> {
        &self.context
    }
}

// PUBLIC INPUTS
// ================================================================================================

pub struct PublicInputs {
    program_hash: [BaseElement; HASH_DIGEST_SIZE],
    inputs: Vec<BaseElement>,
    outputs: Vec<BaseElement>,
}

impl PublicInputs {
    pub fn new(program_hash: [u8; 32], inputs: &[u128], outputs: &[u128]) -> Self {
        let program_hash: &[[u8; 16]] = group_slice_elements(&program_hash);
        let program_hash = [
            BaseElement::try_from(program_hash[0]).unwrap(),
            BaseElement::try_from(program_hash[1]).unwrap(),
        ];

        Self {
            program_hash,
            inputs: inputs
                .iter()
                .map(|&v| BaseElement::try_from(v).unwrap())
                .collect(),
            outputs: outputs
                .iter()
                .map(|&v| BaseElement::try_from(v).unwrap())
                .collect(),
        }
    }
}

impl Serializable for PublicInputs {
    fn write_into<W: winterfell::ByteWriter>(&self, target: &mut W) {
        target.write(&self.program_hash[..]);
        target.write(&self.inputs);
        target.write(&self.outputs);
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
    pub fn from_trace_info(trace_info: &TraceInfo) -> Self {
        let op_count = u64::from_le_bytes(trace_info.meta()[..8].try_into().unwrap()) as usize;
        let ctx_depth = trace_info.meta()[8] as usize;
        let loop_depth = trace_info.meta()[9] as usize;
        let decoder_width = TraceState::<BaseElement>::compute_decoder_width(ctx_depth, loop_depth);
        TraceMetadata {
            op_count,
            ctx_depth,
            loop_depth,
            stack_depth: trace_info.width() - decoder_width,
        }
    }
}
