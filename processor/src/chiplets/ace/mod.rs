use crate::{Felt};
use std::prelude::rust_2024::Vec;
use vm_core::WORD_SIZE;

mod circuit;
mod encoder;
#[cfg(test)]
mod tests;
mod trace;

/// Number of bits used to represent the ID of a node in the evaluation graph.
/// Define as 30 bits to ensure two indices and the operation can be encoded in a single `Felt`
const ID_BITS: u64 = 30;
/// Maximum allowed ID, also equal to the mask extracting an ID from the lower 30 bits of a `uint`.
const MAX_ID: u32 = (1 << ID_BITS) - 1;

/// A `Circuit` is a DAG representing a multivariate polynomial over its inputs.
/// The nodes are laid out linearly, starting with the leaves and ending with the evaluation.
///
/// The constructor and invariants ensures that all instructions reference nodes whose index is
/// in-bounds and cannot reference nodes produced by subsequent instructions.
/// This ensures the circuit can be evaluated in a single pass over the instructions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Circuit {
    num_inputs: usize,
    constants: Vec<Felt>,
    instructions: Vec<Instruction>,
}

/// Arithmetic operation applied to two nodes in the evaluation graph.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Op {
    Sub,
    Mul,
    Add,
}

/// A `NodeID` is the index of a node in the evaluation graph, depending on its type.
/// - `Input` when it is a leaf, and its value is a variable in the circuit
/// - `Const` when it is a leaf, and its value is a constant
/// - `Eval` when it is a node with two incoming edges to which an `Op` is applied.
///
/// The graph is interpreted as a vector of `n_i + n_c + n_e` values,
/// with their kinds encoded as
/// `[Input(0), ..., Input(n_i - 1), Const(0), ..., Const(n_c - 1), Eval(0), ..., Eval(n_e - 1)]`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum NodeID {
    Input(usize),
    Const(usize),
    Eval(usize),
}

/// An `Instruction` indicates how the next `Eval` node should be computed, given existing `Node`s.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    node_l: NodeID,
    node_r: NodeID,
    op: Op,
}

/// An `EncodedCircuit` represents a `Circuit` as a list of field elements, containing both
/// constants and instructions.
struct EncodedCircuit {
    num_vars: usize,
    num_eval: usize,
    encoded_circuit: Vec<Felt>,
}

/// Layout of a circuit representing the number of different `Node`s of each type.
struct CircuitLayout {
    num_inputs: usize,
    num_constants: usize,
    num_instructions: usize,
}

impl CircuitLayout {
    /// Total number of variables (inputs and constants) in the circuit
    pub fn num_vars(&self) -> usize {
        self.num_inputs + self.num_constants
    }

    /// Total number of `Node`s required to represent the evaluation graph of the circuit.
    pub fn num_nodes(&self) -> usize {
        self.num_vars() + self.num_instructions
    }

    /// A `Circuit` is padded when it satisfied the following alignment properties
    /// - Number of inputs and constants are multiples of 2, ensuring the memory regions containing
    ///   them are each word aligned, as each word contains two variables.
    /// - The size of the circuit is double-word aligned to allow efficient un-hashing
    /// - The number of instructions are also word-aligned.
    pub fn is_padded(&self) -> bool {
        (2 * self.num_inputs) % WORD_SIZE == 0
            && (2 * self.num_constants) % WORD_SIZE == 0
            && (2 * self.num_constants + self.num_instructions) % (2 * WORD_SIZE) == 0
            && self.num_instructions % WORD_SIZE == 0
    }

    /// Returns the layout of the padded circuit (see `is_padded`).
    pub fn padded(&self) -> Self {
        // Inputs are padded to next multiple of 2 so they can be word-aligned, since each word
        // contains two inputs.
        // TODO(@adr1anh): does it makes sense to double-word align?
        let num_inputs = self.num_inputs.next_multiple_of(2);

        // The circuit size must be double-word aligned for more efficient hashing.
        // We pad instructions to 4 to minimize number of eval rows,
        // and add more constants to reach a padding of 8.
        let num_instructions = self.num_instructions.next_multiple_of(4);
        let padded_circuit_size = (2 * self.num_constants + num_instructions).next_multiple_of(8);
        let num_constants = (padded_circuit_size - num_instructions) / 2;
        Self {
            num_inputs,
            num_constants,
            num_instructions,
        }
    }

    /// Checks if a `NodeID` is in-bounds relative to this layout.
    pub fn contains_node(&self, node: &NodeID) -> bool {
        match *node {
            NodeID::Input(id) => id < self.num_inputs,
            NodeID::Const(id) => id < self.num_constants,
            NodeID::Eval(id) => id < self.num_instructions,
        }
    }

    /// Given a circuit `NodeID`, returns the index it would be at in the list of all nodes
    /// in the circuit evaluation graph.
    pub fn node_index(&self, node: &NodeID) -> Option<usize> {
        if !self.contains_node(node) {
            return None;
        }
        let id = match *node {
            NodeID::Input(id) => id,
            NodeID::Const(id) => id + self.num_inputs,
            NodeID::Eval(id) => id + self.num_vars(),
        };
        Some(id)
    }
}

#[derive(Debug)]
enum CircuitError {
    InvalidLayout,
    InvalidInstruction,
    InvalidInputs,
    InvalidAlignment,
}
