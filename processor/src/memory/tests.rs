use super::{Felt, FieldElement, Memory, StarkField, TraceFragment, Word, ONE, ZERO};
use vm_core::MEMORY_TRACE_WIDTH;

#[test]
fn mem_init() {
    let mem = Memory::new();
    assert_eq!(0, mem.size());
    assert_eq!(0, mem.trace_len());
}

#[test]
fn mem_read() {
    let mut mem = Memory::new();

    // read a value from address 0; clk = 1
    mem.advance_clock();
    let addr0 = Felt::new(0);
    let value = mem.read(addr0);
    assert_eq!([ZERO; 4], value);
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // read a value from address 3; clk = 2
    mem.advance_clock();
    let addr3 = Felt::new(3);
    let value = mem.read(addr3);
    assert_eq!([ZERO; 4], value);
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // read a value from address 0 again; clk = 3
    mem.advance_clock();
    let value = mem.read(addr0);
    assert_eq!([ZERO; 4], value);
    assert_eq!(2, mem.size());
    assert_eq!(3, mem.trace_len());

    // read a value from address 2; clk = 4
    mem.advance_clock();
    let addr2 = Felt::new(2);
    let value = mem.read(addr2);
    assert_eq!([ZERO; 4], value);
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // check generated trace; rows should be sorted by address and then clock cycle
    let trace = build_trace(mem, 4);

    // address 0
    let expected = build_trace_row(addr0, 1, [ZERO; 4], [ZERO; 4], [ZERO; MEMORY_TRACE_WIDTH]);
    let mut expected_deltas = vec![0]; // clk delta: 1 - 0 - 1
    assert_eq!(expected, read_trace_row(&trace, 0));

    let expected = build_trace_row(addr0, 3, [ZERO; 4], [ZERO; 4], expected);
    expected_deltas.push(1); // clk delta: 3 - 1 - 1
    assert_eq!(expected, read_trace_row(&trace, 1));

    // address 2
    let expected = build_trace_row(addr2, 4, [ZERO; 4], [ZERO; 4], expected);
    expected_deltas.push(2); // addr delta: addr2 - addr0
    assert_eq!(expected, read_trace_row(&trace, 2));

    // address 3
    let expected = build_trace_row(addr3, 2, [ZERO; 4], [ZERO; 4], expected);
    expected_deltas.push(1); // addr delta: addr3 - addr2
    assert_eq!(expected, read_trace_row(&trace, 3));
}

