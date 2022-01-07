use super::{BaseElement, FieldElement, StarkField, Word};
use std::collections::BTreeMap;

// RANDOM ACCESS MEMORY
// ================================================================================================

/// TODO: add comments
pub struct Memory {
    step: usize,
    state: BTreeMap<u64, Word>,
}

impl Memory {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add comments
    pub fn new() -> Self {
        Self {
            step: 0,
            state: BTreeMap::new(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns current size of the memory (in words).
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.state.len()
    }

    // TRACE ACCESSORS AND MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Returns a word (4 elements) located in memory at the specified address.
    ///
    /// If the specified address hasn't been previously written to, 4 ZERO elements are returned.
    /// This effectively implies that memory is initialized to ZERO.
    pub fn read(&mut self, addr: BaseElement) -> Word {
        let int_addr = addr.as_int();
        *self.state.entry(int_addr).or_insert([BaseElement::ZERO; 4])
    }

    /// Writes the provided words (4 elements) at the specified address.
    pub fn write(&mut self, addr: BaseElement, value: Word) {
        let int_addr = addr.as_int();
        self.state.insert(int_addr, value);
    }

    /// Increments the clock cycle.
    pub fn advance_clock(&mut self) {
        self.step += 1;
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    /// Instantiates a new processor for testing purposes.
    #[cfg(test)]
    pub fn get_value(&self, addr: u64) -> Option<Word> {
        self.state.get(&addr).copied()
    }
}
