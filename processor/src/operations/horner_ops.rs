use vm_core::{Felt, Operation};

use crate::{ExecutionError, Process, QuadFelt};

// HORNER EVALUATION OPERATIONS
// ================================================================================================

impl Process {
    // HORNER EVALUATION WITH COEFFICIENTS OVER BASE FIELD
    // --------------------------------------------------------------------------------------------

    /// Performs 8 steps of the Horner evaluation method on a polynomial with coefficients over
    /// the base field, i.e., it computes
    ///
    /// acc' = (((acc_tmp * alpha + c3) * alpha + c2) * alpha + c1) * alpha + c0
    ///
    /// where
    ///
    /// acc_tmp := (((acc * alpha + c7) * alpha + c6) * alpha + c5) * alpha + c4
    ///
    ///
    /// In other words, the intsruction computes the evaluation at alpha of the polynomial
    ///
    /// P(X) := c7 * X^7 + c6 * X^6 + ... + c1 * X + c0
    ///
    /// As can be seen from the two equations defining acc', the instruction can be used in order
    /// to compute the evaluation of polynomials of arbitrary degree by repeated invocations of
    /// the same instruction interleaved with any operation that loads the next batch of 8
    /// coefficients on the top of the operand stack, i.e., `mem_stream` or `adv_pipe`.
    ///
    /// The stack transition of the instruction can be visualized as follows:
    ///
    /// Input:
    ///
    /// +------+------+------+------+------+------+------+------+---+---+---+---+------+------+------+------+
    /// |  c0  |  c1  |  c2  |  c3  |  c4  |  c5  |  c6  |  c7  | - | - | - | - |c_addr|r_addr| acc1 | acc0 |
    /// +------+------+------+------+------+------+------+------+---+---+---+---+------+------+------+------+
    ///
    ///
    /// Output:
    ///
    /// +------+------+------+------+------+------+------+------+---+---+---+---+------+------+-------+-------+
    /// |  c0  |  c1  |  c2  |  c3  |  c4  |  c5  |  c6  |  c7  | - | - | - | - |c_addr|r_addr| acc1' | acc0' |
    /// +------+------+------+------+------+------+------+------+---+---+---+---+------+------+-------+-------+
    ///
    ///
    /// Here:
    ///
    /// 1. ci for i in 0..=7 stands for the the value of the i-th coefficient in the current batch
    ///    of 8 coefficients of the polynomial.
    /// 2. (acc0, acc1) stands for an extension field element accumulating the values of the Horner
    ///    evaluation procedure. (acc0', acc1') is the updated value of this accumulator.
    /// 3. t_addr is the memory address from which we are loading the ci's when using the MSTREAM
    ///    instruction or storing using the ADVPIPE instruction.
    /// 4. r_addr is the memory address of the evaluation point i.e., alpha.
    ///
    /// The instruction also makes use of the helper registers to hold the value of
    /// alpha = (alpha0, alpha1) during the course of its execution.
    pub(super) fn op_horner_eval_base(&mut self) -> Result<(), ExecutionError> {
        // --- read the values of the coefficients, over the base field, from the stack -----------
        let coef = self.get_coeff_as_base_elements();

        // --- read the randomness from memory ----------------------------------------------------
        let alpha = self.get_random_point()?;

        // --- read the accumulator values from stack ---------------------------------------------
        let acc_old = self.get_accumulator();

        // --- compute the updated accumulator values ---------------------------------------------
        let acc_new =
            coef.iter().rev().fold(acc_old, |acc, coef| QuadFelt::from(*coef) + alpha * acc);

        // --- copy the stack ---------------------------------------------------------
        self.stack.copy_state(0);

        // --- update the accumulators ------------------------------------------------------------
        self.stack.set(14, acc_new.to_base_elements()[1]);
        self.stack.set(15, acc_new.to_base_elements()[0]);

        // --- set the helper registers -----------------------------------------------------------
        self.populate_helper_registers(alpha);

        Ok(())
    }

