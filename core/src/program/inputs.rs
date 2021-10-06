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
    pub fn new(public: &[u128], secret_a: &[u128], secret_b: &[u128]) -> ProgramInputs {
        assert!(
            public.len() <= MAX_PUBLIC_INPUTS,
            "expected no more than {} public inputs, but received {}",
            MAX_PUBLIC_INPUTS,
            public.len()
        );
        assert!(secret_a.len() >= secret_b.len(),
            "number of primary secret inputs cannot be smaller than the number of secondary secret inputs");

        ProgramInputs {
            public: public.iter().map(|&v| BaseElement::new(v)).collect(),
            secret: [
                secret_a.iter().map(|&v| BaseElement::new(v)).collect(),
                secret_b.iter().map(|&v| BaseElement::new(v)).collect(),
            ],
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
    pub fn from_public(public: &[u128]) -> ProgramInputs {
        ProgramInputs {
            public: public.iter().map(|&v| BaseElement::new(v)).collect(),
            secret: [vec![], vec![]],
        }
    }

    pub fn public_inputs(&self) -> &[BaseElement] {
        &self.public
    }

    pub fn secret_inputs(&self) -> &[Vec<BaseElement>; 2] {
        &self.secret
    }
}
