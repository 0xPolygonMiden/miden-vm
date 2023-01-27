use super::{
    chiplets::hasher,
    errors::{AdviceSetError, InputError},
    Felt, FieldElement, Word,
};
use winter_utils::{
    collections::{vec, Vec},
    ByteWriter, Serializable,
};

mod advice;
pub use advice::AdviceSet;

mod stack;
pub use stack::StackInputs;
