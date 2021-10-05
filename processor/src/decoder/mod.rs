use crate::{
    op_sponge,
    opcodes::{FlowOps, UserOps},
    BaseElement, FieldElement, StarkField, BASE_CYCLE_LENGTH, MAX_CONTEXT_DEPTH, MAX_LOOP_DEPTH,
    NUM_CF_OP_BITS, NUM_HD_OP_BITS, NUM_LD_OP_BITS, PUSH_OP_ALIGNMENT,
};

// TYPES AND INTERFACES
// ================================================================================================
pub struct Decoder {
    step: usize,

    op_counter: Vec<BaseElement>,
    op_sponge_trace: [Vec<BaseElement>; op_sponge::STATE_WIDTH],
    op_sponge: [BaseElement; op_sponge::STATE_WIDTH],

    cf_op_bits: [Vec<BaseElement>; NUM_CF_OP_BITS],
    ld_op_bits: [Vec<BaseElement>; NUM_LD_OP_BITS],
    hd_op_bits: [Vec<BaseElement>; NUM_HD_OP_BITS],

    ctx_stack: Vec<Vec<BaseElement>>,
    ctx_depth: usize,

    loop_stack: Vec<Vec<BaseElement>>,
    loop_depth: usize,
}

// DECODER IMPLEMENTATION
// ================================================================================================
impl Decoder {
    /// Creates a new instance of instruction decoder.
    pub fn new(init_trace_length: usize) -> Decoder {
        // initialize operation counter
        let op_counter = vec![BaseElement::ZERO; init_trace_length];

        // initialize instruction sponge
        let op_sponge_trace = [
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
        ];
        let op_sponge = [BaseElement::ZERO; op_sponge::STATE_WIDTH];

        // initialize op_bits registers
        let cf_op_bits = [
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
        ];
        let ld_op_bits = [
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
        ];
        let hd_op_bits = [
            vec![BaseElement::ZERO; init_trace_length],
            vec![BaseElement::ZERO; init_trace_length],
        ];

        // initialize the stacks
        let ctx_stack = vec![vec![BaseElement::ZERO; init_trace_length]];
        let ctx_depth = ctx_stack.len();

        let loop_stack = Vec::new();
        let loop_depth = loop_stack.len();

        // create and return decoder
        Decoder {
            step: 0,
            op_counter,
            op_sponge,
            op_sponge_trace,
            cf_op_bits,
            ld_op_bits,
            hd_op_bits,
            ctx_stack,
            ctx_depth,
            loop_stack,
            loop_depth,
        }
    }

    /// Returns trace length of register traces in the decoder.
    pub fn trace_length(&self) -> usize {
        self.op_counter.len()
    }

    /// Returns value of the current step pointer.
    #[allow(unused)]
    pub fn current_step(&self) -> usize {
        self.step
    }

    /// Returns the max value of the op_counter register converted to u64.
    pub fn max_op_counter_value(&self) -> u64 {
        self.op_counter[self.op_counter.len() - 1].as_int() as u64
    }

    /// Returns the max value of the context stack reached during program execution.
    pub fn max_ctx_stack_depth(&self) -> usize {
        // outer-most context doesn't count because it is always just 0
        self.ctx_stack.len() - 1
    }

    /// Returns the max value of the loop stack reached during program execution.
    pub fn max_loop_stack_depth(&self) -> usize {
        self.loop_stack.len()
    }

    /// Returns the state of the stack at the specified `step`.
    #[allow(unused, clippy::vec_init_then_push)]
    pub fn get_state(&self, step: usize) -> Vec<BaseElement> {
        let mut state = Vec::new();

        state.push(self.op_counter[step]);
        for register in self.op_sponge_trace.iter() {
            state.push(register[step]);
        }
        for register in self.cf_op_bits.iter() {
            state.push(register[step]);
        }
        for register in self.ld_op_bits.iter() {
            state.push(register[step]);
        }
        for register in self.hd_op_bits.iter() {
            state.push(register[step]);
        }
        for register in self.ctx_stack.iter() {
            state.push(register[step]);
        }
        for register in self.loop_stack.iter() {
            state.push(register[step]);
        }

        state
    }

    /// Merges all register traces into a single vector of traces.
    pub fn into_register_traces(mut self) -> Vec<Vec<BaseElement>> {
        let mut registers = vec![self.op_counter];

        let [r0, r1, r2, r3] = self.op_sponge_trace;
        registers.push(r0);
        registers.push(r1);
        registers.push(r2);
        registers.push(r3);

        let [r0, r1, r2] = self.cf_op_bits;
        registers.push(r0);
        registers.push(r1);
        registers.push(r2);

        let [r0, r1, r2, r3, r4] = self.ld_op_bits;
        registers.push(r0);
        registers.push(r1);
        registers.push(r2);
        registers.push(r3);
        registers.push(r4);

        let [r0, r1] = self.hd_op_bits;
        registers.push(r0);
        registers.push(r1);

        // for context stack, first get rid of the outer-most context because it is always 0
        self.ctx_stack.pop();
        registers.append(&mut self.ctx_stack);

        registers.append(&mut self.loop_stack);

        registers
    }

