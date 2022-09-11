use super::EvaluationFrame;
use crate::utils::binary_not;
use vm_core::{
    decoder::{NUM_OP_BITS, OP_BITS_RANGE, OP_BIT_EXTRA_COL_IDX},
    Felt, FieldElement, Operation, DECODER_TRACE_OFFSET, ONE, TRACE_WIDTH, ZERO,
};

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ==================================================================================================

/// Total number of degree 7 operations in the VM.
pub const NUM_DEGREE_7_OPS: usize = 64;

/// Total number of degree 6 operations in the VM.
pub const NUM_DEGREE_6_OPS: usize = 16;

/// Total number of degree 4 operations in the VM.
pub const NUM_DEGREE_4_OPS: usize = 8;

/// Opcode at which degree 7 operation starts.
pub const DEGREE_7_OPCODE_STARTS: usize = 0;

/// Opcode at which degree 7 operation ends.
pub const DEGREE_7_OPCODE_ENDS: usize = DEGREE_7_OPCODE_STARTS + 63;

/// Opcode at which degree 6 operation starts.
pub const DEGREE_6_OPCODE_STARTS: usize = DEGREE_7_OPCODE_ENDS + 1;

/// Opcode at which degree 6 operation ends.
pub const DEGREE_6_OPCODE_ENDS: usize = DEGREE_6_OPCODE_STARTS + 31;

/// Opcode at which degree 4 operation starts.
pub const DEGREE_4_OPCODE_STARTS: usize = DEGREE_6_OPCODE_ENDS + 1;

/// Opcode at which degree 4 operation ends.
pub const DEGREE_4_OPCODE_ENDS: usize = DEGREE_4_OPCODE_STARTS + 31;

// Operation Flags
// ================================================================================================

/// Operation flags for all stack operations.
///
/// Computes all the operation flag values of the individual stack operations using intermediate
/// values which helps in reducing the number of field multiplication operations being used in the
/// calculation of the operation flag.
///
/// The operation flag values are computed separately for degree 7 and degree 6 and 4 stack operations.
/// Only one flag will be set to ONE and rest all would be ZERO for an execution trace.   
pub struct OpFlags<E: FieldElement> {
    degree7_op_flags: [E; NUM_DEGREE_7_OPS],
    degree6_op_flags: [E; NUM_DEGREE_6_OPS],
    degree4_op_flags: [E; NUM_DEGREE_4_OPS],
}

#[allow(dead_code)]
impl<E: FieldElement> OpFlags<E> {
    // CONSTRUCTOR
    // =================================================================================================

    /// Returns a new instance of [OpFlags] instantiated with all stack operation flags.
    pub fn new(frame: &EvaluationFrame<E>) -> Self {
        let (degree_6_op_flags, degree_4_op_flags) = set_op_flags_degree_4_and_6(frame);

        Self {
            degree7_op_flags: set_op_flags_degree_7(frame),
            degree6_op_flags: degree_6_op_flags,
            degree4_op_flags: degree_4_op_flags,
        }
    }

    // STATE ACCESSORS
    // ==========================================================================================

    // ------ Degree 7 operations with no shift -------------------------------------------------

