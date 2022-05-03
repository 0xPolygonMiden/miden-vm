use super::{Felt, FieldElement, Memory, StarkField, TraceFragment, Word};

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
    assert_eq!([Felt::ZERO; 4], value);
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // read a value from address 3; clk = 2
    mem.advance_clock();
    let addr3 = Felt::new(3);
    let value = mem.read(addr3);
    assert_eq!([Felt::ZERO; 4], value);
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // read a value from address 0 again; clk = 3
    mem.advance_clock();
    let value = mem.read(addr0);
    assert_eq!([Felt::ZERO; 4], value);
    assert_eq!(2, mem.size());
    assert_eq!(3, mem.trace_len());

    // read a value from address 2; clk = 4
    mem.advance_clock();
    let addr2 = Felt::new(2);
    let value = mem.read(addr2);
    assert_eq!([Felt::ZERO; 4], value);
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // check generated trace; rows should be sorted by address and then clock cycle
    let num_rows = 4;
    let mut trace = (0..14)
        .map(|_| vec![Felt::ZERO; num_rows])
        .collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);

    mem.fill_trace(&mut fragment);

    // address 0
    let expected = build_trace_row(addr0, 1, [Felt::ZERO; 4], [Felt::ZERO; 4], [Felt::ZERO; 14]);
    assert_eq!(expected, read_trace_row(&trace, 0));
    let expected = build_trace_row(addr0, 3, [Felt::ZERO; 4], [Felt::ZERO; 4], expected);
    assert_eq!(expected, read_trace_row(&trace, 1));

    // address 2
    let expected = build_trace_row(addr2, 4, [Felt::ZERO; 4], [Felt::ZERO; 4], expected);
    assert_eq!(expected, read_trace_row(&trace, 2));

    // address 3
    let expected = build_trace_row(addr3, 2, [Felt::ZERO; 4], [Felt::ZERO; 4], expected);
    assert_eq!(expected, read_trace_row(&trace, 3));
}

#[test]
fn mem_write() {
    let mut mem = Memory::new();

    // write a value into address 0; clk = 1
    mem.advance_clock();
    let addr0 = Felt::new(0);
    let value1 = [Felt::ONE, Felt::ZERO, Felt::ZERO, Felt::ZERO];
    mem.write(addr0, value1);
    assert_eq!(value1, mem.get_value(addr0.as_int()).unwrap());
    assert_eq!(1, mem.size());
    assert_eq!(1, mem.trace_len());

    // write a value into address 2; clk = 2
    mem.advance_clock();
    let addr2 = Felt::new(2);
    let value5 = [Felt::new(5), Felt::ZERO, Felt::ZERO, Felt::ZERO];
    mem.write(addr2, value5);
    assert_eq!(value5, mem.get_value(addr2.as_int()).unwrap());
    assert_eq!(2, mem.size());
    assert_eq!(2, mem.trace_len());

    // write a value into address 1; clk = 3
    mem.advance_clock();
    let addr1 = Felt::new(1);
    let value7 = [Felt::new(7), Felt::ZERO, Felt::ZERO, Felt::ZERO];
    mem.write(addr1, value7);
    assert_eq!(value7, mem.get_value(addr1.as_int()).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(3, mem.trace_len());

    // write a value into address 0; clk = 4
    mem.advance_clock();
    let value9 = [Felt::new(9), Felt::ZERO, Felt::ZERO, Felt::ZERO];
    mem.write(addr0, value9);
    assert_eq!(value7, mem.get_value(addr1.as_int()).unwrap());
    assert_eq!(3, mem.size());
    assert_eq!(4, mem.trace_len());

    // check generated trace; rows should be sorted by address and then clock cycle
    let num_rows = 4;
    let mut trace = (0..14)
        .map(|_| vec![Felt::ZERO; num_rows])
        .collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);

    mem.fill_trace(&mut fragment);

    // address 0
    let expected = build_trace_row(addr0, 1, [Felt::ZERO; 4], value1, [Felt::ZERO; 14]);
    assert_eq!(expected, read_trace_row(&trace, 0));
    let expected = build_trace_row(addr0, 4, value1, value9, expected);
    assert_eq!(expected, read_trace_row(&trace, 1));

    // address 1
    let expected = build_trace_row(addr1, 3, [Felt::ZERO; 4], value7, expected);
    assert_eq!(expected, read_trace_row(&trace, 2));

    // address 2
    let expected = build_trace_row(addr2, 2, [Felt::ZERO; 4], value5, expected);
    assert_eq!(expected, read_trace_row(&trace, 3));
}

