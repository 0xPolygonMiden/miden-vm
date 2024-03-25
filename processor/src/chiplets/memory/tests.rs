use super::{
    super::ZERO, Felt, FieldElement, Memory, TraceFragment, ADDR_COL_IDX, CLK_COL_IDX, CTX_COL_IDX,
    D0_COL_IDX, D1_COL_IDX, D_INV_COL_IDX, EMPTY_WORD, ONE, V_COL_RANGE,
};
use crate::ContextId;
use alloc::vec::Vec;
use miden_air::trace::chiplets::memory::{
    Selectors, MEMORY_COPY_READ, MEMORY_INIT_READ, MEMORY_WRITE, TRACE_WIDTH as MEMORY_TRACE_WIDTH,
};
use vm_core::Word;

#[test]
fn mem_init() {
    let mem = Memory::default();
    assert_eq!(0, mem.size());
    assert_eq!(0, mem.trace_len());
}

#[test]
fn mem_read() {
    let mut mem = Memory::default();

    // read a value from address 0; clk = 1
    let addr0 = 0;
    let value = mem.read(ContextId::root(), addr0, 1);
    assert_eq!(EMPTY_WORD, value);
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // read a value from address 3; clk = 2
    let addr3 = 3;
    let value = mem.read(ContextId::root(), addr3, 2);
    assert_eq!(EMPTY_WORD, value);
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // read a value from address 0 again; clk = 3
    let value = mem.read(ContextId::root(), addr0, 3);
    assert_eq!(EMPTY_WORD, value);
    assert_eq!(2, mem.size());
    assert_eq!(3, mem.trace_len());

    // read a value from address 2; clk = 4
    let addr2 = 2;
    let value = mem.read(ContextId::root(), addr2, 4);
    assert_eq!(EMPTY_WORD, value);
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // check generated trace and memory data provided to the ChipletsBus; rows should be sorted by
    // address and then clock cycle
    let trace = build_trace(mem, 4);

    // address 0
    let mut prev_row = [ZERO; MEMORY_TRACE_WIDTH];
    let memory_access = MemoryAccess::new(ContextId::root(), addr0, 1, EMPTY_WORD);
    prev_row = verify_memory_access(&trace, 0, MEMORY_INIT_READ, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr0, 3, EMPTY_WORD);
    prev_row = verify_memory_access(&trace, 1, MEMORY_COPY_READ, &memory_access, prev_row);

    // address 2
    let memory_access = MemoryAccess::new(ContextId::root(), addr2, 4, EMPTY_WORD);
    prev_row = verify_memory_access(&trace, 2, MEMORY_INIT_READ, &memory_access, prev_row);

    // address 3
    let memory_access = MemoryAccess::new(ContextId::root(), addr3, 2, EMPTY_WORD);
    verify_memory_access(&trace, 3, MEMORY_INIT_READ, &memory_access, prev_row);
}

#[test]
fn mem_write() {
    let mut mem = Memory::default();

    // write a value into address 0; clk = 1
    let addr0 = 0;
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), addr0, 1, value1);
    assert_eq!(value1, mem.get_value(ContextId::root(), addr0).unwrap());
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // write a value into address 2; clk = 2
    let addr2 = 2;
    let value5 = [Felt::new(5), ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), addr2, 2, value5);
    assert_eq!(value5, mem.get_value(ContextId::root(), addr2).unwrap());
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // write a value into address 1; clk = 3
    let addr1 = 1;
    let value7 = [Felt::new(7), ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), addr1, 3, value7);
    assert_eq!(value7, mem.get_value(ContextId::root(), addr1).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(3, mem.trace_len());

    // write a value into address 0; clk = 4
    let value9 = [Felt::new(9), ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), addr0, 4, value9);
    assert_eq!(value7, mem.get_value(ContextId::root(), addr1).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // check generated trace and memory data provided to the ChipletsBus; rows should be sorted by
    // address and then clock cycle
    let trace = build_trace(mem, 4);

    // address 0
    let mut prev_row = [ZERO; MEMORY_TRACE_WIDTH];
    let memory_access = MemoryAccess::new(ContextId::root(), addr0, 1, value1);
    prev_row = verify_memory_access(&trace, 0, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr0, 4, value9);
    prev_row = verify_memory_access(&trace, 1, MEMORY_WRITE, &memory_access, prev_row);

    // address 1
    let memory_access = MemoryAccess::new(ContextId::root(), addr1, 3, value7);
    prev_row = verify_memory_access(&trace, 2, MEMORY_WRITE, &memory_access, prev_row);

    // address 2
    let memory_access = MemoryAccess::new(ContextId::root(), addr2, 2, value5);
    verify_memory_access(&trace, 3, MEMORY_WRITE, &memory_access, prev_row);
}

