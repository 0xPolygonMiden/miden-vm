use std::prelude::rust_2024::Vec;
use crate::math::FieldElement;
use crate::QuadFelt;

mod encoder;
mod trace;

const ID_BITS: u64 = 30;
const MAX_ID: u32 = (1 << ID_BITS) - 1;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Op {
    Sub,
    Mul,
    Add,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Node {
    Input(usize),
    Const(usize),
    Eval(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    node_l: Node,
    node_r: Node,
    op: Op,
}


fn pad_inputs(mut inputs: Vec<QuadFelt>)-> Vec<QuadFelt> {
    let padded_len = inputs.len().next_multiple_of(2);
    inputs.resize(padded_len, QuadFelt::ZERO);
    inputs
}