    /// Performs 4 steps of the Horner evaluation method on a polynomial with coefficients over
    /// the quadratic extension field, i.e., it computes
    ///
    /// acc' = (((acc * alpha + c3) * alpha + c2) * alpha + c1) * alpha + c0
    ///
    ///
    /// In other words, the intsruction computes the evaluation at alpha of the polynomial
    ///
    /// P(X) := c3 * X^3 + c2 * X^2 + c1 * X + c0
    ///
    /// As can be seen from the two equations defining acc', the instruction can be used in order
    /// to compute the evaluation of polynomials of arbitrary degree by repeated invocations of
    /// the same instruction interleaved with any operation that loads the next batch of 4
    /// coefficients on the top of the operand stack, i.e., `mem_stream` or `adv_pipe`.
    ///
    /// The stack transition of the instruction can be visualized as follows:
    ///
    /// Input:
    ///
    /// +------+------+------+------+------+------+------+------+---+---+---+---+------+------+------+------+
    /// | c0_1 | c0_0 | c1_1 | c1_0 | c2_1 | c2_0 | c3_1 | c3_0 | - | - | - | - |c_addr|r_addr| acc1 | acc0 |
    /// +------+------+------+------+------+------+------+------+---+---+---+---+------+------+------+------+
    ///
    ///
    /// Output:
    ///
    /// +------+------+------+------+------+------+------+------+---+---+---+---+------+------+-------+-------+
    /// | c0_1 | c0_0 | c1_1 | c1_0 | c2_1 | c2_0 | c3_1 | c3_0 | - | - | - | - |c_addr|r_addr| acc1' | acc0' |
    /// +------+------+------+------+------+------+------+------+---+---+---+---+------+------+-------+-------+
    ///
    ///
    /// Here:
    ///
    /// 1. ci for i in 0..=4 stands for the the value of the i-th coefficient in the current batch
    ///    of 4 extension field coefficients of the polynomial.
    /// 2. (acc0, acc1) stands for an extension field element accumulating the values of the Horner
    ///    evaluation procedure. (acc0', acc1') is the updated value of this accumulator.
    /// 3. t_addr is the memory address from which we are loading the ci's when using the MSTREAM
    ///    instruction or storing using the ADVPIPE instruction.
    /// 4. r_addr is the memory address of the evaluation point i.e., alpha.
    ///
    /// The instruction also makes use of the helper registers to hold the value of
    /// alpha = (alpha0, alpha1) during the course of its execution.
    pub(super) fn op_horner_eval_ext(&mut self) -> Result<(), ExecutionError> {
        // --- read the values of the coefficients, over the extension field, from the stack ------
        let coef = self.get_coeff_as_quad_ext_elements();

        // --- read the randomness from memory ----------------------------------------------------
        let alpha = self.get_random_point()?;

        // --- read the accumulator values from stack ---------------------------------------------
        let acc_old = self.get_accumulator();

        // --- compute the updated accumulator values ---------------------------------------------
        let acc_new = coef.iter().rev().fold(acc_old, |acc, coef| *coef + alpha * acc);

        // --- copy the stack ---------------------------------------------------------
        self.stack.copy_state(0);

        // --- update the accumulators ------------------------------------------------------------
        self.stack.set(14, acc_new.to_base_elements()[1]);
        self.stack.set(15, acc_new.to_base_elements()[0]);

        // --- set the helper registers -----------------------------------------------------------
        self.populate_helper_registers(alpha);

        Ok(())
    }

    //// HELPER METHODS
    //// ------------------------------------------------------------------------------------------

    /// Returns the top 8 elements of the operand stack.
    fn get_coeff_as_base_elements(&self) -> [Felt; 8] {
        let c0 = self.stack.get(0);
        let c1 = self.stack.get(1);
        let c2 = self.stack.get(2);
        let c3 = self.stack.get(3);
        let c4 = self.stack.get(4);
        let c5 = self.stack.get(5);
        let c6 = self.stack.get(6);
        let c7 = self.stack.get(7);

        [c0, c1, c2, c3, c4, c5, c6, c7]
    }

