use crate::{
    OpCode, CF_OP_BITS_RANGE, HD_OP_BITS_RANGE, LD_OP_BITS_RANGE, MIN_CONTEXT_DEPTH,
    MIN_LOOP_DEPTH, MIN_STACK_DEPTH, NUM_CF_OPS, NUM_CF_OP_BITS, NUM_HD_OPS, NUM_HD_OP_BITS,
    NUM_LD_OPS, NUM_LD_OP_BITS, OP_COUNTER_IDX, PROGRAM_DIGEST_SIZE, SPONGE_RANGE, SPONGE_WIDTH,
};
use core::{cmp, fmt};
use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};

// CONSTANTS
// ================================================================================================
const NUM_OP_BITS: usize = NUM_CF_OP_BITS + NUM_LD_OP_BITS + NUM_HD_OP_BITS;
const NUM_STATIC_DECODER_REGISTERS: usize = 1 + SPONGE_WIDTH + NUM_OP_BITS; // 1 is for op_counter

// TYPES AND INTERFACES
// ================================================================================================
#[derive(PartialEq)]
pub struct TraceState {
    op_counter: BaseElement,
    sponge: [BaseElement; SPONGE_WIDTH],
    cf_op_bits: [BaseElement; NUM_CF_OP_BITS],
    ld_op_bits: [BaseElement; NUM_LD_OP_BITS],
    hd_op_bits: [BaseElement; NUM_HD_OP_BITS],
    ctx_stack: Vec<BaseElement>,
    loop_stack: Vec<BaseElement>,
    user_stack: Vec<BaseElement>,

    ctx_depth: usize,
    loop_depth: usize,
    stack_depth: usize,

    cf_op_flags: [BaseElement; NUM_CF_OPS],
    ld_op_flags: [BaseElement; NUM_LD_OPS],
    hd_op_flags: [BaseElement; NUM_HD_OPS],
    begin_flag: BaseElement,
    noop_flag: BaseElement,
    op_flags_set: bool,
}

// TRACE STATE IMPLEMENTATION
// ================================================================================================
impl TraceState {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn new(ctx_depth: usize, loop_depth: usize, stack_depth: usize) -> TraceState {
        TraceState {
            op_counter: BaseElement::ZERO,
            sponge: [BaseElement::ZERO; SPONGE_WIDTH],
            cf_op_bits: [BaseElement::ZERO; NUM_CF_OP_BITS],
            ld_op_bits: [BaseElement::ZERO; NUM_LD_OP_BITS],
            hd_op_bits: [BaseElement::ZERO; NUM_HD_OP_BITS],
            ctx_stack: vec![BaseElement::ZERO; cmp::max(ctx_depth, MIN_CONTEXT_DEPTH)],
            loop_stack: vec![BaseElement::ZERO; cmp::max(loop_depth, MIN_LOOP_DEPTH)],
            user_stack: vec![BaseElement::ZERO; cmp::max(stack_depth, MIN_STACK_DEPTH)],
            ctx_depth: ctx_depth,
            loop_depth: loop_depth,
            stack_depth: stack_depth,
            cf_op_flags: [BaseElement::ZERO; NUM_CF_OPS],
            ld_op_flags: [BaseElement::ZERO; NUM_LD_OPS],
            hd_op_flags: [BaseElement::ZERO; NUM_HD_OPS],
            begin_flag: BaseElement::ZERO,
            noop_flag: BaseElement::ZERO,
            op_flags_set: false,
        }
    }

