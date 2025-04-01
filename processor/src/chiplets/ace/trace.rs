use crate::chiplets::ace::{ ID_BITS, MAX_ID, Op};
use crate::chiplets::memory::Memory;
use crate::{ContextId, Felt, QuadFelt, Word};
use miden_air::RowIndex;
use miden_air::trace::CTX_COL_IDX;
use std::ops::Range;
use std::prelude::rust_2015::Vec;
use vm_core::FieldElement;

/// Contains the variable and evaluation nodes resulting from the evaluation of a circuit.
/// The output value is checked to be equal to 0.
///
/// The set of nodes is used to fill the ACE chiplet trace.
pub struct CircuitEvaluationContext {
    ctx: ContextId,
    ptr: Felt,
    clk: RowIndex,

    wire_bus: WireBus,

    col_ptr: Vec<Felt>,
    col_op: Vec<Felt>,

    col_w0: WireColumn,
    col_w1: WireColumn,
    col_w2: WireColumn,

    num_read_rows: u32,
    num_eval_rows: u32,
}

impl CircuitEvaluationContext {
    /// Generate the nodes in the graph generated by evaluating the inputs and circuit
    /// located in a contiguous memory region.
    pub fn new(
        ctx: ContextId,
        ptr: Felt,
        clk: RowIndex,
        num_read_rows: u32,
        num_eval_rows: u32,
    ) -> Self {
        let num_wires = 2 * num_read_rows  + num_eval_rows;
        let num_rows = num_read_rows + num_eval_rows;

        Self {
            ctx,
            ptr,
            clk,
            wire_bus: WireBus::new(num_wires),
            col_ptr: Vec::with_capacity(num_rows as usize),
            col_op: Vec::with_capacity(num_eval_rows as usize),
            col_w0: WireColumn::new(num_rows as usize),
            col_w1: WireColumn::new(num_rows as usize),
            col_w2: WireColumn::new(num_eval_rows as usize),
            num_read_rows,
            num_eval_rows,
        }
    }

    /// Read the next word in memory, interpreting it as `[v_00, v_01, v_10, v_11]`, and
    /// adds wires with values `v_0 = QuadFelt(v_00, v_01)` and `v_1 = QuadFelt(v_10, v_11)`
    pub fn do_read(&mut self, word: Word) -> Result<(), ()> {
        // Read word containing next two variables
        const PTR_OFFSET: Felt = Felt::new(4);
        self.col_ptr.push(self.ptr);
        self.ptr += PTR_OFFSET;

        // Add first variable as QuadFelt to wire bus
        let v_0 = QuadFelt::new(word[0], word[1]);
        let id_0 = self.wire_bus.insert(v_0);
        self.col_w0.push(id_0, v_0);

        // Add second variable as QuadFelt to wire bus
        let v_1 = QuadFelt::new(word[2], word[3]);
        let id_1 = self.wire_bus.insert(v_1);
        self.col_w1.push(id_1, v_1);
        Ok(())
    }

    /// Read the next element in memory as an instruction, requests the inputs from the wire bus
    /// and inserts a new wire with the result.
    pub fn do_eval(&mut self, instruction: Felt) -> Result<(), ()> {
        // Read instruction at `ptr` and increment it by 1.
        const PTR_OFFSET: Felt = Felt::new(4);
        self.col_ptr.push(self.ptr);
        self.ptr += PTR_OFFSET;

        // Decode instruction, ensuring it is valid
        let (id_l, id_r, op) = decode_instruction(instruction).expect("TODO");

        // Read value of id_l from wire bus, increasing its multiplicity
        let v_l = self.wire_bus.read_value(id_l).expect("TODO");
        let id_l = Felt::from(id_l);
        self.col_w1.push(id_l, v_l);

        // Read value of id_r from wire bus, increasing its multiplicity
        let v_r = self.wire_bus.read_value(id_r).expect("TODO");
        let id_r = Felt::from(id_r);
        self.col_w2.push(id_r, v_r);

        // Compute v_out and insert it into the wire bus.
        let v_out = match op {
            Op::Sub => v_l - v_r,
            Op::Mul => v_l * v_r,
            Op::Add => v_l + v_r,
        };
        let id_out = self.wire_bus.insert(v_out);
        self.col_w0.push(id_out, v_out);

        // Add op to column
        let op_sub = -Felt::ONE;
        let op_mul = Felt::ZERO;
        let op_add = Felt::ONE;
        let op = match op {
            Op::Sub => op_sub,
            Op::Mul => op_mul,
            Op::Add => op_add,
        };
        self.col_op.push(op);
        Ok(())
    }