    /// Returns the top 8 elements of the operand stack.
    fn get_coeff_as_quad_ext_elements(&self) -> [QuadFelt; 4] {
        let c0 = self.stack.get(0);
        let c1 = self.stack.get(1);
        let c2 = self.stack.get(2);
        let c3 = self.stack.get(3);
        let c4 = self.stack.get(4);
        let c5 = self.stack.get(5);
        let c6 = self.stack.get(6);
        let c7 = self.stack.get(7);

        [
            QuadFelt::new(c0, c1),
            QuadFelt::new(c2, c3),
            QuadFelt::new(c4, c5),
            QuadFelt::new(c6, c7),
        ]
    }

    /// Returns randomness.
    fn get_random_point(&mut self) -> Result<QuadFelt, ExecutionError> {
        let ctx = self.system.ctx();
        let addr = self.stack.get(13);
        let word = self.chiplets.memory.read_word(ctx, addr, self.system.clk())?;
        let a0 = word[0];
        let a1 = word[1];

        Ok(QuadFelt::new(a0, a1))
    }

    /// Reads the accumulator values.
    fn get_accumulator(&self) -> QuadFelt {
        let acc1 = self.stack.get(14);
        let acc0 = self.stack.get(15);

        QuadFelt::new(acc0, acc1)
    }

