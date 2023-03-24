mod crypto;

use miden_stdlib::StdLibrary;
pub use processor::{AdviceInputs, StackInputs};
use processor::{ExecutionError, ExecutionTrace, MemAdviceProvider};
pub use vm_core::utils::{group_slice_elements, IntoBytes};
pub use vm_core::{stack::STACK_TOP_SIZE, Felt, FieldElement, Program, StackOutputs};

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

    /// Builds a final stack from the provided stack-ordered array and asserts that executing the
    /// test will result in the expected final stack state.
    pub fn expect_stack(&self, final_stack: &[u64]) {
        let expected = convert_to_stack(final_stack);
        let result = self.get_last_stack_state();

        assert_eq!(expected, result);
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
