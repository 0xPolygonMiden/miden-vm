use crate::chiplets::ace::{CircuitError, ID_BITS, MAX_ID, Op};
use crate::{Felt, QuadFelt, Word};
use std::prelude::rust_2015::Vec;

/// Contains the variable and evaluation nodes resulting from the evaluation of a circuit.
/// The output value is checked to be equal to 0.
///
/// The set of nodes is used to fill the ACE chiplet trace.
pub struct CircuitEvaluation {
    vars: Vec<VarNode>,
    evals: Vec<EvalNode>,
}

impl CircuitEvaluation {
    pub fn new(num_vars: usize, num_evals: usize, mem: &[Word]) -> Result<Self, CircuitError> {
        let num_nodes = num_vars + num_evals;
        // Node indices cannot exceed MAX_ID.
        // Ensures usize -> u32 conversion is safe.
        if num_nodes >= MAX_ID as usize {
            return Err(CircuitError::InvalidLayout);
        }

        // Ensure vars and instructions are word-aligned
        if num_vars % 2 != 0 || num_evals % 4 != 0 {
            return Err(CircuitError::InvalidAlignment);
        }
        let (mem_read, mem_evals) = mem.split_at(num_vars / 2);

        let mut var_nodes = Vec::<VarNode>::with_capacity(num_vars);
        let mut eval_nodes = Vec::<EvalNode>::with_capacity(num_evals);

        // Create nodes for all the variables (inputs and constants) with the reversed id
        let vars_iter = mem_read.iter().flat_map(|&[v_00, v_01, v_10, v_11]| {
            [QuadFelt::new(v_00, v_01), QuadFelt::new(v_10, v_11)]
        });
        for (id_usize, v) in vars_iter.enumerate() {
            // Safety: u32::MAX > MAX_ID >= num_nodes > id_usize
            let id = (num_nodes - 1 - id_usize) as u32;
            var_nodes.push(VarNode { id, v, m: 0 })
        }

        let get_node_value = |id: u32,
                              vars: &mut [VarNode],
                              evals: &mut [EvalNode]|
         -> Result<QuadFelt, CircuitError> {
            if (id as usize) >= num_nodes {
                return Err(CircuitError::InvalidLayout);
            }
            let idx = num_nodes - 1 - id as usize;

            if idx < num_vars {
                vars.get_mut(idx).map(|var| {
                    var.m += 1;
                    var.v
                })
            } else {
                evals.get_mut(idx - num_vars).map(|eval| {
                    eval.m_out += 1;
                    eval.v_out
                })
            }
            .ok_or(CircuitError::InvalidLayout)
        };

        // Evaluate each instruction, ensuring they reference only previously evaluated nodes
        for (id_usize, instruction) in mem_evals.iter().flatten().enumerate() {
            let id_out = (num_evals - 1 - id_usize) as u32;
            let (id_l, id_r, op) =
                decode_instruction(*instruction).ok_or(CircuitError::InvalidInstruction)?;

            let v_l = get_node_value(id_l, &mut var_nodes, &mut eval_nodes)?;
            let v_r = get_node_value(id_r, &mut var_nodes, &mut eval_nodes)?;

            let v_out = match op {
                Op::Sub => v_l - v_r,
                Op::Mul => v_l * v_r,
                Op::Add => v_l + v_r,
            };

            let node_out = EvalNode {
                op,
                id_l,
                v_l,
                id_r,
                v_r,
                id_out,
                v_out,
                m_out: 0,
            };
            eval_nodes.push(node_out);
        }

        Ok(Self { vars: var_nodes, evals: eval_nodes })
    }

    pub fn eval(&self) -> QuadFelt {
        self.evals.last().unwrap().v_out
    }

    // #[cfg(test)]
    // fn check(&self) {
    //     let mut bus = BTreeMap::<u32, (QuadFelt, u32)>::new();
    //     for node in self.vars {
    //         bus.insert(node.id, (node.v, node.m))
    //             .expect("all var nodes should be unique");
    //     }
    //     assert!(bus.keys().tuple_windows().all(|(id_0, id_1)| *id_1 == id_0 + 1));
    //     for node in self.evals {
    //         let (v_l_expected, m_l) = bus.get_mut(&node.id_l).expect("");
    //         m_l.checked_sub(1).expect("");
    //         assert_eq!(*v_l_expected, node.v_l);
    //
    //         let (v_r_expected, m_r) = bus.get_mut(&node.id_r).expect("");
    //         m_r.checked_sub(1).expect("");
    //         assert_eq!(*v_r_expected, node.v_r);
    //
    //         let v_out_expected = match node.op {
    //             Op::Sub => node.v_l - node.v_r,
    //             Op::Mul => node.v_l * node.v_r,
    //             Op::Add => node.v_l + node.v_r,
    //         };
    //         assert_eq!(node.v_out, v_out_expected);
    //
    //         bus.insert(node.id_out, (node.v_out, node.m_out)).expect("");
    //     }
    //
    //     let (v_final, _) = bus.get(&0).expect("");
    //     assert_eq!(*v_final, QuadFelt::ZERO);
    //
    //     // Ensure all nodes are in order, and that multiplicities are 0
    //     for i in 0..bus.len() {
    //         let id = i as u32;
    //         let (_, m) = bus.get(&id).unwrap();
    //         assert_eq!(*m, 0);
    //     }
    //
    // }
}

struct VarNode {
    id: u32,
    v: QuadFelt,
    m: u32,
}

struct EvalNode {
    op: Op,

    id_l: u32,
    v_l: QuadFelt,

    id_r: u32,
    v_r: QuadFelt,

    id_out: u32,
    v_out: QuadFelt,
    m_out: u32,
}

// fn eval_circuit(num_vars: usize, num_eval: usize, mem: &[Word]) -> Result<(), ()> {
//     let evaluation = CircuitEvaluation::new(num_vars, num_eval, mem)?;
//
//     Ok(())
// }

/// Given a `Felt`, try to recover the components `id_l, id_r, op`.
fn decode_instruction(instruction: Felt) -> Option<(u32, u32, Op)> {
    const OP_BITS: u64 = 2;
    const ID_MASK: u64 = MAX_ID as u64;
    const OP_MASK: u64 = (1 << OP_BITS) - 1;

    let mut remaining = instruction.as_int();
    let id_l = (remaining & ID_MASK) as u32;
    remaining >>= ID_BITS;
    let id_r = (remaining & ID_MASK) as u32;
    remaining >>= ID_BITS;

    let op = match remaining {
        0 => Op::Add,
        1 => Op::Mul,
        2 => Op::Sub,
        _ => return None,
    };
    Some((id_l, id_r, op))
}