    #[inline(always)]
    pub fn noop(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Noop.op_code())]
    }

    #[inline(always)]
    pub fn eqz(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Eqz.op_code())]
    }

    #[inline(always)]
    pub fn neg(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Neg.op_code())]
    }

    #[inline(always)]
    pub fn inv(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Inv.op_code())]
    }

    #[inline(always)]
    pub fn incr(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Incr.op_code())]
    }

    #[inline(always)]
    pub fn not(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Not.op_code())]
    }

    #[inline(always)]
    pub fn fmpadd(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::FmpAdd.op_code())]
    }

    #[inline(always)]
    pub fn mload(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MLoad.op_code())]
    }

    #[inline(always)]
    pub fn swap(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Swap.op_code())]
    }

    #[inline(always)]
    pub fn movup2(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp2.op_code())]
    }

    #[inline(always)]
    pub fn movdn2(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn2.op_code())]
    }

    #[inline(always)]
    pub fn movup3(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp3.op_code())]
    }

    #[inline(always)]
    pub fn movdn3(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn3.op_code())]
    }

    #[inline(always)]
    pub fn readw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::ReadW.op_code())]
    }

    #[inline(always)]
    pub fn movup4(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp4.op_code())]
    }

    #[inline(always)]
    pub fn movdn4(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn4.op_code())]
    }

    #[inline(always)]
    pub fn movup5(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp5.op_code())]
    }

    #[inline(always)]
    pub fn movdn5(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn5.op_code())]
    }

    #[inline(always)]
    pub fn movup6(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp6.op_code())]
    }

    #[inline(always)]
    pub fn movdn6(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn6.op_code())]
    }

    #[inline(always)]
    pub fn movup7(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp7.op_code())]
    }

    #[inline(always)]
    pub fn movdn7(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn7.op_code())]
    }

    #[inline(always)]
    pub fn swapw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SwapW.op_code())]
    }

    #[inline(always)]
    pub fn movup8(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp8.op_code())]
    }

    #[inline(always)]
    pub fn movdn8(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn8.op_code())]
    }

    #[inline(always)]
    pub fn swapw2(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SwapW2.op_code())]
    }

    #[inline(always)]
    pub fn swapw3(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SwapW3.op_code())]
    }

    #[inline(always)]
    pub fn swapdw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SwapDW.op_code())]
    }

    // ------ Degree 7 operations with left shift ----------------------------------------

    #[inline(always)]
    pub fn assert(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Assert.op_code())]
    }

    #[inline(always)]
    pub fn eq(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Eq.op_code())]
    }

    #[inline(always)]
    pub fn add(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Add.op_code())]
    }

    #[inline(always)]
    pub fn mul(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Mul.op_code())]
    }

    #[inline(always)]
    pub fn and(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::And.op_code())]
    }

    #[inline(always)]
    pub fn or(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Or.op_code())]
    }

    #[inline(always)]
    pub fn u32and(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::U32and.op_code())]
    }

    #[inline(always)]
    pub fn u32xor(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::U32xor.op_code())]
    }

    #[inline(always)]
    pub fn drop(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Drop.op_code())]
    }

    #[inline(always)]
    pub fn cswap(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::CSwap.op_code())]
    }

    #[inline(always)]
    pub fn cswapw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::CSwapW.op_code())]
    }

    #[inline(always)]
    pub fn mloadw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MLoadW.op_code())]
    }

    #[inline(always)]
    pub fn mstore(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MStore.op_code())]
    }

    #[inline(always)]
    pub fn mstorew(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MStoreW.op_code())]
    }

    #[inline(always)]
    pub fn fmpupdate(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::FmpUpdate.op_code())]
    }

    // ------ Degree 7 operations with right shift -----------------------------------------------

    #[inline(always)]
    pub fn pad(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Pad.op_code())]
    }

    #[inline(always)]
    pub fn dup(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup0.op_code())]
    }

    #[inline(always)]
    pub fn dup1(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup1.op_code())]
    }

    #[inline(always)]
    pub fn dup2(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup2.op_code())]
    }

    #[inline(always)]
    pub fn dup3(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup3.op_code())]
    }

    #[inline(always)]
    pub fn dup4(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup4.op_code())]
    }

    #[inline(always)]
    pub fn dup5(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup5.op_code())]
    }

    #[inline(always)]
    pub fn dup6(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup6.op_code())]
    }

    #[inline(always)]
    pub fn dup7(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup7.op_code())]
    }

    #[inline(always)]
    pub fn dup9(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup9.op_code())]
    }

    #[inline(always)]
    pub fn dup11(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup11.op_code())]
    }

    #[inline(always)]
    pub fn dup13(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup13.op_code())]
    }

    #[inline(always)]
    pub fn dup15(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup15.op_code())]
    }

    #[inline(always)]
    pub fn read(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Read.op_code())]
    }

    #[inline(always)]
    pub fn sdepth(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SDepth.op_code())]
    }

    // ------ Degree 6 u32 operations  --------------------------------------------------------

    #[inline(always)]
    pub fn u32add(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32add.op_code())]
    }

    #[inline(always)]
    pub fn u32sub(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32sub.op_code())]
    }

    #[inline(always)]
    pub fn u32mul(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32mul.op_code())]
    }

    #[inline(always)]
    pub fn u32div(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32div.op_code())]
    }

    #[inline(always)]
    pub fn u32split(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32split.op_code())]
    }

    #[inline(always)]
    pub fn u32assert2(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32assert2.op_code())]
    }

    #[inline(always)]
    pub fn u32add3(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32add3.op_code())]
    }

    #[inline(always)]
    pub fn u32madd(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32madd.op_code())]
    }

    // ------ Degree 6 non u32 operations  ---------------------------------------------------

    #[inline(always)]
    pub fn rpperm(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::RpPerm.op_code())]
    }

    #[inline(always)]
    pub fn mpverify(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::MpVerify.op_code())]
    }

    #[inline(always)]
    pub fn span(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::Span.op_code())]
    }

    #[inline(always)]
    pub fn join(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::Join.op_code())]
    }

    #[inline(always)]
    pub fn split(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::Split.op_code())]
    }

    #[inline(always)]
    pub fn loop_op(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::Loop.op_code())]
    }

    // ------ Degree 4 stack operations  -----------------------------------------------------

    #[inline(always)]
    pub fn mrupdate(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::MrUpdate(true).op_code())]
    }

    #[inline(always)]
    pub fn push(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Push(ONE).op_code())]
    }

    #[inline(always)]
    pub fn end(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::End.op_code())]
    }

    #[inline(always)]
    pub fn repeat(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Repeat.op_code())]
    }

    #[inline(always)]
    pub fn respan(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Respan.op_code())]
    }

    #[inline(always)]
    pub fn halt(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Halt.op_code())]
    }
}

