use crate::{
    BaseElement, FieldElement, StarkField, CF_OP_BITS_RANGE, HD_OP_BITS_RANGE, LD_OP_BITS_RANGE,
    MIN_CONTEXT_DEPTH, MIN_LOOP_DEPTH, MIN_STACK_DEPTH, NUM_CF_OP_BITS, NUM_HD_OP_BITS,
    NUM_LD_OP_BITS, OP_COUNTER_IDX, OP_SPONGE_RANGE, OP_SPONGE_WIDTH, PROGRAM_DIGEST_SIZE,
};
use core::{cmp, fmt};

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================
const NUM_OP_BITS: usize = NUM_CF_OP_BITS + NUM_LD_OP_BITS + NUM_HD_OP_BITS;
const NUM_STATIC_DECODER_REGISTERS: usize = 1 + OP_SPONGE_WIDTH + NUM_OP_BITS; // 1 is for op_counter

// TYPES AND INTERFACES
// ================================================================================================
#[derive(Clone, PartialEq)]
pub struct TraceState<E: FieldElement<BaseField = BaseElement>> {
    op_counter: E,
    op_sponge: [E; OP_SPONGE_WIDTH],
    cf_op_bits: [E; NUM_CF_OP_BITS],
    ld_op_bits: [E; NUM_LD_OP_BITS],
    hd_op_bits: [E; NUM_HD_OP_BITS],
    ctx_stack: Vec<E>,
    loop_stack: Vec<E>,
    user_stack: Vec<E>,

    ctx_depth: usize,
    loop_depth: usize,
    stack_depth: usize,
}

// TRACE STATE IMPLEMENTATION
// ================================================================================================
impl<E: FieldElement<BaseField = BaseElement>> TraceState<E> {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn new(ctx_depth: usize, loop_depth: usize, stack_depth: usize) -> Self {
        Self {
            op_counter: E::ZERO,
            op_sponge: [E::ZERO; OP_SPONGE_WIDTH],
            cf_op_bits: [E::ZERO; NUM_CF_OP_BITS],
            ld_op_bits: [E::ZERO; NUM_LD_OP_BITS],
            hd_op_bits: [E::ZERO; NUM_HD_OP_BITS],
            ctx_stack: vec![E::ZERO; cmp::max(ctx_depth, MIN_CONTEXT_DEPTH)],
            loop_stack: vec![E::ZERO; cmp::max(loop_depth, MIN_LOOP_DEPTH)],
            user_stack: vec![E::ZERO; cmp::max(stack_depth, MIN_STACK_DEPTH)],
            ctx_depth,
            loop_depth,
            stack_depth,
        }
    }

    pub fn from_slice(
        ctx_depth: usize,
        loop_depth: usize,
        stack_depth: usize,
        state: &[E],
    ) -> Self {
        let op_counter = state[OP_COUNTER_IDX];

        let mut op_sponge = [E::ZERO; OP_SPONGE_WIDTH];
        op_sponge.copy_from_slice(&state[OP_SPONGE_RANGE]);

        let mut cf_op_bits = [E::ZERO; NUM_CF_OP_BITS];
        cf_op_bits.copy_from_slice(&state[CF_OP_BITS_RANGE]);

        let mut ld_op_bits = [E::ZERO; NUM_LD_OP_BITS];
        ld_op_bits.copy_from_slice(&state[LD_OP_BITS_RANGE]);

        let mut hd_op_bits = [E::ZERO; NUM_HD_OP_BITS];
        hd_op_bits.copy_from_slice(&state[HD_OP_BITS_RANGE]);

        let mut ctx_stack = vec![E::ZERO; cmp::max(ctx_depth, MIN_CONTEXT_DEPTH)];
        let ctx_stack_end = HD_OP_BITS_RANGE.end + ctx_depth;
        ctx_stack[..ctx_depth].copy_from_slice(&state[HD_OP_BITS_RANGE.end..ctx_stack_end]);

        let mut loop_stack = vec![E::ZERO; cmp::max(loop_depth, MIN_LOOP_DEPTH)];
        let loop_stack_end = ctx_stack_end + loop_depth;
        loop_stack[..loop_depth].copy_from_slice(&state[ctx_stack_end..loop_stack_end]);

        let mut user_stack = vec![E::ZERO; cmp::max(stack_depth, MIN_STACK_DEPTH)];
        user_stack[..stack_depth].copy_from_slice(&state[loop_stack_end..]);

        TraceState {
            op_counter,
            op_sponge,
            cf_op_bits,
            ld_op_bits,
            hd_op_bits,
            ctx_stack,
            loop_stack,
            user_stack,
            ctx_depth,
            loop_depth,
            stack_depth,
        }
    }

    #[cfg(test)]
    pub fn from_u128_slice(
        ctx_depth: usize,
        loop_depth: usize,
        stack_depth: usize,
        state: &[u128],
    ) -> Self {
        let state = state
            .iter()
            .map(|&v| E::from(BaseElement::new(v)))
            .collect::<Vec<_>>();
        Self::from_slice(ctx_depth, loop_depth, stack_depth, &state)
    }

