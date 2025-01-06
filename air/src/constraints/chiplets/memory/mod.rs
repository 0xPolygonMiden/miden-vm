use alloc::vec::Vec;

use winter_air::TransitionConstraintDegree;

use super::{EvaluationFrame, FieldElement};
use crate::{
    trace::chiplets::{
        MEMORY_BATCH_COL_IDX, MEMORY_CLK_COL_IDX, MEMORY_CTX_COL_IDX, MEMORY_D0_COL_IDX,
        MEMORY_D1_COL_IDX, MEMORY_D_INV_COL_IDX, MEMORY_FLAG_SAME_BATCH_AND_CONTEXT,
        MEMORY_IDX0_COL_IDX, MEMORY_IDX1_COL_IDX, MEMORY_IS_READ_COL_IDX,
        MEMORY_IS_WORD_ACCESS_COL_IDX, MEMORY_V_COL_RANGE,
    },
    utils::{binary_not, is_binary, EvaluationResult},
};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// The number of constraints on the management of the memory chiplet.
pub const NUM_CONSTRAINTS: usize = 22;
/// The degrees of constraints on the management of the memory chiplet. All constraint degrees are
/// increased by 3 due to the selectors for the memory chiplet.
pub const CONSTRAINT_DEGREES: [usize; NUM_CONSTRAINTS] = [
    5, 5, 5, 5, // Enforce that rw, ew, idx0 and idx1 are binary.
    7, 6, 9, 8, // Constrain the values in the d inverse column.
    8, // Enforce values in ctx, batch, clk transition correctly.
    7, // Enforce the correct value for the f_scb flag.
    9, 9, 9, 9, // Constrain the values in the first row of the chiplet.
    9, 9, 9, 9, // Constrain the values in non-first rows, new batch or context is started.
    9, 9, 9, 9, // Constrain the values in non-first rows, same batch or context.
];

// MEMORY TRANSITION CONSTRAINTS
// ================================================================================================

/// Builds the transition constraint degrees for the memory chiplet.
pub fn get_transition_constraint_degrees() -> Vec<TransitionConstraintDegree> {
    CONSTRAINT_DEGREES
        .iter()
        .map(|&degree| TransitionConstraintDegree::new(degree))
        .collect()
}

/// Returns the number of transition constraints for the memory chiplet.
pub fn get_transition_constraint_count() -> usize {
    NUM_CONSTRAINTS
}

/// Enforces constraints for the memory chiplet.
pub fn enforce_constraints<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    memory_flag: E,
    memory_flag_no_last: E,
    memory_flag_first_row: E,
) {
    // Constrain the binary columns.
    let mut index = enforce_binary_columns(frame, result, memory_flag);

    // Constrain the values in the d inverse column.
    index += enforce_d_inv(frame, &mut result[index..], memory_flag_no_last);

    // Enforce values in ctx, addr, clk transition correctly.
    index += enforce_delta(frame, &mut result[index..], memory_flag_no_last);

    // Enforce the correct value for the f_scb flag.
    index += enforce_flag_same_context_and_batch(frame, &mut result[index..], memory_flag_no_last);

    // Constrain the memory values.
    enforce_values(frame, &mut result[index..], memory_flag_no_last, memory_flag_first_row);
}

// TRANSITION CONSTRAINT HELPERS
// ================================================================================================

fn enforce_binary_columns<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    memory_flag: E,
) -> usize {
    result[0] = memory_flag * is_binary(frame.is_read());
    result[1] = memory_flag * is_binary(frame.is_word_access());
    result[2] = memory_flag * is_binary(frame.idx0());
    result[3] = memory_flag * is_binary(frame.idx1());

    4
}

// TODO(plafer): review these constraints
/// A constraint evaluation function to enforce that the `d_inv` "delta inverse" column used to
/// constrain the delta between two consecutive contexts, addresses, or clock cycles is updated
/// correctly.
fn enforce_d_inv<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    memory_flag_no_last: E,
) -> usize {
    let constraint_count = 4;

    // n0 is binary
    result[0] = memory_flag_no_last * is_binary(frame.n0());
    // when the context changes, n0 should be set to 1.
    result[1] = memory_flag_no_last * frame.not_n0() * frame.ctx_change();
    // when n0 is 0, n1 is binary.
    result[2] = memory_flag_no_last * frame.not_n0() * is_binary(frame.n1());
    // TODO(plafer)
    result[3] = memory_flag_no_last * frame.not_n0() * frame.not_n1() * frame.addr_change();

    constraint_count
}

