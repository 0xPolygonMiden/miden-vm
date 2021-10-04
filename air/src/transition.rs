use super::{utils::binary_not, TraceState};
use distaff_processor::{OpCode, NUM_CF_OPS, NUM_HD_OPS, NUM_LD_OPS};
use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    EvaluationFrame,
};

// VM TRANSITION
// ================================================================================================

pub struct VmTransition<E: FieldElement<BaseField = BaseElement>> {
    current: TraceState<E>,
    next: TraceState<E>,
    cf_op_flags: [E; NUM_CF_OPS],
    ld_op_flags: [E; NUM_LD_OPS],
    hd_op_flags: [E; NUM_HD_OPS],
    begin_flag: E,
    noop_flag: E,
}

impl<E: FieldElement<BaseField = BaseElement>> VmTransition<E> {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------

    pub fn new(ctx_depth: usize, loop_depth: usize, stack_depth: usize) -> Self {
        Self {
            current: TraceState::new(ctx_depth, loop_depth, stack_depth),
            next: TraceState::new(ctx_depth, loop_depth, stack_depth),
            cf_op_flags: [E::ZERO; NUM_CF_OPS],
            ld_op_flags: [E::ZERO; NUM_LD_OPS],
            hd_op_flags: [E::ZERO; NUM_HD_OPS],
            begin_flag: E::ZERO,
            noop_flag: E::ZERO,
        }
    }

    #[cfg(test)]
    pub fn from_states(current: TraceState<E>, next: TraceState<E>) -> Self {
        let mut result = Self {
            current,
            next,
            cf_op_flags: [E::ZERO; NUM_CF_OPS],
            ld_op_flags: [E::ZERO; NUM_LD_OPS],
            hd_op_flags: [E::ZERO; NUM_HD_OPS],
            begin_flag: E::ZERO,
            noop_flag: E::ZERO,
        };
        result.set_op_flags();
        result
    }

    // DATA MUTATORS
    // --------------------------------------------------------------------------------------------

    pub fn update(&mut self, frame: &EvaluationFrame<E>) {
        self.current.update(frame.current());
        self.next.update(frame.next());
        self.set_op_flags();
    }

    // STATE ACCESSORS
    // --------------------------------------------------------------------------------------------

    pub fn current(&self) -> &TraceState<E> {
        &self.current
    }

    pub fn next(&self) -> &TraceState<E> {
        &self.next
    }

    // OP FLAGS
    // --------------------------------------------------------------------------------------------
    pub fn cf_op_flags(&self) -> [E; NUM_CF_OPS] {
        self.cf_op_flags
    }

    pub fn ld_op_flags(&self) -> [E; NUM_LD_OPS] {
        self.ld_op_flags
    }

    pub fn hd_op_flags(&self) -> [E; NUM_HD_OPS] {
        self.hd_op_flags
    }

    pub fn begin_flag(&self) -> E {
        self.begin_flag
    }

