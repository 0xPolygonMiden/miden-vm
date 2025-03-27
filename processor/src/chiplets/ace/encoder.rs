use crate::math::FieldElement;
use crate::{Felt, QuadFelt};
use std::prelude::rust_2024::Vec;
use crate::chiplets::ace::{Instruction, Node, Op, ID_BITS, MAX_ID};

struct Circuit {
    num_vars: usize,
    num_nodes: usize,
    encoded_circuit: Vec<Felt>,
}

enum CircuitError {
    TooManyNodes,
    NonLinearInstruction(usize),
    InvalidNode(usize),
}

impl Circuit {
    ///
    /// To be padded with `Mult(0)` to the next multiple of 2 to ensure the memory region
    /// containing the constants can be word-aligned.
    /// Padded with `(E::ZERO, Mult(0))` to the next multiple of 2 to ensure the first
    /// instruction is word-aligned
    /// Padded to next multiple of 4 or 8 to ensure the circuit section (constants & instructions)
    /// is double-word aligned to facilitate un-hashing.
    fn new(
        num_inputs: usize,
        constants: &[Felt],
        instructions: &[Instruction],
    ) -> Result<Circuit, CircuitError> {
        // Compute the real number of nodes in the circuit
        let num_nodes = {
            let num_vars = num_inputs + constants.len();
            num_vars + instructions.len()
        };
        if num_nodes > MAX_ID as usize {
            return Err(CircuitError::TooManyNodes);
        }

        // Inputs are padded to next multiple of 2 so they can be word-aligned, since each word
        // contains two inputs.
        // TODO(@adr1anh): does it makes sense to double-word align?
        let num_inputs_padded = num_inputs.next_multiple_of(2);

        // There are 4 instructions per word, we pad it to next multiple of 4.
        let num_instructions_padded = instructions.len().next_multiple_of(4);

        // The circuit size must be double-word aligned for more efficient hashing.
        let circuit_size_padded =
            (2 * constants.len() + num_instructions_padded).next_multiple_of(8);

        // Constants are represented as `QuadFelt`s so there are two per word.
        // It is more efficient to pad the constants to ensure double-word alignment, since
        // it would only require one additional row to add 1 word.
        let num_constants_padded = (circuit_size_padded - num_instructions_padded) / 2;

        let mut encoded_circuit = Vec::with_capacity(circuit_size_padded);

        // Add the constants.
        encoded_circuit.extend(constants.iter().flat_map(|c| [Felt::ZERO, *c]));
        // Pad with zeros.
        encoded_circuit.resize(num_constants_padded, Felt::ZERO);

        let num_vars_padded = num_inputs_padded + num_constants_padded;
        let num_nodes_padded = num_vars_padded + num_instructions_padded;

        let is_valid_node = |node: Node| match node {
            Node::Input(id) => id < num_inputs,
            Node::Const(id) => id < constants.len(),
            Node::Eval(id) => id < instructions.len(),
        };

        let normalize_node = |node: Node| {
            // map err invalid id
            let id = match node {
                Node::Input(id) => id,
                Node::Const(id) => num_inputs_padded + id,
                Node::Eval(id) => num_vars_padded + id,
            };
            let reversed_id = num_nodes_padded - 1 - id;
            reversed_id as u32
        };

        let encode_instruction = |i: Instruction| {
            let Instruction { node_l, node_r, op } = i;
            let id_l = normalize_node(node_l);
            let id_r = normalize_node(node_r);
            let encoded = encode_instruction(id_l, id_r, op);
            encoded.into()
        };

        for (i, instruction) in instructions.iter().enumerate() {
            let current_node = Node::Eval(i);

            if !(is_valid_node(instruction.node_l) && is_valid_node(instruction.node_r)) {
                return Err(CircuitError::InvalidNode(i));
            }

            // Ensure we are only referencing previous nodes as this allows the circuit
            // to be evaluated in a single pass
            if instruction.node_l >= current_node || instruction.node_r >= current_node {
                return Err(CircuitError::NonLinearInstruction(i));
            };

            // Encode the instruction as a `Felt`, applying the necessary transformation on the ids
            encoded_circuit.push(encode_instruction(*instruction));
        }

        // Pad instructions with squaring of the last node. Since we expect the evaluation to
        // be zero, this is equivalent and does not require knowing which provided constant is zero.
        for i in instructions.len()..num_instructions_padded {
            let prev_node = Node::Eval(i - 1);
            let padding_instruction = Instruction {
                node_l: prev_node,
                node_r: prev_node,
                op: Op::Mul,
            };
            encoded_circuit.push(encode_instruction(padding_instruction));
        }
        assert_eq!(encoded_circuit.len(), circuit_size_padded);

        Ok(Self { num_vars, num_nodes, encoded_circuit })
    }
}

fn encode_instruction(id_l: u32, id_r: u32, op: Op) -> Felt {
    let op = match op {
        Op::Sub => 0,
        Op::Mul => 1,
        Op::Add => 2,
    };
    let encoded = id_l as u64 + (id_r as u64) << ID_BITS + op << (2 * ID_BITS);
    encoded.into()
}