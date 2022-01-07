use super::{super::STACK_TOP_SIZE, BaseElement};

// PROGRAM INPUTS
// ================================================================================================

/// TODO: add docs
#[derive(Clone, Debug)]
pub struct ProgramInputs {
    stack_init: Vec<BaseElement>,
    advice_tape: Vec<BaseElement>,
}

impl ProgramInputs {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// TODO: add comments
    pub fn new(stack_init: &[u64], advice_tape: &[u64]) -> ProgramInputs {
        assert!(
            stack_init.len() <= STACK_TOP_SIZE,
            "expected no more than {} initial stack values, but received {}",
            STACK_TOP_SIZE,
            stack_init.len()
        );

        // TODO: make sure there is no overflow
        ProgramInputs {
            stack_init: stack_init.iter().map(|&v| BaseElement::new(v)).collect(),
            advice_tape: advice_tape.iter().map(|&v| BaseElement::new(v)).collect(),
        }
    }

    /// Returns `ProgramInputs` with no initial stack values.
    pub fn none() -> ProgramInputs {
        ProgramInputs {
            stack_init: Vec::new(),
            advice_tape: Vec::new(),
        }
    }

    /// Returns `ProgramInputs` initialized with the provided initial stack values.
    pub fn from_public(stack_init: &[u64]) -> ProgramInputs {
        Self::new(stack_init, &[])
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns initial stack values.
    pub fn stack_init(&self) -> &[BaseElement] {
        &self.stack_init
    }

    /// Returns a reference to the advice tape.
    pub fn advice_tape(&self) -> &[BaseElement] {
        &self.advice_tape
    }
}
