use crate::{
    OpCode, OpHint, BASE_CYCLE_LENGTH, HACC_NUM_ROUNDS, PROGRAM_DIGEST_SIZE, SPONGE_WIDTH,
};
pub use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};

pub mod blocks;
use blocks::{Group, ProgramBlock};

mod inputs;
pub use inputs::ProgramInputs;

mod hashing;
use hashing::{hash_acc, hash_op, hash_seq};

//#[cfg(test)]
//mod tests;

// TYPES AND INTERFACES
// ================================================================================================
#[derive(Clone)]
pub struct Program {
    root: Group,
    hash: [u8; 32],
}

// PROGRAM IMPLEMENTATION
// ================================================================================================
impl Program {
    /// Constructs a new program from the specified root block.
    pub fn new(root: Group) -> Program {
        // make sure the root block starts with BEGIN operation
        match &root.body()[0] {
            ProgramBlock::Span(block) => {
                let (op_code, _) = block.get_op(0);
                assert!(
                    op_code == OpCode::Begin,
                    "a program must start with BEGIN operation"
                );
            }
            _ => panic!("a program must start with a Span block"),
        }

        // compute program hash
        let (v0, v1) = root.get_hash();
        let hash = hash_acc(BaseElement::ZERO, v0, v1);
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(BaseElement::elements_as_bytes(&hash[..PROGRAM_DIGEST_SIZE]));

        Program {
            root,
            hash: hash_bytes,
        }
    }
    /// Returns the root block of the program.
    pub fn root(&self) -> &Group {
        &self.root
    }

    /// Returns hash of the program.
    pub fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut body_code = format!("{:?}", self.root);
        // get rid of extra `begin` token
        body_code.replace_range(..6, "");
        write!(f, "{}", body_code)?;

        Ok(())
    }
}
