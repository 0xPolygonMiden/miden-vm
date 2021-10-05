use super::{BaseElement, MAX_PUBLIC_INPUTS};

// PROGRAM INPUTS
// ================================================================================================

#[derive(Clone, Debug)]
pub struct ProgramInputs {
    public: Vec<BaseElement>,
    secret: [Vec<BaseElement>; 2],
}

impl ProgramInputs {
    /// Returns `ProgramInputs` initialized with the provided public and secret inputs.
    pub fn new(
        public: &[BaseElement],
        secret_a: &[BaseElement],
        secret_b: &[BaseElement],
    ) -> ProgramInputs {
        assert!(
            public.len() <= MAX_PUBLIC_INPUTS,
            "expected no more than {} public inputs, but received {}",
            MAX_PUBLIC_INPUTS,
            public.len()
        );
        assert!(secret_a.len() >= secret_b.len(),
            "number of primary secret inputs cannot be smaller than the number of secondary secret inputs");

        ProgramInputs {
            public: public.to_vec(),
            secret: [secret_a.to_vec(), secret_b.to_vec()],
        }
    }

    /// Returns `ProgramInputs` with public and secret input tapes set to empty vectors.
    pub fn none() -> ProgramInputs {
        ProgramInputs {
            public: Vec::new(),
            secret: [Vec::new(), Vec::new()],
        }
    }

    /// Returns `ProgramInputs` initialized with the provided public inputs and secret
    /// input tapes set to empty vectors.
    pub fn from_public(public: &[BaseElement]) -> ProgramInputs {
        ProgramInputs {
            public: public.to_vec(),
            secret: [vec![], vec![]],
        }
    }

    pub fn get_public_inputs(&self) -> &[BaseElement] {
        &self.public
    }

    pub fn get_secret_inputs(&self) -> &[Vec<BaseElement>; 2] {
        &self.secret
    }
}
