use std::prelude::rust_2015::Vec;
use crate::chiplets::ace::{Circuit, EncodedCircuit, Instruction, Node, Op};
use crate::{Felt, QuadFelt, Word};
use miden_air::FieldElement;
use crate::chiplets::ace::trace::CircuitEvaluation;
// impl Circuit {
//     fn new_random() -> Self {
//
//     }
// }

fn check_eval(circuit: &Circuit, inputs: &[QuadFelt], eval: impl Fn(&[QuadFelt]) -> QuadFelt) -> QuadFelt {
    let result = circuit.eval(inputs).expect("failed to evaluate");
    let expected = eval(inputs);
    assert_eq!(result, expected);
    result
}

fn check_encoded_eval(circuit: &Circuit, inputs: &[QuadFelt]) {
    let encoded_circuit = EncodedCircuit::try_from_circuit(circuit).expect("cannot encode");
    let mem = generate_memory(&encoded_circuit, inputs);
    let evaluator =CircuitEvaluation::new(encoded_circuit.num_vars, encoded_circuit.num_eval, &mem).expect("failed to evaluate");
    let eval= evaluator.eval();
    assert_eq!(eval, QuadFelt::ZERO);
}

fn generate_memory(circuit: &EncodedCircuit, inputs: &[QuadFelt]) -> Vec<Word> {
    /// Generate a mock memory region that represents the inputs and un-hashed circuit.
    let mut mem = Vec::with_capacity(2 * inputs.len() + circuit.encoded_circuit.len());
    mem.extend(inputs.iter().flat_map(|input| input.to_base_elements()));
    if mem.len() % 4 != 0 {
        mem.extend([Felt::ZERO, Felt::ZERO])
    }

    mem.extend(circuit.encoded_circuit.iter());
    mem.chunks_exact(4).map(|word| word.try_into().unwrap()).collect()
}


#[test]
fn test_var_plus_one() {
    let constants = vec![Felt::ONE];
    let instructions = vec![Instruction {
        node_l: Node::Input(0),
        node_r: Node::Const(0),
        op: Op::Add,
    }];
    let circuit = Circuit::new(1, constants, instructions).expect("failed to create circuit");

    let inputs = [QuadFelt::ZERO, QuadFelt::ONE, -QuadFelt::ONE];

    for input in inputs {
        check_eval(&circuit, &[input], |inputs| inputs[0] + QuadFelt::ONE);
    }

    check_encoded_eval(&circuit, &[-QuadFelt::ONE]);
}