    // STATIC FUNCTIONS
    // --------------------------------------------------------------------------------------------
    pub fn compute_decoder_width(ctx_depth: usize, loop_depth: usize) -> usize {
        NUM_STATIC_DECODER_REGISTERS + ctx_depth + loop_depth
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    #[cfg(test)]
    pub fn width(&self) -> usize {
        HD_OP_BITS_RANGE.end + self.ctx_depth + self.loop_depth + self.stack_depth
    }

    #[cfg(test)]
    pub fn stack_depth(&self) -> usize {
        self.stack_depth
    }

    // OPERATION COUNTER
    // --------------------------------------------------------------------------------------------
    pub fn op_counter(&self) -> E {
        self.op_counter
    }

    pub fn set_op_counter(&mut self, value: E) {
        self.op_counter = value;
    }

    // SPONGE
    // --------------------------------------------------------------------------------------------
    pub fn op_sponge(&self) -> &[E] {
        &self.op_sponge
    }

    pub fn program_hash(&self) -> &[E] {
        &self.op_sponge[..PROGRAM_DIGEST_SIZE]
    }

    // OP BITS
    // --------------------------------------------------------------------------------------------
    pub fn cf_op_bits(&self) -> &[E] {
        &self.cf_op_bits
    }

    pub fn ld_op_bits(&self) -> &[E] {
        &self.ld_op_bits
    }

    pub fn hd_op_bits(&self) -> &[E] {
        &self.hd_op_bits
    }

    pub fn op_code(&self) -> E {
        let mut result = self.ld_op_bits[0];
        result += self.ld_op_bits[1] * E::from(2u32);
        result += self.ld_op_bits[2] * E::from(4u32);
        result += self.ld_op_bits[3] * E::from(8u32);
        result += self.ld_op_bits[4] * E::from(16u32);
        result += self.hd_op_bits[0] * E::from(32u32);
        result += self.hd_op_bits[1] * E::from(64u32);
        result
    }

    pub fn set_op_bits(&mut self, bits: [E; NUM_OP_BITS]) {
        self.cf_op_bits.copy_from_slice(&bits[..3]);
        self.ld_op_bits.copy_from_slice(&bits[3..8]);
        self.hd_op_bits.copy_from_slice(&bits[8..]);
    }

    pub fn get_void_op_flag(&self) -> E {
        // VOID opcode is 111
        self.cf_op_bits[0] * self.cf_op_bits[1] * self.cf_op_bits[2]
    }

    // STACKS
    // --------------------------------------------------------------------------------------------
    pub fn ctx_stack(&self) -> &[E] {
        &self.ctx_stack
    }

    pub fn loop_stack(&self) -> &[E] {
        &self.loop_stack
    }

    pub fn user_stack(&self) -> &[E] {
        &self.user_stack
    }

    // RAW STATE
    // --------------------------------------------------------------------------------------------
    #[cfg(test)]
    pub fn to_vec(&self) -> Vec<E> {
        let mut result = Vec::with_capacity(self.width());
        result.push(self.op_counter);
        result.extend_from_slice(&self.op_sponge);
        result.extend_from_slice(&self.cf_op_bits);
        result.extend_from_slice(&self.ld_op_bits);
        result.extend_from_slice(&self.hd_op_bits);
        result.extend_from_slice(&self.ctx_stack[..self.ctx_depth]);
        result.extend_from_slice(&self.loop_stack[..self.loop_depth]);
        result.extend_from_slice(&self.user_stack[..self.stack_depth]);
        result
    }

    pub fn update(&mut self, row: &[E]) {
        self.op_counter = row[OP_COUNTER_IDX];

        for (i, j) in OP_SPONGE_RANGE.enumerate() {
            self.op_sponge[i] = row[j];
        }
        for (i, j) in CF_OP_BITS_RANGE.enumerate() {
            self.cf_op_bits[i] = row[j];
        }
        for (i, j) in LD_OP_BITS_RANGE.enumerate() {
            self.ld_op_bits[i] = row[j];
        }
        for (i, j) in HD_OP_BITS_RANGE.enumerate() {
            self.hd_op_bits[i] = row[j];
        }

        let ctx_stack_start = HD_OP_BITS_RANGE.end;
        let ctx_stack_end = ctx_stack_start + self.ctx_depth;
        for (i, j) in (ctx_stack_start..ctx_stack_end).enumerate() {
            self.ctx_stack[i] = row[j];
        }

        let loop_stack_end = ctx_stack_end + self.loop_depth;
        for (i, j) in (ctx_stack_end..loop_stack_end).enumerate() {
            self.loop_stack[i] = row[j];
        }

        let user_stack_end = loop_stack_end + self.stack_depth;
        for (i, j) in (loop_stack_end..user_stack_end).enumerate() {
            self.user_stack[i] = row[j];
        }
    }
}

impl fmt::Debug for TraceState<BaseElement> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:>4}] {:>32X?} {:?} {:?} {:?} {:>32X?} {:>32X?} {:?}",
            self.op_counter.as_int(),
            self.op_sponge
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<_>>(),
            self.cf_op_bits
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<_>>(),
            self.ld_op_bits
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<_>>(),
            self.hd_op_bits
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<_>>(),
            self.ctx_stack
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<_>>(),
            self.loop_stack
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<_>>(),
            self.user_stack
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<_>>()
        )
    }
}

impl fmt::Display for TraceState<BaseElement> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:>4}] {:>16X?} {:?} {:?} {:?} {:>16X?} {:>16X?} {:?}",
            self.op_counter.as_int(),
            self.op_sponge
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
