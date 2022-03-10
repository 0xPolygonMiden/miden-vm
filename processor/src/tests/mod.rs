use super::{
    execute, ExecutionError, ExecutionTrace, Felt, FieldElement, Process, ProgramInputs, Script,
    Word, MIN_STACK_DEPTH,
};
use proptest::prelude::*;

mod aux_table_trace;
mod crypto_ops;
mod field_ops;
mod flow_control;
mod io_ops;
mod stdlib;
mod u32_ops;

// TESTS
// ================================================================================================

#[test]
fn simple_program() {
    let test = Test::new("begin push.1 push.2 add end");
    test.expect_stack(&[3]);
}

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
/// * - Execution test: check that running a script compiled from the given source has the specified
/// results for the given (optional) inputs.
/// * - Proptest: run an execution test inside a proptest.
///
/// Types of failure tests:
/// * - Assembly error test: check that attempting to compile the given source causes an
/// AssemblyError which contains the specified substring.
/// * - Execution error test: check that running a script compiled from the given source causes an
/// ExecutionError which contains the specified substring.
pub struct Test {
    source: String,
    inputs: ProgramInputs,
}

impl Test {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    /// Creates the simplest possible new test, with only a source string and no inputs.
    fn new(source: &str) -> Self {
        Test {
            source: String::from(source),
            inputs: ProgramInputs::none(),
        }
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    /// Asserts that running the test for the expected TestError variant will result in an error
    /// that contains the TestError's error substring in its error message.
    fn expect_error(&self, error: TestError) {
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
    fn expect_stack(&self, final_stack: &[u64]) {
        let expected = convert_to_stack(final_stack);
        let result = self.get_last_stack_state();

        assert_eq!(expected, result);
    }

    /// Executes the test and validates that the process memory has the elements of `expected_mem`
    /// at address `mem_addr` and that the end of the stack execution trace matches the
    /// `final_stack`.
    fn expect_stack_and_memory(&self, final_stack: &[u64], mem_addr: u64, expected_mem: &[u64]) {
        let mut process = Process::new(self.inputs.clone());

        // execute the test
        process.execute_code_block(self.compile().root()).unwrap();

        // validate the memory state
        let mem_state = process.memory.get_value(mem_addr).unwrap();
        let expected_mem: Vec<Felt> = expected_mem.iter().map(|&v| Felt::new(v)).collect();
        assert_eq!(expected_mem, mem_state);

        // validate the stack state
        let stack_state = ExecutionTrace::new(process).last_stack_state();
        let expected_stack = convert_to_stack(final_stack);
        assert_eq!(expected_stack, stack_state);
    }

    /// Asserts that executing the test inside a proptest results in the expected final stack state.
    /// The proptest will return a test failure instead of panicking if the assertion condition
    /// fails.
    fn prop_expect_stack(
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

    /// Compiles a test's source and returns the resulting Script.
    fn compile(&self) -> Script {
        let assembler = assembly::Assembler::new();
        assembler
            .compile_script(&self.source)
            .expect("Failed to compile test source.")
    }

    /// Compiles the test's source to a Script and executes it with the tests inputs. Returns a
    /// resulting execution trace or error.
    fn execute(&self) -> Result<ExecutionTrace, ExecutionError> {
        let script = self.compile();
        execute(&script, &self.inputs)
    }

    /// Returns the last state of the stack after executing a test.
    fn get_last_stack_state(&self) -> [Felt; MIN_STACK_DEPTH] {
        let trace = self.execute().unwrap();

        trace.last_stack_state()
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Takes an array of u64 values and builds a stack, perserving their order and converting them to
/// field elements.
fn convert_to_stack(values: &[u64]) -> [Felt; MIN_STACK_DEPTH] {
    let mut result = [Felt::ZERO; MIN_STACK_DEPTH];
    for (&value, result) in values.iter().zip(result.iter_mut()) {
        *result = Felt::new(value);
    }
    result
}

// This is a proptest strategy for generating a random word with 4 values of type T.
fn prop_randw<T: proptest::arbitrary::Arbitrary>() -> impl Strategy<Value = Vec<T>> {
    prop::collection::vec(any::<T>(), 4)
}

// MACROS
// ================================================================================================

/// Returns a Test struct from the provided source string and any specified stack and advice inputs.
///
/// Parameters are expected in the following order:
/// `source`, `stack_inputs` (optional), `advice_tape` (optional), `advice_sets` (optional)
///
/// * `source`: a well-formed source string.
/// * `stack_inputs` (optional): the initial inputs which must be at the top of the stack before
/// executing the `source`. Stack inputs can be provided independently without any advice inputs.
/// * `advice_tape` (optional): the initial advice tape values. When provided, `stack_inputs` and
/// `advice_sets` are also expected.
/// * `advice_sets` (optional): the initial advice set values. When provided, `stack_inputs` and
/// `advice_tape` are also expected.
#[macro_export]
macro_rules! build_test {
    ($source:expr) => {{
        $crate::tests::Test::new($source)
    }};
    ($source:expr, $stack_inputs:expr) => {{
        let inputs = $crate::tests::ProgramInputs::new($stack_inputs, &[], vec![]).unwrap();

        $crate::tests::Test {
            source: String::from($source),
            inputs,
        }
    }};
    ($source:expr, $stack_inputs:expr, $advice_tape:expr, $advice_sets:expr) => {{
        let inputs =
            $crate::tests::ProgramInputs::new($stack_inputs, $advice_tape, $advice_sets).unwrap();

        $crate::tests::Test {
            source: String::from($source),
            inputs,
        }
    }};
}

/// Returns a Test struct from a string of one or more operations and any specified stack and advice
/// inputs.
///
/// Parameters are expected in the following order:
/// `source`, `stack_inputs` (optional), `advice_tape` (optional), `advice_sets` (optional)
///
/// * `source`: a well-formed source string.
/// * `stack_inputs` (optional): the initial inputs which must be at the top of the stack before
/// executing the `source`. Stack inputs can be provided independently without any advice inputs.
/// * `advice_tape` (optional): the initial advice tape values. When provided, `stack_inputs` and
/// `advice_sets` are also expected.
/// * `advice_sets` (optional): the initial advice set values. When provided, `stack_inputs` and
/// `advice_tape` are also expected.
#[macro_export]
macro_rules! build_op_test {
    ($op_str:expr) => {{
        let source = format!("begin {} end", $op_str);
        $crate::build_test!(&source)
    }};
    ($op_str:expr, $($tail:tt)+) => {{
        let source = format!("begin {} end", $op_str);
        $crate::build_test!(&source, $($tail)+)
    }};
}
