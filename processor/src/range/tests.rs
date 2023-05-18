use super::{BTreeMap, Felt, RangeChecker, Vec, ONE, ZERO};
use crate::{utils::get_trace_len, RangeCheckTrace};
use rand_utils::rand_array;
use vm_core::{utils::ToElements, StarkField};

#[test]
fn range_checks() {
    let mut checker = RangeChecker::new();

    let values = [0, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 100, 355, 620].to_elements();

    for &value in values.iter() {
        checker.add_value(value.as_int() as u16)
    }

    let RangeCheckTrace {
        trace,
        aux_builder: _,
    } = checker.into_trace(16384, 0);
    validate_trace(&trace, &values);

    // skip padded rows
    let mut i = 0;
    while trace[0][i] == ZERO && trace[1][i] == ZERO && trace[2][i] == ZERO {
        i += 1;
    }

    // make sure the values are arranged as expected
    validate_row(&trace, &mut i, 0, 1);
    validate_row(&trace, &mut i, 1, 1);
    validate_row(&trace, &mut i, 2, 4);
    validate_row(&trace, &mut i, 3, 2);
    validate_row(&trace, &mut i, 3, 1);
    validate_row(&trace, &mut i, 4, 2);

    for n in (4..100).step_by(8).skip(1) {
        validate_row(&trace, &mut i, n, 0);
    }

    validate_row(&trace, &mut i, 100, 1);

    for n in (100..355).step_by(8).skip(1) {
        validate_row(&trace, &mut i, n, 0);
    }

    validate_row(&trace, &mut i, 355, 1);

    for n in (355..620).step_by(8).skip(1) {
        validate_row(&trace, &mut i, n, 0);
    }

    validate_row(&trace, &mut i, 620, 1);
}

#[test]
fn range_checks_rand() {
    let mut checker = RangeChecker::new();
    let values = rand_array::<u64, 300>();
    let values = values.into_iter().map(|v| Felt::new(v as u16 as u64)).collect::<Vec<_>>();

    for &value in values.iter() {
        checker.add_value(value.as_int() as u16);
    }

    let trace_len = checker.trace_len().next_power_of_two();
    let RangeCheckTrace {
        trace,
        aux_builder: _,
    } = checker.into_trace(trace_len, 0);
    validate_trace(&trace, &values);
}

// HELPER FUNCTIONS
// ================================================================================================

fn validate_row(trace: &[Vec<Felt>], row_idx: &mut usize, value: u64, num_lookups: u64) {
    let (s0, s1) = match num_lookups {
        0 => (ZERO, ZERO),
        1 => (ONE, ZERO),
        2 => (ZERO, ONE),
        4 => (ONE, ONE),
        _ => panic!("invalid lookup value"),
    };
    assert_eq!(s0, trace[0][*row_idx]);
    assert_eq!(s1, trace[1][*row_idx]);
    assert_eq!(Felt::new(value), trace[2][*row_idx]);
    *row_idx += 1;
}

fn validate_trace(trace: &[Vec<Felt>], lookups: &[Felt]) {
    assert_eq!(3, trace.len());

    // trace length must be a power of two
    let trace_len = get_trace_len(trace);
    assert!(trace_len.is_power_of_two());

    // --- validate the trace ---------------------------
    let mut i = 0;
    let mut lookups_16bit = BTreeMap::new();

    // process the first row
    assert_eq!(ZERO, trace[2][i]);
    let count = get_lookup_count(trace, i);
    lookups_16bit.insert(0u16, count);
    i += 1;

    // process all other rows
    let mut prev_value = 0u16;
    while i < trace_len {
        // make sure the value is a 16-bit value
        let value = trace[2][i].as_int();
        assert!(value <= 65535, "not a 16-bit value");
        let value = value as u16;

        // make sure the delta between this and the previous value is less than 8
        let delta = value - prev_value;
        assert!(delta <= 8);

        // keep track of lookup count for each value
        let count = get_lookup_count(trace, i);
        lookups_16bit.entry(value).and_modify(|value| *value += count).or_insert(count);

        i += 1;
        prev_value = value;
    }

    // validate the last row (must be 65535)
    let last_value = trace[2][i - 1].as_int();
    assert_eq!(65535, last_value);

    // remove all the looked up values from the lookup table
    for value in lookups {
        let value = value.as_int();
        assert!(value <= 65535, "not a 16-bit value");
        let value = value as u16;

        assert!(lookups_16bit.contains_key(&value));
        lookups_16bit.entry(value).and_modify(|count| {
            assert!(*count > 0);
            *count -= 1;
        });
    }

    // make sure 16-bit table is empty
    for &value in lookups_16bit.values() {
        assert_eq!(0, value);
    }
}

fn get_lookup_count(trace: &[Vec<Felt>], step: usize) -> usize {
    if trace[0][step] == ZERO && trace[1][step] == ZERO {
        0
    } else if trace[0][step] == ONE && trace[1][step] == ZERO {
        1
    } else if trace[0][step] == ZERO && trace[1][step] == ONE {
        2
    } else if trace[0][step] == ONE && trace[1][step] == ONE {
        4
    } else {
        panic!("not a valid count");
    }
}
