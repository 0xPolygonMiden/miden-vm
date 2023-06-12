use super::{crypto::hash::Rpo256, utils::collections::Vec, Felt, FieldElement};
use crypto::{hash::rpo::RpoDigest, Word, ZERO};
use math::StarkField;

// RE-EXPORTS
// ================================================================================================

pub use winter_crypto::{DefaultRandomCoin as WinterRandomCoin, RandomCoin, RandomCoinError};

// CONSTANTS
// ================================================================================================

const STATE_WIDTH: usize = Rpo256::STATE_WIDTH;
const RATE_START: usize = Rpo256::RATE_RANGE.start;
const RATE_END: usize = Rpo256::RATE_RANGE.end;
const HALF_RATE_WIDTH: usize = (Rpo256::RATE_RANGE.end - Rpo256::RATE_RANGE.start) / 2;

// RPO RANDOM COIN
// ================================================================================================
/// A simplified version of the `SPONGE_PRG` reseedable pseudo-random number generator algorithm
/// described in https://eprint.iacr.org/2011/499.pdf. The simplification is related to
/// to the following facts:
/// 1. A call to the reseed method implies one and only one call to the permutation function.
///  This is possible because in our case we never reseed with more than 4 field elements.
/// 2. As a result of the previous point, we dont make use of an input buffer to accumulate seed
///  material.
/// It is important to note that the current implementation of `RPORandomCoin` assumes that
/// `draw_integers()` is called immediately after `reseed_with_int()`.
pub struct RpoRandomCoin {
    state: [Felt; STATE_WIDTH],
    current: usize,
}

impl RpoRandomCoin {
    fn draw_basefield(&mut self) -> Felt {
        if self.current == RATE_END {
            Rpo256::apply_permutation(&mut self.state);
            self.current = RATE_START;
        }

        self.current += 1;
        self.state[self.current - 1]
    }
}

impl RandomCoin for RpoRandomCoin {
    type BaseField = Felt;
    type Hasher = Rpo256;

    fn new(seed: &[Self::BaseField]) -> Self {
        let mut state = [ZERO; STATE_WIDTH];

        let digest = Rpo256::hash_elements(seed);
        let digest_elem = digest.as_elements();

        for i in 0..HALF_RATE_WIDTH {
            state[RATE_START + i] += digest_elem[i];
        }

        // Absorb
        Rpo256::apply_permutation(&mut state);

        RpoRandomCoin {
            state,
            current: RATE_START,
        }
    }

    fn reseed(&mut self, data: RpoDigest) {
        // Reset buffer
        self.current = RATE_START;

        // Add the new seed material to the first half of the rate portion of the RPO state
        let data: Word = data.into();

        self.state[RATE_START] += data[0];
        self.state[RATE_START + 1] += data[1];
        self.state[RATE_START + 2] += data[2];
        self.state[RATE_START + 3] += data[3];

        // Absorb
        Rpo256::apply_permutation(&mut self.state);
    }

    fn reseed_with_int(&mut self, value: u64) {
        // Reset buffer
        self.current = RATE_START;

        let value = Felt::new(value);
        self.state[RATE_START] += value;
        Rpo256::apply_permutation(&mut self.state);
    }

    fn leading_zeros(&self) -> u32 {
        let first_rate_element = self.state[RATE_START].as_int();
        first_rate_element.trailing_zeros()
    }

    fn check_leading_zeros(&self, value: u64) -> u32 {
        let value = Felt::new(value);
        let mut state_tmp = self.state;

        state_tmp[RATE_START] += value;

        Rpo256::apply_permutation(&mut state_tmp);

        let first_rate_element = state_tmp[RATE_START].as_int();
        first_rate_element.trailing_zeros()
    }

    fn draw<E: FieldElement<BaseField = Felt>>(&mut self) -> Result<E, RandomCoinError> {
        let ext_degree = E::EXTENSION_DEGREE;
        let mut result = vec![ZERO; ext_degree];
        for r in result.iter_mut().take(ext_degree) {
            *r = self.draw_basefield();
        }

        let result = E::slice_from_base_elements(&result);
        Ok(result[0])
    }

    fn draw_integers(
        &mut self,
        num_values: usize,
        domain_size: usize,
    ) -> Result<Vec<usize>, RandomCoinError> {
        assert!(domain_size.is_power_of_two(), "domain size must be a power of two");
        assert!(num_values < domain_size, "number of values must be smaller than domain size");

        // Since the first element of the rate portion is used for proof-of-work and thus is not
        // random, we need to make sure that it is not used for generating a random index.
        self.current += 1;

        // determine how many bits are needed to represent valid values in the domain
        let v_mask = (domain_size - 1) as u64;

        // draw values from PRNG until we get as many unique values as specified by num_queries
        let mut values = Vec::new();
        for _ in 0..1000 {
            // get the next pseudo-random field element
            let value = self.draw_basefield().as_int();

            // use the mask to get a value within the range
            let value = (value & v_mask) as usize;

            if values.contains(&value) {
                continue;
            }
            values.push(value);
            if values.len() == num_values {
                break;
            }
        }

        if values.len() < num_values {
            return Err(RandomCoinError::FailedToDrawIntegers(num_values, values.len(), 1000));
        }

        Ok(values)
    }
}