    // OPERATION DECODERS
    // --------------------------------------------------------------------------------------------

    /// Initiates a new program block (Group or Switch).
    pub fn start_block(&mut self) {
        assert!(
            self.step % BASE_CYCLE_LENGTH == BASE_CYCLE_LENGTH - 1,
            "cannot start context block at step {}: operation alignment is not valid",
            self.step
        );

        self.advance_step(false);
        self.save_context();
        self.copy_loop_stack();
        self.set_op_bits(FlowOps::Begin, UserOps::Noop);
        self.set_sponge([BaseElement::ZERO; 4]);
    }

    /// Terminates a program block (Group, Switch, or Loop).
    pub fn end_block(&mut self, sibling_hash: BaseElement, true_branch: bool) {
        assert!(
            self.step % BASE_CYCLE_LENGTH == 0,
            "cannot exit context block at step {}: operation alignment is not valid",
            self.step
        );

        self.advance_step(false);
        let context_hash = self.pop_context();
        self.copy_loop_stack();

        let block_hash = self.op_sponge[0];
        if true_branch {
            // we are closing true branch of execution
            self.set_op_bits(FlowOps::Tend, UserOps::Noop);
            self.set_sponge([context_hash, block_hash, sibling_hash, BaseElement::ZERO]);
        } else {
            // we are closing false branch of execution
            self.set_op_bits(FlowOps::Fend, UserOps::Noop);
            self.set_sponge([context_hash, sibling_hash, block_hash, BaseElement::ZERO]);
        }
    }

    /// Initiates a new Loop block
    pub fn start_loop(&mut self, loop_image: BaseElement) {
        assert!(
            self.step % BASE_CYCLE_LENGTH == BASE_CYCLE_LENGTH - 1,
            "cannot start a loop at step {}: operation alignment is not valid",
            self.step
        );

        self.advance_step(false);
        self.save_context();
        self.save_loop_image(loop_image);
        self.set_op_bits(FlowOps::Loop, UserOps::Noop);
        self.set_sponge([BaseElement::ZERO; 4]);
    }

    /// Prepares the decoder for the next iteration of a loop.
    pub fn wrap_loop(&mut self) {
        assert!(
            self.step % BASE_CYCLE_LENGTH == BASE_CYCLE_LENGTH - 1,
            "cannot wrap a loop at step {}: operation alignment is not valid",
            self.step
        );

        self.advance_step(false);
        self.copy_context_stack();
        let top_loop_image = self.peek_loop_image();
        assert!(
            self.op_sponge[0] == top_loop_image,
            "cannot wrap a loop at step {}: hash of the last iteration doesn't match loop image",
            self.step
        );
        self.set_op_bits(FlowOps::Wrap, UserOps::Noop);
        self.set_sponge([BaseElement::ZERO; 4]);
    }

    /// Prepares the decoder for exiting a loop.
    pub fn break_loop(&mut self) {
        assert!(
            self.step % BASE_CYCLE_LENGTH == BASE_CYCLE_LENGTH - 1,
            "cannot break a loop at step {}: operation alignment is not valid",
            self.step
        );

        self.advance_step(false);
        self.copy_context_stack();
        let top_loop_image = self.pop_loop_image();
        assert!(
            self.op_sponge[0] == top_loop_image,
            "cannot break a loop at step {}: hash of the last iteration doesn't match loop image",
            self.step
        );
        self.set_op_bits(FlowOps::Break, UserOps::Noop);
        self.set_sponge(self.op_sponge);
    }

    /// Updates the decoder with the value of the specified operation.
    pub fn decode_op(&mut self, op_code: UserOps, op_value: BaseElement) {
        // op_value can be provided only for a PUSH operation and only
        // at steps which are multiples of 8
        if op_value != BaseElement::ZERO {
            match op_code {
                UserOps::Push => assert!(
                    self.step % PUSH_OP_ALIGNMENT == 0,
                    "invalid PUSH operation alignment at step {}",
                    self.step
                ),
                _ => panic!(
                    "invalid {:?} operation at step {}: op_value is non-zero",
                    op_code, self.step
                ),
            }
        }

        self.advance_step(true);
        self.copy_context_stack();
        self.copy_loop_stack();
        self.set_op_bits(FlowOps::Hacc, op_code);
        self.apply_hacc_round(op_code, op_value);
    }

