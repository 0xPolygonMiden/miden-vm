use super::{Felt, FieldElement, StarkField};
use winter_utils::{collections::BTreeMap, uninit_vector};

#[cfg(test)]
mod tests;

// RANGE CHECKER
// ================================================================================================

/// TODO: add docs
#[allow(dead_code)]
pub struct RangeChecker {
    /// Tracks lookup count for each checked value.
    lookups: BTreeMap<u16, usize>,
}

#[allow(dead_code)]
impl RangeChecker {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add docs
    pub fn new() -> Self {
        let mut lookups = BTreeMap::new();
        lookups.insert(0, 0);
        lookups.insert(u16::MAX, 0);
        Self { lookups }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn trace_len(&self) -> usize {
        let (lookups_8bit, num_16bit_rows) = self.build_8bit_lookup();

        let mut result = num_16bit_rows;

        for (_, &num_lookups) in lookups_8bit.iter() {
            if num_lookups == 0 {
                result += 1;
            } else {
                let (num_rows4, num_lookups) = div_rem(num_lookups, 4);
                let (num_rows2, num_rows1) = div_rem(num_lookups, 2);
                result += num_rows4 + num_rows2 + num_rows1;
            }
        }

        result
    }

    // TRACE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn check(&mut self, value: Felt) {
        let value = value.as_int() as u16;

        self.lookups
            .entry(value)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    // EXECUTION TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn build_trace(self, target_length: usize) -> Vec<Vec<Felt>> {
        let mut trace = unsafe {
            vec![
                uninit_vector(target_length),
                uninit_vector(target_length),
                uninit_vector(target_length),
                uninit_vector(target_length),
            ]
        };

        let (lookups_8bit, _) = self.build_8bit_lookup();

        let mut i = 0;

        // build the 8-bit segment of the trace table
        for (value, num_lookups) in lookups_8bit {
            write_value(&mut trace, &mut i, num_lookups, value as u64);
        }

        // fill in the first column to indicate where the 8-bit segment ends and where the
        // 16-bit segment begins
        trace[0][..i].fill(Felt::ZERO);
        trace[0][i..].fill(Felt::ONE);

        // build the 16-bit segment of the trace table
        let mut prev_value = 0u16;
        for (&value, &num_lookups) in self.lookups.iter() {
            for value in (prev_value..value).step_by(255).skip(1) {
                write_value(&mut trace, &mut i, 0, value as u64);
            }
            write_value(&mut trace, &mut i, num_lookups, value as u64);
            prev_value = value;
        }

        trace[1][i..].fill(Felt::ZERO);
        trace[2][i..].fill(Felt::ZERO);
        trace[3][i..].fill(Felt::ZERO);

        trace
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    fn build_8bit_lookup(&self) -> (BTreeMap<u8, usize>, usize) {
        let mut result = BTreeMap::new();
        for i in 0u16..256 {
            result.insert(i as u8, 0usize);
        }

        let prev_value = 0u16;

        let mut num_16bit_rows = 0;

        for (&value, &num_lookups) in self.lookups.iter() {
            if num_lookups == 0 {
                num_16bit_rows += 1;
            } else {
                let (num_rows4, num_lookups) = div_rem(num_lookups, 4);
                let (num_rows2, num_rows1) = div_rem(num_lookups, 2);
                let num_rows = num_rows4 + num_rows2 + num_rows1;

                result.entry(0).and_modify(|count| *count += num_rows - 1);
                num_16bit_rows += num_rows;
            }

            let delta = value - prev_value;
            let (delta_q, delta_r) = div_rem(delta as usize, 255);
            num_16bit_rows += delta_q;

            if delta_q != 0 {
                result.entry(255).and_modify(|count| *count += delta_q);
            }
            if delta_r != 0 {
                result.entry(delta_r as u8).and_modify(|count| *count += 1);
            }
        }

        (result, num_16bit_rows)
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn div_rem(value: usize, divisor: usize) -> (usize, usize) {
    let q = value / divisor;
    let r = value % divisor;
    (q, r)
}

fn write_value(trace: &mut [Vec<Felt>], step: &mut usize, num_lookups: usize, value: u64) {
    if num_lookups == 0 {
        write_trace_row(trace, step, Felt::ZERO, Felt::ZERO, value as u64);
        return;
    }

    let (num_rows, num_lookups) = div_rem(num_lookups, 4);
    for _ in 0..num_rows {
        write_trace_row(trace, step, Felt::ONE, Felt::ONE, value as u64);
    }

    let (num_rows, num_lookups) = div_rem(num_lookups, 2);
    for _ in 0..num_rows {
        write_trace_row(trace, step, Felt::ZERO, Felt::ONE, value as u64);
    }

    for _ in 0..num_lookups {
        write_trace_row(trace, step, Felt::ONE, Felt::ZERO, value as u64);
    }
}

fn write_trace_row(trace: &mut [Vec<Felt>], step: &mut usize, s0: Felt, s1: Felt, value: u64) {
    trace[1][*step] = s0;
    trace[2][*step] = s1;
    trace[3][*step] = Felt::new(value);
    *step += 1;
}
