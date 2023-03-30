use super::{EvaluationFrame, B0_COL_IDX};
use crate::utils::binary_not;
use vm_core::{
    decoder::{IS_LOOP_FLAG_COL_IDX, NUM_OP_BITS, OP_BITS_RANGE, OP_BIT_EXTRA_COL_IDX},
    stack::H0_COL_IDX,
    Felt, FieldElement, Operation, DECODER_TRACE_OFFSET, ONE, STACK_TRACE_OFFSET, TRACE_WIDTH,
    ZERO,
};

#[cfg(test)]
pub mod tests;

// CONSTANTS
// ==================================================================================================

/// Total number of degree 7 operations in the VM.
const NUM_DEGREE_7_OPS: usize = 64;

/// Total number of degree 6 operations in the VM.
const NUM_DEGREE_6_OPS: usize = 16;

/// Total number of degree 4 operations in the VM.
const NUM_DEGREE_4_OPS: usize = 8;

/// Total number of composite flags per stack impact type in the VM.
const NUM_STACK_IMPACT_FLAGS: usize = 16;

/// Opcode at which degree 7 operation starts.
const DEGREE_7_OPCODE_STARTS: usize = 0;

/// Opcode at which degree 7 operation ends.
const DEGREE_7_OPCODE_ENDS: usize = DEGREE_7_OPCODE_STARTS + 63;

/// Opcode at which degree 6 operation starts.
const DEGREE_6_OPCODE_STARTS: usize = DEGREE_7_OPCODE_ENDS + 1;

/// Opcode at which degree 6 operation ends.
const DEGREE_6_OPCODE_ENDS: usize = DEGREE_6_OPCODE_STARTS + 31;

/// Opcode at which degree 4 operation starts.
const DEGREE_4_OPCODE_STARTS: usize = DEGREE_6_OPCODE_ENDS + 1;

/// Opcode at which degree 4 operation ends.
#[allow(dead_code)]
const DEGREE_4_OPCODE_ENDS: usize = DEGREE_4_OPCODE_STARTS + 31;

// Operation Flags
// ================================================================================================

/// Operation flags for all stack operations.
///
/// Computes all the operation flag values of the individual stack operations using intermediate
/// values which helps in reducing the number of field multiplication operations being used in the
/// calculation of the operation flag.
///
/// The operation flag values are computed separately for degree 7 and degree 6 and 4 stack operations.
/// Only one flag will be set to ONE and rest all would be ZERO for an execution trace. It also computes
/// the composite flags using individual stack operation flags for generic stack constraints.
pub struct OpFlags<E: FieldElement> {
    degree7_op_flags: [E; NUM_DEGREE_7_OPS],
    degree6_op_flags: [E; NUM_DEGREE_6_OPS],
    degree4_op_flags: [E; NUM_DEGREE_4_OPS],
    no_shift_flags: [E; NUM_STACK_IMPACT_FLAGS],
    left_shift_flags: [E; NUM_STACK_IMPACT_FLAGS],
    right_shift_flags: [E; NUM_STACK_IMPACT_FLAGS],

    left_shift: E,
    right_shift: E,
    control_flow: E,
    overflow: E,
    top_binary: E,
    u32_rc_op: E,
}

#[allow(dead_code)]
impl<E: FieldElement> OpFlags<E> {
    // CONSTRUCTOR
    // =================================================================================================