    /// Populate all register traces with values for steps between the current step
    /// and the end of the trace.
    pub fn finalize_trace(&mut self) {
        // don't increase counter for void instructions
        let last_op_count = self.op_counter[self.step];
        fill_register(&mut self.op_counter, self.step + 1, last_op_count);

        // set all bit registers to 1 to indicate NOOP operation
        for register in self.cf_op_bits.iter_mut() {
            fill_register(register, self.step, BaseElement::ONE);
        }
        for register in self.ld_op_bits.iter_mut() {
            fill_register(register, self.step, BaseElement::ONE);
        }
        for register in self.hd_op_bits.iter_mut() {
            fill_register(register, self.step, BaseElement::ONE);
        }

        // for sponge and stack registers, just copy the value of the last state of the register
        for register in self.op_sponge_trace.iter_mut() {
            fill_register(register, self.step + 1, register[self.step]);
        }
        for register in self.ctx_stack.iter_mut() {
            fill_register(register, self.step + 1, register[self.step]);
        }
        for register in self.loop_stack.iter_mut() {
            fill_register(register, self.step + 1, register[self.step]);
        }

        // update the step pointer to point to the last step
        self.step = self.trace_length() - 1;
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Moves step pointer to the next step and ensures that register traces have sufficient size.
    fn advance_step(&mut self, is_user_op: bool) {
        // increment step by 1
        self.step += 1;

        // make sure there is enough memory allocated for register traces
        if self.step >= self.trace_length() {
            let new_length = self.trace_length() * 2;

            self.op_counter.resize(new_length, BaseElement::ZERO);
            for register in self.op_sponge_trace.iter_mut() {
                register.resize(new_length, BaseElement::ZERO);
            }
            for register in self.cf_op_bits.iter_mut() {
                register.resize(new_length, BaseElement::ZERO);
            }
            for register in self.ld_op_bits.iter_mut() {
                register.resize(new_length, BaseElement::ZERO);
            }
            for register in self.hd_op_bits.iter_mut() {
                register.resize(new_length, BaseElement::ZERO);
            }
            for register in self.ctx_stack.iter_mut() {
                register.resize(new_length, BaseElement::ZERO);
            }
            for register in self.loop_stack.iter_mut() {
                register.resize(new_length, BaseElement::ZERO);
            }
        }

        // for user ops, increment counter by 1; otherwise, copy counter from thee previous step
        if is_user_op {
            self.op_counter[self.step] = self.op_counter[self.step - 1] + BaseElement::ONE;
        } else {
            self.op_counter[self.step] = self.op_counter[self.step - 1];
        }
    }

    /// Populates all bits registers based on the opcodes for control flow and user operations.
    fn set_op_bits(&mut self, flow_op: FlowOps, user_op: UserOps) {
        // op_bits are always populated for the previous step
        let step = self.step - 1;

        let flow_op = flow_op as u8;
        for i in 0..NUM_CF_OP_BITS {
            self.cf_op_bits[i][step] = BaseElement::new(((flow_op >> i) & 1) as u128);
        }

        let user_op = user_op as u8;
        for i in 0..NUM_LD_OP_BITS {
            self.ld_op_bits[i][step] = BaseElement::new(((user_op >> i) & 1) as u128);
        }

        for i in 0..NUM_HD_OP_BITS {
            self.hd_op_bits[i][step] =
                BaseElement::new(((user_op >> (i + NUM_LD_OP_BITS)) & 1) as u128);
        }
    }

    // CONTEXT STACK HELPERS
    // --------------------------------------------------------------------------------------------

    /// Pushes hash of the current program block onto the context stack.
    fn save_context(&mut self) {
        // increment context depth and make sure it doesn't overflow the stack
        self.ctx_depth += 1;
        assert!(
            self.ctx_depth <= MAX_CONTEXT_DEPTH,
            "context stack overflow at step {}",
            self.step
        );

        // if the depth exceeds current number of registers allocated for the context stack,
        // add a new register trace to the stack
        if self.ctx_depth > self.ctx_stack.len() {
            self.ctx_stack
                .push(vec![BaseElement::ZERO; self.trace_length()]);
        }

        // shift all stack values by one item to the right
        for i in 1..self.ctx_stack.len() {
            self.ctx_stack[i][self.step] = self.ctx_stack[i - 1][self.step - 1];
        }

        // set the top of the stack to the hash of the current program block
        // which is located in the first register of the sponge
        self.ctx_stack[0][self.step] = self.op_sponge[0]
    }

    /// Removes the top value from the context stack and returns it.
    fn pop_context(&mut self) -> BaseElement {
        // make sure the stack is not empty
        assert!(
            self.ctx_depth > 0,
            "context stack underflow at step {}",
            self.step
        );

        // shift all stack values by one item to the left
        for i in 1..self.ctx_stack.len() {
            self.ctx_stack[i - 1][self.step] = self.ctx_stack[i][self.step - 1];
        }

        // update the stack depth and return the value that was at the top of the stack
        // before it was shifted to the left
        self.ctx_depth -= 1;
        self.ctx_stack[0][self.step - 1]
    }

    /// Copies contents of the context stack from the previous to the current step.
    fn copy_context_stack(&mut self) {
        for i in 0..self.ctx_stack.len() {
            self.ctx_stack[i][self.step] = self.ctx_stack[i][self.step - 1];
        }
    }

    // LOOP STACK HELPERS
    // --------------------------------------------------------------------------------------------

    /// Pushes `loop_image` onto the loop stack.
    fn save_loop_image(&mut self, loop_image: BaseElement) {
        // increment loop depth and make sure it doesn't overflow the stack
        self.loop_depth += 1;
        assert!(
            self.loop_depth <= MAX_LOOP_DEPTH,
            "loop stack overflow at step {}",
            self.step
        );

        // if the depth exceeds current number of registers allocated for the loop stack,
        // add a new register trace to the stack
        if self.loop_depth > self.loop_stack.len() {
            self.loop_stack
                .push(vec![BaseElement::ZERO; self.trace_length()]);
        }

        // shift all stack values by one to the right
        for i in 1..self.loop_stack.len() {
            self.loop_stack[i][self.step] = self.loop_stack[i - 1][self.step - 1];
        }

        // set the top of the stack to loop_image
        self.loop_stack[0][self.step] = loop_image;
    }

    /// Copies contents of the loop stack from the previous to the current step and returns
    /// the top value of the stack.
    fn peek_loop_image(&mut self) -> BaseElement {
        // make sure the stack is not empty
        assert!(
            self.loop_depth > 0,
            "loop stack underflow at step {}",
            self.step
        );

        // copy all values of the stack from the last step to the current step
        for i in 0..self.loop_stack.len() {
            self.loop_stack[i][self.step] = self.loop_stack[i][self.step - 1];
        }

        // return top value of the stack
        self.loop_stack[0][self.step]
    }

    // Removes the top value from the loop stack and returns it.
    fn pop_loop_image(&mut self) -> BaseElement {
        // make sure the stack is not empty
        assert!(
            self.loop_depth > 0,
            "loop stack underflow at step {}",
            self.step
        );

        // shift all stack values by one item to the left
        for i in 1..self.loop_stack.len() {
            self.loop_stack[i - 1][self.step] = self.loop_stack[i][self.step - 1];
        }

        // update the stack depth and return the value that was at the top of the stack
        // before it was shifted to the left
        self.loop_depth -= 1;
        self.loop_stack[0][self.step - 1]
    }

    /// Copies contents of the loop stack from the previous to the current step.
    fn copy_loop_stack(&mut self) {
        for i in 0..self.loop_stack.len() {
            self.loop_stack[i][self.step] = self.loop_stack[i][self.step - 1];
        }
    }

    // HASH ACCUMULATOR HELPERS
    // --------------------------------------------------------------------------------------------

    /// Sets the states of the sponge to the provided values and updates `sponge_trace` registers
    /// at the current step.
    fn set_sponge(&mut self, state: [BaseElement; op_sponge::STATE_WIDTH]) {
        self.op_sponge = state;
        self.op_sponge_trace[0][self.step] = state[0];
        self.op_sponge_trace[1][self.step] = state[1];
        self.op_sponge_trace[2][self.step] = state[2];
        self.op_sponge_trace[3][self.step] = state[3];
    }

    /// Applies a modified version of Rescue round to the sponge state and copies the result
    /// into `sponge_trace` registers.
    fn apply_hacc_round(&mut self, op_code: UserOps, op_value: BaseElement) {
        // apply single round of sponge function
        op_sponge::apply_round(
            &mut self.op_sponge,
            BaseElement::new(op_code as u128),
            op_value,
            self.step - 1,
        );

        // copy the new sponge state into the sponge_trace registers
        for i in 0..op_sponge::STATE_WIDTH {
            self.op_sponge_trace[i][self.step] = self.op_sponge[i];
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================
fn fill_register(register: &mut Vec<BaseElement>, from: usize, value: BaseElement) {
    let to = register.len();
    register.resize(from, BaseElement::ZERO);
    register.resize(to, value);
}