#[test]
fn mem_write_read() {
    let mut mem = Memory::default();

    // write 1 into address 5; clk = 1
    let addr5 = 5;
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), addr5, 1, value1);

    // write 4 into address 2; clk = 2
    let addr2 = 2;
    let value4 = [Felt::new(4), ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), addr2, 2, value4);

    // read a value from address 5; clk = 3
    mem.read(ContextId::root(), addr5, 3);

    // write 2 into address 5; clk = 4
    let value2 = [Felt::new(2), ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), addr5, 4, value2);

    // read a value from address 2; clk = 5
    mem.read(ContextId::root(), addr2, 5);

    // write 7 into address 2; clk = 6
    let value7 = [Felt::new(7), ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), addr2, 6, value7);

    // read a value from address 5; clk = 7
    mem.read(ContextId::root(), addr5, 7);

    // read a value from address 2; clk = 8
    mem.read(ContextId::root(), addr2, 8);

    // read a value from address 5; clk = 9
    mem.read(ContextId::root(), addr5, 9);

    // check generated trace and memory data provided to the ChipletsBus; rows should be sorted by
    // address and then clock cycle
    let trace = build_trace(mem, 9);

    // address 2
    let mut prev_row = [ZERO; MEMORY_TRACE_WIDTH];
    let memory_access = MemoryAccess::new(ContextId::root(), addr2, 2, value4);
    prev_row = verify_memory_access(&trace, 0, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr2, 5, value4);
    prev_row = verify_memory_access(&trace, 1, MEMORY_COPY_READ, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr2, 6, value7);
    prev_row = verify_memory_access(&trace, 2, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr2, 8, value7);
    prev_row = verify_memory_access(&trace, 3, MEMORY_COPY_READ, &memory_access, prev_row);

    // address 5
    let memory_access = MemoryAccess::new(ContextId::root(), addr5, 1, value1);
    prev_row = verify_memory_access(&trace, 4, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr5, 3, value1);
    prev_row = verify_memory_access(&trace, 5, MEMORY_COPY_READ, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr5, 4, value2);
    prev_row = verify_memory_access(&trace, 6, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr5, 7, value2);
    prev_row = verify_memory_access(&trace, 7, MEMORY_COPY_READ, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), addr5, 9, value2);
    verify_memory_access(&trace, 8, MEMORY_COPY_READ, &memory_access, prev_row);
}

#[test]
fn mem_multi_context() {
    let mut mem = Memory::default();

    // write a value into ctx = ContextId::root(), addr = 0; clk = 1
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), 0, 1, value1);
    assert_eq!(value1, mem.get_value(ContextId::root(), 0).unwrap());
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // write a value into ctx = 3, addr = 1; clk = 4
    let value2 = [ZERO, ONE, ZERO, ZERO];
    mem.write(3.into(), 1, 4, value2);
    assert_eq!(value2, mem.get_value(3.into(), 1).unwrap());
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // read a value from ctx = 3, addr = 1; clk = 6
    let value = mem.read(3.into(), 1, 6);
    assert_eq!(value2, value);
    assert_eq!(2, mem.size());
    assert_eq!(3, mem.trace_len());

    // write a value into ctx = 3, addr = 0; clk = 7
    let value3 = [ZERO, ZERO, ONE, ZERO];
    mem.write(3.into(), 0, 7, value3);
    assert_eq!(value3, mem.get_value(3.into(), 0).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // read a value from ctx = 0, addr = 0; clk = 9
    let value = mem.read(ContextId::root(), 0, 9);
    assert_eq!(value1, value);
    assert_eq!(3, mem.size());
    assert_eq!(5, mem.trace_len());

    // check generated trace and memory data provided to the ChipletsBus; rows should be sorted by
    // address and then clock cycle
    let trace = build_trace(mem, 5);

    // ctx = 0, addr = 0
    let mut prev_row = [ZERO; MEMORY_TRACE_WIDTH];
    let memory_access = MemoryAccess::new(ContextId::root(), 0, 1, value1);
    prev_row = verify_memory_access(&trace, 0, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(ContextId::root(), 0, 9, value1);
    prev_row = verify_memory_access(&trace, 1, MEMORY_COPY_READ, &memory_access, prev_row);

    // ctx = 3, addr = 0
    let memory_access = MemoryAccess::new(3.into(), 0, 7, value3);
    prev_row = verify_memory_access(&trace, 2, MEMORY_WRITE, &memory_access, prev_row);

    // ctx = 3, addr = 1
    let memory_access = MemoryAccess::new(3.into(), 1, 4, value2);
    prev_row = verify_memory_access(&trace, 3, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryAccess::new(3.into(), 1, 6, value2);
    verify_memory_access(&trace, 4, MEMORY_COPY_READ, &memory_access, prev_row);
}

#[test]
fn mem_get_state_at() {
    let mut mem = Memory::default();

    // Write 1 into (ctx = 0, addr = 5) at clk = 1.
    // This means that mem[5] = 1 at the beginning of clk = 2
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), 5, 1, value1);

    // Write 4 into (ctx = 0, addr = 2) at clk = 2.
    // This means that mem[2] = 4 at the beginning of clk = 3
    let value4 = [Felt::new(4), ZERO, ZERO, ZERO];
    mem.write(ContextId::root(), 2, 2, value4);

    // write 7 into (ctx = 3, addr = 3) at clk = 4
    // This means that mem[3] = 7 at the beginning of clk = 4
    let value7 = [Felt::new(7), ZERO, ZERO, ZERO];
    mem.write(3.into(), 3, 4, value7);

    // Check memory state at clk = 2
    assert_eq!(mem.get_state_at(ContextId::root(), 2), vec![(5, value1)]);
    assert_eq!(mem.get_state_at(3.into(), 2), vec![]);

    // Check memory state at clk = 3
    assert_eq!(mem.get_state_at(ContextId::root(), 3), vec![(2, value4), (5, value1)]);
    assert_eq!(mem.get_state_at(3.into(), 3), vec![]);

    // Check memory state at clk = 4
    assert_eq!(mem.get_state_at(ContextId::root(), 4), vec![(2, value4), (5, value1)]);
    assert_eq!(mem.get_state_at(3.into(), 4), vec![]);

    // Check memory state at clk = 5
    assert_eq!(mem.get_state_at(ContextId::root(), 5), vec![(2, value4), (5, value1)]);
    assert_eq!(mem.get_state_at(3.into(), 5), vec![(3, value7)]);
}

