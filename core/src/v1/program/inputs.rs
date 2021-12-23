use super::{super::STACK_TOP_SIZE, BaseElement};

// PROGRAM INPUTS
// ================================================================================================

#[derive(Clone, Debug)]
pub struct ProgramInputs {
    stack_init: Vec<BaseElement>,
}

impl ProgramInputs {
    // TODO: add comments
    pub fn new(stack_init: &[u64]) -> ProgramInputs {
        assert!(
            stack_init.len() <= STACK_TOP_SIZE,
            "expected no more than {} initial stack values, but received {}",
            STACK_TOP_SIZE,
            stack_init.len()
        );

        // TODO: make sure there is no overflow
        ProgramInputs {
            stack_init: stack_init.iter().map(|&v| BaseElement::new(v)).collect(),
        }
    }

    /// Returns `ProgramInputs` with no initial stack values.
    pub fn none() -> ProgramInputs {
        ProgramInputs {
            stack_init: Vec::new(),
        }
    }

    /// Returns `ProgramInputs` initialized with the provided initial stack values.
    pub fn from_public(stack_init: &[u64]) -> ProgramInputs {
        Self::new(stack_init)
    }

    /// Returns initial stack values.
    pub fn stack_init(&self) -> &[BaseElement] {
        &self.stack_init
    }
}
