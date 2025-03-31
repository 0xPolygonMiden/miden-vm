use crate::Word;
use crate::chiplets::ace::{
    Circuit, CircuitError, CircuitLayout, EncodedCircuit, ID_BITS, Instruction, MAX_ID, Node, Op,
};
use crate::crypto::ElementHasher;
use crate::math::FieldElement;
use crate::{Felt, QuadFelt};
use std::prelude::rust_2024::Vec;

impl EncodedCircuit {
    /// Try to create an `EncodedCircuit` matching the layout
    pub fn try_from_circuit(circuit: &Circuit) -> Result<Self, CircuitError> {
        let layout = circuit.layout().padded();

        if layout.num_nodes() > MAX_ID as usize {
            return Err(CircuitError::InvalidLayout);
        }

        // Encoded circuit contains constants followed by instructions.
        // Constants are mapped to `QuadFelt`s represented by two `Felt`s.
        // Instructions are mapped to a single `Felt`.
        let circuit_size = 2 * layout.num_constants + layout.num_instructions;

        let mut encoded_circuit = Vec::with_capacity(circuit_size);

        // Add constants encoded as `QuadFelt`s
        encoded_circuit.extend(circuit.constants.iter().flat_map(|c| [*c, Felt::ZERO]));
        // Pad with zero constants.
        let encoded_constants_size = 2 * layout.num_constants;
        encoded_circuit.resize(encoded_constants_size, Felt::ZERO);

        // Encode the instructions to single `Felt`s, reversing the ids
        let encoded_instructions_iter = circuit
            .instructions
            .iter()
            .map(|instruction| instruction.encode(&layout).unwrap());
        // Add the encoded instructions to the circuit
        encoded_circuit.extend(encoded_instructions_iter);

        // Add instructions squaring the final value. Since we care about the output being 0,
        // this has no effect. Moreover, it avoids having to know the index of the zero constant.
        let mut last_eval_node_index = circuit.instructions.len() - 1;
        while encoded_circuit.len() < circuit_size {
            let last_eval_node = Node::Eval(last_eval_node_index);
            let square_last_instruction = Instruction {
                node_l: last_eval_node,
                node_r: last_eval_node,
                op: Op::Mul,
            };
            let encoded_instruction = square_last_instruction.encode(&layout).expect("");
            encoded_circuit.push(encoded_instruction);
            last_eval_node_index += 1;
        }
        debug_assert_eq!(last_eval_node_index, layout.num_instructions - 1);

        Ok(Self {
            num_vars: layout.num_vars(),
            num_eval: layout.num_instructions,
            encoded_circuit,
        })
    }

    /// Compute the number of
    fn circuit_hash<H: ElementHasher<BaseField = Felt>>(&self) -> H::Digest {
        H::hash_elements(&self.encoded_circuit)
    }
}

impl CircuitLayout {
    /// Same as `node_to_index`, but reverses the index relative to `num_nodes`.
    ///
    /// For example, the first input node has `id = layout.num_nodes() - 1`
    /// and the last instruction produces a node with `id = 0`.
    pub fn encoded_node_id(&self, node: &Node) -> Option<u32> {
        let id = self.node_index(node)?;
        Some((self.num_nodes() - 1 - id) as u32)
    }
}

impl Instruction {
    /// Encode an instruction as a `Felt`, packed as
    /// `[ id_l (30 bits) || id_r (30 bits) || op (2 bits) ]`,
    /// where `id_{l, r}` are is the index of the node in the graph, reversed
    /// with regard to the total number of nodes.
    fn encode(&self, layout: &CircuitLayout) -> Option<Felt> {
        if layout.num_nodes() > MAX_ID as usize {
            return None;
        }

        let id_l = layout.encoded_node_id(&self.node_l)?;
        let id_r = layout.encoded_node_id(&self.node_r)?;

        let op = match self.op {
            Op::Sub => 0,
            Op::Mul => 1,
            Op::Add => 2,
        };

        let encoded = id_l as u64 + ((id_r as u64) << ID_BITS) + (op << (2 * ID_BITS));
        Some(Felt::new(encoded))
    }
}
