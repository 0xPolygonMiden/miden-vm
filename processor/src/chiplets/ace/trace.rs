use alloc::vec::Vec;
use core::ops::Range;

use miden_air::{
    RowIndex,
    trace::chiplets::ace::{
        CLK_IDX, CTX_IDX, EVAL_OP_IDX, ID_0_IDX, ID_1_IDX, ID_2_IDX, M_0_IDX, M_1_IDX, PTR_IDX,
        READ_NUM_EVAL_IDX, SELECTOR_BLOCK_IDX, SELECTOR_START_IDX, V_0_0_IDX, V_0_1_IDX, V_1_0_IDX,
        V_1_1_IDX, V_2_0_IDX, V_2_1_IDX,
    },
};
use vm_core::FieldElement;

use crate::{
    ContextId, ExecutionError, Felt, QuadFelt, Word,
    chiplets::ace::instruction::{Op, decode_instruction},
    errors::AceError,
};
/// Number of LogUp fractions in the wiring bus for rows in the `READ` section.
pub const NUM_ACE_LOGUP_FRACTIONS_READ: usize = 2;
/// Number of LogUp fractions in the wiring bus for rows in the `EVAL` section.
pub const NUM_ACE_LOGUP_FRACTIONS_EVAL: usize = 3;

/// Contains the variable and evaluation nodes resulting from the evaluation of a circuit.
/// The output value is checked to be equal to 0.
///
/// The set of nodes is used to fill the ACE chiplet trace.
#[derive(Debug)]
pub struct EvaluationContext {
    ctx: ContextId,
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

impl EvaluationContext {
    /// Generates the nodes in the graph generated by evaluating the inputs and circuit
    /// located in a contiguous memory region.
    pub fn new(ctx: ContextId, clk: RowIndex, num_read_rows: u32, num_eval_rows: u32) -> Self {
        let num_wires = 2 * num_read_rows + num_eval_rows;
        let num_rows = num_read_rows + num_eval_rows;

        Self {
            ctx,
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

    pub fn num_rows(&self) -> usize {
        (self.num_read_rows + self.num_eval_rows) as usize
    }

    pub fn clk(&self) -> u32 {
        self.clk.into()
    }

    pub fn ctx(&self) -> u32 {
        self.ctx.into()
    }

    pub fn num_read_rows(&self) -> u32 {
        self.num_read_rows
    }

    pub fn num_eval_rows(&self) -> u32 {
        self.num_eval_rows
    }

    /// Reads the word from memory at `ptr`, interpreting it as `[v_00, v_01, v_10, v_11]`, and
    /// adds wires with values `v_0 = QuadFelt(v_00, v_01)` and `v_1 = QuadFelt(v_10, v_11)`.
    /// Returns the pointer for the next operation.
    pub fn do_read(&mut self, ptr: Felt, word: Word) -> Result<Felt, ExecutionError> {
        // Add first variable as QuadFelt to wire bus
        let v_0 = QuadFelt::new(word[0], word[1]);
        let id_0 = self.wire_bus.insert(v_0);
        self.col_w0.push(id_0, v_0);

        // Add second variable as QuadFelt to wire bus
        let v_1 = QuadFelt::new(word[2], word[3]);
        let id_1 = self.wire_bus.insert(v_1);
        self.col_w1.push(id_1, v_1);

        // Add pointer to trace, and return the next one
        const PTR_OFFSET: Felt = Felt::new(4);
        self.col_ptr.push(ptr);
        Ok(ptr + PTR_OFFSET)
    }

    /// Reads the next instruction at `ptr`, requests the inputs from the wire bus
    /// and inserts a new wire with the result.
    /// Returns the pointer for the next operation.
    pub fn do_eval(&mut self, ptr: Felt, instruction: Felt) -> Result<Felt, ExecutionError> {
        // Decode instruction, ensuring it is valid
        let (id_l, id_r, op) = decode_instruction(instruction)
            .ok_or_else(|| ExecutionError::AceError(AceError::FailedDecodeInstruction))?;

        // Read value of id_l from wire bus, increasing its multiplicity
        let v_l = self
            .wire_bus
            .read_value(id_l)
            .ok_or_else(|| ExecutionError::AceError(AceError::FailedMemoryRead))?;
        let id_l = Felt::from(id_l);
        self.col_w1.push(id_l, v_l);

        // Read value of id_r from wire bus, increasing its multiplicity
        let v_r = self
            .wire_bus
            .read_value(id_r)
            .ok_or_else(|| ExecutionError::AceError(AceError::FailedMemoryRead))?;
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

        // Read instruction at `ptr` and increment it by 1.
        const PTR_OFFSET: Felt = Felt::ONE;
        self.col_ptr.push(ptr);
        Ok(ptr + PTR_OFFSET)
    }

    pub fn fill(&self, offset: usize, columns: &mut [Vec<Felt>]) {
        let num_read_rows = self.num_read_rows as usize;
        let num_eval_rows = self.num_eval_rows as usize;
        let num_rows = num_read_rows + num_eval_rows;
        let read_range = Range {
            start: offset,
            end: offset + num_read_rows,
        };
        let eval_range = Range {
            start: read_range.end,
            end: read_range.end + num_eval_rows,
        };

        // Fill start selector
        columns[SELECTOR_START_IDX][offset] = Felt::ONE;
        columns[SELECTOR_START_IDX][(offset + 1)..].fill(Felt::ZERO);

        // Block flag column
        let f_read = Felt::ZERO;
        let f_eval = Felt::ONE;
        columns[SELECTOR_BLOCK_IDX][read_range.clone()].fill(f_read);
        columns[SELECTOR_BLOCK_IDX][eval_range.clone()].fill(f_eval);

        // Fill ctx column which is constant across the section
        let ctx_felt = self.ctx.into();
        columns[CTX_IDX][offset..offset + num_rows].fill(ctx_felt);

        // Fill ptr column.
        columns[PTR_IDX][offset..offset + num_rows].copy_from_slice(&self.col_ptr);

        // Fill clk column which is constant across the section
        let clt_felt = self.clk.into();
        columns[CLK_IDX][offset..offset + num_rows].fill(clt_felt);

        // Fill n_eval which is constant across the read block
        let n_eval_felt = Felt::from(self.num_eval_rows - 1);
        columns[READ_NUM_EVAL_IDX][read_range.clone()].fill(n_eval_felt);

        // Fill OP column for EVAL rows
        columns[EVAL_OP_IDX][eval_range.clone()].copy_from_slice(&self.col_op);

        // Fill wire 0 columns for all rows
        columns[ID_0_IDX][offset..offset + num_rows].copy_from_slice(&self.col_w0.id);
        columns[V_0_0_IDX][offset..offset + num_rows].copy_from_slice(&self.col_w0.v_0);
        columns[V_0_1_IDX][offset..offset + num_rows].copy_from_slice(&self.col_w0.v_1);

        // Fill wire 1 columns for all rows
        columns[ID_1_IDX][offset..offset + num_rows].copy_from_slice(&self.col_w1.id);
        columns[V_1_0_IDX][offset..offset + num_rows].copy_from_slice(&self.col_w1.v_0);
        columns[V_1_1_IDX][offset..offset + num_rows].copy_from_slice(&self.col_w1.v_1);

        // Fill wire 2 columns for EVAL rows
        columns[ID_2_IDX][eval_range.clone()].copy_from_slice(&self.col_w2.id);
        columns[V_2_0_IDX][eval_range.clone()].copy_from_slice(&self.col_w2.v_0);
        columns[V_2_1_IDX][eval_range.clone()].copy_from_slice(&self.col_w2.v_1);

        // Fill multiplicity 0 column for all rows
        let mut multiplicities_iter = self.wire_bus.wires.iter().map(|(_v, m)| Felt::from(*m));
        // In the READ block, we inserted wires w_0 and w_1
        for row_index in read_range {
            let m_0 = multiplicities_iter
                .next()
                .expect("the m0 multiplicities were not constructed properly");
            let m_1 = multiplicities_iter
                .next()
                .expect("the m0 multiplicities were not constructed properly");
            columns[M_0_IDX][row_index] = m_0;
            columns[M_1_IDX][row_index] = m_1;
        }
        // In the EVAL block, we inserted wire w_0
        for row_index in eval_range {
            let m_0 = multiplicities_iter
                .next()
                .expect("the m0 multiplicities were not constructed properly");
            columns[M_0_IDX][row_index] = m_0;
        }

        debug_assert!(multiplicities_iter.next().is_none());
    }

    /// Returns the output value, if the circuit has finished evaluating.
    pub fn output_value(&self) -> Option<QuadFelt> {
        if !self.wire_bus.is_finalized() {
            return None;
        }
        self.wire_bus.wires.last().map(|(v, _m)| *v)
    }
}

/// Set of columns for a given wire containing `[id, v_0, v_1]`
#[derive(Debug)]
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

#[derive(Debug)]
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