    pub fn from_vec(
        ctx_depth: usize,
        loop_depth: usize,
        stack_depth: usize,
        state: &[BaseElement],
    ) -> TraceState {
        let op_counter = state[OP_COUNTER_IDX];

        let mut sponge = [BaseElement::ZERO; SPONGE_WIDTH];
        sponge.copy_from_slice(&state[SPONGE_RANGE]);

        let mut cf_op_bits = [BaseElement::ZERO; NUM_CF_OP_BITS];
        cf_op_bits.copy_from_slice(&state[CF_OP_BITS_RANGE]);

        let mut ld_op_bits = [BaseElement::ZERO; NUM_LD_OP_BITS];
        ld_op_bits.copy_from_slice(&state[LD_OP_BITS_RANGE]);

        let mut hd_op_bits = [BaseElement::ZERO; NUM_HD_OP_BITS];
        hd_op_bits.copy_from_slice(&state[HD_OP_BITS_RANGE]);

        let mut ctx_stack = vec![BaseElement::ZERO; cmp::max(ctx_depth, MIN_CONTEXT_DEPTH)];
        let ctx_stack_end = HD_OP_BITS_RANGE.end + ctx_depth;
        ctx_stack[..ctx_depth].copy_from_slice(&state[HD_OP_BITS_RANGE.end..ctx_stack_end]);

        let mut loop_stack = vec![BaseElement::ZERO; cmp::max(loop_depth, MIN_LOOP_DEPTH)];
        let loop_stack_end = ctx_stack_end + loop_depth;
        loop_stack[..loop_depth].copy_from_slice(&state[ctx_stack_end..loop_stack_end]);

        let mut user_stack = vec![BaseElement::ZERO; cmp::max(stack_depth, MIN_STACK_DEPTH)];
        user_stack[..stack_depth].copy_from_slice(&state[loop_stack_end..]);

        TraceState {
            op_counter,
            sponge,
            cf_op_bits,
            ld_op_bits,
            hd_op_bits,
            ctx_stack,
            loop_stack,
            user_stack,
            ctx_depth,
            loop_depth,
            stack_depth,
            cf_op_flags: [BaseElement::ZERO; NUM_CF_OPS],
            ld_op_flags: [BaseElement::ZERO; NUM_LD_OPS],
            hd_op_flags: [BaseElement::ZERO; NUM_HD_OPS],
            begin_flag: BaseElement::ZERO,
            noop_flag: BaseElement::ZERO,
            op_flags_set: false,
        }
    }

    #[cfg(test)]
    pub fn from_u128_slice(
        ctx_depth: usize,
        loop_depth: usize,
        stack_depth: usize,
        state: &[u128],
    ) -> TraceState {
        let state = state
            .iter()
            .map(|&v| BaseElement::new(v))
            .collect::<Vec<_>>();
        Self::from_vec(ctx_depth, loop_depth, stack_depth, &state)
    }

