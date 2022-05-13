use vm_core::{
    hasher::Digest,
    utils::{ByteWriter, Serializable},
    CLK_COL_IDX, FMP_COL_IDX, MIN_STACK_DEPTH, STACK_TRACE_OFFSET,
};
use winter_air::{
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions as WinterProofOptions, TraceInfo,
    TransitionConstraintDegree,
};

mod aux_table;
mod options;
mod range;
mod utils;
use utils::TransitionConstraintRange;

// EXPORTS
// ================================================================================================

pub use options::ProofOptions;
pub use vm_core::{utils::ToElements, Felt, FieldElement, StarkField};
pub use winter_air::{FieldExtension, HashFunction};

// PROCESSOR AIR
// ================================================================================================

/// TODO: add docs
pub struct ProcessorAir {
    context: AirContext<Felt>,
    stack_inputs: Vec<Felt>,
    stack_outputs: Vec<Felt>,
    constraint_ranges: TransitionConstraintRange,
}

impl ProcessorAir {
    /// Returns last step of the execution trace.
    pub fn last_step(&self) -> usize {
        self.trace_length() - self.context().num_transition_exemptions()
    }
}

impl Air for ProcessorAir {
    type BaseField = Felt;
    type PublicInputs = PublicInputs;

    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: WinterProofOptions) -> Self {
        // --- system -----------------------------------------------------------------------------
        let mut degrees = vec![
            TransitionConstraintDegree::new(1), // clk' = clk + 1
        ];

        // --- range checker ----------------------------------------------------------------------
        let mut range_checker_degrees = range::get_transition_constraint_degrees();
        degrees.append(&mut range_checker_degrees);

        // --- auxiliary table of co-processors (hasher, bitwise, memory) -------------------------
        let mut aux_table_degrees = aux_table::get_transition_constraint_degrees();
        degrees.append(&mut aux_table_degrees);

        // Define the transition constraint ranges.
        let constraint_ranges = TransitionConstraintRange::new(
            1,
            range::get_transition_constraint_count(),
            aux_table::get_transition_constraint_count(),
        );

        // TODO: determine dynamically
        let num_assertions = 2
            + pub_inputs.stack_inputs.len()
            + pub_inputs.stack_outputs.len()
            + range::NUM_ASSERTIONS;

        // create the context and set the number of transition constraint exemptions to two; this
        // allows us to inject random values into the last row of the execution trace
        let context = AirContext::new(trace_info, degrees, num_assertions, options)
            .set_num_transition_exemptions(2);

        Self {
            context,
            stack_inputs: pub_inputs.stack_inputs,
            stack_outputs: pub_inputs.stack_outputs,
            constraint_ranges,
        }
    }

    // PERIODIC COLUMNS
    // --------------------------------------------------------------------------------------------

    /// Returns a set of periodic columns for the ProcessorAir.
    ///
    /// The columns consist of:
    /// - k0 column, which has a repeating pattern of a single one followed by 7 zeros.
    /// - k1 column, which has a repeating pattern of a 7 ones followed by a single zero.
    fn get_periodic_column_values(&self) -> Vec<Vec<Felt>> {
        vec![
            aux_table::BITWISE_POW2_K0_MASK.to_vec(),
            aux_table::BITWISE_POW2_K1_MASK.to_vec(),
        ]
    }

    // ASSERTIONS
    // --------------------------------------------------------------------------------------------

    #[allow(clippy::vec_init_then_push)]
    fn get_assertions(&self) -> Vec<Assertion<Felt>> {
        let mut result = Vec::new();

        // --- set assertions for the first step --------------------------------------------------
        // first value of clk is 0
        result.push(Assertion::single(CLK_COL_IDX, 0, Felt::ZERO));

        // first value of fmp is 2^30
        result.push(Assertion::single(FMP_COL_IDX, 0, Felt::new(2u64.pow(30))));

        // stack columns at the first step should be set to stack inputs
        for (i, &value) in self.stack_inputs.iter().enumerate() {
            result.push(Assertion::single(STACK_TRACE_OFFSET + i, 0, value));
        }

        // Add initial assertions for the range checker.
        range::get_assertions_first_step(&mut result);

        // --- set assertions for the last step ---------------------------------------------------
        let last_step = self.last_step();

        // stack columns at the last step should be set to stack outputs
        for (i, &value) in self.stack_outputs.iter().enumerate() {
            result.push(Assertion::single(STACK_TRACE_OFFSET + i, last_step, value));
        }

        // Add the range checker's assertions for the last step.
        range::get_assertions_last_step(&mut result, last_step);

        result
    }

    // TRANSITION CONSTRAINTS
    // --------------------------------------------------------------------------------------------

    fn evaluate_transition<E: FieldElement<BaseField = Felt>>(
        &self,
        frame: &EvaluationFrame<E>,
        periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();

        // --- system -----------------------------------------------------------------------------
        // clk' = clk + 1
        result[0] = next[CLK_COL_IDX] - (current[CLK_COL_IDX] + E::ONE);

        // --- range checker ----------------------------------------------------------------------
        range::enforce_constraints::<E>(
            frame,
            select_result_range!(result, self.constraint_ranges.range_checker),
        );

        // --- auxiliary table of co-processors (hasher, bitwise, memory) -------------------------
        aux_table::enforce_constraints::<E>(
            frame,
            periodic_values,
            select_result_range!(result, self.constraint_ranges.aux_table),
        );
    }

    fn context(&self) -> &AirContext<Felt> {
        &self.context
    }
}

// PUBLIC INPUTS
// ================================================================================================

#[derive(Debug)]
pub struct PublicInputs {
    program_hash: Digest,
    stack_inputs: Vec<Felt>,
    stack_outputs: Vec<Felt>,
}

impl PublicInputs {
    pub fn new(program_hash: Digest, stack_inputs: Vec<Felt>, stack_outputs: Vec<Felt>) -> Self {
        assert!(
            stack_inputs.len() <= MIN_STACK_DEPTH,
            "too many stack inputs"
        );
        assert!(
            stack_outputs.len() <= MIN_STACK_DEPTH,
            "too many stack outputs"
        );

        Self {
            program_hash,
            stack_inputs,
            stack_outputs,
        }
    }
}

impl Serializable for PublicInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write(self.program_hash.as_elements());
        target.write(self.stack_inputs.as_slice());
        target.write(self.stack_outputs.as_slice());
    }
}
