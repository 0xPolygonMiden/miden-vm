use crate::chiplets::ace::trace::{CircuitEvaluation, decode_instruction};
use crate::chiplets::ace::{Circuit, CircuitLayout, EncodedCircuit, Instruction, NodeID, Op};
use crate::{Felt, QuadFelt, Word};
use miden_air::FieldElement;
use std::prelude::rust_2015::Vec;

/// Evaluate a `Circuit` for a given set of `inputs`, comparing the result with the native
/// evaluation given by `eval_fn`.
fn check_eval(
    circuit: &Circuit,
    inputs: &[QuadFelt],
    eval_fn: impl Fn(&[QuadFelt]) -> QuadFelt,
) -> QuadFelt {
    let result = circuit.evaluate(inputs).expect("failed to evaluate");
    let expected = eval_fn(inputs);
    assert_eq!(result, expected);
    result
}

fn check_encoded_eval(circuit: &Circuit, inputs: &[QuadFelt]) {
    let encoded_circuit = EncodedCircuit::try_from_circuit(circuit).expect("cannot encode");
    let mem = generate_memory(&encoded_circuit, inputs);
    let evaluator =
        CircuitEvaluation::new(encoded_circuit.num_vars, encoded_circuit.num_eval, &mem)
            .expect("failed to evaluate");
    let eval = evaluator.evaluation();
    assert_eq!(eval, QuadFelt::ZERO);
}

/// Generate a mock memory region that represents the inputs and un-hashed circuit.
fn generate_memory(circuit: &EncodedCircuit, inputs: &[QuadFelt]) -> Vec<Word> {
    // Inputs are store two by two in the fest set of words, followed by the instructions.
    let mut mem = Vec::with_capacity(2 * inputs.len() + circuit.encoded_circuit.len());
    mem.extend(inputs.iter().flat_map(|input| input.to_base_elements()));
    if mem.len() % 4 != 0 {
        mem.extend([Felt::ZERO, Felt::ZERO])
    }
    assert_eq!(
        circuit.encoded_circuit.len() % 8,
        0,
        "encoded circuit must be double-word aligned"
    );

    mem.extend(circuit.encoded_circuit.iter());
    mem.chunks_exact(4).map(|word| word.try_into().unwrap()).collect()
}

#[test]
fn test_var_plus_one() {
    let constants = vec![Felt::ONE];
    let instructions = vec![Instruction {
        node_l: NodeID::Input(0),
        node_r: NodeID::Const(0),
        op: Op::Add,
    }];
    let circuit = Circuit::new(1, constants, instructions).expect("failed to create circuit");

    let inputs = [[QuadFelt::ZERO], [QuadFelt::ONE], [-QuadFelt::ONE]];

    for input in &inputs {
        check_eval(&circuit, input, |inputs| inputs[0] + QuadFelt::ONE);
    }

    check_encoded_eval(&circuit, &[-QuadFelt::ONE]);
}

#[test]
fn test_bool_check() {
    let constants = vec![-Felt::ONE];
    let neg_one = NodeID::Const(0);
    let x = NodeID::Input(0);
    let x_min_1 = NodeID::Eval(0);
    let x_times_x_min_one = NodeID::Eval(1);
    let result_expected = NodeID::Input(1);

    let instructions = vec![
        Instruction { node_l: x, node_r: neg_one, op: Op::Add },
        Instruction { node_l: x, node_r: x_min_1, op: Op::Mul },
        Instruction {
            node_l: x_times_x_min_one,
            node_r: result_expected,
            op: Op::Sub,
        },
    ];

    let circuit = Circuit::new(2, constants, instructions).unwrap();
    let inputs: Vec<_> = (0u8..20)
        .map(|x_int| {
            let x = QuadFelt::from(x_int);
            let result = x * (x - QuadFelt::ONE);
            [x, result]
        })
        .collect();

    for input in &inputs {
        check_eval(&circuit, input, |_| QuadFelt::ZERO);
        check_encoded_eval(&circuit, input);
    }



}

/// Check round-trip encoding and decoding of instructions.
#[test]
fn encode_decode_instruction() {
    let layout = CircuitLayout {
        num_inputs: 4,
        num_constants: 2,
        num_instructions: 4,
    };

    let instructions = [
        Instruction {
            node_l: NodeID::Const(0),
            node_r: NodeID::Input(0),
            op: Op::Sub,
        },
        Instruction {
            node_l: NodeID::Const(1),
            node_r: NodeID::Input(3),
            op: Op::Add,
        },
        Instruction {
            node_l: NodeID::Eval(0),
            node_r: NodeID::Eval(3),
            op: Op::Add,
        },
        Instruction {
            node_l: NodeID::Eval(2),
            node_r: NodeID::Eval(2),
            op: Op::Mul,
        },
    ];

    for instruction in instructions {
        let encoded = instruction.encode(&layout).unwrap();
        let (id_l, id_r, op) = decode_instruction(encoded).unwrap();
        let id_l_expected = layout.encoded_node_id(&instruction.node_l).unwrap();
        let id_r_expected = layout.encoded_node_id(&instruction.node_r).unwrap();
        assert_eq!(id_l, id_l_expected);
        assert_eq!(id_r, id_r_expected);
        assert_eq!(op, instruction.op);
    }
}