// HELPER STRUCT & FUNCTIONS
// ================================================================================================

/// Contains data representing a memory access.
pub struct MemoryAccess {
    ctx: ContextId,
    addr: Felt,
    clk: Felt,
    word: [Felt; 4],
}

impl MemoryAccess {
    pub fn new(ctx: ContextId, addr: u32, clk: u32, word: Word) -> Self {
        Self {
            ctx,
            addr: Felt::from(addr),
            clk: Felt::from(clk),
            word,
        }
    }
}

/// Builds a trace of the specified length and fills it with data from the provided Memory instance.
fn build_trace(mem: Memory, num_rows: usize) -> Vec<Vec<Felt>> {
    let mut trace = (0..MEMORY_TRACE_WIDTH).map(|_| vec![ZERO; num_rows]).collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);
    mem.fill_trace(&mut fragment);

    trace
}

fn read_trace_row(trace: &[Vec<Felt>], step: usize) -> [Felt; MEMORY_TRACE_WIDTH] {
    let mut row = [ZERO; MEMORY_TRACE_WIDTH];
    for (value, column) in row.iter_mut().zip(trace) {
        *value = column[step];
    }
    row
}

fn build_trace_row(
    memory_access: &MemoryAccess,
    op_selectors: Selectors,
    prev_row: [Felt; MEMORY_TRACE_WIDTH],
) -> [Felt; MEMORY_TRACE_WIDTH] {
    let MemoryAccess {
        ctx,
        addr,
        clk,
        word: new_val,
    } = *memory_access;

    let mut row = [ZERO; MEMORY_TRACE_WIDTH];

    row[0] = op_selectors[0];
    row[1] = op_selectors[1];
    row[CTX_COL_IDX] = ctx.into();
    row[ADDR_COL_IDX] = addr;
    row[CLK_COL_IDX] = clk;
    row[V_COL_RANGE.start] = new_val[0];
    row[V_COL_RANGE.start + 1] = new_val[1];
    row[V_COL_RANGE.start + 2] = new_val[2];
    row[V_COL_RANGE.start + 3] = new_val[3];

    if prev_row != [ZERO; MEMORY_TRACE_WIDTH] {
        let delta = if row[CTX_COL_IDX] != prev_row[CTX_COL_IDX] {
            row[CTX_COL_IDX] - prev_row[CTX_COL_IDX]
        } else if row[ADDR_COL_IDX] != prev_row[ADDR_COL_IDX] {
            row[ADDR_COL_IDX] - prev_row[ADDR_COL_IDX]
        } else {
            row[CLK_COL_IDX] - prev_row[CLK_COL_IDX] - ONE
        };

        let (hi, lo) = super::split_element_u32_into_u16(delta);
        row[D0_COL_IDX] = lo;
        row[D1_COL_IDX] = hi;
        row[D_INV_COL_IDX] = delta.inv();
    }

    row
}

fn verify_memory_access(
    trace: &[Vec<Felt>],
    row: u32,
    op_selectors: Selectors,
    memory_access: &MemoryAccess,
    prev_row: [Felt; MEMORY_TRACE_WIDTH],
) -> [Felt; MEMORY_TRACE_WIDTH] {
    let expected_row = build_trace_row(memory_access, op_selectors, prev_row);
    assert_eq!(expected_row, read_trace_row(trace, row as usize));

    expected_row
}
