use assembly::Library;
pub use processor::{
    AdviceInputs, ExecutionError, ExecutionTrace, MemAdviceProvider, Process, ProgramInfo,
    StackInputs, VmStateIterator,
};
use proptest::prelude::*;
pub use prover::{ProofOptions, StarkProof};
use std::panic::UnwindSafe;
pub use vm_core::{
    crypto::merkle::MerkleStore, stack::STACK_TOP_SIZE, Felt, FieldElement, Program, StackOutputs,
};

// CONSTANTS
// ================================================================================================
pub const U32_BOUND: u64 = u32::MAX as u64 + 1;

// TEST HANDLER
// ================================================================================================

/// This is used to specify the expected error type when using Test to test errors.
/// `Test::expect_error` will try to either compile or execute the test data, according to the
/// provided TestError variant. Then it will validate that the resulting error contains the
/// TestError variant's string slice.
pub enum TestError<'a> {
    AssemblyError(&'a str),
    ExecutionError(&'a str),
}

/// This is a container for the data required to run tests, which allows for running several
/// different types of tests.
///
/// Types of valid result tests:
/// - Execution test: check that running a program compiled from the given source has the
///   specified results for the given (optional) inputs.
/// - Proptest: run an execution test inside a proptest.
///
/// Types of failure tests:
/// - Assembly error test: check that attempting to compile the given source causes an
/// AssemblyError which contains the specified substring.
/// - Execution error test: check that running a program compiled from the given source causes
///   an ExecutionError which contains the specified substring.
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

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    /// Asserts that running the test for the expected TestError variant will result in an error
    /// that contains the TestError's error substring in its error message.
    pub fn expect_error<L>(&self, error: TestError, libraries: Vec<L>)
    where
        L: Library + UnwindSafe,
    {
        match error {
            TestError::AssemblyError(substr) => {
                assert_eq!(
                    std::panic::catch_unwind(|| self.compile(libraries))
                        .err()
                        .and_then(|a| { a.downcast_ref::<String>().map(|s| s.contains(substr)) }),
                    Some(true)
                );
            }
            TestError::ExecutionError(substr) => {
                assert_eq!(
                    std::panic::catch_unwind(|| self.execute(libraries).unwrap())
                        .err()
                        .and_then(|a| { a.downcast_ref::<String>().map(|s| s.contains(substr)) }),
                    Some(true)
                );
            }
        }
    }

    /// Builds a final stack from the provided stack-ordered array and asserts that executing the
    /// test will result in the expected final stack state.
    pub fn expect_stack<L>(&self, final_stack: &[u64], libraries: Vec<L>)
    where
        L: Library,
    {
        let expected = convert_to_stack(final_stack);
        let result = self.get_last_stack_state(libraries);

        assert_eq!(expected, result);
    }

    /// Executes the test and validates that the process memory has the elements of `expected_mem`
    /// at address `mem_addr` and that the end of the stack execution trace matches the
    /// `final_stack`.
    pub fn expect_stack_and_memory<L>(
        &self,
        final_stack: &[u64],
        mem_addr: u64,
        expected_mem: &[u64],
        libraries: Vec<L>,
    ) where
        L: Library,
    {
        // compile the program
        let program = self.compile(libraries);
        let advice_provider = MemAdviceProvider::from(self.advice_inputs.clone());

        // execute the test
        let mut process =
            Process::new(program.kernel().clone(), self.stack_inputs.clone(), advice_provider);
        let stack_outputs = process.execute(&program).unwrap();

        // validate the memory state
        let mem_state = process.get_memory_value(0, mem_addr).unwrap();
        let expected_mem: Vec<Felt> = expected_mem.iter().map(|&v| Felt::new(v)).collect();
        assert_eq!(expected_mem, mem_state);

        // validate the stack state
        assert_eq!(convert_to_stack(final_stack), stack_outputs.stack_top());
    }

    /// Asserts that executing the test inside a proptest results in the expected final stack state.
    /// The proptest will return a test failure instead of panicking if the assertion condition
    /// fails.
    pub fn prop_expect_stack<L>(
        &self,
        final_stack: &[u64],
        libraries: Vec<L>,
    ) -> Result<(), proptest::test_runner::TestCaseError>
    where
        L: Library,
    {
        let expected = convert_to_stack(final_stack);
        let result = self.get_last_stack_state(libraries);

        prop_assert_eq!(expected, result);

        Ok(())
    }

    // UTILITY METHODS
    // --------------------------------------------------------------------------------------------

    /// Compiles a test's source and returns the resulting Program.
    pub fn compile<L>(&self, libraries: Vec<L>) -> Program
    where
        L: Library,
    {
        let assembler = assembly::Assembler::default()
            .with_debug_mode(self.in_debug_mode)
            .with_libraries(libraries.into_iter())
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
    pub fn execute<L>(&self, libraries: Vec<L>) -> Result<ExecutionTrace, ExecutionError>
    where
        L: Library,
    {
        let program = self.compile(libraries);
        let advice_provider = MemAdviceProvider::from(self.advice_inputs.clone());
        processor::execute(&program, self.stack_inputs.clone(), advice_provider)
    }

    /// Compiles the test's code into a program, then generates and verifies a proof of execution
    /// using the given public inputs and the specified number of stack outputs. When `test_fail`
    /// is true, this function will force a failure by modifying the first output.
    pub fn prove_and_verify<L>(&self, pub_inputs: Vec<u64>, test_fail: bool, libraries: Vec<L>)
    where
        L: Library,
    {
        let stack_inputs = StackInputs::try_from_values(pub_inputs).unwrap();
        let program = self.compile(libraries);
        let advice_provider = MemAdviceProvider::from(self.advice_inputs.clone());
        let (mut stack_outputs, proof) =
            prover::prove(&program, stack_inputs.clone(), advice_provider, ProofOptions::default())
                .unwrap();

        let program_info = ProgramInfo::from(program);
        if test_fail {
            stack_outputs.stack_mut()[0] += 1;
            assert!(verifier::verify(program_info, stack_inputs, stack_outputs, proof).is_err());
        } else {
            let result = verifier::verify(program_info, stack_inputs, stack_outputs, proof);
            assert!(result.is_ok(), "error: {result:?}");
        }
    }

    /// Compiles the test's source to a Program and executes it with the tests inputs. Returns a
    /// VmStateIterator that allows us to iterate through each clock cycle and inspect the process
    /// state.
    pub fn execute_iter<L>(&self, libraries: Vec<L>) -> VmStateIterator
    where
        L: Library,
    {
        let program = self.compile(libraries);
        let advice_provider = MemAdviceProvider::from(self.advice_inputs.clone());
        processor::execute_iter(&program, self.stack_inputs.clone(), advice_provider)
    }

    /// Returns the last state of the stack after executing a test.
    pub fn get_last_stack_state<L>(&self, libraries: Vec<L>) -> [Felt; STACK_TOP_SIZE]
    where
        L: Library,
    {
        let trace = self.execute(libraries).unwrap();

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

// This is a proptest strategy for generating a random word with 4 values of type T.
pub fn prop_randw<T: proptest::arbitrary::Arbitrary>() -> impl Strategy<Value = Vec<T>> {
    prop::collection::vec(any::<T>(), 4)
}
