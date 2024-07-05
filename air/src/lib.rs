#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::vec::Vec;

use gkr_proof::{GkrCircuitProof, GkrCircuitVerifier};
use vm_core::{
    utils::{ByteReader, ByteWriter, Deserializable, Serializable},
    ExtensionOf, ProgramInfo, StackInputs, StackOutputs, ONE, ZERO,
};
use winter_air::{
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions as WinterProofOptions, TraceInfo,
    TransitionConstraintDegree,
};
use winter_prover::matrix::ColMatrix;

mod constraints;
pub use constraints::stack;
use constraints::{chiplets, logup, range};

// TODOP: Cleanup
pub mod trace;
use trace::{logup::LAGRANGE_KERNEL_COL_IDX, *};

mod errors;
pub mod gkr_proof;
mod gkr_verifier;
pub use gkr_verifier::{
    verify_virtual_bus, CompositionPolyQueryBuilder, SumCheckVerifier, SumCheckVerifierError,
};
mod options;
mod proof;

mod utils;
use utils::TransitionConstraintRange;

// RE-EXPORTS
// ================================================================================================

pub use errors::ExecutionOptionsError;
pub use options::{ExecutionOptions, ProvingOptions};
pub use proof::{ExecutionProof, HashFunction};
pub use vm_core::{
    utils::{DeserializationError, ToElements},
    Felt, FieldElement, StarkField,
};
pub use winter_air::{AuxRandElements, FieldExtension, GkrRandElements};

// PROCESSOR AIR
// ================================================================================================

/// TODO: add docs
pub struct ProcessorAir {
    context: AirContext<Felt>,
    stack_inputs: StackInputs,
    stack_outputs: StackOutputs,
    main_trace_first_row: Vec<Felt>,
    constraint_ranges: TransitionConstraintRange,
}

impl ProcessorAir {
    /// Returns last step of the execution trace.
    pub fn last_step(&self) -> usize {
        self.trace_length() - self.context().num_transition_exemptions()
    }
}