trait EvaluationFrameExt<E: FieldElement> {
    // --- Operation bit accessors ---------------------------------------------------------------

    /// Returns the current value of the specified operation bit in the decoder. It assumes that
    /// the index is a valid index.
    fn op_bit(&self, index: usize) -> E;

    /// Returns the value of operation bit extra column which is used to reduce the degree of op
    /// flags for degree 4 operations where the two most significant bits are ONE.
    fn op_bit_helper(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Operation bit accessors ---------------------------------------------------------------

    #[inline]
    fn op_bit(&self, idx: usize) -> E {
        self.current()[DECODER_TRACE_OFFSET + OP_BITS_RANGE.start + idx]
    }

    #[inline]
    fn op_bit_helper(&self) -> E {
        self.current()[DECODER_TRACE_OFFSET + OP_BIT_EXTRA_COL_IDX]
    }
}

// HELPER FUNCTIONS
// ------------------------------------------------------------------------------------------------

/// This helper function generates all the flag values of degree seven stack operations.
/// This includes all the three different operations that shifts the stack to the right,
/// left and don't shift at all.
fn set_op_flags_degree_7<E: FieldElement>(frame: &EvaluationFrame<E>) -> [E; NUM_DEGREE_7_OPS] {
    let mut degree_7_intermediate_values = [E::ZERO; NUM_DEGREE_7_OPS];

    let not_0 = binary_not(frame.op_bit(0));
    let not_1 = binary_not(frame.op_bit(1));
    degree_7_intermediate_values[0] = not_1 * not_0;
    degree_7_intermediate_values[1] = frame.op_bit(0) * not_1;
    degree_7_intermediate_values[2] = not_0 * frame.op_bit(1);
    degree_7_intermediate_values[3] = frame.op_bit(1) * frame.op_bit(0);
    degree_7_intermediate_values.copy_within(0..4, 4);

    let not_2 = binary_not(frame.op_bit(2));
    degree_7_intermediate_values
        .iter_mut()
        .take(4)
        .for_each(|v| *v *= not_2);
    degree_7_intermediate_values
        .iter_mut()
        .take(8)
        .skip(4)
        .for_each(|v| *v *= frame.op_bit(2));
    degree_7_intermediate_values.copy_within(0..8, 8);

    let not_3 = binary_not(frame.op_bit(3));
    degree_7_intermediate_values
        .iter_mut()
        .take(8)
        .for_each(|v| *v *= not_3);
    degree_7_intermediate_values
        .iter_mut()
        .take(16)
        .skip(8)
        .for_each(|v| *v *= frame.op_bit(3));
    degree_7_intermediate_values.copy_within(0..16, 16);

    let not_4 = binary_not(frame.op_bit(4));
    degree_7_intermediate_values
        .iter_mut()
        .take(16)
        .for_each(|v| *v *= not_4);
    degree_7_intermediate_values
        .iter_mut()
        .take(32)
        .skip(16)
        .for_each(|v| *v *= frame.op_bit(4));
    degree_7_intermediate_values.copy_within(0..32, 32);

    let not_5 = binary_not(frame.op_bit(5));
    let not_6 = binary_not(frame.op_bit(6));

    // The first op_bit for a degree seven operation is 0. In order to save redundant
    // multiplication ops and given the first bit will always be 0, a binary_not is
    // being multiplied with both 2nd op bit as well as it's boolean not.
    let not5_not6 = not_5 * not_6;
    let not6_5 = not_6 * frame.op_bit(5);
    degree_7_intermediate_values
        .iter_mut()
        .take(32)
        .for_each(|v| *v *= not5_not6);
    degree_7_intermediate_values
        .iter_mut()
        .take(64)
        .skip(32)
        .for_each(|v| *v *= not6_5);

    degree_7_intermediate_values
}

/// This helper function generates flag values of all the degree six and four stack
/// operations.
fn set_op_flags_degree_4_and_6<E: FieldElement>(
    frame: &EvaluationFrame<E>,
) -> ([E; NUM_DEGREE_6_OPS], [E; NUM_DEGREE_4_OPS]) {
    let mut degree_6_intermediate_values = [E::ZERO; NUM_DEGREE_6_OPS];
    let mut degree_4_intermediate_values = [E::ZERO; NUM_DEGREE_4_OPS];

    let not_3 = binary_not(frame.op_bit(3));
    let not_2 = binary_not(frame.op_bit(2));
    let helper = frame.op_bit_helper();

    degree_4_intermediate_values[0] = not_3 * not_2;
    degree_4_intermediate_values[1] = frame.op_bit(2) * not_3;
    degree_4_intermediate_values[2] = not_2 * frame.op_bit(3);
    degree_4_intermediate_values[3] = frame.op_bit(3) * frame.op_bit(2);
    degree_4_intermediate_values.copy_within(0..4, 4);

    let not_4 = binary_not(frame.op_bit(4));
    for i in 0..4 {
        degree_4_intermediate_values[i] *= not_4;
        degree_6_intermediate_values[2 * i] = degree_4_intermediate_values[i];
    }
    for i in 4..8 {
        degree_4_intermediate_values[i] *= frame.op_bit(4);
        degree_6_intermediate_values[2 * i] = degree_4_intermediate_values[i];
    }

    // helper register is multiplied with the intermediate values to enumerate all the possible
    // degree 4 operations flags.
    degree_4_intermediate_values
        .iter_mut()
        .take(8)
        .for_each(|v| *v *= helper);

    // The degree 6 flag (`10xxxxx`) is clubbed with both 6th bit and it's boolean not value to
    // reduce the number of multiplications.
    let not_1 = binary_not(frame.op_bit(1));
    let degree_six_flag = frame.op_bit(6) * binary_not(frame.op_bit(5));
    let degree_six_flag_not_1 = degree_six_flag * not_1;
    let degree_six_flag_yes_1 = degree_six_flag * frame.op_bit(1);
    for i in (0..16).step_by(2) {
        degree_6_intermediate_values[i + 1] =
            degree_6_intermediate_values[i] * degree_six_flag_yes_1;
        degree_6_intermediate_values[i] *= degree_six_flag_not_1;
    }

    (degree_6_intermediate_values, degree_4_intermediate_values)
}

/// Maps opcode of an operation with the index in the respective degree flags. It accepts
/// an Operation as input.
pub const fn get_op_index(opcode: u8) -> usize {
    let opcode = opcode as usize;

    // opcode should be in between 0-128.
    if opcode < 64 {
        // index of a degree 7 operation in the degree 7 flag's array.
        opcode
    } else if opcode < DEGREE_6_OPCODE_ENDS {
        // index of a degree 6 operation in the degree 6 flag's array.
        (opcode - DEGREE_6_OPCODE_STARTS) / 2
    } else {
        // index of a degree 4 operation in the degree 4 flag's array.
        (opcode - DEGREE_4_OPCODE_STARTS) / 4
    }
}

/// Accepts an integer which is a unique representation of an operation and returns
/// a trace constaining the op bits of the operation.
pub fn generate_evaluation_frame(opcode: usize) -> EvaluationFrame<Felt> {
    let operation_bit_array = get_op_bits(opcode);

    // Initialize the rows.
    let mut current = vec![ZERO; TRACE_WIDTH];
    let next = vec![ZERO; TRACE_WIDTH];

    // set op bits of the operation in the decoder.
    for i in 0..NUM_OP_BITS {
        current[DECODER_TRACE_OFFSET + OP_BITS_RANGE.start + i] = operation_bit_array[i];
    }

    // set helper value in the decoder. It will be ONE only in the case of a
    // degree four operation.
    current[DECODER_TRACE_OFFSET + OP_BIT_EXTRA_COL_IDX] = current
        [DECODER_TRACE_OFFSET + OP_BITS_RANGE.end - 1]
        * current[DECODER_TRACE_OFFSET + OP_BITS_RANGE.end - 2];

    EvaluationFrame::<Felt>::from_rows(current, next)
}

/// Returns a 7-bits array representation of an operation. The method accepts an integer
/// value which is an unique representation of an operation.
pub fn get_op_bits(opcode: usize) -> [Felt; NUM_OP_BITS] {
    let mut opcode_copy = opcode;

    // initialise the bit array with 0.
    let mut bit_array = [ZERO; NUM_OP_BITS];

    for i in bit_array.iter_mut() {
        // returns the least significant bit of the opcode.
        let bit = opcode_copy & 1;
        *i = Felt::new(bit as u64);
        // one left shift.
        opcode_copy >>= 1;
    }

    // Assert opcode to 0 after 7 bit shifts as opcode was a 7-bit integer.
    assert_eq!(opcode_copy, 0);

    bit_array
}