#[test]
fn mem_write() {
    let mut mem = Memory::new();

    // write a value into address 0; clk = 1
    mem.advance_clock();
    let addr0 = Felt::new(0);
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(addr0, value1);
    assert_eq!(value1, mem.get_value(addr0.as_int()).unwrap());
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // write a value into address 2; clk = 2
    mem.advance_clock();
    let addr2 = Felt::new(2);
    let value5 = [Felt::new(5), ZERO, ZERO, ZERO];
    mem.write(addr2, value5);
    assert_eq!(value5, mem.get_value(addr2.as_int()).unwrap());
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // write a value into address 1; clk = 3
    mem.advance_clock();
    let addr1 = Felt::new(1);
    let value7 = [Felt::new(7), ZERO, ZERO, ZERO];
    mem.write(addr1, value7);
    assert_eq!(value7, mem.get_value(addr1.as_int()).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(3, mem.trace_len());

    // write a value into address 0; clk = 4
    mem.advance_clock();
    let value9 = [Felt::new(9), ZERO, ZERO, ZERO];
    mem.write(addr0, value9);
    assert_eq!(value7, mem.get_value(addr1.as_int()).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // check generated trace; rows should be sorted by address and then clock cycle
    let trace = build_trace(mem, 4);

    // address 0
    let expected = build_trace_row(addr0, 1, [ZERO; 4], value1, [ZERO; MEMORY_TRACE_WIDTH]);
    let mut expected_deltas = vec![0]; // clk delta: 1 - 0 - 1
    assert_eq!(expected, read_trace_row(&trace, 0));

    let expected = build_trace_row(addr0, 4, value1, value9, expected);
    expected_deltas.push(2); // clk delta: 4 - 1 - 1
    assert_eq!(expected, read_trace_row(&trace, 1));

    // address 1
    let expected = build_trace_row(addr1, 3, [ZERO; 4], value7, expected);
    expected_deltas.push(1); // addr delta: addr1 - addr0
    assert_eq!(expected, read_trace_row(&trace, 2));

    // address 2
    let expected = build_trace_row(addr2, 2, [ZERO; 4], value5, expected);
    expected_deltas.push(1); // addr delta: addr2 - addr1
    assert_eq!(expected, read_trace_row(&trace, 3));
}

#[test]
fn mem_write_read() {
    let mut mem = Memory::new();

    // write 1 into address 5; clk = 1
    mem.advance_clock();
    let addr5 = Felt::new(5);
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(addr5, value1);

    // write 4 into address 2; clk = 2
    mem.advance_clock();
    let addr2 = Felt::new(2);
    let value4 = [Felt::new(4), ZERO, ZERO, ZERO];
    mem.write(addr2, value4);

    // read a value from address 5; clk = 3
    mem.advance_clock();
    let _ = mem.read(addr5);

    // write 2 into address 5; clk = 4
    mem.advance_clock();
    let value2 = [Felt::new(2), ZERO, ZERO, ZERO];
    mem.write(addr5, value2);

    // read a value from address 2; clk = 5
    mem.advance_clock();
    let _ = mem.read(addr2);

    // write 7 into address 2; clk = 6
    mem.advance_clock();
    let value7 = [Felt::new(7), ZERO, ZERO, ZERO];
    mem.write(addr2, value7);

    // read a value from address 5; clk = 7
    mem.advance_clock();
    let _ = mem.read(addr5);

    // read a value from address 2; clk = 8
    mem.advance_clock();
    let _ = mem.read(addr2);

    // read a value from address 5; clk = 9
    mem.advance_clock();
    let _ = mem.read(addr5);

    // check generated trace; rows should be sorted by address and then clock cycle
    let trace = build_trace(mem, 9);

    // address 2
    let expected = build_trace_row(addr2, 2, [ZERO; 4], value4, [ZERO; MEMORY_TRACE_WIDTH]);
    let mut expected_deltas = vec![0]; // clk delta: 2 - 1 - 1
    assert_eq!(expected, read_trace_row(&trace, 0));

    let expected = build_trace_row(addr2, 5, value4, value4, expected);
    expected_deltas.push(2); // clk delta: 5 - 2 - 1
    assert_eq!(expected, read_trace_row(&trace, 1));

    let expected = build_trace_row(addr2, 6, value4, value7, expected);
    expected_deltas.push(0); // clk delta: 6 - 5 - 1
    assert_eq!(expected, read_trace_row(&trace, 2));

    let expected = build_trace_row(addr2, 8, value7, value7, expected);
    expected_deltas.push(1); // clk delta: 8 - 6 - 1
    assert_eq!(expected, read_trace_row(&trace, 3));

    // address 5
    let expected = build_trace_row(addr5, 1, [ZERO; 4], value1, expected);
    expected_deltas.push(3); // addr delta: 5 - 2
    assert_eq!(expected, read_trace_row(&trace, 4));

    let expected = build_trace_row(addr5, 3, value1, value1, expected);
    expected_deltas.push(1); // clk delta: 3 - 1 - 1
    assert_eq!(expected, read_trace_row(&trace, 5));

    let expected = build_trace_row(addr5, 4, value1, value2, expected);
    expected_deltas.push(0); // clk delta: 4 - 3 - 1
    assert_eq!(expected, read_trace_row(&trace, 6));

    let expected = build_trace_row(addr5, 7, value2, value2, expected);
    expected_deltas.push(2); // clk delta: 7 - 4 - 1
    assert_eq!(expected, read_trace_row(&trace, 7));

    let expected = build_trace_row(addr5, 9, value2, value2, expected);
    expected_deltas.push(1); // clk delta: 9 - 7 - 1
    assert_eq!(expected, read_trace_row(&trace, 8));
}

#[test]
fn mem_get_values_at() {
    let mut mem = Memory::new();

    // Write 1 into address 5 at clk = 1.
    // This means that mem[5] = 1 at the beginning of `clk=2`
    mem.advance_clock();
    let addr5 = Felt::new(5);
    let value1 = [ONE, ZERO, ZERO, ZERO];
    mem.write(addr5, value1);

    // Write 4 into address 2 at clk = 2.
    // This means that mem[4] = 2 at the beginning of `clk=3`
    mem.advance_clock();
    let addr2 = Felt::new(2);
    let value4 = [Felt::new(4), ZERO, ZERO, ZERO];
    mem.write(addr2, value4);

    mem.advance_clock();
    // Check that mem[5] == 1 at clk = 2.
    assert_eq!(vec![(5_u64, value1)], mem.get_values_at(0..=5, 2));

    // Check that mem[5] == 1 and mem[4] == 2 at clk = 3 and 4
    assert_eq!(
        vec![(2_u64, value4), (5_u64, value1)],
        mem.get_values_at(0..=5, 3)
    );
    assert_eq!(
        vec![(2_u64, value4), (5_u64, value1)],
        mem.get_values_at(0..=5, 4)
    );

    // Check that range works as expected
    assert_eq!(vec![(2_u64, value4)], mem.get_values_at(0..=4, 4));
}

// HELPER FUNCTIONS
// ================================================================================================

/// Builds a trace of the specified length and fills it with data from the provided Memory instance.
fn build_trace(mem: Memory, num_rows: usize) -> Vec<Vec<Felt>> {
    let mut trace = (0..MEMORY_TRACE_WIDTH)
        .map(|_| vec![Felt::ZERO; num_rows])
        .collect::<Vec<_>>();
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
    addr: Felt,
    clk: u64,
    old_val: Word,
    new_val: Word,
    prev_row: [Felt; MEMORY_TRACE_WIDTH],
) -> [Felt; MEMORY_TRACE_WIDTH] {
    let mut row = [ZERO; MEMORY_TRACE_WIDTH];
    row[0] = ZERO; // ctx
    row[1] = addr;
    row[2] = Felt::new(clk);
    row[3] = old_val[0];
    row[4] = old_val[1];
    row[5] = old_val[2];
    row[6] = old_val[3];
    row[7] = new_val[0];
    row[8] = new_val[1];
    row[9] = new_val[2];
    row[10] = new_val[3];

    if prev_row != [ZERO; MEMORY_TRACE_WIDTH] {
        let delta = if row[0] != prev_row[0] {
            row[0] - prev_row[0]
        } else if row[1] != prev_row[1] {
            row[1] - prev_row[1]
        } else {
            row[2] - prev_row[2] - ONE
        };

        let (hi, lo) = super::split_element_u32_into_u16(delta);
        row[11] = lo;
        row[12] = hi;
        row[13] = delta.inv();
    }

    row
}