impl Air for ProcessorAir {
    type GkrProof<E: FieldElement> = GkrCircuitProof<E>;
    type GkrVerifier<E: FieldElement> = GkrCircuitVerifier;
    type BaseField = Felt;
    type PublicInputs = PublicInputs;

    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: WinterProofOptions) -> Self {
        // --- system -----------------------------------------------------------------------------
        let mut main_degrees = vec![
            TransitionConstraintDegree::new(1), // clk' = clk + 1
        ];

        // --- stack constraints -------------------------------------------------------------------
        let mut stack_degrees = stack::get_transition_constraint_degrees();
        main_degrees.append(&mut stack_degrees);

        // --- range checker ----------------------------------------------------------------------
        let mut range_checker_degrees = range::get_transition_constraint_degrees();
        main_degrees.append(&mut range_checker_degrees);

        let aux_degrees = logup::get_aux_transition_constraint_degrees();

        // --- chiplets (hasher, bitwise, memory) -------------------------
        let mut chiplets_degrees = chiplets::get_transition_constraint_degrees();
        main_degrees.append(&mut chiplets_degrees);

        // Define the transition constraint ranges.
        let constraint_ranges = TransitionConstraintRange::new(
            1,
            stack::get_transition_constraint_count(),
            range::get_transition_constraint_count(),
            chiplets::get_transition_constraint_count(),
        );

        // Define the number of boundary constraints for the main execution trace segment.
        // TODO: determine dynamically
        let num_main_assertions =
            2 + stack::NUM_ASSERTIONS + range::NUM_ASSERTIONS + logup::NUM_ASSERTIONS;

        // Define the number of boundary constraints for the auxiliary execution trace segment.
        let num_aux_assertions = stack::NUM_AUX_ASSERTIONS + logup::NUM_AUX_ASSERTIONS;

        // Create the context and set the number of transition constraint exemptions to two; this
        // allows us to inject random values into the last row of the execution trace.
        let context = AirContext::new_multi_segment(
            trace_info,
            main_degrees,
            aux_degrees,
            num_main_assertions,
            num_aux_assertions,
            Some(LAGRANGE_KERNEL_COL_IDX),
            options,
        )
        .set_num_transition_exemptions(2);

        Self {
            context,
            stack_inputs: pub_inputs.stack_inputs,
            stack_outputs: pub_inputs.stack_outputs,
            main_trace_first_row: pub_inputs.first_main_trace_row,
            constraint_ranges,
        }
    }

    // PERIODIC COLUMNS
    // --------------------------------------------------------------------------------------------

    /// Returns a set of periodic columns for the ProcessorAir.
    fn get_periodic_column_values(&self) -> Vec<Vec<Felt>> {
        chiplets::get_periodic_column_values()
    }

    // ASSERTIONS
    // --------------------------------------------------------------------------------------------

    #[allow(clippy::vec_init_then_push)]
    fn get_assertions(&self) -> Vec<Assertion<Felt>> {
        let mut result = Vec::new();

        // --- set assertions for the first step --------------------------------------------------
        // first value of clk is 0
        result.push(Assertion::single(CLK_COL_IDX, 0, ZERO));

        // first value of fmp is 2^30
        result.push(Assertion::single(FMP_COL_IDX, 0, Felt::new(2u64.pow(30))));

        // add initial assertions for the stack.
        stack::get_assertions_first_step(&mut result, self.stack_inputs.values());

        // add initial assertions for the range checker.
        range::get_assertions_first_step(&mut result);

        // add initial assertions for logup
        logup::get_assertions_first_step(&mut result, &self.main_trace_first_row);

        // --- set assertions for the last step ---------------------------------------------------
        let last_step = self.last_step();

        // add the stack's assertions for the last step.
        stack::get_assertions_last_step(&mut result, last_step, &self.stack_outputs);

        // Add the range checker's assertions for the last step.
        range::get_assertions_last_step(&mut result, last_step);

        result
    }

    fn get_aux_assertions<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        aux_rand_elements: &AuxRandElements<E>,
        gkr_proof: Option<&Self::GkrProof<E>>,
    ) -> Vec<Assertion<E>> {
        let mut result: Vec<Assertion<E>> = Vec::new();

        let openings_combining_randomness = aux_rand_elements
            .gkr_openings_combining_randomness()
            .expect("GKR openings combining randomness not present in AuxRandElements");
        let lagrange_kernel_rand_elements = aux_rand_elements
            .lagrange()
            .expect("GKR Lagrange kernel random elements not present in AuxRandElements");

        // --- set assertions for the first step --------------------------------------------------

        // add initial assertions for the stack's auxiliary columns.
        stack::get_aux_assertions_first_step(
            &mut result,
            aux_rand_elements.rand_elements(),
            self.stack_inputs.values(),
        );

        logup::get_aux_assertions_first_step(
            &mut result,
            lagrange_kernel_rand_elements,
            &self.main_trace_first_row,
            openings_combining_randomness,
        );

        // --- set assertions for the last step ---------------------------------------------------
        let last_step = self.last_step();

        // add the stack's auxiliary column assertions for the last step.
        stack::get_aux_assertions_last_step(
            &mut result,
            aux_rand_elements.rand_elements(),
            &self.stack_outputs,
            last_step,
        );

        // Add LogUp's "s" column assertions for the last step.
        {
            let openings =
                gkr_proof.expect("GKR proof not present").get_final_opening_claim().openings;

            logup::get_aux_assertions_last_step(
                &mut result,
                openings_combining_randomness,
                &openings,
                last_step,
            );
        }

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

        // --- stack operations -------------------------------------------------------------------
        stack::enforce_constraints::<E>(
            frame,
            select_result_range!(result, self.constraint_ranges.stack),
        );

        // --- range checker ----------------------------------------------------------------------
        range::enforce_constraints::<E>(
            frame,
            select_result_range!(result, self.constraint_ranges.range_checker),
        );

        // --- chiplets (hasher, bitwise, memory) -------------------------
        chiplets::enforce_constraints::<E>(
            frame,
            periodic_values,
            select_result_range!(result, self.constraint_ranges.chiplets),
        );
    }

    fn evaluate_aux_transition<F, E>(
        &self,
        main_frame: &EvaluationFrame<F>,
        aux_frame: &EvaluationFrame<E>,
        _periodic_values: &[F],
        aux_rand_elements: &AuxRandElements<E>,
        result: &mut [E],
    ) where
        F: FieldElement<BaseField = Felt>,
        E: FieldElement<BaseField = Felt> + ExtensionOf<F>,
    {
        let gkr_openings_randomness = aux_rand_elements
            .gkr_openings_combining_randomness()
            .expect("GKR openings randomness expected to be present");

        logup::enforce_aux_constraints(main_frame, aux_frame, gkr_openings_randomness, result)
    }

    fn context(&self) -> &AirContext<Felt> {
        &self.context
    }
}

// PUBLIC INPUTS
// ================================================================================================

#[derive(Debug)]
pub struct PublicInputs {
    program_info: ProgramInfo,
    stack_inputs: StackInputs,
    stack_outputs: StackOutputs,
    first_main_trace_row: Vec<Felt>,
}

impl PublicInputs {
    pub fn new(
        program_info: ProgramInfo,
        stack_inputs: StackInputs,
        stack_outputs: StackOutputs,
        first_main_trace_row: Vec<Felt>,
    ) -> Self {
        Self {
            program_info,
            stack_inputs,
            stack_outputs,
            first_main_trace_row,
        }
    }
}

impl vm_core::ToElements<Felt> for PublicInputs {
    fn to_elements(&self) -> Vec<Felt> {
        let mut result = self.program_info.to_elements();
        result.append(&mut self.stack_inputs.to_elements());
        result.append(&mut self.stack_outputs.to_elements());
        result
    }
}

// SERIALIZATION
// ================================================================================================

impl Serializable for PublicInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.program_info.write_into(target);
        self.stack_inputs.write_into(target);
        self.stack_outputs.write_into(target);
        self.first_main_trace_row.write_into(target);
    }
}

impl Deserializable for PublicInputs {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let program_info = ProgramInfo::read_from(source)?;
        let stack_inputs = StackInputs::read_from(source)?;
        let stack_outputs = StackOutputs::read_from(source)?;
        let first_main_trace_row = Deserializable::read_from(source)?;

        Ok(PublicInputs {
            program_info,
            stack_inputs,
            stack_outputs,
            first_main_trace_row,
        })
    }
}
