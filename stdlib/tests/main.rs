mod crypto;
mod math;
mod mem;
mod sys;

use miden_stdlib::StdLibrary;
use processor::{AdviceInputs, MerkleSet, StackInputs};
use processor::{ExecutionError, ExecutionTrace, MemAdviceProvider};
use proptest::prelude::*;
use std::collections::BTreeMap;
use vm_core::{stack::STACK_TOP_SIZE, Felt, FieldElement, Program};

/// Following section suffers from code-duplication, why ?
///
/// Test helper routines were originally living in `miden-vm` crate's
/// integration test helper module ( ../miden/tests/integration/helpers/mod.rs ).
/// But for addressing issue described in https://github.com/0xPolygonMiden/miden-vm/issues/723
/// we move standard library related tests out of `miden-vm` crate and put them
/// here inside `miden-stdlib` crate. But we can't just export test helper routines
/// from `miden-vm` crate and start using that here by simply importing them - because
/// `miden-vm` crate is an aggregation of all components of this repository, meaning it's
/// dependent on `miden-stdlib` ( this crate ) too. Attempting to bring in test helper
/// routines from `miden-vm` crate results in circular dependency.
///
/// I think we can solve this code duplication problem by keeping test helper routines
/// here in `miden-stdlib` crate and by making them exportable, we ensure that they can
/// be used in `miden-vm` crate, which anyway imports this crate as development dependency.

pub const U32_BOUND: u64 = u32::MAX as u64 + 1;

pub enum TestError<'a> {
    AssemblyError(&'a str),
    ExecutionError(&'a str),
}

pub struct Test {
    pub source: String,
    pub kernel: Option<String>,
    pub stack_inputs: StackInputs,
    pub advice_inputs: AdviceInputs,
    pub in_debug_mode: bool,
}

impl Test {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates the simplest possible new test, with only a source string and no inputs.
    pub fn new(source: &str, in_debug_mode: bool) -> Self {
        Test {
            source: String::from(source),
            kernel: None,
            stack_inputs: StackInputs::default(),
            advice_inputs: AdviceInputs::default(),
            in_debug_mode,
        }
    }

    /// Creates a new test, with a MASM program source string and initial stack state.
    pub fn with_stack(source: &str, in_debug_mode: bool, stack_init: &[u64]) -> Self {
        let stack = StackInputs::try_from_values(stack_init.to_vec()).unwrap();
        Test {
            source: String::from(source),
            kernel: None,
            stack_inputs: stack,
            advice_inputs: AdviceInputs::default(),
            in_debug_mode,
        }
    }

    /// Creates a new test, with a MASM program source string, initial stack state and
    /// non-deterministic input via advice provider.
    pub fn with_advice(
        source: &str,
        in_debug_mode: bool,
        stack_init: &[u64],
        adv_tape: &[u64],
        adv_set: Vec<MerkleSet>,
        adv_map: BTreeMap<[u8; 32], Vec<Felt>>,
    ) -> Self {
        let stack = StackInputs::try_from_values(stack_init.to_vec()).unwrap();
        let advice = AdviceInputs::default()
            .with_tape_values(adv_tape.to_vec())
            .unwrap()
            .with_merkle_sets(adv_set)
            .unwrap()
            .with_values_map(adv_map);

        Test {
            source: String::from(source),
            kernel: None,
            stack_inputs: stack,
            advice_inputs: advice,
            in_debug_mode,
        }
    }

    /// Asserts that running the test for the expected TestError variant will result in an error
    /// that contains the TestError's error substring in its error message.
    pub fn expect_error(&self, error: TestError) {
        match error {
            TestError::AssemblyError(substr) => {
                assert_eq!(
                    std::panic::catch_unwind(|| self.compile())
                        .err()
                        .and_then(|a| { a.downcast_ref::<String>().map(|s| s.contains(substr)) }),
                    Some(true)
                );
            }
            TestError::ExecutionError(substr) => {
                assert_eq!(
                    std::panic::catch_unwind(|| self.execute().unwrap())
                        .err()
                        .and_then(|a| { a.downcast_ref::<String>().map(|s| s.contains(substr)) }),
                    Some(true)
                );
            }
        }
    }

    /// Builds a final stack from the provided stack-ordered array and asserts that executing the
    /// test will result in the expected final stack state.
    pub fn expect_stack(&self, final_stack: &[u64]) {
        let expected = convert_to_stack(final_stack);
        let result = self.get_last_stack_state();

        assert_eq!(expected, result);
    }

    /// Asserts that executing the test inside a proptest results in the expected final stack state.
    /// The proptest will return a test failure instead of panicking if the assertion condition
    /// fails.
    pub fn prop_expect_stack(
        &self,
        final_stack: &[u64],
    ) -> Result<(), proptest::test_runner::TestCaseError> {
        let expected = convert_to_stack(final_stack);
        let result = self.get_last_stack_state();

        prop_assert_eq!(expected, result);

        Ok(())
    }

    // UTILITY METHODS
    // --------------------------------------------------------------------------------------------

    /// Compiles a test's source and returns the resulting Program.
    pub fn compile(&self) -> Program {
        let assembler = assembly::Assembler::default()
            .with_debug_mode(self.in_debug_mode)
            .with_library(&StdLibrary::default())
            .expect("failed to load stdlib");

        match self.kernel.as_ref() {
            Some(kernel) => assembler.with_kernel(kernel).expect("kernel compilation failed"),
            None => assembler,
        }
        .compile(&self.source)
        .expect("Failed to compile test source.")
    }

    /// Compiles the test's source to a Program and executes it with the tests inputs. Returns a
    /// resulting execution trace or error.
    pub fn execute(&self) -> Result<ExecutionTrace, ExecutionError> {
        let program = self.compile();
        let advice_provider = MemAdviceProvider::from(self.advice_inputs.clone());
        processor::execute(&program, self.stack_inputs.clone(), advice_provider)
    }

    /// Returns the last state of the stack after executing a test.
    pub fn get_last_stack_state(&self) -> [Felt; STACK_TOP_SIZE] {
        let trace = self.execute().unwrap();

        trace.last_stack_state()
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Takes an array of u64 values and builds a stack, preserving their order and converting them to
/// field elements.
pub fn convert_to_stack(values: &[u64]) -> [Felt; STACK_TOP_SIZE] {
    let mut result = [Felt::ZERO; STACK_TOP_SIZE];
    for (&value, result) in values.iter().zip(result.iter_mut()) {
        *result = Felt::new(value);
    }
    result
}
