use crate::chiplets::ace::{ID_BITS, MAX_ID, Op};
use crate::math::FieldElement;
use crate::{ContextId, Felt, QuadFelt, Word};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::prelude::rust_2015::Vec;

/// Contains the variable and evaluation nodes resulting from the evaluation of a circuit.
/// The output value is checked to be equal to 0.
///
/// The set of nodes is used to fill the ACE chiplet trace.
struct CircuitEvaluation {
    vars: Vec<VarNode>,
    evals: Vec<EvalNode>,
}

impl CircuitEvaluation {
    fn new(num_vars: usize, num_evals: usize, mem: &[Word]) -> Result<Self, ()> {
        let num_nodes = num_vars + num_evals;
        // Node indices cannot exceed MAX_ID.
        // Ensures usize -> u32 conversion is safe.
        if num_nodes >= MAX_ID as usize {
            return Err(());
        }

        // Ensure vars and instructions are word-aligned
        if num_vars % 2 != 0 || num_evals % 4 != 0 {
            return Err(());
        }
        let (mem_read, mem_evals) = mem.split_at(num_vars / 2);

        let mut vars = Vec::with_capacity(num_vars);
        let mut evals = Vec::with_capacity(num_evals);

        let get_node_value = |id: u32| -> Option<QuadFelt> {
            if (id as usize) >= num_nodes {
                return None;
            }
            let id_usize = num_nodes - 1 - id as usize;

            match id_usize {
                (0..num_vars) => vars.get_mut(id_usize).map(|var| {
                    var.m += 1;
                    var.v
                }),
                num_vars..num_nodes => evals.get_mut(id_usize - num_vars).map(|eval| {
                    eval.m_out += 1;
                    eval.v_out
                }),
                _ => unreachable!("id_usize is always < num_nodes"),
            }
        };

        let vars_iter = mem_read.iter().flat_map(|&[v_00, v_01, v_10, v_11]| {
            [QuadFelt::new(v_00, v_01), QuadFelt::new(v_10, v_11)]
        });
        for (id_usize, value) in vars_iter.enumerate() {
            // Safety: u32::MAX > MAX_ID >= num_nodes > id_usize
            let id = (num_nodes - 1 - id_usize) as u32;
            vars.push(VarNode::new(id, value))
        }

        // Evaluate each instruction, assuming they
        for (id_usize, instruction) in mem_evals.iter().flatten().enumerate() {
            let id_out = (num_evals - 1 - id_usize) as u32;
            let (id_l, id_r, op) = decode_instruction(*instruction).expect("");

            let v_l = get_node_value(id_l).ok_or(())?;
            let v_r = get_node_value(id_r).ok_or(())?;

            let node_out = EvalNode::new(id_out, op, id_l, v_l, id_r, v_r);
            evals.push(node_out);
        }

        // Ensure we had at least one eval instruction
        let final_node = evals.last().ok_or(())?;
        if final_node.v_out != QuadFelt::ZERO {
            return Err(());
        }

        Ok(Self { vars, evals })
    }

    #[cfg(test)]
    fn check(&self) {
        let mut bus = BTreeMap::<u32, (QuadFelt, u32)>::new();
        for node in self.vars {
            bus.insert(node.id, (node.v, node.m))
                .expect("all var nodes should be unique");
        }
        assert!(bus.keys().tuple_windows().all(|(id_0, id_1)| *id_1 == id_0 + 1));
        for node in self.evals {
            let (v_l_expected, m_l) = bus.get_mut(&node.id_l).expect("");
            m_l.checked_sub(1).expect("");
            assert_eq!(*v_l_expected, node.v_l);

            let (v_r_expected, m_r) = bus.get_mut(&node.id_r).expect("");
            m_r.checked_sub(1).expect("");
            assert_eq!(*v_r_expected, node.v_r);

            let v_out_expected = match node.op {
                Op::Sub => node.v_l - node.v_r,
                Op::Mul => node.v_l * node.v_r,
                Op::Add => node.v_l + node.v_r,
            };
            assert_eq!(node.v_out, v_out_expected);

            bus.insert(node.id_out, (node.v_out, node.m_out)).expect("");
        }

        let (v_final, _) = bus.get(&0).expect("");
        assert_eq!(*v_final, QuadFelt::ZERO);

        // Ensure all nodes are in order, and that multiplicities are 0
        for i in 0..bus.len() {
            let id = i as u32;
            let (_, m) = bus.get(&id).unwrap();
            assert_eq!(*m, 0);
        }

    }
}

struct VarNode {
    id: u32,
    v: QuadFelt,
    m: u32,
}

impl VarNode {
    fn new(id: u32, v: QuadFelt) -> Self {
        Self { id, v, m: 0 }
    }
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

impl EvalNode {
    fn new(id_out: u32, op: Op, id_l: u32, v_l: QuadFelt, id_r: u32, v_r: QuadFelt) -> Self {
        let v_out = match op {
            Op::Sub => v_l - v_r,
            Op::Mul => v_l * v_r,
            Op::Add => v_l + v_r,
        };
        Self {
            op,
            id_l,
            v_l,
            id_r,
            v_r,
            id_out,
            v_out,
            m_out: 0,
        }
    }
}

fn eval_circuit(num_vars: usize, num_eval: usize, mem: &[Word]) -> Result<(), ()> {
    let evaluation = CircuitEvaluation::new(num_vars, num_eval, mem)?;

    Ok(())
}

fn decode_instruction(instruction: Felt) -> Option<(u32, u32, Op)> {
    const OP_BITS: u64 = 2;
    const ID_MASK: u64 = MAX_ID.into();
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