    /// Returns a new instance of [OpFlags] instantiated with all stack operation flags.
    /// It computes the following operations:
    /// - degree 7 operations which doesn't shift the stack.
    /// - degree 7 operations which shifts the stack to the left.
    /// - degree 7 operations which shifts the stack to the right.
    /// - degree 6 and 4 operations.
    /// - composite flags for individual stack items whose value has been copied over.
    /// - composite flags for individual stack items which has been shifted to the left.
    /// - composite flags for individual stack items which has been shifted to the right.
    /// - composite flag for the stack if the stack has been shifted to the left.
    /// - composite flag for the stack if the stack has been shifted to the right.
    /// - composite flag if the current operation being executed is a control flow operation or not.
    /// - composite flag if the current operation being executed has a binary element constraint on
    /// the top element in the stack.
    pub fn new(frame: &EvaluationFrame<E>) -> Self {
        // intermediary array to cache the value of intermediate flags.
        let mut degree7_op_flags = [E::ZERO; NUM_DEGREE_7_OPS];
        let mut degree6_op_flags = [E::ZERO; NUM_DEGREE_6_OPS];
        let mut degree4_op_flags = [E::ZERO; NUM_DEGREE_4_OPS];
        let mut no_shift_flags = [E::ZERO; NUM_STACK_IMPACT_FLAGS];
        let mut left_shift_flags = [E::ZERO; NUM_STACK_IMPACT_FLAGS];
        let mut right_shift_flags = [E::ZERO; NUM_STACK_IMPACT_FLAGS];

        // binary not of all the operation bits.
        let not_6 = binary_not(frame.op_bit(6));
        let not_5 = binary_not(frame.op_bit(5));
        let not_4 = binary_not(frame.op_bit(4));
        let not_3 = binary_not(frame.op_bit(3));
        let not_2 = binary_not(frame.op_bit(2));
        let not_1 = binary_not(frame.op_bit(1));
        let not_0 = binary_not(frame.op_bit(0));

        // The intermediary value are computed from the most significant bits side.
        degree7_op_flags[0] = not_5 * not_4;
        degree7_op_flags[16] = not_5 * frame.op_bit(4);
        degree7_op_flags[32] = frame.op_bit(5) * not_4;
        degree7_op_flags[48] = frame.op_bit(5) * frame.op_bit(4);

        // flag of prefix when the first 4 elements in op_bits are `100`. It's a flag of
        // all the degree 6 u32 operations.
        let f100 = degree7_op_flags[0] * frame.op_bit(6);
        // flag of prefix when the first 4 elements in op_bits are `1000`. Caching the result
        // for the compution of no_change_2. It's a flag for u32 arithmetic operations.
        let f1000 = f100 * not_3;
        // flag of prefix when the first 4 elements in op_bits are `1011`. Caching the result
        // for the compution of control flow op flag. It's a flag for control flow op flag whose degree is
        // 6.
        let f1011 = degree7_op_flags[16] * frame.op_bit(3) * frame.op_bit(6);
        // flag of prefix when the first 3 elements in op_bits are `111`. Caching the result
        // for the compution of control flow op flag. It's a flag for control flow op flag whose degree is
        // 4.
        let f111 = degree7_op_flags[48] * frame.op_bit(6);

        let not_6_not_3 = not_6 * not_3;
        let not_6_yes_3 = not_6 * frame.op_bit(3);

        // adding the fourth most significant bit along with the most significant bit(to save on
        // multiplication costs).
        for i in (0..64).step_by(16) {
            degree7_op_flags[i + 8] = degree7_op_flags[i] * not_6_yes_3;
            degree7_op_flags[i] *= not_6_not_3;
        }

        // flag of prefix when the first 3 elements in op_bits are `011`. Caching the result
        // for the compution of right_shift_0 flag. It's a flag for degree 7 operation which has
        // shifted the stack to the right.
        let f011 = degree7_op_flags[48] + degree7_op_flags[56];
        // flag of prefix when the first 3 elements in op_bits are `010`. Caching the result
        // for the compution of the left_shift_flag. It's a flag for degree 7 operation which has
        // shifted the stack to the left.
        let f010 = degree7_op_flags[32] + degree7_op_flags[40];
        // flag of prefix when the first 4 elements in op_bits are `0000`. Caching the result
        // for the compution of the no_change_1. It's a flag for degree 7 operation which has
        // not shifted the flag and from 1 element onwards all the stack items are copied over
        // to the next frame.
        let f0000 = degree7_op_flags[0];
        // flag of prefix when the first 4 elements in op_bits are `0100`. Caching the result
        // for the compution of the left_shift_2. It's a flag for degree 7 operation which has
        // shifted the stack to the left from third element onwards.
        let f0100 = degree7_op_flags[32];

        // adding the fifth most significant bit.
        for i in (0..64).step_by(8) {
            degree7_op_flags[i + 4] = degree7_op_flags[i] * frame.op_bit(2);
            degree7_op_flags[i] *= not_2;
        }

        // adding the sixth most significant bit.
        for i in (0..64).step_by(4) {
            degree7_op_flags[i + 2] = degree7_op_flags[i] * frame.op_bit(1);
            degree7_op_flags[i] *= not_1;
        }

        // flags of all the mov{up/dn}{2-8}, swapw{2-3} operations.
        let mov2_flag = degree7_op_flags[10];
        let mov3_flag = degree7_op_flags[12];
        let mov4_flag = degree7_op_flags[16];
        let mov5_flag = degree7_op_flags[18];
        let mov6_flag = degree7_op_flags[20];
        let mov7_flag = degree7_op_flags[22];
        let mov8_flag = degree7_op_flags[26];
        let swapwx_flag = degree7_op_flags[28];
        let adv_popw_expacc = degree7_op_flags[14];

        // adding the least significant bit.
        for i in (0..64).step_by(2) {
            degree7_op_flags[i + 1] = degree7_op_flags[i] * frame.op_bit(0);
            degree7_op_flags[i] *= not_0;
        }

        let ext2mul_flag = degree7_op_flags[25];

        // flag when the items from first point onwards are copied over. It doesn't have noop.
        let no_change_1_flag = f0000 - degree7_op_flags[0];
        // flag when the items from second point onwards are shifted to the left. It doesn't have assert.
        let left_change_1_flag = f0100 - degree7_op_flags[32];

        let helper = frame.op_bit_helper();

        // computation of degree 6 and 4 flags.
        degree4_op_flags[0] = not_3 * not_2;
        degree4_op_flags[1] = frame.op_bit(2) * not_3;
        degree4_op_flags[2] = not_2 * frame.op_bit(3);
        degree4_op_flags[3] = frame.op_bit(3) * frame.op_bit(2);
        degree4_op_flags.copy_within(0..4, 4);

        for i in 0..4 {
            degree4_op_flags[i] *= not_4;
            degree6_op_flags[2 * i] = degree4_op_flags[i];
        }
        for i in 4..8 {
            degree4_op_flags[i] *= frame.op_bit(4);
            degree6_op_flags[2 * i] = degree4_op_flags[i];
        }

        // helper register is multiplied with the intermediate values to enumerate all the possible
        // degree 4 operations flags.
        degree4_op_flags.iter_mut().take(8).for_each(|v| *v *= helper);

        // flag of END operation shifting stack to the left. It's effect on stack depends if the
        // current block being executed is a loop block or not.
        let shift_left_on_end = degree4_op_flags[4] * frame.is_loop_end();

        // The degree 6 flag (`10xxxxx`) is clubbed with both 6th bit and it's boolean not value to
        // reduce the number of multiplications.
        let degree_six_flag = frame.op_bit(6) * not_5;
        let degree_six_flag_not_1 = degree_six_flag * not_1;
        let degree_six_flag_yes_1 = degree_six_flag * frame.op_bit(1);
        for i in (0..16).step_by(2) {
            degree6_op_flags[i + 1] = degree6_op_flags[i] * degree_six_flag_yes_1;
            degree6_op_flags[i] *= degree_six_flag_not_1;
        }

        // -------------------------- no shift composite flags computation ------------------------

        no_shift_flags[0] = degree7_op_flags[0]
            + degree6_op_flags[5]
            + degree6_op_flags[9]
            + degree6_op_flags[12]
            + degree6_op_flags[13]
            + degree4_op_flags[6]
            + degree4_op_flags[7]
            + degree4_op_flags[3]
            + degree4_op_flags[4] * binary_not(frame.is_loop_end());

        no_shift_flags[1] = no_shift_flags[0] + no_change_1_flag;
        no_shift_flags[2] = no_shift_flags[1] + degree7_op_flags[8] + f1000;
        no_shift_flags[3] = no_shift_flags[2] + mov2_flag;
        no_shift_flags[4] = no_shift_flags[3]
            + mov3_flag
            + adv_popw_expacc
            + swapwx_flag
            + ext2mul_flag
            + degree4_op_flags[0];

        no_shift_flags[5] = no_shift_flags[4] + mov4_flag;
        no_shift_flags[6] = no_shift_flags[5] + mov5_flag;
        no_shift_flags[7] = no_shift_flags[6] + mov6_flag;
        no_shift_flags[8] =
            no_shift_flags[7] + mov7_flag + degree7_op_flags[24] - degree7_op_flags[28];

        no_shift_flags[9] = no_shift_flags[8] + mov8_flag;
        no_shift_flags[10] = no_shift_flags[9];
        no_shift_flags[11] = no_shift_flags[9];
        no_shift_flags[12] =
            no_shift_flags[9] - degree7_op_flags[29] + degree7_op_flags[28] + degree6_op_flags[8];
        no_shift_flags[13] = no_shift_flags[12];
        no_shift_flags[14] = no_shift_flags[12];
        no_shift_flags[15] = no_shift_flags[12];

        // -------------------------- left shift composite flags computation ----------------------

        let movdnn_flag = degree7_op_flags[11]
            + degree7_op_flags[13]
            + degree7_op_flags[17]
            + degree7_op_flags[19]
            + degree7_op_flags[21]
            + degree7_op_flags[23]
            + degree7_op_flags[27];

        let split_loop_flag = degree6_op_flags[14] + degree6_op_flags[15];
        let add3_madd_flag = degree6_op_flags[6] + degree6_op_flags[7];

        left_shift_flags[1] = degree7_op_flags[32]
            + movdnn_flag
            + degree7_op_flags[41]
            + degree7_op_flags[45]
            + degree7_op_flags[47]
            + degree7_op_flags[46]
            + split_loop_flag
            + shift_left_on_end;

        left_shift_flags[2] = left_shift_flags[1] + left_change_1_flag;
        left_shift_flags[3] =
            left_shift_flags[2] + add3_madd_flag + degree7_op_flags[42] - degree7_op_flags[11];
        left_shift_flags[4] = left_shift_flags[3] - degree7_op_flags[13];
        left_shift_flags[5] = left_shift_flags[4] + degree7_op_flags[44] - degree7_op_flags[17];
        left_shift_flags[6] = left_shift_flags[5] - degree7_op_flags[19];
        left_shift_flags[7] = left_shift_flags[6] - degree7_op_flags[21];
        left_shift_flags[8] = left_shift_flags[7] - degree7_op_flags[23];
        left_shift_flags[9] = left_shift_flags[8] + degree7_op_flags[43] - degree7_op_flags[27];
        left_shift_flags[10] = left_shift_flags[9];
        left_shift_flags[11] = left_shift_flags[9];
        left_shift_flags[12] = left_shift_flags[9];
        left_shift_flags[13] = left_shift_flags[9];
        left_shift_flags[14] = left_shift_flags[9];
        left_shift_flags[15] = left_shift_flags[9];

        // -------------------------- right shift composite flags computation ---------------------

        let movupn_flag = degree7_op_flags[10]
            + degree7_op_flags[12]
            + degree7_op_flags[16]
            + degree7_op_flags[18]
            + degree7_op_flags[20]
            + degree7_op_flags[22]
            + degree7_op_flags[26];

        right_shift_flags[0] = f011 + degree4_op_flags[1] + movupn_flag;

        right_shift_flags[1] = right_shift_flags[0] + degree6_op_flags[4];

        right_shift_flags[2] = right_shift_flags[1] - degree7_op_flags[10];
        right_shift_flags[3] = right_shift_flags[2] - degree7_op_flags[12];
        right_shift_flags[4] = right_shift_flags[3] - degree7_op_flags[16];
        right_shift_flags[5] = right_shift_flags[4] - degree7_op_flags[18];
        right_shift_flags[6] = right_shift_flags[5] - degree7_op_flags[20];
        right_shift_flags[7] = right_shift_flags[6] - degree7_op_flags[22];
        right_shift_flags[8] = right_shift_flags[7] - degree7_op_flags[26];
        right_shift_flags[9] = right_shift_flags[8];
        right_shift_flags[10] = right_shift_flags[8];
        right_shift_flags[11] = right_shift_flags[8];
        right_shift_flags[12] = right_shift_flags[8];
        right_shift_flags[13] = right_shift_flags[8];
        right_shift_flags[14] = right_shift_flags[8];
        right_shift_flags[15] = right_shift_flags[8];

        // Flag if the stack has been shifted to the right.
        let right_shift = f011 + degree4_op_flags[1] + degree6_op_flags[4];

        // Flag if the stack has been shifted to the left.
        let left_shift =
            f010 + add3_madd_flag + split_loop_flag + degree4_op_flags[5] + shift_left_on_end;

        // Flag if the current operation being executed is a control flow operation.
        let control_flow = f111
            + f1011
            + degree4_op_flags[2]  // SYSCALL op
            + degree4_op_flags[3]; // CALL op

        // Flag if the current operation being executed is a degree 6 u32 operation.
        let u32_rc_op = f100;

        // Flag if the top element in the stack should be binary or not.
        let top_binary = degree7_op_flags[5] // OR op
            + degree7_op_flags[15]  // EXPACC op
            + degree7_op_flags[36]  // AND op
            + degree7_op_flags[37]  // OR op
            + degree7_op_flags[42]  // CSWAP op
            + degree7_op_flags[43]; // CSWAPW op

        // Flag if the overflow table contains values or not.
        let overflow = (frame.stack_depth() - E::from(16u32)) * frame.overflow_register();

        Self {
            degree7_op_flags,
            degree6_op_flags,
            degree4_op_flags,
            no_shift_flags,
            left_shift_flags,
            right_shift_flags,
            left_shift,
            right_shift,
            control_flow,
            overflow,
            top_binary,
            u32_rc_op,
        }
    }

