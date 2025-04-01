use crate::chiplets::ace::trace::{
    CircuitEvaluationContext, EVAL_OP_IDX, ID_0_IDX, ID_1_IDX, ID_2_IDX, M_0_IDX, M_1_IDX,
    NUM_COLS, SELECTOR_BLOCK_IDX, SELECTOR_START_IDX, V_0_0_IDX, V_0_1_IDX, V_1_0_IDX, V_1_1_IDX,
    V_2_0_IDX, V_2_1_IDX, decode_instruction,
};
use crate::chiplets::ace::{Circuit, CircuitLayout, EncodedCircuit, Instruction, NodeID, Op};
use crate::{ContextId, Felt, QuadFelt, Word};
use miden_air::{FieldElement, RowIndex};
use std::collections::HashMap;
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

/// Performs encoding of circuit and evaluate it by the ACE chiplet.
fn check_encoded_eval(circuit: &Circuit, inputs: &[QuadFelt]) {
    let encoded_circuit = EncodedCircuit::try_from_circuit(circuit).expect("cannot encode");
    let num_read_rows = encoded_circuit.num_vars as u32 / 2;
    let num_eval_rows = encoded_circuit.num_eval as u32;
    let circuit_mem = generate_memory(&encoded_circuit, inputs);
    let ctx = ContextId::default();
    let ptr = Felt::ZERO;
    let clk = RowIndex::from(0);

    let mut evaluator = CircuitEvaluationContext::new(ctx, ptr, clk, num_read_rows, num_eval_rows);

    let mut mem_iter = circuit_mem.iter();
    for word in mem_iter.by_ref().take(num_read_rows as usize) {
        evaluator.do_read(*word).expect("TODO");
    }
    for instruction in mem_iter.by_ref().flatten() {
        evaluator.do_eval(*instruction).expect("TODO");
    }

    let eval = evaluator.output_value().unwrap();
    assert_eq!(eval, QuadFelt::ZERO);

    verify_trace(evaluator, num_read_rows as usize, num_eval_rows as usize);
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

fn verify_trace(context: CircuitEvaluationContext, num_read_rows: usize, num_eval_rows: usize) {
    let num_rows = num_read_rows + num_eval_rows;
    let mut columns: Vec<_> = (0..NUM_COLS).map(|_| vec![Felt::from(42u8); num_rows]).collect();
    let mut columns_ref: Vec<_> = columns.iter_mut().map(|col| col.as_mut_slice()).collect();

    context.fill(&mut columns_ref);

    let num_wires = num_read_rows * 2 + num_eval_rows;

    // All wire indices in order
    let mut wire_idx_iter = (0..num_wires).map(|index| num_wires as u64 - 1 - index as u64);

    // Maps id -> (value, multiplicity)
    let mut bus = HashMap::new();
    for row_idx in 0..num_read_rows {
        // ensure `f_start` is true only in first row
        let is_first = columns[SELECTOR_START_IDX][row_idx];
        if row_idx == 0 {
            assert_eq!(is_first, Felt::ONE);
        } else {
            assert_eq!(is_first, Felt::ZERO);
        }

        // ensure block flag is read
        assert_eq!(columns[SELECTOR_BLOCK_IDX][row_idx], Felt::ZERO);

        // Get value 0
        let v_00 = columns[V_0_0_IDX][row_idx];
        let v_01 = columns[V_0_1_IDX][row_idx];
        let v_0 = QuadFelt::new(v_00, v_01);

        // Insert wire 0
        let id_0 = columns[ID_0_IDX][row_idx].as_int();
        let m_0 = columns[M_0_IDX][row_idx];
        assert_eq!(id_0, wire_idx_iter.next().unwrap());
        assert!(bus.insert(id_0, (v_0, m_0)).is_none());

        // Get value 1
        let v_10 = columns[V_1_0_IDX][row_idx];
        let v_11 = columns[V_1_1_IDX][row_idx];
        let v_1 = QuadFelt::new(v_10, v_11);

        // Insert wire 1
        let id_1 = columns[ID_1_IDX][row_idx].as_int();
        let m_1 = columns[M_1_IDX][row_idx];
        assert_eq!(id_1, wire_idx_iter.next().unwrap());
        assert!(bus.insert(id_1, (v_1, m_1)).is_none());
    }

    for row_idx in num_read_rows..(num_read_rows + num_eval_rows) {
        let is_first = columns[SELECTOR_START_IDX][row_idx];
        assert_eq!(is_first, Felt::ZERO);

        // ensure block flag is eval
        assert_eq!(columns[SELECTOR_BLOCK_IDX][row_idx], Felt::ONE);

        // Get value 0
        let v_00 = columns[V_0_0_IDX][row_idx];
        let v_01 = columns[V_0_1_IDX][row_idx];
        let v_0 = QuadFelt::new(v_00, v_01);

        // Insert wire 0
        let id_0 = columns[ID_0_IDX][row_idx].as_int();
        let m_0 = columns[M_0_IDX][row_idx];
        assert_eq!(id_0, wire_idx_iter.next().unwrap());
        assert!(bus.insert(id_0, (v_0, m_0)).is_none());

        // Get wire 1
        let id_1 = columns[ID_1_IDX][row_idx].as_int();
        let (v_l, m_1) = bus.get_mut(&id_1).unwrap();
        *m_1 -= Felt::ONE;

        // Get value 1
        let v_10 = columns[V_1_0_IDX][row_idx];
        let v_11 = columns[V_1_1_IDX][row_idx];
        let v_1 = QuadFelt::new(v_10, v_11);
        assert_eq!(*v_l, v_1);

        // Get wire 2
        let id_2 = columns[ID_2_IDX][row_idx].as_int();
        let (v_r, m_2) = bus.get_mut(&id_2).unwrap();
        *m_2 -= Felt::ONE;

        // Get value 2
        let v_20 = columns[V_2_0_IDX][row_idx];
        let v_21 = columns[V_2_1_IDX][row_idx];
        let v_2 = QuadFelt::new(v_20, v_21);
        assert_eq!(*v_r, v_2);

        // Check operation
        let op = columns[EVAL_OP_IDX][row_idx];
        let v_out = if op == -Felt::ONE {
            v_1 - v_2
        } else if op == Felt::ZERO {
            v_1 * v_2
        } else if op == Felt::ONE {
            v_1 + v_2
        } else {
            panic!("bad op")
        };
        assert_eq!(v_0, v_out);
    }
    // Ensure we've iterated through all IDs
    assert!(wire_idx_iter.next().is_none());

    // Ensure all multiplicities are 0
    for (_id, (_v, m)) in bus {
        assert_eq!(m, Felt::ZERO);
    }
}