    // STATIC FUNCTIONS
    // --------------------------------------------------------------------------------------------
    pub fn compute_decoder_width(ctx_depth: usize, loop_depth: usize) -> usize {
        NUM_STATIC_DECODER_REGISTERS + ctx_depth + loop_depth
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    pub fn width(&self) -> usize {
        HD_OP_BITS_RANGE.end + self.ctx_depth + self.loop_depth + self.stack_depth
    }

    pub fn stack_depth(&self) -> usize {
        self.stack_depth
    }

    // OPERATION COUNTER
    // --------------------------------------------------------------------------------------------
    pub fn op_counter(&self) -> BaseElement {
        self.op_counter
    }

    #[cfg(test)]
    pub fn set_op_counter(&mut self, value: BaseElement) {
        self.op_counter = value;
    }

    // SPONGE
    // --------------------------------------------------------------------------------------------
    pub fn sponge(&self) -> &[BaseElement] {
        &self.sponge
    }

    pub fn program_hash(&self) -> &[BaseElement] {
        &self.sponge[..PROGRAM_DIGEST_SIZE]
    }

    // OP BITS
    // --------------------------------------------------------------------------------------------
    pub fn cf_op_bits(&self) -> &[BaseElement] {
        &self.cf_op_bits
    }

    pub fn ld_op_bits(&self) -> &[BaseElement] {
        &self.ld_op_bits
    }

    pub fn hd_op_bits(&self) -> &[BaseElement] {
        &self.hd_op_bits
    }

    pub fn op_code(&self) -> BaseElement {
        let mut result = self.ld_op_bits[0];
        result += self.ld_op_bits[1] * BaseElement::new(2);
        result += self.ld_op_bits[2] * BaseElement::new(4);
        result += self.ld_op_bits[3] * BaseElement::new(8);
        result += self.ld_op_bits[4] * BaseElement::new(16);
        result += self.hd_op_bits[0] * BaseElement::new(32);
        result += self.hd_op_bits[1] * BaseElement::new(64);
        result
    }

    pub fn set_op_bits(&mut self, bits: [BaseElement; NUM_OP_BITS]) {
        self.cf_op_bits.copy_from_slice(&bits[..3]);
        self.ld_op_bits.copy_from_slice(&bits[3..8]);
        self.hd_op_bits.copy_from_slice(&bits[8..]);
    }

    // OP FLAGS
    // --------------------------------------------------------------------------------------------
    pub fn cf_op_flags(&self) -> [BaseElement; NUM_CF_OPS] {
        if !self.op_flags_set {
            unsafe {
                let mutable_self = &mut *(self as *const _ as *mut TraceState);
                mutable_self.set_op_flags();
            }
        }
        self.cf_op_flags
    }

    pub fn ld_op_flags(&self) -> [BaseElement; NUM_LD_OPS] {
        if !self.op_flags_set {
            unsafe {
                let mutable_self = &mut *(self as *const _ as *mut TraceState);
                mutable_self.set_op_flags();
            }
        }
        self.ld_op_flags
    }

    pub fn hd_op_flags(&self) -> [BaseElement; NUM_HD_OPS] {
        if !self.op_flags_set {
            unsafe {
                let mutable_self = &mut *(self as *const _ as *mut TraceState);
                mutable_self.set_op_flags();
            }
        }
        self.hd_op_flags
    }

    pub fn begin_flag(&self) -> BaseElement {
        self.begin_flag
    }

    pub fn noop_flag(&self) -> BaseElement {
        self.noop_flag
    }

    // STACKS
    // --------------------------------------------------------------------------------------------
    pub fn ctx_stack(&self) -> &[BaseElement] {
        &self.ctx_stack
    }

    pub fn loop_stack(&self) -> &[BaseElement] {
        &self.loop_stack
    }

    pub fn user_stack(&self) -> &[BaseElement] {
        &self.user_stack
    }

    // RAW STATE
    // --------------------------------------------------------------------------------------------
    pub fn to_vec(&self) -> Vec<BaseElement> {
        let mut result = Vec::with_capacity(self.width());
        result.push(self.op_counter);
        result.extend_from_slice(&self.sponge);
        result.extend_from_slice(&self.cf_op_bits);
        result.extend_from_slice(&self.ld_op_bits);
        result.extend_from_slice(&self.hd_op_bits);
        result.extend_from_slice(&self.ctx_stack[..self.ctx_depth]);
        result.extend_from_slice(&self.loop_stack[..self.loop_depth]);
        result.extend_from_slice(&self.user_stack[..self.stack_depth]);
        result
    }

    pub fn update_from_trace(&mut self, trace: &[Vec<BaseElement>], step: usize) {
        self.op_counter = trace[OP_COUNTER_IDX][step];

        for (i, j) in SPONGE_RANGE.enumerate() {
            self.sponge[i] = trace[j][step];
        }
        for (i, j) in CF_OP_BITS_RANGE.enumerate() {
            self.cf_op_bits[i] = trace[j][step];
        }
        for (i, j) in LD_OP_BITS_RANGE.enumerate() {
            self.ld_op_bits[i] = trace[j][step];
        }
        for (i, j) in HD_OP_BITS_RANGE.enumerate() {
            self.hd_op_bits[i] = trace[j][step];
        }

        let ctx_stack_start = HD_OP_BITS_RANGE.end;
        let ctx_stack_end = ctx_stack_start + self.ctx_depth;
        for (i, j) in (ctx_stack_start..ctx_stack_end).enumerate() {
            self.ctx_stack[i] = trace[j][step];
        }

        let loop_stack_end = ctx_stack_end + self.loop_depth;
        for (i, j) in (ctx_stack_end..loop_stack_end).enumerate() {
            self.loop_stack[i] = trace[j][step];
        }

        let user_stack_end = loop_stack_end + self.stack_depth;
        for (i, j) in (loop_stack_end..user_stack_end).enumerate() {
            self.user_stack[i] = trace[j][step];
        }

        self.op_flags_set = false;
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------
    fn set_op_flags(&mut self) {
        // set control flow flags
        let not_0 = binary_not(self.cf_op_bits[0]);
        let not_1 = binary_not(self.cf_op_bits[1]);
        self.cf_op_flags[0] = not_0 * not_1;
        self.cf_op_flags[1] = self.cf_op_bits[0] * not_1;
        self.cf_op_flags[2] = not_0 * self.cf_op_bits[1];
        self.cf_op_flags[3] = self.cf_op_bits[0] * self.cf_op_bits[1];
        self.cf_op_flags.copy_within(0..4, 4);

        let not_2 = binary_not(self.cf_op_bits[2]);
        for i in 0..4 {
            self.cf_op_flags[i] = self.cf_op_flags[i] * not_2;
        }
        for i in 4..8 {
            self.cf_op_flags[i] = self.cf_op_flags[i] * self.cf_op_bits[2];
        }

        // set low-degree operation flags
        let not_0 = binary_not(self.ld_op_bits[0]);
        let not_1 = binary_not(self.ld_op_bits[1]);
        self.ld_op_flags[0] = not_0 * not_1;
        self.ld_op_flags[1] = self.ld_op_bits[0] * not_1;
        self.ld_op_flags[2] = not_0 * self.cf_op_bits[1];
        self.ld_op_flags[3] = self.ld_op_bits[0] * self.ld_op_bits[1];
        self.ld_op_flags.copy_within(0..4, 4);

        let not_2 = binary_not(self.ld_op_bits[2]);
        for i in 0..4 {
            self.ld_op_flags[i] = self.ld_op_flags[i] * not_2;
        }
        for i in 4..8 {
            self.ld_op_flags[i] = self.ld_op_flags[i] * self.ld_op_bits[2];
        }
        self.ld_op_flags.copy_within(0..8, 8);

        let not_3 = binary_not(self.ld_op_bits[3]);
        for i in 0..8 {
            self.ld_op_flags[i] = self.ld_op_flags[i] * not_3;
        }
        for i in 8..16 {
            self.ld_op_flags[i] = self.ld_op_flags[i] * self.ld_op_bits[3];
        }
        self.ld_op_flags.copy_within(0..16, 16);

        let not_4 = binary_not(self.ld_op_bits[4]);
        for i in 0..16 {
            self.ld_op_flags[i] = self.ld_op_flags[i] * not_4;
        }
        for i in 16..32 {
            self.ld_op_flags[i] = self.ld_op_flags[i] * self.ld_op_bits[4];
        }

        // set high-degree operation flags
        let not_0 = binary_not(self.hd_op_bits[0]);
        let not_1 = binary_not(self.hd_op_bits[1]);
        self.hd_op_flags[0] = not_0 * not_1;
        self.hd_op_flags[1] = self.hd_op_bits[0] * not_1;
        self.hd_op_flags[2] = not_0 * self.hd_op_bits[1];
        self.hd_op_flags[3] = self.hd_op_bits[0] * self.hd_op_bits[1];

        // compute flag for BEGIN operation which is just 0000000; the below is equivalent
        // to multiplying binary inverses of all op bits together.
        self.begin_flag =
            self.ld_op_flags[OpCode::Begin.ld_index()] * self.hd_op_flags[OpCode::Begin.hd_index()];

        // compute flag for NOOP operation which is just 1111111; the below is equivalent to
        // multiplying all op bits together.
        self.noop_flag =
            self.ld_op_flags[OpCode::Noop.ld_index()] * self.hd_op_flags[OpCode::Noop.hd_index()];

        // we need to make special adjustments for PUSH and ASSERT op flags so that they
        // don't coincide with BEGIN operation; we do this by multiplying each flag by a
        // single op_bit from another op bank; this increases degree of each flag by 1
        debug_assert!(OpCode::Push.hd_index() == 0, "PUSH index is not 0!");
        self.hd_op_flags[0] = self.hd_op_flags[0] * self.ld_op_bits[0];

        debug_assert!(OpCode::Assert.ld_index() == 0, "ASSERT index is not 0!");
        self.ld_op_flags[0] = self.ld_op_flags[0] * self.hd_op_bits[0];

        // mark flags as set
        self.op_flags_set = true;
    }
}

impl fmt::Debug for TraceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:>4}] {:>32X?} {:?} {:?} {:?} {:>32X?} {:>32X?} {:?}",
            self.op_counter,
            self.sponge,
            self.cf_op_bits,
            self.ld_op_bits,
            self.hd_op_bits,
            self.ctx_stack,
            self.loop_stack,
            self.user_stack
        )
    }
}

