use super::{Felt, FieldElement, Memory, TraceFragment, Word};

#[test]
fn mem_init() {
    let mem = Memory::new();
    assert_eq!(0, mem.size());
    assert_eq!(0, mem.trace_len());
}

#[test]
fn mem_read() {
    let mut mem = Memory::new();
    mem.advance_clock();

    // read a value from address 0; clk = 1
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