/// A constraint evaluation function to enforce that the delta between two consecutive context IDs,
/// addresses, or clock cycles is updated and decomposed into the `d1` and `d0` columns correctly.
fn enforce_delta<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    memory_flag_no_last: E,
) -> usize {
    let constraint_count = 1;

    // If the context changed, include the difference.
    result[0] = memory_flag_no_last * frame.n0() * frame.ctx_change();
    // If the context is the same, include the batch difference if it changed or else include the
    // clock change.
    result.agg_constraint(
        0,
        memory_flag_no_last * frame.not_n0(),
        frame.n1() * frame.addr_change() + frame.not_n1() * frame.clk_change(),
    );
    // Always subtract the delta. It should offset the other changes.
    result[0] -= memory_flag_no_last * frame.delta_next();

    constraint_count
}

/// A constraint evaluation function to enforce that the `f_scb` flag is set to 1 when the next row
/// is in the same context and batch, and 0 otherwise.
fn enforce_flag_same_context_and_batch<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    memory_flag_no_last: E,
) -> usize {
    result[0] = memory_flag_no_last
        * (frame.f_scb_next() - binary_not(frame.n0() + frame.not_n0() * frame.n1()));

    1
}

/// A constraint evaluation function to enforce that memory is initialized to zero when it is read
/// before being written and that when existing memory values are read they remain unchanged.
///
/// The constraints on the values depend on a few factors:
/// - When in the first row of a new context or batch, any of the 4 values of the batch that are not
///   written to must be set to 0.
///   - This is because the memory is initialized to 0 when a new context or batch is started.
/// - When we remain in the same context and batch, then this is when we want to enforce the "memory
///   property" that what was previously written must be read. Therefore, the values that are not
///   being written need to be equal to the values in the previous row (i.e. previously written, or
///   initialized to 0).
///   - The implication is that in a given evaluation frame, we always constrain the "next" value,
///     since that constraint depends on the "current" value.
fn enforce_values<E: FieldElement>(
    frame: &EvaluationFrame<E>,
    result: &mut [E],
    memory_flag_no_last: E,
    memory_flag_first_row: E,
) -> usize {
    // c_i is set to 1 when `v'[i]` is not written to, and 0 otherwise.
    //
    // In other words, c_i is set to 1 when `v'[i]` needs to be constrained (to either 0 or `v[i]`).
    //
    // Note that `c_i` only uses values in the "next" row. This is because it must be used to
    // constrain the first row of the memory chiplet, where that row sits in the "next" position of
    // the frame, and the "current" row belongs to the previous chiplet (and hence the "current" row
    // must not be accessed).
    //
    // As a result, `c_i` does not include the constraint of being in the memory chiplet, or in the
    // same context and batch - these must be enforced separately.
    let (c0, c1, c2, c3) = {
        // intuition: the i'th `f` flag is set to 1 when `i == 2 * idx1 + idx0`
        let f0 = binary_not(frame.idx1_next()) * binary_not(frame.idx0_next());
        let f1 = binary_not(frame.idx1_next()) * frame.idx0_next();
        let f2 = frame.idx1_next() * binary_not(frame.idx0_next());
        let f3 = frame.idx1_next() * frame.idx0_next();

        let c_i = |f_i| {
            // z_i is set to 1 when `v'[i]` is not being accessed.
            let z_i = binary_not(frame.is_word_access_next()) * binary_not(f_i);
            let is_read_next = frame.is_read_next();

            is_read_next + binary_not(is_read_next) * z_i
        };

        (c_i(f0), c_i(f1), c_i(f2), c_i(f3))
    };

    // first row constraints: when row' is the first row, and v'[i] is not written to, then v'[i]
    // must be 0.
    result[0] = memory_flag_first_row * c0 * frame.v_next(0);
    result[1] = memory_flag_first_row * c1 * frame.v_next(1);
    result[2] = memory_flag_first_row * c2 * frame.v_next(2);
    result[3] = memory_flag_first_row * c3 * frame.v_next(3);

    // non-first row, new batch or context constraints: when row' is a new batch/ctx, and v'[i] is
    // not written to, then v'[i] must be 0.
    result[4] = memory_flag_no_last * binary_not(frame.f_scb_next()) * c0 * frame.v_next(0);
    result[5] = memory_flag_no_last * binary_not(frame.f_scb_next()) * c1 * frame.v_next(1);
    result[6] = memory_flag_no_last * binary_not(frame.f_scb_next()) * c2 * frame.v_next(2);
    result[7] = memory_flag_no_last * binary_not(frame.f_scb_next()) * c3 * frame.v_next(3);

    // non-first row, same batch or context constraints: when row' is in the same batch/ctx, and
    // v'[i] is not written to, then v'[i] must be equal to v[i].
    result[8] = memory_flag_no_last * frame.f_scb_next() * c0 * (frame.v_next(0) - frame.v(0));
    result[9] = memory_flag_no_last * frame.f_scb_next() * c1 * (frame.v_next(1) - frame.v(1));
    result[10] = memory_flag_no_last * frame.f_scb_next() * c2 * (frame.v_next(2) - frame.v(2));
    result[11] = memory_flag_no_last * frame.f_scb_next() * c3 * (frame.v_next(3) - frame.v(3));

    12
}

