use crate::chiplets::ace::{Circuit, CircuitError, CircuitLayout, Instruction, MAX_ID, Node, Op};
use crate::{Felt, QuadFelt};
use std::prelude::rust_2015::Vec;
use vm_core::FieldElement;

impl Circuit {
    /// Create a new circuit given the expected number of nodes, the constants
    /// and instructions to be evaluated to obtain the result.
    ///
    /// Returns an error if
    /// - The circuit contains no instructions
    /// -
    pub fn new(
        num_inputs: usize,
        constants: Vec<Felt>,
        instructions: Vec<Instruction>,
    ) -> Result<Self, CircuitError> {
        let layout = CircuitLayout {
            num_inputs,
            num_constants: constants.len(),
            num_instructions: instructions.len(),
        };

        // Circuit must contain at least one instruction
        if instructions.len() == 0 {
            return Err(CircuitError::InvalidLayout);
        }

        // Ensure all instructions reference valid nodes and allow sequential evaluation
        for (instruction_idx, instruction) in instructions.iter().enumerate() {
            // Get the overall index of the node produced by this instruction
            let eval_node = Node::Eval(instruction_idx);

            // Check that each input node index is valid for this layout
            // and precedes the evaluation node
            let valid_node = |node: Node| layout.contains_node(&node) && node < eval_node;

            if !(valid_node(instruction.node_l) && valid_node(instruction.node_r)) {
                return Err(CircuitError::InvalidInstruction);
            }
        }

        Ok(Self { num_inputs, constants, instructions })
    }

    /// Given a list of inputs, compute the evaluation of the circuit.
    ///
    /// Returns an error if the number of inputs is larger than expected.
    /// When there are fewer inputs, they will be assumed to be 0 to match the padding
    /// behavior.
    pub fn eval(&self, inputs: &[QuadFelt]) -> Result<QuadFelt, CircuitError> {
        if inputs.len() != self.num_inputs {
            return Err(CircuitError::InvalidInputs);
        }
        let mut eval_nodes = Vec::with_capacity(self.instructions.len());

        let get_val = |node: Node, evals: &[QuadFelt]| match node {
            Node::Input(id) => inputs[id],
            Node::Const(id) => QuadFelt::new(self.constants[id], Felt::ZERO),
            Node::Eval(id) => evals[id],
        };

        for instruction in &self.instructions {
            let v_l = get_val(instruction.node_l, &eval_nodes);
            let v_r = get_val(instruction.node_r, &eval_nodes);
            let v_out = match instruction.op {
                Op::Sub => v_l - v_r,
                Op::Mul => v_l * v_r,
                Op::Add => v_l + v_r,
            };
            eval_nodes.push(v_out)
        }

        // Safe to unwrap since there are
        Ok(*eval_nodes.last().unwrap())
    }

    /// Layout of the circuit.
    pub fn layout(&self) -> CircuitLayout {
        CircuitLayout {
            num_inputs: self.num_inputs,
            num_constants: self.constants.len(),
            num_instructions: self.instructions.len(),
        }
    }
}
