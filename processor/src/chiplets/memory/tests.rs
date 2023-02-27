use super::{
    super::bus::{ChipletsLookup, ChipletsLookupRow},
    ChipletsBus, Felt, FieldElement, Memory, MemoryLookup, StarkField, TraceFragment, Vec,
    ADDR_COL_IDX, CLK_COL_IDX, CTX_COL_IDX, D0_COL_IDX, D1_COL_IDX, D_INV_COL_IDX, ONE,
    V_COL_RANGE, ZERO,
};
use vm_core::chiplets::memory::{
    Selectors, MEMORY_COPY_READ, MEMORY_INIT_READ, MEMORY_READ_LABEL, MEMORY_WRITE,
    MEMORY_WRITE_LABEL, TRACE_WIDTH as MEMORY_TRACE_WIDTH,
};

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
    let addr0 = Felt::new(0);
    let value = mem.read(0, addr0, 1);
    assert_eq!([ZERO; 4], value);
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // read a value from address 3; clk = 2
    let addr3 = Felt::new(3);
    let value = mem.read(0, addr3, 2);
    assert_eq!([ZERO; 4], value);
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // read a value from address 0 again; clk = 3
    let value = mem.read(0, addr0, 3);
    assert_eq!([ZERO; 4], value);
    assert_eq!(2, mem.size());
    assert_eq!(3, mem.trace_len());

    // read a value from address 2; clk = 4
    let addr2 = Felt::new(2);
    let value = mem.read(0, addr2, 4);
    assert_eq!([ZERO; 4], value);
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // check generated trace and memory data provided to the ChipletsBus; rows should be sorted by
    // address and then clock cycle
    let (trace, chiplets_bus) = build_trace(mem, 4);

    // address 0
    let mut prev_row = [ZERO; MEMORY_TRACE_WIDTH];
    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr0, 1, [ZERO; 4]);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 0, MEMORY_INIT_READ, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr0, 3, [ZERO; 4]);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 1, MEMORY_COPY_READ, &memory_access, prev_row);

    // address 2
    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr2, 4, [ZERO; 4]);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 2, MEMORY_INIT_READ, &memory_access, prev_row);

    // address 3
    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr3, 2, [ZERO; 4]);
    verify_memory_access(&trace, &chiplets_bus, 3, MEMORY_INIT_READ, &memory_access, prev_row);
}

#[test]
fn mem_write() {
    let mut mem = Memory::default();

    // write a value into address 0; clk = 1
    let addr0 = Felt::new(0);
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(0, addr0, 1, value1);
    assert_eq!(value1, mem.get_value(0, addr0.as_int()).unwrap());
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // write a value into address 2; clk = 2
    let addr2 = Felt::new(2);
    let value5 = [Felt::new(5), ZERO, ZERO, ZERO];
    mem.write(0, addr2, 2, value5);
    assert_eq!(value5, mem.get_value(0, addr2.as_int()).unwrap());
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // write a value into address 1; clk = 3
    let addr1 = Felt::new(1);
    let value7 = [Felt::new(7), ZERO, ZERO, ZERO];
    mem.write(0, addr1, 3, value7);
    assert_eq!(value7, mem.get_value(0, addr1.as_int()).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(3, mem.trace_len());

    // write a value into address 0; clk = 4
    let value9 = [Felt::new(9), ZERO, ZERO, ZERO];
    mem.write(0, addr0, 4, value9);
    assert_eq!(value7, mem.get_value(0, addr1.as_int()).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // check generated trace and memory data provided to the ChipletsBus; rows should be sorted by
    // address and then clock cycle
    let (trace, chiplets_bus) = build_trace(mem, 4);

    // address 0
    let mut prev_row = [ZERO; MEMORY_TRACE_WIDTH];
    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, addr0, 1, value1);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 0, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, addr0, 4, value9);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 1, MEMORY_WRITE, &memory_access, prev_row);

    // address 1
    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, addr1, 3, value7);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 2, MEMORY_WRITE, &memory_access, prev_row);

    // address 2
    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, addr2, 2, value5);
    verify_memory_access(&trace, &chiplets_bus, 3, MEMORY_WRITE, &memory_access, prev_row);
}