// MEMORY FRAME EXTENSION TRAIT
// ================================================================================================

/// Trait to allow easy access to column values and intermediate variables used in constraint
/// calculations for the Memory chiplet.
trait EvaluationFrameExt<E: FieldElement> {
    // --- Column accessors -----------------------------------------------------------------------

    /// The value of the read/write column in the current row.
    ///
    /// 0: write, 1: read
    fn is_read(&self) -> E;
    /// The value of the read/write column in the next row.
    ///
    /// 0: write, 1: read
    fn is_read_next(&self) -> E;
    /// The value of the element/word column in the current row.
    ///
    /// 0: element, 1: word
    fn is_word_access(&self) -> E;
    /// The value of the element/word column in the next row.
    ///
    /// 0: element, 1: word
    fn is_word_access_next(&self) -> E;
    /// The current context value.
    #[allow(dead_code)]
    fn ctx(&self) -> E;
    /// The current address.
    #[allow(dead_code)]
    fn batch(&self) -> E;
    /// The 0'th bit of the index of the memory address in the current batch.
    fn idx0(&self) -> E;
    /// The 0'th bit of the index of the memory address in the next batch.
    fn idx0_next(&self) -> E;
    /// The 1st bit of the index of the memory address in the current batch.
    fn idx1(&self) -> E;
    /// The 1st bit of the index of the memory address in the next batch.
    fn idx1_next(&self) -> E;
    /// The current clock cycle.
    #[allow(dead_code)]
    fn clk(&self) -> E;
    /// The next clock cycle.
    #[allow(dead_code)]
    fn clk_next(&self) -> E;
    /// The value from the specified index of the values (0, 1, 2, 3) in the current row.
    fn v(&self, index: usize) -> E;
    /// The value from the specified index of the values (0, 1, 2, 3) in the next row.
    fn v_next(&self, index: usize) -> E;
    /// The next value of the lower 16-bits of the delta value being tracked between two consecutive
    /// context IDs, addresses, or clock cycles.
    fn d0_next(&self) -> E;
    /// The next value of the upper 16-bits of the delta value being tracked between two consecutive
    /// context IDs, addresses, or clock cycles.
    fn d1_next(&self) -> E;
    /// The next value of the column tracking the inverse delta used for constraint evaluations.
    fn d_inv_next(&self) -> E;

    // The flag that indicates whether the next row is in the same batch and context as the current
    // row.
    fn f_scb_next(&self) -> E;

    // --- Intermediate variables & helpers -------------------------------------------------------