    pub fn noop_flag(&self) -> E {
        self.noop_flag
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    fn set_op_flags(&mut self) {
        // set control flow flags
        let not_0 = binary_not(self.current.cf_op_bits()[0]);
        let not_1 = binary_not(self.current.cf_op_bits()[1]);
        self.cf_op_flags[0] = not_0 * not_1;
        self.cf_op_flags[1] = self.current.cf_op_bits()[0] * not_1;
        self.cf_op_flags[2] = not_0 * self.current.cf_op_bits()[1];
        self.cf_op_flags[3] = self.current.cf_op_bits()[0] * self.current.cf_op_bits()[1];
        self.cf_op_flags.copy_within(0..4, 4);

        let not_2 = binary_not(self.current.cf_op_bits()[2]);
        for i in 0..4 {
            self.cf_op_flags[i] *= not_2;
        }
        for i in 4..8 {
            self.cf_op_flags[i] *= self.current.cf_op_bits()[2];
        }

        // set low-degree operation flags
        let not_0 = binary_not(self.current.ld_op_bits()[0]);
        let not_1 = binary_not(self.current.ld_op_bits()[1]);
        self.ld_op_flags[0] = not_0 * not_1;
        self.ld_op_flags[1] = self.current.ld_op_bits()[0] * not_1;
        self.ld_op_flags[2] = not_0 * self.current.cf_op_bits()[1];
        self.ld_op_flags[3] = self.current.ld_op_bits()[0] * self.current.ld_op_bits()[1];
        self.ld_op_flags.copy_within(0..4, 4);

        let not_2 = binary_not(self.current.ld_op_bits()[2]);
        for i in 0..4 {
            self.ld_op_flags[i] *= not_2;
        }
        for i in 4..8 {
            self.ld_op_flags[i] *= self.current.ld_op_bits()[2];
        }
        self.ld_op_flags.copy_within(0..8, 8);

        let not_3 = binary_not(self.current.ld_op_bits()[3]);
        for i in 0..8 {
            self.ld_op_flags[i] *= not_3;
        }
        for i in 8..16 {
            self.ld_op_flags[i] *= self.current.ld_op_bits()[3];
        }
        self.ld_op_flags.copy_within(0..16, 16);

        let not_4 = binary_not(self.current.ld_op_bits()[4]);
        for i in 0..16 {
            self.ld_op_flags[i] *= not_4;
        }
        for i in 16..32 {
            self.ld_op_flags[i] *= self.current.ld_op_bits()[4];
        }

        // set high-degree operation flags
        let not_0 = binary_not(self.current.hd_op_bits()[0]);
        let not_1 = binary_not(self.current.hd_op_bits()[1]);
        self.hd_op_flags[0] = not_0 * not_1;
        self.hd_op_flags[1] = self.current.hd_op_bits()[0] * not_1;
        self.hd_op_flags[2] = not_0 * self.current.hd_op_bits()[1];
        self.hd_op_flags[3] = self.current.hd_op_bits()[0] * self.current.hd_op_bits()[1];

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
        self.hd_op_flags[0] *= self.current.ld_op_bits()[0];

        debug_assert!(OpCode::Assert.ld_index() == 0, "ASSERT index is not 0!");
        self.ld_op_flags[0] *= self.current.hd_op_bits()[0];
    }
}

// TESTS
// ================================================================================================
#[cfg(test)]
mod tests {
    use super::{super::utils::ToElements, VmTransition};
    use winterfell::{
        math::{fields::f128::BaseElement, FieldElement, StarkField},
        EvaluationFrame,
    };

    #[test]
    fn op_flags() {
        // all zeros
        let transition = vm_transition_from_current(&[
            101, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 16, 17,
        ]);

        assert_eq!(
            [1, 0, 0, 0, 0, 0, 0, 0].to_elements(),
            transition.cf_op_flags()
        );
        assert_eq!(
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
            .to_elements(),
            transition.ld_op_flags()
        );
        assert_eq!([0, 0, 0, 0].to_elements(), transition.hd_op_flags());
        assert_eq!(1, transition.begin_flag().as_int());
        assert_eq!(0, transition.noop_flag().as_int());

        // all ones
        let transition = vm_transition_from_current(&[
            101, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 15, 16, 17,
        ]);

        assert_eq!(
            [0, 0, 0, 0, 0, 0, 0, 1].to_elements(),
            transition.cf_op_flags()
        );
        assert_eq!(
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ]
            .to_elements(),
            transition.ld_op_flags()
        );
        assert_eq!([0, 0, 0, 1].to_elements(), transition.hd_op_flags());
        assert_eq!(0, transition.begin_flag().as_int());
        assert_eq!(1, transition.noop_flag().as_int());

        // mixed 1
        let transition = vm_transition_from_current(&[
            101, 1, 2, 3, 4, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 15, 16, 17,
        ]);

        assert_eq!(
            [0, 1, 0, 0, 0, 0, 0, 0].to_elements(),
            transition.cf_op_flags()
        );
        assert_eq!(
            [
                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
            .to_elements(),
            transition.ld_op_flags()
        );
        assert_eq!([0, 1, 0, 0].to_elements(), transition.hd_op_flags());
        assert_eq!(0, transition.begin_flag().as_int());
        assert_eq!(0, transition.noop_flag().as_int());

        // mixed 2
        let transition = vm_transition_from_current(&[
            101, 1, 2, 3, 4, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 15, 16, 17,
        ]);

        assert_eq!(
            [0, 0, 0, 1, 0, 0, 0, 0].to_elements(),
            transition.cf_op_flags()
        );
        assert_eq!(
            [
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
            .to_elements(),
            transition.ld_op_flags()
        );
        assert_eq!([0, 0, 1, 0].to_elements(), transition.hd_op_flags());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn vm_transition_from_current(current_row: &[u128]) -> VmTransition<BaseElement> {
        let mut result = VmTransition::new(1, 0, 2);
        let current = current_row.iter().map(|&v| BaseElement::new(v)).collect();
        let frame = EvaluationFrame::from_rows(current, vec![BaseElement::ZERO; current_row.len()]);
        result.update(&frame);
        result
    }
}