    /// Populates helper registers with OOD values and randomness.
    fn populate_helper_registers(&mut self, alpha: QuadFelt) {
        let [a0, a1] = alpha.to_base_elements();
        let values = [a0, a1];
        self.decoder.set_user_op_helpers(Operation::HornerBase, &values);
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use std::vec::Vec;

    use test_utils::{build_test, rand::rand_array};
    use vm_core::{Felt, FieldElement, Operation, StackInputs, ZERO};

    use crate::{ContextId, DefaultHost, Process, QuadFelt};

    #[test]
    fn horner_eval_base() {
        // --- build stack inputs -----------------------------------------------------------------
        let mut inputs = rand_array::<Felt, 16>();

        // set c_addr to
        inputs[12] = Felt::ZERO;

        // set r_addr pointer to c_addr + offset, where offset is a large enough value.
        let offset = Felt::new(1000);
        inputs[13] = inputs[12] + offset;

        // set initial accumulator to zero
        inputs[14] = ZERO;
        inputs[15] = ZERO;
        inputs.reverse();

        // --- setup the operand stack ------------------------------------------------------------
        let mut host = DefaultHost::default();
        let stack_inputs = StackInputs::new(inputs.to_vec()).expect("inputs lenght too long");
        let mut process = Process::new_dummy_with_decoder_helpers(stack_inputs);

        // --- setup memory -----------------------------------------------------------------------
        let ctx = ContextId::root();

        let a = rand_array::<Felt, 4>();
        process
            .chiplets
            .memory
            .write_word(
                ctx,
                inputs[2].as_int().try_into().expect("Shouldn't fail by construction"),
                process.system.clk(),
                a,
            )
            .unwrap();
        process.execute_op(Operation::Noop, &mut host).unwrap();

        // --- execute HORNER_BASE operation ------------------------------------------------------
        process.execute_op(Operation::HornerBase, &mut host).unwrap();

        // --- check that the top 8 stack elements were not affected ------------------------------
        let stack_state = process.stack.trace_state();
        inputs.reverse();
        assert_eq!(stack_state[0], inputs[0]);
        assert_eq!(stack_state[1], inputs[1]);
        assert_eq!(stack_state[2], inputs[2]);
        assert_eq!(stack_state[3], inputs[3]);
        assert_eq!(stack_state[4], inputs[4]);
        assert_eq!(stack_state[5], inputs[5]);
        assert_eq!(stack_state[6], inputs[6]);
        assert_eq!(stack_state[7], inputs[7]);

        // --- check that the accumulator was updated correctly -----------------------------------
        let acc1_old = inputs[14];
        let acc0_old = inputs[15];
        let acc_old = QuadFelt::new(acc0_old, acc1_old);

        let a0 = a[0];
        let a1 = a[1];
        let alpha = QuadFelt::new(a0, a1);

        let acc_new = stack_state
            .iter()
            .take(8)
            .rev()
            .fold(acc_old, |acc, coef| QuadFelt::from(*coef) + alpha * acc);

        assert_eq!(acc_new.to_base_elements()[1], stack_state[14]);
        assert_eq!(acc_new.to_base_elements()[0], stack_state[15]);

        // --- check that memory pointers were untouched ------------------------------------------
        assert_eq!(inputs[12], stack_state[12]);
        assert_eq!(inputs[13], stack_state[13]);

        // --- check that the helper registers were updated correctly -----------------------------
        let helper_reg_expected = [a0, a1, ZERO, ZERO, ZERO, ZERO];
        assert_eq!(helper_reg_expected, process.decoder.get_user_op_helpers());
    }

    #[test]
    fn horner_eval_ext() {
        // --- build stack inputs -----------------------------------------------------------------
        let mut inputs = rand_array::<Felt, 16>();

        // set c_addr to
        inputs[12] = Felt::ZERO;

        // set r_addr pointer to c_addr + offset, where offset is a large enough value.
        let offset = Felt::new(1000);
        inputs[13] = inputs[12] + offset;

        // set initial accumulator to zero
        inputs[14] = ZERO;
        inputs[15] = ZERO;
        inputs.reverse();

        // --- setup the operand stack ------------------------------------------------------------
        let mut host = DefaultHost::default();
        let stack_inputs = StackInputs::new(inputs.to_vec()).expect("inputs lenght too long");
        let mut process = Process::new_dummy_with_decoder_helpers(stack_inputs);

        // --- setup memory -----------------------------------------------------------------------
        let ctx = ContextId::root();

        let a = rand_array::<Felt, 4>();
        process
            .chiplets
            .memory
            .write_word(
                ctx,
                inputs[2].as_int().try_into().expect("Shouldn't fail by construction"),
                process.system.clk(),
                a,
            )
            .unwrap();
        process.execute_op(Operation::Noop, &mut host).unwrap();

        // --- execute HORNER_BASE operation ------------------------------------------------------
        process.execute_op(Operation::HornerExt, &mut host).unwrap();

        // --- check that the top 8 stack elements were not affected ------------------------------
        let stack_state = process.stack.trace_state();
        inputs.reverse();
        assert_eq!(stack_state[0], inputs[0]);
        assert_eq!(stack_state[1], inputs[1]);
        assert_eq!(stack_state[2], inputs[2]);
        assert_eq!(stack_state[3], inputs[3]);
        assert_eq!(stack_state[4], inputs[4]);
        assert_eq!(stack_state[5], inputs[5]);
        assert_eq!(stack_state[6], inputs[6]);
        assert_eq!(stack_state[7], inputs[7]);

        // --- check that the accumulator was updated correctly -----------------------------------
        let acc1_old = inputs[14];
        let acc0_old = inputs[15];
        let acc_old = QuadFelt::new(acc0_old, acc1_old);

        let a0 = a[0];
        let a1 = a[1];
        let alpha = QuadFelt::new(a0, a1);

        let acc_new = stack_state
            .chunks(2)
            .take(4)
            .rev()
            .fold(acc_old, |acc, coef| QuadFelt::new(coef[0], coef[1]) + alpha * acc);

        assert_eq!(acc_new.to_base_elements()[1], stack_state[14]);
        assert_eq!(acc_new.to_base_elements()[0], stack_state[15]);

        // --- check that memory pointers were untouched ------------------------------------------
        assert_eq!(inputs[12], stack_state[12]);
        assert_eq!(inputs[13], stack_state[13]);

        // --- check that the helper registers were updated correctly -----------------------------
        let helper_reg_expected = [a0, a1, ZERO, ZERO, ZERO, ZERO];
        assert_eq!(helper_reg_expected, process.decoder.get_user_op_helpers());
    }

    #[test]
    fn prove_verify_horner_base() {
        let source = format!(
            "
            begin
                # Load the evaluation point from the advice stack and store it at `r_addr`
                padw
                adv_loadw
                push.1000
                mem_storew
                dropw

                # Execute
                horner_eval_base
            end
        "
        );

        // --- build stack inputs -----------------------------------------------------------------
        let mut inputs = rand_array::<Felt, 16>();

        // set c_addr to zero
        inputs[12] = Felt::ZERO;

        // set r_addr pointer to c_addr + offset, where offset is a large enough value.
        let offset = Felt::new(1000);
        inputs[13] = inputs[12] + offset;

        // sample a random evaluation point
        let a: [Felt; 2] = rand_array();
        let a0 = a[0];
        let a1 = a[1];
        let alpha = QuadFelt::new(a0, a1);

        // compute the evaluation
        let acc_old = QuadFelt::new(inputs[15], inputs[14]);
        let acc_new = inputs
            .iter()
            .take(8)
            .rev()
            .fold(acc_old, |acc, coef| QuadFelt::from(*coef) + alpha * acc);
        inputs.reverse();

        // prepare the advice stack with the generated data
        let adv_stack = [a[0], a[1], ZERO, ZERO];
        let adv_stack: Vec<u64> = adv_stack.iter().map(|e| e.as_int()).collect();

        // create the expected operand stack
        let mut expected = Vec::new();
        // updated accumulators
        expected.extend_from_slice(&[acc_new.to_base_elements()[0], acc_new.to_base_elements()[1]]);
        // the rest of the stack should remain unchanged
        expected.extend_from_slice(&inputs[2..]);
        let expected: Vec<u64> = expected.iter().rev().map(|e| e.as_int()).collect();

        // convert input stack
        let inputs: Vec<u64> = inputs.iter().map(|e| e.as_int()).collect();

        let test = build_test!(source, &inputs, &adv_stack);
        test.expect_stack(&expected);

        let pub_inputs: Vec<u64> = Vec::new();
        test.prove_and_verify(pub_inputs, false);
    }

    #[test]
    fn prove_verify_horner_ext() {
        let source = format!(
            "
            begin
                # Load the evaluation point from the advice stack and store it at `r_addr`
                padw
                adv_loadw
                push.1000
                mem_storew
                dropw

                # Execute
                horner_eval_ext
            end
        "
        );

        // --- build stack inputs -----------------------------------------------------------------
        let mut inputs = rand_array::<Felt, 16>();

        // set c_addr to zero
        inputs[12] = Felt::ZERO;

        // set r_addr pointer to c_addr + offset, where offset is a large enough value.
        let offset = Felt::new(1000);
        inputs[13] = inputs[12] + offset;

        // sample a random evaluation point
        let a: [Felt; 2] = rand_array();
        let a0 = a[0];
        let a1 = a[1];
        let alpha = QuadFelt::new(a0, a1);

        // compute the evaluation
        let acc_old = QuadFelt::new(inputs[15], inputs[14]);
        let acc_new = inputs
            .chunks(2)
            .take(4)
            .rev()
            .fold(acc_old, |acc, coef| QuadFelt::new(coef[0], coef[1]) + alpha * acc);
        inputs.reverse();

        // prepare the advice stack with the generated data
        let adv_stack = [a[0], a[1], ZERO, ZERO];
        let adv_stack: Vec<u64> = adv_stack.iter().map(|e| e.as_int()).collect();

        // create the expected operand stack
        let mut expected = Vec::new();
        // updated accumulators
        expected.extend_from_slice(&[acc_new.to_base_elements()[0], acc_new.to_base_elements()[1]]);
        // the rest of the stack should remain unchanged
        expected.extend_from_slice(&inputs[2..]);
        let expected: Vec<u64> = expected.iter().rev().map(|e| e.as_int()).collect();

        // convert input stack
        let inputs: Vec<u64> = inputs.iter().map(|e| e.as_int()).collect();

        let test = build_test!(source, &inputs, &adv_stack);
        test.expect_stack(&expected);

        let pub_inputs: Vec<u64> = Vec::new();
        test.prove_and_verify(pub_inputs, false);
    }
}
