#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::vec::Vec;

use vm_core::{
    utils::{ByteReader, ByteWriter, Deserializable, Serializable},
    ExtensionOf, ProgramInfo, StackInputs, StackOutputs, ONE, ZERO,
};
use winter_air::{
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions as WinterProofOptions, TraceInfo,
    TransitionConstraintDegree,
};

mod constraints;
pub use constraints::stack;
use constraints::{chiplets, range};

pub mod trace;
pub use trace::rows::RowIndex;
use trace::*;

mod errors;
mod options;
mod proof;

mod utils;
// RE-EXPORTS
// ================================================================================================
pub use errors::ExecutionOptionsError;
pub use options::{ExecutionOptions, ProvingOptions};
pub use proof::{ExecutionProof, HashFunction};
use utils::TransitionConstraintRange;
pub use vm_core::{
    utils::{DeserializationError, ToElements},
    Felt, FieldElement, StarkField,
};
pub use winter_air::{
    AuxRandElements, FieldExtension, LagrangeKernelEvaluationFrame, PartitionOptions,
};

// PROCESSOR AIR
// ================================================================================================

/// TODO: add docs
pub struct ProcessorAir {
    context: AirContext<Felt>,
    stack_inputs: StackInputs,
    stack_outputs: StackOutputs,
    constraint_ranges: TransitionConstraintRange,
}

impl ProcessorAir {
    /// Returns last step of the execution trace.
    pub fn last_step(&self) -> usize {
        self.trace_length() - self.context().num_transition_exemptions()
    }
}

impl Air for ProcessorAir {
    type GkrProof = ();
    type GkrVerifier = ();
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

        let aux_degrees = range::get_aux_transition_constraint_degrees();

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
        let num_main_assertions = 2 + stack::NUM_ASSERTIONS + range::NUM_ASSERTIONS;

        // Define the number of boundary constraints for the auxiliary execution trace segment.
        let num_aux_assertions = stack::NUM_AUX_ASSERTIONS + range::NUM_AUX_ASSERTIONS;

        // Create the context and set the number of transition constraint exemptions to two; this
        // allows us to inject random values into the last row of the execution trace.
        let context = AirContext::new_multi_segment(
            trace_info,
            main_degrees,
            aux_degrees,
            num_main_assertions,
            num_aux_assertions,
            None,
            options,
        )
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
        stack::get_assertions_first_step(&mut result, &*self.stack_inputs);

        // Add initial assertions for the range checker.
        range::get_assertions_first_step(&mut result);

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
        _aux_rand_elements: &AuxRandElements<E>,
    ) -> Vec<Assertion<E>> {
        let mut result: Vec<Assertion<E>> = Vec::new();

        // --- set assertions for the first step --------------------------------------------------

        // add initial assertions for the stack's auxiliary columns.
        stack::get_aux_assertions_first_step(&mut result);

        // Add initial assertions for the range checker's auxiliary columns.
        range::get_aux_assertions_first_step::<E>(&mut result);

        // --- set assertions for the last step ---------------------------------------------------
        let last_step = self.last_step();

        // add the stack's auxiliary column assertions for the last step.
        stack::get_aux_assertions_last_step(&mut result, last_step);

        // Add the range checker's auxiliary column assertions for the last step.
        range::get_aux_assertions_last_step::<E>(&mut result, last_step);

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
        // --- range checker ----------------------------------------------------------------------
        range::enforce_aux_constraints::<F, E>(
            main_frame,
            aux_frame,
            aux_rand_elements.rand_elements(),
            result,
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
    program_info: ProgramInfo,
    stack_inputs: StackInputs,
    stack_outputs: StackOutputs,
}

impl PublicInputs {
    pub fn new(
        program_info: ProgramInfo,
        stack_inputs: StackInputs,
        stack_outputs: StackOutputs,
    ) -> Self {
        Self {
            program_info,
            stack_inputs,
            stack_outputs,
        }
    }
}

impl vm_core::ToElements<Felt> for PublicInputs {
    fn to_elements(&self) -> Vec<Felt> {
        let mut result = self.program_info.to_elements();
        result.append(&mut self.stack_inputs.to_vec());
        result.append(&mut self.stack_outputs.to_vec());
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
    }
}

impl Deserializable for PublicInputs {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let program_info = ProgramInfo::read_from(source)?;
        let stack_inputs = StackInputs::read_from(source)?;
        let stack_outputs = StackOutputs::read_from(source)?;

        Ok(PublicInputs {
            program_info,
            stack_inputs,
            stack_outputs,
        })
    }
}