#[test]
fn mem_write_read() {
    let mut mem = Memory::default();

    // write 1 into address 5; clk = 1
    let addr5 = Felt::new(5);
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(0, addr5, 1, value1);

    // write 4 into address 2; clk = 2
    let addr2 = Felt::new(2);
    let value4 = [Felt::new(4), ZERO, ZERO, ZERO];
    mem.write(0, addr2, 2, value4);

    // read a value from address 5; clk = 3
    mem.read(0, addr5, 3);

    // write 2 into address 5; clk = 4
    let value2 = [Felt::new(2), ZERO, ZERO, ZERO];
    mem.write(0, addr5, 4, value2);

    // read a value from address 2; clk = 5
    mem.read(0, addr2, 5);

    // write 7 into address 2; clk = 6
    let value7 = [Felt::new(7), ZERO, ZERO, ZERO];
    mem.write(0, addr2, 6, value7);

    // read a value from address 5; clk = 7
    mem.read(0, addr5, 7);

    // read a value from address 2; clk = 8
    mem.read(0, addr2, 8);

    // read a value from address 5; clk = 9
    mem.read(0, addr5, 9);

    // check generated trace and memory data provided to the ChipletsBus; rows should be sorted by
    // address and then clock cycle
    let (trace, chiplets_bus) = build_trace(mem, 9);

    // address 2
    let mut prev_row = [ZERO; MEMORY_TRACE_WIDTH];
    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, addr2, 2, value4);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 0, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr2, 5, value4);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 1, MEMORY_COPY_READ, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, addr2, 6, value7);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 2, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr2, 8, value7);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 3, MEMORY_COPY_READ, &memory_access, prev_row);

    // address 5
    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, addr5, 1, value1);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 4, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr5, 3, value1);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 5, MEMORY_COPY_READ, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, addr5, 4, value2);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 6, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr5, 7, value2);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 7, MEMORY_COPY_READ, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, addr5, 9, value2);
    verify_memory_access(&trace, &chiplets_bus, 8, MEMORY_COPY_READ, &memory_access, prev_row);
}

#[test]
fn mem_multi_context() {
    let mut mem = Memory::default();

    // write a value into ctx = 0, addr = 0; clk = 1
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(0, ZERO, 1, value1);
    assert_eq!(value1, mem.get_value(0, 0).unwrap());
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // write a value into ctx = 3, addr = 1; clk = 4
    let value2 = [ZERO, ONE, ZERO, ZERO];
    mem.write(3, ONE, 4, value2);
    assert_eq!(value2, mem.get_value(3, 1).unwrap());
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // read a value from ctx = 3, addr = 1; clk = 6
    let value = mem.read(3, ONE, 6);
    assert_eq!(value2, value);
    assert_eq!(2, mem.size());
    assert_eq!(3, mem.trace_len());

    // write a value into ctx = 3, addr = 0; clk = 7
    let value3 = [ZERO, ZERO, ONE, ZERO];
    mem.write(3, ZERO, 7, value3);
    assert_eq!(value3, mem.get_value(3, 0).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // read a value from ctx = 0, addr = 0; clk = 9
    let value = mem.read(0, ZERO, 9);
    assert_eq!(value1, value);
    assert_eq!(3, mem.size());
    assert_eq!(5, mem.trace_len());

    // check generated trace and memory data provided to the ChipletsBus; rows should be sorted by
    // address and then clock cycle
    let (trace, chiplets_bus) = build_trace(mem, 5);

    // ctx = 0, addr = 0
    let mut prev_row = [ZERO; MEMORY_TRACE_WIDTH];
    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 0, ZERO, 1, value1);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 0, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 0, ZERO, 9, value1);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 1, MEMORY_COPY_READ, &memory_access, prev_row);

    // ctx = 3, addr = 0
    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 3, ZERO, 7, value3);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 2, MEMORY_WRITE, &memory_access, prev_row);

    // ctx = 3, addr = 1
    let memory_access = MemoryLookup::from_ints(MEMORY_WRITE_LABEL, 3, ONE, 4, value2);
    prev_row =
        verify_memory_access(&trace, &chiplets_bus, 3, MEMORY_WRITE, &memory_access, prev_row);

    let memory_access = MemoryLookup::from_ints(MEMORY_READ_LABEL, 3, ONE, 6, value2);
    verify_memory_access(&trace, &chiplets_bus, 4, MEMORY_COPY_READ, &memory_access, prev_row);
}

