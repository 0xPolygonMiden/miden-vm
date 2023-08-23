// FRI OPERATIONS
// ================================================================================================

use vm_core::{Felt, Operation, ONE, ZERO};

use crate::{AdviceProvider, ExecutionError, Process, QuadFelt};

impl<A> Process<A>
where
    A: AdviceProvider,
{
    // COMBINE VALUES USING RANDOM VALUES
    // --------------------------------------------------------------------------------------------
    /// Performs a single step of the random linear combination defining the DEEP composition
    /// polynomial i.e., the input to the FRI protocol. More precisely, the sum in question is:
    /// \sum_{i=0}^k{\alpha_i \cdot \left(\frac{T_i(x) - T_i(z)}{x - z} +
    ///            \frac{T_i(x) - T_i(g \cdot z)}{x - g \cdot z} \right)}
    ///
    /// and the following instruction computes the denominators $\alpha_i \cdot (T_i(x) - T_i(z))$
    /// and $\alpha_i \cdot (T_i(x) - T_i(g \cdot z))$ and stores the values in two accumulators
    /// $r$ and $p$, respectively. This instruction is specialized to main trace columns i.e.
    /// the values $T_i(x)$ are base field elements.
    ///
    /// The stack transition of the instruction can be visualized as follows:
    ///
    /// +------+------+------+------+------+------+------+------+------+------+------+------+------+------+------+---+
    /// |  T7  |  T6  |  T5  |  T4  |  T3  |  T2  |  T1  |  T0  |  p1  |  p0  |  r1  |  r0  |x_addr|z_addr|a_addr| - |
    /// +------+------+------+------+------+------+------+------+------+------+------+------+------+------+------+---+
    ///
    ///                                                       ||
    ///                                                       \/
    ///                                                    
    /// +------+------+------+------+------+------+------+------+------+------+------+------+------+--------+--------+---+
    /// |  T0  |  T7  |  T6  |  T5  |  T4  |  T3  |  T2  |  T1  |  p1' |  p0' |  r1' |  r0' |x_addr|z_addr+1|a_addr+1| - |
    /// +------+------+------+------+------+------+------+------+------+------+------+------+------+--------+--------+---+
    ///
    ///
    /// Here:
    /// 1. Ti for i in 0..=7 stands for the the value of the i-th trace polynomial for the current
    ///  query i.e. T_i(x).
    /// 2. (p0, p1) stands for an extension field element accumulating the values for the quotients
    ///  with common denominator (x - gz).
    /// 3. (r0, r1) stands for an extension field element accumulating the values for the quotients
    ///  with common denominator (x - z).
    /// 4. x_addr is the memory address from which we are loading the Ti's using the MSTREAM
    ///  instruction.
    /// 5. z_addr is the memory address to the i-th OOD evaluation frame at z and gz
    ///  i.e. T_i(z):= (T_i(z)0, T_i(z)1) and T_i(gz):= (T_i(gz)0, T_i(gz)1).
    /// 6. a_addr is the memory address of the i-th random element used in batching the trace
    ///  polynomial quotients.
    ///
    /// Stack transition for this operation looks as follows:
    ///
    /// Input:
    /// [T7, T6, T5, T4, T3, T2, T1, T0, p1, p0, r1, r0, x_addr, z_addr, a_addr, 0]
    ///
    /// Output:
    /// [T0, T7, T6, T5, T4, T3, T2, T1, p1', p0', r1', r0', x_addr, z_addr+1, a_addr+1, 0]
    pub(super) fn op_rcomb_base(&mut self) -> Result<(), ExecutionError> {
        // -------------------- read the T_i(x) value from the stack ------------------------------
        let [v7, v6, v5, v4, v3, v2, v1, v0] = self.get_trace_values();

        // ---------------------- read the randomness from memory ---------------------------------
        let alpha = self.get_randomness();

        // ---------------------- read the OOD values from memory ---------------------------------
        let [tz, tgz] = self.get_ood_values();

        // ----------------------------- set the helper registers ---------------------------------
        self.set_helper_reg(alpha, tz, tgz);

        // ----------------------- read the accumulator values from stack -------------------------
        let [p_new, r_new] = self.compute_new_accumulator(tz, tgz, alpha);

        // ----------------------- rotate the top 8 elements of the stack -------------------------
        self.stack.set(15, v0);
        self.stack.set(14, v7);
        self.stack.set(13, v6);
        self.stack.set(12, v5);
        self.stack.set(11, v4);
        self.stack.set(10, v3);
        self.stack.set(9, v2);
        self.stack.set(8, v1);

        // -------------------------------- update the accumulators -------------------------------
        self.stack.set(7, p_new.to_base_elements()[1]);
        self.stack.set(6, p_new.to_base_elements()[0]);
        self.stack.set(5, r_new.to_base_elements()[1]);
        self.stack.set(4, r_new.to_base_elements()[0]);

        // ------------------------------- update the memory pointers -----------------------------
        self.stack.set(2, self.stack.get(2) + ONE);
        self.stack.set(1, self.stack.get(1) + ONE);

        Ok(())
    }

    //// HELPER METHODS
    //// --------------------------------------------------------------------------------------------

    /// Returns the top 8 elements of the operand stack representing cell values in the main trace
    /// i.e., base field elements.
    fn get_trace_values(&self) -> [Felt; 8] {
        let v7 = self.stack.get(15);
        let v6 = self.stack.get(14);
        let v5 = self.stack.get(13);
        let v4 = self.stack.get(12);
        let v3 = self.stack.get(11);
        let v2 = self.stack.get(10);
        let v1 = self.stack.get(9);
        let v0 = self.stack.get(8);

        [v7, v6, v5, v4, v3, v2, v1, v0]
    }

    /// Returns queried position in the folded domain.
    fn get_randomness(&mut self) -> QuadFelt {
        let ctx = self.system.ctx();
        let addr = self.stack.get(1);
        let word = self.chiplets.read_mem(ctx, addr);
        let a0 = word[0];
        let a1 = word[1];
        QuadFelt::new(a0, a1)
    }

    /// Returns segment identifier of the position in the source domain.
    ///
    /// This can be computed as pos / n, where pos is the position in the source domain and n is
    /// the size of the folded domain. Since source domain size is always 4x bigger than the
    /// folded domain size, possible set of values for domain segment are: 0, 1, 2, 3.
    fn get_ood_values(&mut self) -> [QuadFelt; 2] {
        let ctx = self.system.ctx();
        let addr = self.stack.get(2);
        let word = self.chiplets.read_mem(ctx, addr);

        [QuadFelt::new(word[0], word[1]), QuadFelt::new(word[2], word[3])]
    }

    /// Compute the updated accumulator values.
    fn compute_new_accumulator(
        &self,
        tz: QuadFelt,
        tgz: QuadFelt,
        alpha: QuadFelt,
    ) -> [QuadFelt; 2] {
        let p1 = self.stack.get(7);
        let p0 = self.stack.get(6);
        let p = QuadFelt::new(p0, p1);

        let r1 = self.stack.get(5);
        let r0 = self.stack.get(4);
        let r = QuadFelt::new(r0, r1);

        let v0 = self.stack.get(8);
        let tx = QuadFelt::new(v0, ZERO);

        [p * alpha + (tx - tz), r * alpha + (tx - tgz)]
    }

    /// Populates helper registers with intermediate values used in the folding procedure.
    fn set_helper_reg(&mut self, alpha: QuadFelt, tz: QuadFelt, tgz: QuadFelt) {
        let [a0, a1] = alpha.to_base_elements();
        let [tz0, tz1] = tz.to_base_elements();
        let [tgz0, tgz1] = tgz.to_base_elements();
        let values = [tz0, tz1, tgz0, tgz1, a0, a1];
        self.decoder.set_user_op_helpers(Operation::RCOMB1, &values);
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use rand_utils::rand_array;
    use vm_core::{Felt, FieldElement, Operation, StackInputs, ZERO};

    use crate::{Process, QuadFelt};

    #[test]
    fn rcombine_main() {
        // --- build stack inputs ---------------------------------------------
        let mut inputs = rand_array::<Felt, 16>();
        //let mut inputs: Vec<Felt> = (0..16).into_iter().map(Felt::new).collect();

        // set x_addr to
        inputs[12] = Felt::ZERO;

        // set z_addr pointer to x_addr + offset, where offset is a large enough value.
        let offset = Felt::new(1000);
        inputs[13] = inputs[12] + offset;

        // set a_addr to z_addr + offset
        inputs[14] = inputs[13] + offset;

        // --- execute RCOMB1 operation --------------------------------------
        let stack_inputs = StackInputs::new(inputs.to_vec());
        let mut process = Process::new_dummy_with_decoder_helpers(stack_inputs);

        // --- setup memory ---------------------------------------------
        let ctx = 0;
        let tztgz = rand_array::<Felt, 4>();
        process.chiplets.write_mem(ctx, inputs[13], tztgz);

        let a = rand_array::<Felt, 4>();
        process.chiplets.write_mem(ctx, inputs[14], a);

        process.execute_op(Operation::RCOMB1).unwrap();

        // --- check the stack state-------------------------------------------
        let mut stack_state = process.stack.trace_state();
        stack_state.reverse();

        // check that the top 8 stack elements are correctly rotated
        assert_eq!(stack_state[1], inputs[0]);
        assert_eq!(stack_state[2], inputs[1]);
        assert_eq!(stack_state[3], inputs[2]);
        assert_eq!(stack_state[4], inputs[3]);
        assert_eq!(stack_state[5], inputs[4]);
        assert_eq!(stack_state[6], inputs[5]);
        assert_eq!(stack_state[7], inputs[6]);
        assert_eq!(stack_state[0], inputs[7]);

        // check that the accumulator was updated correctly
        let p1 = inputs[8];
        let p0 = inputs[9];
        let p = QuadFelt::new(p0, p1);

        let r1 = inputs[10];
        let r0 = inputs[11];
        let r = QuadFelt::new(r0, r1);

        let tz0 = tztgz[0];
        let tz1 = tztgz[1];
        let tz = QuadFelt::new(tz0, tz1);
        let tgz0 = tztgz[2];
        let tgz1 = tztgz[3];
        let tgz = QuadFelt::new(tgz0, tgz1);

        let tx = QuadFelt::new(inputs[7], ZERO);

        let a0 = a[0];
        let a1 = a[1];
        let alpha = QuadFelt::new(a0, a1);

        // --- Compute new accumulator values ---------------------
        let p_new = p * alpha + (tx - tz);
        let r_new = r * alpha + (tx - tgz);

        assert_eq!(p_new.to_base_elements()[1], stack_state[8]);
        assert_eq!(p_new.to_base_elements()[0], stack_state[9]);
        assert_eq!(r_new.to_base_elements()[1], stack_state[10]);
        assert_eq!(r_new.to_base_elements()[0], stack_state[11]);
    }
}
