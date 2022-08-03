use super::{Felt, FieldElement, HasherState, LookupTableRow, StarkField};
use vm_core::chiplets::hasher::{
    CAPACITY_LEN, DIGEST_RANGE, LINEAR_HASH_LABEL, MP_VERIFY_LABEL, MR_UPDATE_NEW_LABEL,
    MR_UPDATE_OLD_LABEL, RETURN_HASH_LABEL, RETURN_STATE_LABEL, STATE_WIDTH,
};

// CONSTANTS
// ================================================================================================
const NUM_HEADER_ALPHAS: usize = 4;

// HASHER LOOKUPS
// ================================================================================================

/// Specifies the context of the [HasherLookup], indicating whether it describes the beginning of a
/// hash operation, the return of a specified result, or the absorption of additional elements,
/// initiating a new hash cycle with the provided [HasherState].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HasherLookupContext {
    Start,
    // TODO: benchmark removing this and getting it from the trace instead
    Absorb(HasherState),
    Return,
}

/// Contains the data required to describe and verify hash operations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HasherLookup {
    // unique label identifying the hash operation
    label: u8,
    // TODO: benchmark removing this and getting it from the trace instead
    // hasher state
    state: HasherState,
    // row address in the Hasher table
    addr: u32,
    // node index
    index: Felt,
    // context
    context: HasherLookupContext,
}

impl HasherLookup {
    /// Creates a new HasherLookup.
    pub(super) fn new(
        label: u8,
        state: HasherState,
        addr: u32,
        index: Felt,
        context: HasherLookupContext,
    ) -> Self {
        Self {
            label,
            state,
            addr,
            index,
            context,
        }
    }

    /// The cycle at which the lookup is provided by the hasher.
    pub(super) fn cycle(&self) -> usize {
        // the hasher's addresses start from one instead of zero, so the cycle at which each lookup
        // is provided is one less than its address
        self.addr as usize - 1
    }

    /// Returns the common header value which describes this hash operation. It is a combination of
    /// the transition label, the row address, and the node index.
    fn get_header_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        let transition_label = match self.context {
            HasherLookupContext::Start => E::from(self.label) + E::from(16_u8),
            _ => E::from(self.label) + E::from(32_u8),
        };

        alphas[0]
            + alphas[1].mul(transition_label)
            + alphas[2].mul(E::from(self.addr))
            + alphas[3].mul_base(self.index)
    }
}

impl LookupTableRow for HasherLookup {
    /// Reduces this row to a single field element in the field specified by E. This requires
    /// at least 16 alpha values.
    fn to_value<E: FieldElement<BaseField = Felt>>(&self, alphas: &[E]) -> E {
        let header = self.get_header_value(&alphas[..NUM_HEADER_ALPHAS]);
        // computing the rest of the value requires an alpha for each element in the [HasherState]
        let alphas = &alphas[NUM_HEADER_ALPHAS..(NUM_HEADER_ALPHAS + STATE_WIDTH)];

        match self.context {
            HasherLookupContext::Start => {
                if self.label == LINEAR_HASH_LABEL {
                    // include the entire state when initializing a linear hash.
                    header + build_value(alphas, &self.state)
                } else {
                    assert!(
                        self.label == MR_UPDATE_OLD_LABEL
                            || self.label == MR_UPDATE_NEW_LABEL
                            || self.label == MP_VERIFY_LABEL,
                        "unrecognized hash operation"
                    );
                    // build the leaf value by selecting from the left and right words of the state.
                    // the same alphas must be used in both cases, since whichever word is selected
                    // by the index bit will be the leaf node, and the value must be computed in the
                    // same way in both cases.
                    let bit = (self.index.as_int() >> 1) & 1;
                    let left_word = build_value(&alphas[DIGEST_RANGE], &self.state[DIGEST_RANGE]);
                    let right_word =
                        build_value(&alphas[DIGEST_RANGE], &self.state[DIGEST_RANGE.end..]);

                    header + E::from(1 - bit).mul(left_word) + E::from(bit).mul(right_word)
                }
            }
            HasherLookupContext::Absorb(next_state) => {
                assert!(
                    self.label == LINEAR_HASH_LABEL,
                    "unrecognized hash operation"
                );
                // build the value from the delta of the hasher state's rate before and after the
                // absorption of new elements.
                let next_state_value =
                    build_value(&alphas[CAPACITY_LEN..], &next_state[CAPACITY_LEN..]);
                let state_value = build_value(&alphas[CAPACITY_LEN..], &self.state[CAPACITY_LEN..]);

                header + next_state_value - state_value
            }
            HasherLookupContext::Return => {
                if self.label == RETURN_STATE_LABEL {
                    // build the value from the result, which is the entire state
                    header + build_value(alphas, &self.state)
                } else {
                    assert!(
                        self.label == RETURN_HASH_LABEL,
                        "unrecognized hash operation"
                    );
                    // build the value from the result, which is the digest portion of the state
                    header + build_value(&alphas[DIGEST_RANGE], &self.state[DIGEST_RANGE])
                }
            }
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Reduces a slice of elements to a single field element in the field specified by E using a slice
/// of alphas of matching length. This can be used to build the value for a single word or for an
/// entire [HasherState].
fn build_value<E: FieldElement<BaseField = Felt>>(alphas: &[E], elements: &[Felt]) -> E {
    let mut value = E::ZERO;
    for (&alpha, &element) in alphas.iter().zip(elements.iter()) {
        value += alpha.mul_base(element);
    }
    value
}