impl fmt::Display for TraceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:>4}] {:>16X?} {:?} {:?} {:?} {:>16X?} {:>16X?} {:?}",
            self.op_counter,
            self.sponge
                .iter()
                .map(|x| x.as_int() >> 64)
                .collect::<Vec<u128>>(),
            self.cf_op_bits,
            self.ld_op_bits,
            self.hd_op_bits,
            self.ctx_stack
                .iter()
                .map(|x| x.as_int() >> 64)
                .collect::<Vec<u128>>(),
            self.loop_stack
                .iter()
                .map(|x| x.as_int() >> 64)
                .collect::<Vec<u128>>(),
            &self.user_stack[..self.stack_depth]
        )
    }
}

// HELPER FUNCTIONS
// ================================================================================================
#[inline(always)]
fn binary_not(v: BaseElement) -> BaseElement {
    BaseElement::ONE - v
}

// TESTS
// ================================================================================================
#[cfg(test)]
mod tests {

    use super::TraceState;
    use crate::air::utils::ToElements;
    use winterfell::math::{fields::f128::BaseElement, FieldElement, StarkField};

    #[test]
    fn from_vec() {
        // empty context and loop stacks
        let state = TraceState::from_u128_slice(
            0,
            0,
            2,
            &[101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        );

        assert_eq!(101, state.op_counter().as_int());
        assert_eq!([1, 2, 3, 4].to_elements(), state.sponge());
        assert_eq!([5, 6, 7].to_elements(), state.cf_op_bits());
        assert_eq!([8, 9, 10, 11, 12].to_elements(), state.ld_op_bits());
        assert_eq!([13, 14].to_elements(), state.hd_op_bits());
        assert_eq!([0].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!([15, 16, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
        assert_eq!(17, state.width());
        assert_eq!(2, state.stack_depth());
        assert_eq!(
            [101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].to_elements(),
            state.to_vec()
        );

        // 1 item on context stack, empty loop stack
        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[
                101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
            ],
        );

        assert_eq!(101, state.op_counter().as_int());
        assert_eq!([1, 2, 3, 4].to_elements(), state.sponge());
        assert_eq!([5, 6, 7].to_elements(), state.cf_op_bits());
        assert_eq!([8, 9, 10, 11, 12].to_elements(), state.ld_op_bits());
        assert_eq!([13, 14].to_elements(), state.hd_op_bits());
        assert_eq!([15].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!([16, 17, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
        assert_eq!(18, state.width());
        assert_eq!(2, state.stack_depth());
        assert_eq!(
            [101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17].to_elements(),
            state.to_vec()
        );

        // non-empty loop stack
        let state = TraceState::from_u128_slice(
            2,
            1,
            9,
            &[
                101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26,
            ],
        );

        assert_eq!(101, state.op_counter().as_int());
        assert_eq!([1, 2, 3, 4].to_elements(), state.sponge());
        assert_eq!([5, 6, 7].to_elements(), state.cf_op_bits());
        assert_eq!([8, 9, 10, 11, 12].to_elements(), state.ld_op_bits());
        assert_eq!([13, 14].to_elements(), state.hd_op_bits());
        assert_eq!([15, 16].to_elements(), state.ctx_stack());
        assert_eq!([17].to_elements(), state.loop_stack());
        assert_eq!(
            [18, 19, 20, 21, 22, 23, 24, 25, 26].to_elements(),
            state.user_stack()
        );
        assert_eq!(27, state.width());
        assert_eq!(9, state.stack_depth());
        assert_eq!(
            [
                101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26,
            ]
            .to_elements(),
            state.to_vec()
        );
    }

    #[test]
    fn update_from_trace() {
        let data = vec![
            101, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let mut trace = Vec::with_capacity(data.len());
        for i in 0..data.len() {
            trace.push(vec![
                BaseElement::ZERO,
                BaseElement::new(data[i]),
                BaseElement::ZERO,
            ]);
        }

        // first row
        let mut state = TraceState::new(2, 1, 3);
        state.update_from_trace(&trace, 0);

        assert_eq!(0, state.op_counter().as_int());
        assert_eq!([0, 0, 0, 0].to_elements(), state.sponge());
        assert_eq!([0, 0, 0].to_elements(), state.cf_op_bits());
        assert_eq!([0, 0, 0, 0, 0].to_elements(), state.ld_op_bits());
        assert_eq!([0, 0].to_elements(), state.hd_op_bits());
        assert_eq!([0, 0].to_elements(), state.ctx_stack());
        assert_eq!([0].to_elements(), state.loop_stack());
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.user_stack());
        assert_eq!(21, state.width());
        assert_eq!(3, state.stack_depth());

        // second row
        state.update_from_trace(&trace, 1);

        assert_eq!(101, state.op_counter().as_int());
        assert_eq!([1, 2, 3, 4].to_elements(), state.sponge());
        assert_eq!([5, 6, 7].to_elements(), state.cf_op_bits());
        assert_eq!([8, 9, 10, 11, 12].to_elements(), state.ld_op_bits());
        assert_eq!([13, 14].to_elements(), state.hd_op_bits());
        assert_eq!([15, 16].to_elements(), state.ctx_stack());
        assert_eq!([17].to_elements(), state.loop_stack());
        assert_eq!(
            [18, 19, 20, 0, 0, 0, 0, 0].to_elements(),
            state.user_stack()
        );
        assert_eq!(21, state.width());
        assert_eq!(3, state.stack_depth());
    }

    #[test]
    fn op_flags() {
        // all zeros
        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[101, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 16, 17],
        );

        assert_eq!([1, 0, 0, 0, 0, 0, 0, 0].to_elements(), state.cf_op_flags());
        assert_eq!(
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
            .to_elements(),
            state.ld_op_flags()
        );
        assert_eq!([0, 0, 0, 0].to_elements(), state.hd_op_flags());
        assert_eq!(1, state.begin_flag().as_int());
        assert_eq!(0, state.noop_flag().as_int());

        // all ones
        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[101, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 15, 16, 17],
        );

        assert_eq!([0, 0, 0, 0, 0, 0, 0, 1].to_elements(), state.cf_op_flags());
        assert_eq!(
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ]
            .to_elements(),
            state.ld_op_flags()
        );
        assert_eq!([0, 0, 0, 1].to_elements(), state.hd_op_flags());
        assert_eq!(0, state.begin_flag().as_int());
        assert_eq!(1, state.noop_flag().as_int());

        // mixed 1
        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[101, 1, 2, 3, 4, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 15, 16, 17],
        );

        assert_eq!([0, 1, 0, 0, 0, 0, 0, 0].to_elements(), state.cf_op_flags());
        assert_eq!(
            [
                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
            .to_elements(),
            state.ld_op_flags()
        );
        assert_eq!([0, 1, 0, 0].to_elements(), state.hd_op_flags());
        assert_eq!(0, state.begin_flag().as_int());
        assert_eq!(0, state.noop_flag().as_int());

        // mixed 2
        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[101, 1, 2, 3, 4, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 15, 16, 17],
        );

        assert_eq!([0, 0, 0, 1, 0, 0, 0, 0].to_elements(), state.cf_op_flags());
        assert_eq!(
            [
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
            .to_elements(),
            state.ld_op_flags()
        );
        assert_eq!([0, 0, 1, 0].to_elements(), state.hd_op_flags());
    }

    #[test]
    fn op_code() {
        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[101, 1, 2, 3, 4, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 15, 16, 17],
        );
        assert_eq!(0, state.op_code().as_int());

        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[101, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 15, 16, 17],
        );
        assert_eq!(127, state.op_code().as_int());

        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[101, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 15, 16, 17],
        );
        assert_eq!(63, state.op_code().as_int());

        let state = TraceState::from_u128_slice(
            1,
            0,
            2,
            &[101, 1, 2, 3, 4, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 15, 16, 17],
        );
        assert_eq!(97, state.op_code().as_int());
    }
}