    /// The change between the current value in the specified column and the next value, calculated
    /// as `next - current`.
    fn change(&self, column: usize) -> E;
    /// An intermediate variable to help constrain context change updates in the delta inverse
    /// column.
    fn n0(&self) -> E;
    /// `1 - n0`
    fn not_n0(&self) -> E;
    /// An intermediate variable to help constrain address changes in the delta inverse column when
    /// the context doesn't change.
    fn n1(&self) -> E;
    /// `1 - n1`
    fn not_n1(&self) -> E;
    /// The difference between the next context and the current context.
    fn ctx_change(&self) -> E;
    /// The difference between the next address and the current address.
    fn addr_change(&self) -> E;
    /// The difference between the next clock value and the current one, minus 1.
    fn clk_change(&self) -> E;
    /// The delta between two consecutive context IDs, addresses, or clock cycles.
    fn delta_next(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Column accessors -----------------------------------------------------------------------

    #[inline(always)]
    fn is_read(&self) -> E {
        self.current()[MEMORY_IS_READ_COL_IDX]
    }

    #[inline(always)]
    fn is_read_next(&self) -> E {
        self.next()[MEMORY_IS_READ_COL_IDX]
    }

    #[inline(always)]
    fn is_word_access(&self) -> E {
        self.current()[MEMORY_IS_WORD_ACCESS_COL_IDX]
    }

    #[inline(always)]
    fn is_word_access_next(&self) -> E {
        self.next()[MEMORY_IS_WORD_ACCESS_COL_IDX]
    }

    #[inline(always)]
    fn ctx(&self) -> E {
        self.current()[MEMORY_CTX_COL_IDX]
    }

    #[inline(always)]
    fn batch(&self) -> E {
        self.next()[MEMORY_BATCH_COL_IDX]
    }

    #[inline(always)]
    fn idx0(&self) -> E {
        self.current()[MEMORY_IDX0_COL_IDX]
    }

    #[inline(always)]
    fn idx0_next(&self) -> E {
        self.next()[MEMORY_IDX0_COL_IDX]
    }

    #[inline(always)]
    fn idx1(&self) -> E {
        self.current()[MEMORY_IDX1_COL_IDX]
    }

    #[inline(always)]
    fn idx1_next(&self) -> E {
        self.next()[MEMORY_IDX1_COL_IDX]
    }

    #[inline(always)]
    fn clk(&self) -> E {
        self.current()[MEMORY_CLK_COL_IDX]
    }

    #[inline(always)]
    fn clk_next(&self) -> E {
        self.next()[MEMORY_CLK_COL_IDX]
    }

    #[inline(always)]
    fn v(&self, index: usize) -> E {
        self.current()[MEMORY_V_COL_RANGE.start + index]
    }

    #[inline(always)]
    fn v_next(&self, index: usize) -> E {
        self.next()[MEMORY_V_COL_RANGE.start + index]
    }

    #[inline(always)]
    fn d0_next(&self) -> E {
        self.next()[MEMORY_D0_COL_IDX]
    }

    #[inline(always)]
    fn d1_next(&self) -> E {
        self.next()[MEMORY_D1_COL_IDX]
    }

    #[inline(always)]
    fn d_inv_next(&self) -> E {
        self.next()[MEMORY_D_INV_COL_IDX]
    }

    #[inline(always)]
    fn f_scb_next(&self) -> E {
        self.next()[MEMORY_FLAG_SAME_BATCH_AND_CONTEXT]
    }

    // --- Intermediate variables & helpers -------------------------------------------------------

    #[inline(always)]
    fn change(&self, column: usize) -> E {
        self.next()[column] - self.current()[column]
    }

    #[inline(always)]
    fn n0(&self) -> E {
        self.change(MEMORY_CTX_COL_IDX) * self.d_inv_next()
    }

    #[inline(always)]
    fn not_n0(&self) -> E {
        binary_not(self.n0())
    }

    #[inline(always)]
    fn n1(&self) -> E {
        self.change(MEMORY_BATCH_COL_IDX) * self.d_inv_next()
    }

    #[inline(always)]
    fn not_n1(&self) -> E {
        binary_not(self.n1())
    }

    #[inline(always)]
    fn ctx_change(&self) -> E {
        self.change(MEMORY_CTX_COL_IDX)
    }

    #[inline(always)]
    fn addr_change(&self) -> E {
        self.change(MEMORY_BATCH_COL_IDX)
    }

    #[inline(always)]
    fn clk_change(&self) -> E {
        self.change(MEMORY_CLK_COL_IDX) - E::ONE
    }

    #[inline(always)]
    fn delta_next(&self) -> E {
        E::from(2_u32.pow(16)) * self.d1_next() + self.d0_next()
    }
}