    pub fn fill<'a>(&self, columns: &mut [&'a mut [Felt]]) {
        let num_read_rows = self.num_read_rows as usize;
        let num_eval_rows = self.num_eval_rows as usize;
        let read_range = Range { start: 0, end: num_read_rows };
        let eval_range = Range {
            start: num_read_rows,
            end: num_read_rows + num_eval_rows,
        };

        // Fill start selector
        columns[SELECTOR_START_IDX][0] = Felt::ONE;
        columns[SELECTOR_START_IDX][1..].fill(Felt::ZERO);

        // Block flag column
        let f_read = Felt::ZERO;
        let f_eval = Felt::ONE;
        columns[SELECTOR_BLOCK_IDX][read_range.clone()].fill(f_read);
        columns[SELECTOR_BLOCK_IDX][eval_range.clone()].fill(f_eval);

        // Fill ctx column which is constant across the section
        let ctx_felt = self.ctx.into();
        columns[CTX_COL_IDX].fill(ctx_felt);

        // Fill ptr column.
        columns[PTR_IDX].copy_from_slice(&self.col_ptr);

        // Fill clk column which is constant across the section
        let clt_felt = self.clk.into();
        columns[CTX_COL_IDX].fill(clt_felt);

        // Fill n_eval which is constant across the read block
        let n_eval_felt = Felt::from(self.num_eval_rows - 1);
        columns[READ_NUM_EVAL_IDX][read_range.clone()].fill(n_eval_felt);

        // Fill OP column for EVAL rows
        columns[EVAL_OP_IDX][eval_range.clone()].copy_from_slice(&self.col_op);

        // Fill wire 0 columns for all rows
        columns[ID_0_IDX].copy_from_slice(&self.col_w0.id);
        columns[V_0_0_IDX].copy_from_slice(&self.col_w0.v_0);
        columns[V_0_1_IDX].copy_from_slice(&self.col_w0.v_1);

        // Fill wire 1 columns for all rows
        columns[ID_1_IDX].copy_from_slice(&self.col_w1.id);
        columns[V_1_0_IDX].copy_from_slice(&self.col_w1.v_0);
        columns[V_1_1_IDX].copy_from_slice(&self.col_w1.v_1);

        // Fill wire 2 columns for EVAL rows
        columns[ID_2_IDX][eval_range.clone()].copy_from_slice(&self.col_w2.id);
        columns[V_2_0_IDX][eval_range.clone()].copy_from_slice(&self.col_w2.v_0);
        columns[V_2_1_IDX][eval_range.clone()].copy_from_slice(&self.col_w2.v_1);

        // Fill multiplicity 0 column for all rows
        let mut multiplicities_iter = self.wire_bus.wires.iter().map(|(_v, m)| Felt::from(*m));
        for row_index in read_range {
            let m_0 = multiplicities_iter.next().expect("TODO");
            let m_1 = multiplicities_iter.next().expect("TODO");
            columns[M_0_IDX][row_index] = Felt::from(m_0);
            columns[M_1_IDX][row_index] = Felt::from(m_1);
        }
        for row_index in eval_range {
            let m_0 = multiplicities_iter.next().expect("TODO");
            columns[M_0_IDX][row_index] = Felt::from(m_0);
        }

        debug_assert!(multiplicities_iter.next().is_none());
    }

    /// If the circuit has finished evaluating, return the output value
    pub fn output_value(&self) -> Option<QuadFelt> {
        if !self.wire_bus.is_finalized() {
            return None;
        }
        self.wire_bus.wires.last().map(|(v, _m)| *v)
    }
}

/// Set of columns for a given wire containing `[id, v_0, v_1]`
struct WireColumn {
    id: Vec<Felt>,
    v_0: Vec<Felt>,
    v_1: Vec<Felt>,
}

impl WireColumn {
    fn new(num_rows: usize) -> Self {
        Self {
            id: Vec::with_capacity(num_rows),
            v_0: Vec::with_capacity(num_rows),
            v_1: Vec::with_capacity(num_rows),
        }
    }

    /// Pushes the wire `(id, v)` to the columns.
    fn push(&mut self, id: Felt, v: QuadFelt) {
        self.id.push(id);
        let [v_0, v_1] = v.to_base_elements();
        self.v_0.push(v_0);
        self.v_1.push(v_1);
    }
}


struct WireBus {
    // Circuit ID as Felt of the next wire to be inserted
    id_next: Felt,
    // Pairs of values and multiplicities
    // The wire with index `id` is stored at `num_wires - 1 - id`
    wires: Vec<(QuadFelt, u32)>,
    // Total expected number of wires to be inserted.
    num_wires: u32,
}

impl WireBus {
    fn new(num_wires: u32) -> Self {
        Self {
            wires: Vec::with_capacity(num_wires as usize),
            num_wires,
            id_next: Felt::from(num_wires - 1),
        }
    }

    /// Inserts a new value into the bus, and returns its expected id as `Felt`
    fn insert(&mut self, value: QuadFelt) -> Felt {
        debug_assert!(!self.is_finalized());
        self.wires.push((value, 0));
        let id = self.id_next;
        self.id_next -= Felt::ONE;
        id
    }