#[test]
fn mem_get_state_at() {
    let mut mem = Memory::default();

    // Write 1 into (ctx = 0, addr = 5) at clk = 1.
    // This means that mem[5] = 1 at the beginning of clk = 2
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(0, Felt::new(5), 1, value1);

    // Write 4 into (ctx = 0, addr = 2) at clk = 2.
    // This means that mem[2] = 4 at the beginning of clk = 3
    let value4 = [Felt::new(4), ZERO, ZERO, ZERO];
    mem.write(0, Felt::new(2), 2, value4);

    // write 7 into (ctx = 3, addr = 3) at clk = 4
    // This means that mem[3] = 7 at the beginning of clk = 4
    let value7 = [Felt::new(7), ZERO, ZERO, ZERO];
    mem.write(3, Felt::new(3), 4, value7);

    // Check memory state at clk = 2
    assert_eq!(mem.get_state_at(0, 2), vec![(5, value1)]);
    assert_eq!(mem.get_state_at(3, 2), vec![]);

    // Check memory state at clk = 3
    assert_eq!(mem.get_state_at(0, 3), vec![(2, value4), (5, value1)]);
    assert_eq!(mem.get_state_at(3, 3), vec![]);

    // Check memory state at clk = 4
    assert_eq!(mem.get_state_at(0, 4), vec![(2, value4), (5, value1)]);
    assert_eq!(mem.get_state_at(3, 4), vec![]);

    // Check memory state at clk = 5
    assert_eq!(mem.get_state_at(0, 5), vec![(2, value4), (5, value1)]);
    assert_eq!(mem.get_state_at(3, 5), vec![(3, value7)]);
}

// HELPER STRUCT & FUNCTIONS
// ================================================================================================

/// Builds a trace of the specified length and fills it with data from the provided Memory instance.
fn build_trace(mem: Memory, num_rows: usize) -> (Vec<Vec<Felt>>, ChipletsBus) {
    let mut chiplets_bus = ChipletsBus::default();
    let mut trace = (0..MEMORY_TRACE_WIDTH).map(|_| vec![Felt::ZERO; num_rows]).collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);
    mem.fill_trace(&mut fragment, &mut chiplets_bus, 0);

    (trace, chiplets_bus)
}

fn read_trace_row(trace: &[Vec<Felt>], step: usize) -> [Felt; MEMORY_TRACE_WIDTH] {
    let mut row = [ZERO; MEMORY_TRACE_WIDTH];
    for (value, column) in row.iter_mut().zip(trace) {
        *value = column[step];
    }
    row
}

fn build_trace_row(
    memory_access: &MemoryLookup,
    op_selectors: Selectors,
    prev_row: [Felt; MEMORY_TRACE_WIDTH],
) -> [Felt; MEMORY_TRACE_WIDTH] {
    let MemoryLookup {
        label: _,
        ctx,
        addr,
        clk,
        word: new_val,
    } = *memory_access;

    let mut row = [ZERO; MEMORY_TRACE_WIDTH];

    row[0] = op_selectors[0];
    row[1] = op_selectors[1];
    row[CTX_COL_IDX] = ctx;
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
    chiplets_bus: &ChipletsBus,
    row: u32,
    op_selectors: Selectors,
    memory_access: &MemoryLookup,
    prev_row: [Felt; MEMORY_TRACE_WIDTH],
) -> [Felt; MEMORY_TRACE_WIDTH] {
    let expected_row = build_trace_row(memory_access, op_selectors, prev_row);
    let expected_lookup = ChipletsLookupRow::Memory(*memory_access);
    let expected_hint = ChipletsLookup::Response(row as usize);

    let lookup = chiplets_bus.get_response_row(row as usize);
    let hint = chiplets_bus.get_lookup_hint(row).unwrap();

    assert_eq!(expected_row, read_trace_row(trace, row as usize));
    assert_eq!(expected_lookup, lookup);
    assert_eq!(&expected_hint, hint);

    expected_row
}