    // STATE ACCESSORS
    // ============================================================================================

    // ------ Degree 7 operations with no shift ---------------------------------------------------

    /// Operation Flag of NOOP operation.
    #[inline(always)]
    pub fn noop(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Noop.op_code())]
    }

    /// Operation Flag of EQZ operation.
    #[inline(always)]
    pub fn eqz(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Eqz.op_code())]
    }

    /// Operation Flag of NEG operation.
    #[inline(always)]
    pub fn neg(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Neg.op_code())]
    }

    /// Operation Flag of INV operation.
    #[inline(always)]
    pub fn inv(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Inv.op_code())]
    }

    /// Operation Flag of INCR operation.
    #[inline(always)]
    pub fn incr(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Incr.op_code())]
    }

    /// Operation Flag of NOT operation.
    #[inline(always)]
    pub fn not(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Not.op_code())]
    }

    /// Operation Flag of FMPADD operation.
    #[inline(always)]
    pub fn fmpadd(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::FmpAdd.op_code())]
    }

    /// Operation Flag of MLOAD operation.
    #[inline(always)]
    pub fn mload(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MLoad.op_code())]
    }

    /// Operation Flag of SWAP operation.
    #[inline(always)]
    pub fn swap(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Swap.op_code())]
    }

    /// Operation Flag of MOVUP2 operation.
    #[inline(always)]
    pub fn movup2(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp2.op_code())]
    }

    /// Operation Flag of MOVDN2 operation.
    #[inline(always)]
    pub fn movdn2(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn2.op_code())]
    }

    /// Operation Flag of MOVUP3 operation.
    #[inline(always)]
    pub fn movup3(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp3.op_code())]
    }

    /// Operation Flag of MOVDN3 operation.
    #[inline(always)]
    pub fn movdn3(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn3.op_code())]
    }

    /// Operation Flag of ADVPOPW operation.
    #[inline(always)]
    pub fn advpopw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::AdvPopW.op_code())]
    }

    /// Operation Flag of EXPACC operation.
    #[inline(always)]
    pub fn expacc(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Expacc.op_code())]
    }

    /// Operation Flag of MOVUP4 operation.
    #[inline(always)]
    pub fn movup4(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp4.op_code())]
    }

    /// Operation Flag of MOVDN4 operation.
    #[inline(always)]
    pub fn movdn4(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn4.op_code())]
    }

    /// Operation Flag of MOVUP5 operation.
    #[inline(always)]
    pub fn movup5(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp5.op_code())]
    }

    /// Operation Flag of MOVDN5 operation.
    #[inline(always)]
    pub fn movdn5(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn5.op_code())]
    }

    /// Operation Flag of MOVUP6 operation.
    #[inline(always)]
    pub fn movup6(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp6.op_code())]
    }

    /// Operation Flag of MOVDN6 operation.
    #[inline(always)]
    pub fn movdn6(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn6.op_code())]
    }

    /// Operation Flag of MOVUP7 operation.
    #[inline(always)]
    pub fn movup7(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp7.op_code())]
    }

    /// Operation Flag of MOVDN7 operation.
    #[inline(always)]
    pub fn movdn7(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn7.op_code())]
    }

    /// Operation Flag of SWAPW operation.
    #[inline(always)]
    pub fn swapw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SwapW.op_code())]
    }

    /// Operation Flag of MOVUP8 operation.
    #[inline(always)]
    pub fn movup8(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovUp8.op_code())]
    }

    /// Operation Flag of MOVDN8 operation.
    #[inline(always)]
    pub fn movdn8(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MovDn8.op_code())]
    }

    /// Operation Flag of SWAPW2 operation.
    #[inline(always)]
    pub fn swapw2(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SwapW2.op_code())]
    }

    /// Operation Flag of SWAPW3 operation.
    #[inline(always)]
    pub fn swapw3(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SwapW3.op_code())]
    }

    /// Operation Flag of SWAPDW operation.
    #[inline(always)]
    pub fn swapdw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SwapDW.op_code())]
    }

    /// Operation Flag of EXT2MUL operation.
    #[inline(always)]
    pub fn ext2mul(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Ext2Mul.op_code())]
    }

    // ------ Degree 7 operations with left shift -------------------------------------------------

    /// Operation Flag of ASSERT operation.
    #[inline(always)]
    pub fn assert(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Assert.op_code())]
    }

    /// Operation Flag of EQ operation.
    #[inline(always)]
    pub fn eq(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Eq.op_code())]
    }

    /// Operation Flag of ADD operation.
    #[inline(always)]
    pub fn add(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Add.op_code())]
    }

    /// Operation Flag of MUL operation.
    #[inline(always)]
    pub fn mul(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Mul.op_code())]
    }

    /// Operation Flag of AND operation.
    #[inline(always)]
    pub fn and(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::And.op_code())]
    }

    /// Operation Flag of OR operation.
    #[inline(always)]
    pub fn or(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Or.op_code())]
    }

    /// Operation Flag of U32AND operation.
    #[inline(always)]
    pub fn u32and(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::U32and.op_code())]
    }

    /// Operation Flag of U32XOR operation.
    #[inline(always)]
    pub fn u32xor(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::U32xor.op_code())]
    }

    /// Operation Flag of DROP operation.
    #[inline(always)]
    pub fn drop(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Drop.op_code())]
    }

    /// Operation Flag of CSWAP operation.
    #[inline(always)]
    pub fn cswap(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::CSwap.op_code())]
    }

    /// Operation Flag of CSWAPW operation.
    #[inline(always)]
    pub fn cswapw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::CSwapW.op_code())]
    }

    /// Operation Flag of MLOADW operation.
    #[inline(always)]
    pub fn mloadw(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MLoadW.op_code())]
    }

    /// Operation Flag of MSTORE operation.
    #[inline(always)]
    pub fn mstore(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MStore.op_code())]
    }

    /// Operation Flag of MSTOREW operation.
    #[inline(always)]
    pub fn mstorew(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::MStoreW.op_code())]
    }

    /// Operation Flag of FMPUPDATE operation.
    #[inline(always)]
    pub fn fmpupdate(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::FmpUpdate.op_code())]
    }

    // ------ Degree 7 operations with right shift ------------------------------------------------

    /// Operation Flag of PAD operation.
    #[inline(always)]
    pub fn pad(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Pad.op_code())]
    }

    /// Operation Flag of DUP operation.
    #[inline(always)]
    pub fn dup(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup0.op_code())]
    }

    /// Operation Flag of DUP1 operation.
    #[inline(always)]
    pub fn dup1(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup1.op_code())]
    }

    /// Operation Flag of DUP2 operation.
    #[inline(always)]
    pub fn dup2(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup2.op_code())]
    }

    /// Operation Flag of DUP3 operation.
    #[inline(always)]
    pub fn dup3(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup3.op_code())]
    }

    /// Operation Flag of DUP4 operation.
    #[inline(always)]
    pub fn dup4(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup4.op_code())]
    }

    /// Operation Flag of DUP5 operation.
    #[inline(always)]
    pub fn dup5(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup5.op_code())]
    }

    /// Operation Flag of DUP6 operation.
    #[inline(always)]
    pub fn dup6(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup6.op_code())]
    }

    /// Operation Flag of DUP7 operation.
    #[inline(always)]
    pub fn dup7(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup7.op_code())]
    }

    /// Operation Flag of DUP9 operation.
    #[inline(always)]
    pub fn dup9(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup9.op_code())]
    }

    /// Operation Flag of DUP11 operation.
    #[inline(always)]
    pub fn dup11(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup11.op_code())]
    }

    /// Operation Flag of DUP13 operation.
    #[inline(always)]
    pub fn dup13(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup13.op_code())]
    }

    /// Operation Flag of DUP15 operation.
    #[inline(always)]
    pub fn dup15(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::Dup15.op_code())]
    }

    /// Operation Flag of ADVPOP operation.
    #[inline(always)]
    pub fn advpop(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::AdvPop.op_code())]
    }

    /// Operation Flag of SDEPTH operation.
    #[inline(always)]
    pub fn sdepth(&self) -> E {
        self.degree7_op_flags[get_op_index(Operation::SDepth.op_code())]
    }

    // ------ Degree 6 u32 operations  ------------------------------------------------------------

    /// Operation Flag of U32ADD operation.
    #[inline(always)]
    pub fn u32add(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32add.op_code())]
    }

    /// Operation Flag of U32SUB operation.
    #[inline(always)]
    pub fn u32sub(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32sub.op_code())]
    }

    /// Operation Flag of U32MUL operation.
    #[inline(always)]
    pub fn u32mul(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32mul.op_code())]
    }

    /// Operation Flag of U32DIV operation.
    #[inline(always)]
    pub fn u32div(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32div.op_code())]
    }

    /// Operation Flag of U32SPLIT operation.
    #[inline(always)]
    pub fn u32split(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32split.op_code())]
    }

    /// Operation Flag of U32ASSERT2 operation.
    #[inline(always)]
    pub fn u32assert2(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32assert2.op_code())]
    }

    /// Operation Flag of U32ADD3 operation.
    #[inline(always)]
    pub fn u32add3(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32add3.op_code())]
    }

    /// Operation Flag of U32MADD operation.
    #[inline(always)]
    pub fn u32madd(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::U32madd.op_code())]
    }

    // ------ Degree 6 non u32 operations  --------------------------------------------------------

    /// Operation Flag of HPERM operation.
    #[inline(always)]
    pub fn hperm(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::HPerm.op_code())]
    }

    /// Operation Flag of MPVERIFY operation.
    #[inline(always)]
    pub fn mpverify(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::MpVerify.op_code())]
    }

    /// Operation Flag of SPAN operation.
    #[inline(always)]
    pub fn span(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::Span.op_code())]
    }

    /// Operation Flag of JOIN operation.
    #[inline(always)]
    pub fn join(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::Join.op_code())]
    }

    /// Operation Flag of SPLIT operation.
    #[inline(always)]
    pub fn split(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::Split.op_code())]
    }

    /// Operation Flag of LOOP operation.
    #[inline(always)]
    pub fn loop_op(&self) -> E {
        self.degree6_op_flags[get_op_index(Operation::Loop.op_code())]
    }

    // ------ Degree 4 stack operations  ----------------------------------------------------------

    /// Operation Flag of MRUPDATE operation.
    #[inline(always)]
    pub fn mrupdate(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::MrUpdate.op_code())]
    }

    /// Operation Flag of PUSH operation.
    #[inline(always)]
    pub fn push(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Push(ONE).op_code())]
    }

    /// Operation Flag of CALL operation.
    #[inline(always)]
    pub fn call(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Call.op_code())]
    }

    /// Operation Flag of SYSCALL operation.
    #[inline(always)]
    pub fn syscall(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::SysCall.op_code())]
    }

    /// Operation Flag of END operation.
    #[inline(always)]
    pub fn end(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::End.op_code())]
    }

    /// Operation Flag of REPEAT operation.
    #[inline(always)]
    pub fn repeat(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Repeat.op_code())]
    }

    /// Operation Flag of RESPAN operation.
    #[inline(always)]
    pub fn respan(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Respan.op_code())]
    }

    /// Operation Flag of HALT operation.
    #[inline(always)]
    pub fn halt(&self) -> E {
        self.degree4_op_flags[get_op_index(Operation::Halt.op_code())]
    }

    // ------ Composite Flags ---------------------------------------------------------------------

    /// Returns ONE when the stack item at the specified depth remains unchanged during an
    /// operation, and ZERO otherwise
    #[inline(always)]
    pub fn no_shift_at(&self, index: usize) -> E {
        self.no_shift_flags[index]
    }

    /// Returns ONE when the stack item at the specified depth shifts to the left during an
    /// operation, and ZERO otherwise. The left shift is not defined on the first postion in the
    /// stack and therefore, a ZERO is returned.
    #[inline(always)]
    pub fn left_shift_at(&self, index: usize) -> E {
        self.left_shift_flags[index]
    }

    /// Returns ONE when the stack item at the specified depth shifts to the right during an
    /// operation, and ZERO otherwise. The right shift is not defined on the last postion in the
    /// stack and therefore, a ZERO is returned.
    #[inline(always)]
    pub fn right_shift_at(&self, index: usize) -> E {
        self.right_shift_flags[index]
    }

    // --------------------------- other composite flags -----------------------------------------

    /// Returns the flag when the stack operation shifts the flag to the right.
    #[inline(always)]
    pub fn right_shift(&self) -> E {
        self.right_shift
    }

    /// Returns the flag when the stack operation shifts the flag to the left.
    #[inline(always)]
    pub fn left_shift(&self) -> E {
        self.left_shift
    }

    /// Returns the flag when the stack operation is a control flow operation.
    #[inline(always)]
    pub fn control_flow(&self) -> E {
        self.control_flow
    }

    /// Returns the flag when the stack operation is a u32 operation.
    #[inline(always)]
    pub fn u32_rc_op(&self) -> E {
        self.u32_rc_op
    }

    /// Returns the flag if the stack overflow table contains values or not.
    #[inline(always)]
    pub fn overflow(&self) -> E {
        self.overflow
    }

    /// Returns the flag when the stack operation needs the top element to be binary.
    #[inline(always)]
    pub fn top_binary(&self) -> E {
        self.top_binary
    }
}