    /// Reads the value of a wire with given `id`, incrementing its multiplicity.
    /// Returns `None` if the requested wire has not been inserted yet.
    fn read_value(&mut self, id: u32) -> Option<QuadFelt> {
        // Ensures subtracting the id from num_wires results in a valid wire index
        let (v, m) = self
            .num_wires
            .checked_sub(id + 1)
            .and_then(|id| self.wires.get_mut(id as usize))?;
        *m += 1;
        Some(*v)
    }

    /// Return true if the expected number of wires have been inserted.
    fn is_finalized(&self) -> bool {
        self.wires.len() == self.num_wires as usize
    }
}

fn eval_circuit(
    ctx: ContextId,
    ptr: Felt,
    clk: RowIndex,
    num_vars: Felt,
    num_eval: Felt,
    mem: &mut Memory,
) -> Result<(), ()> {
    let num_vars = num_vars.as_int();
    let num_eval = num_eval.as_int();

    // Ensure vars and instructions are word-aligned and non-empty
    if num_vars % 2 != 0 || num_vars == 0 {
        return Err(());
    }
    if num_eval % 4 != 0 || num_eval == 0 {
        return Err(());
    }

    //Check the number of wires are within the range that fits in an instruction
    let num_wires = num_vars + num_eval;
    if num_wires > MAX_ID as u64 {
        return Err(());
    }
    // Ensure instructions are word-aligned and non-empty
    let num_read_rows = num_vars as u32 / 2;
    let num_eval_rows = num_eval as u32;

    let mut evaluation_context =
        CircuitEvaluationContext::new(ctx, ptr, clk, num_read_rows, num_read_rows);

    // perform READ operations
    for _ in 0..num_read_rows {
        let word = mem.read_word(ctx, evaluation_context.ptr, clk).expect("TODO");
        evaluation_context.do_read(word)?;
    }
    // perform EVAL operations
    for _ in 0..num_eval_rows {
        let instruction = mem.read(ctx, evaluation_context.ptr, clk).expect("TODO");
        evaluation_context.do_eval(instruction)?;
    }

    // Ensure the circuit evaluated to zero.
    if !evaluation_context.output_value().is_some_and(|eval| eval == QuadFelt::ZERO) {
        return Err(());
    }

    Ok(())
}

/// Given a `Felt`, try to recover the components `id_l, id_r, op`.
pub fn decode_instruction(instruction: Felt) -> Option<(u32, u32, Op)> {
    const OP_BITS: u64 = 2;
    const ID_MASK: u64 = MAX_ID as u64;
    const OP_MASK: u64 = (1 << OP_BITS) - 1;

    let mut remaining = instruction.as_int();
    let id_l = (remaining & ID_MASK) as u32;
    remaining >>= ID_BITS;
    let id_r = (remaining & ID_MASK) as u32;
    remaining >>= ID_BITS;

    // Ensure the ID did not overflow
    if id_l > MAX_ID || id_r > MAX_ID {
        return None;
    }

    let op = match remaining {
        0 => Op::Sub,
        1 => Op::Mul,
        2 => Op::Add,
        _ => return None,
    };
    Some((id_l, id_r, op))
}

/// The index of the column containing the flag indicating the start of a new circuit evaluation.
pub const SELECTOR_START_IDX: usize = 0;
/// The index of the column containing the flag indicating whether the current row performs
/// a READ or EVAL operation.
pub const SELECTOR_BLOCK_IDX: usize = 1;
/// The index of the column containing memory context.
pub const CTX_IDX: usize = 2;
/// The index of the column containing the pointer from which to read the next two variables
/// or instruction.
pub const PTR_IDX: usize = 3;
/// The index of the column containing memory clk at which the memory read is performed.
pub const CLK_IDX: usize = 4;
/// The index of the column containing the flag indicating which arithmetic operation to perform.
pub const EVAL_OP_IDX: usize = 5;

/// The index of the column containing ID of the first wire.
pub const ID_0_IDX: usize = 6;
/// The index of the column containing the first base-field element of the value of the first wire.
pub const V_0_0_IDX: usize = 7;
/// The index of the column containing the second base-field element of the value of the first wire.
pub const V_0_1_IDX: usize = 8;
/// The index of the column containing the multiplicity of the first wire.
pub const M_0_IDX: usize = 15;

/// The index of the column containing ID of the second wire.
pub const ID_1_IDX: usize = 9;
/// The index of the column containing the first base-field element of the value of the second wire.
pub const V_1_0_IDX: usize = 10;
/// The index of the column containing the second base-field element of the value of the second wire.
pub const V_1_1_IDX: usize = 11;
/// The index of the column containing the multiplicity of the second wire.
pub const M_1_IDX: usize = 14;

/// The index of the column containing ID of the third wire.
pub const ID_2_IDX: usize = 12;
/// The index of the column containing the first base-field element of the value of the third wire.
pub const V_2_0_IDX: usize = 13;
/// The index of the column containing the second base-field element of the value of the third wire.
pub const V_2_1_IDX: usize = 14;

/// The index of the column containing the index of the first wire being evaluated.
pub const READ_NUM_EVAL_IDX: usize = 12;

pub const NUM_COLS: usize = 16;