#[test]
fn mem_write_read() {
    let mut mem = Memory::new();

    // write 1 into address 5; clk = 1
    mem.advance_clock();
    let addr5 = Felt::new(5);
    let value1 = [Felt::ONE, Felt::ZERO, Felt::ZERO, Felt::ZERO];
    mem.write(addr5, value1);

    // write 4 into address 2; clk = 2
    mem.advance_clock();
    let addr2 = Felt::new(2);
    let value4 = [Felt::new(4), Felt::ZERO, Felt::ZERO, Felt::ZERO];
    mem.write(addr2, value4);

    // read a value from address 5; clk = 3
    mem.advance_clock();
    let _ = mem.read(addr5);

    // write 2 into address 5; clk = 4
    mem.advance_clock();
    let value2 = [Felt::new(2), Felt::ZERO, Felt::ZERO, Felt::ZERO];
    mem.write(addr5, value2);

    // read a value from address 2; clk = 5
    mem.advance_clock();
    let _ = mem.read(addr2);

    // write 7 into address 2; clk = 6
    mem.advance_clock();
    let value7 = [Felt::new(7), Felt::ZERO, Felt::ZERO, Felt::ZERO];
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
    let num_rows = 9;
    let mut trace = (0..14)
        .map(|_| vec![Felt::ZERO; num_rows])
        .collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);

    mem.fill_trace(&mut fragment);

    // address 2
    let expected = build_trace_row(addr2, 2, [Felt::ZERO; 4], value4, [Felt::ZERO; 14]);
    assert_eq!(expected, read_trace_row(&trace, 0));
    let expected = build_trace_row(addr2, 5, value4, value4, expected);
    assert_eq!(expected, read_trace_row(&trace, 1));
    let expected = build_trace_row(addr2, 6, value4, value7, expected);
    assert_eq!(expected, read_trace_row(&trace, 2));
    let expected = build_trace_row(addr2, 8, value7, value7, expected);
    assert_eq!(expected, read_trace_row(&trace, 3));

    // address 5
    let expected = build_trace_row(addr5, 1, [Felt::ZERO; 4], value1, expected);
    assert_eq!(expected, read_trace_row(&trace, 4));
    let expected = build_trace_row(addr5, 3, value1, value1, expected);
    assert_eq!(expected, read_trace_row(&trace, 5));
    let expected = build_trace_row(addr5, 4, value1, value2, expected);
    assert_eq!(expected, read_trace_row(&trace, 6));
    let expected = build_trace_row(addr5, 7, value2, value2, expected);
    assert_eq!(expected, read_trace_row(&trace, 7));
    let expected = build_trace_row(addr5, 9, value2, value2, expected);
    assert_eq!(expected, read_trace_row(&trace, 8));
}

#[test]
fn mem_get_values_at() {
    let mut mem = Memory::new();

    // Write 1 into address 5 at clk = 1.
    // This means that mem[5] = 1 at the beginning of `clk=2`
    mem.advance_clock();
    let addr5 = Felt::new(5);
    let value1 = [Felt::ONE, Felt::ZERO, Felt::ZERO, Felt::ZERO];
    mem.write(addr5, value1);

    // Write 4 into address 2 at clk = 2.
    // This means that mem[4] = 2 at the beginning of `clk=3`
    mem.advance_clock();
    let addr2 = Felt::new(2);
    let value4 = [Felt::new(4), Felt::ZERO, Felt::ZERO, Felt::ZERO];
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

fn read_trace_row(trace: &[Vec<Felt>], step: usize) -> [Felt; 14] {
    let mut row = [Felt::ZERO; 14];
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
    prev_row: [Felt; 14],
) -> [Felt; 14] {
    let mut row = [Felt::ZERO; 14];
    row[0] = Felt::ZERO; // ctx
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

    if prev_row != [Felt::ZERO; 14] {
        let delta = if row[0] != prev_row[0] {
            row[0] - prev_row[0]
        } else if row[1] != prev_row[1] {
            row[1] - prev_row[1]
        } else {
            row[2] - prev_row[2] - Felt::ONE
        };

        let (hi, lo) = super::split_u32_into_u16(delta);
        row[11] = lo;
        row[12] = hi;
        row[13] = delta.inv();
    }

    row
}