trait EvaluationFrameExt<E: FieldElement> {
    // --- Operation bit accessors ----------------------------------------------------------------

    /// Returns the current value of the specified operation bit in the decoder. It assumes that
    /// the index is a valid index.
    fn op_bit(&self, index: usize) -> E;

    /// Returns the value of operation bit extra column which is used to reduce the degree of op
    /// flags for degree 4 operations where the two most significant bits are ONE.
    fn op_bit_helper(&self) -> E;

    /// Returns the h0 bookeeeping register value in the current frame.
    fn overflow_register(&self) -> E;

    /// Returns the depth of the stack at the current step.
    fn stack_depth(&self) -> E;

    /// Returns the value if the `h5` helper register in the decoder which is set to ONE if the
    /// exiting block is a `LOOP` block.
    fn is_loop_end(&self) -> E;
}

impl<E: FieldElement> EvaluationFrameExt<E> for &EvaluationFrame<E> {
    // --- Operation bit accessors ----------------------------------------------------------------

    #[inline]
    fn op_bit(&self, idx: usize) -> E {
        self.current()[DECODER_TRACE_OFFSET + OP_BITS_RANGE.start + idx]
    }

    #[inline]
    fn op_bit_helper(&self) -> E {
        self.current()[DECODER_TRACE_OFFSET + OP_BIT_EXTRA_COL_IDX]
    }

    #[inline]
    fn overflow_register(&self) -> E {
        self.current()[STACK_TRACE_OFFSET + H0_COL_IDX]
    }

    #[inline]
    fn stack_depth(&self) -> E {
        self.current()[B0_COL_IDX]
    }

    #[inline]
    fn is_loop_end(&self) -> E {
        self.current()[DECODER_TRACE_OFFSET + IS_LOOP_FLAG_COL_IDX]
    }
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

/// Maps opcode of an operation with the index in the respective degree flags. It accepts
/// an Operation as input.
pub const fn get_right_shift(idx: usize) -> usize {
    idx
}

/// Accepts an integer which is a unique representation of an operation and returns
/// a trace containing the op bits of the operation.
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
