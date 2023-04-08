use core::fmt;
use std::{marker::PhantomData};
use crypto::{Hasher, Digest};
use miden_air::{FieldElement, StarkField};
use winterfell::crypto;

pub struct RandomCoin<B, H>
where
    B: StarkField,
    H: Hasher,
{
    seed: H::Digest,
    counter: u64,
    _base_field: PhantomData<B>,
}

impl<B: StarkField, H: Hasher> RandomCoin<B, H> {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new random coin instantiated with the provided `seed`.
    pub fn new(seed: &[u8]) -> Self {
        let seed = H::hash(seed);
        RandomCoin {
            seed,
            counter: 0,
            _base_field: PhantomData,
        }
    }

    // RESEEDING
    // --------------------------------------------------------------------------------------------

    /// Reseeds the coin with the specified data by setting the new seed to hash(`seed` || `data`).
    ///
    /// # Examples
    /// ```
    /// # use winter_crypto::{RandomCoin, Hasher, hashers::Blake3_256};
    /// # use math::fields::f128::BaseElement;
    /// let mut coin1 = RandomCoin::<BaseElement, Blake3_256<BaseElement>>::new(&[1, 2, 3, 4]);
    /// let mut coin2 = RandomCoin::<BaseElement, Blake3_256<BaseElement>>::new(&[1, 2, 3, 4]);
    ///
    /// // should draw the same element form both coins
    /// let e1 = coin1.draw::<BaseElement>().unwrap();
    /// let e2 = coin2.draw::<BaseElement>().unwrap();;
    /// assert_eq!(e1, e2);
    ///
    /// // after reseeding should draw different elements
    /// coin2.reseed(Blake3_256::<BaseElement>::hash(&[2, 3, 4, 5]));
    /// let e1 = coin1.draw::<BaseElement>().unwrap();;
    /// let e2 = coin2.draw::<BaseElement>().unwrap();;
    /// assert_ne!(e1, e2);
    /// ```
    pub fn reseed(&mut self, data: H::Digest) {
        self.seed = H::merge(&[self.seed, data]);
        self.counter = 0;
    }

    /// Reseeds the coin with the specified value by setting the new seed to hash(`seed` ||
    /// `value`).
    ///
    /// # Examples
    /// ```
    /// # use winter_crypto::{RandomCoin, Hasher, hashers::Blake3_256};
    /// # use math::fields::f128::BaseElement;
    /// let mut coin1 = RandomCoin::<BaseElement, Blake3_256<BaseElement>>::new(&[1, 2, 3, 4]);
    /// let mut coin2 = RandomCoin::<BaseElement, Blake3_256<BaseElement>>::new(&[1, 2, 3, 4]);
    ///
    /// // should draw the same element form both coins
    /// let e1 = coin1.draw::<BaseElement>().unwrap();;
    /// let e2 = coin2.draw::<BaseElement>().unwrap();;
    /// assert_eq!(e1, e2);
    ///
    /// // after reseeding should draw different elements
    /// coin2.reseed_with_int(42);
    /// let e1 = coin1.draw::<BaseElement>().unwrap();;
    /// let e2 = coin2.draw::<BaseElement>().unwrap();;
    /// assert_ne!(e1, e2);
    /// ```
    pub fn reseed_with_int(&mut self, value: u64) {
        self.seed = H::merge_with_int(self.seed, value);
        self.counter = 0;
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns the number of leading zeros in the seed if it is interpreted as an integer in
    /// big-endian byte order.
    ///
    /// # Examples
    /// ```
    /// # use winter_crypto::{RandomCoin, hashers::Blake3_256};
    /// # use math::fields::f128::BaseElement;
    /// let mut coin = RandomCoin::<BaseElement, Blake3_256<BaseElement>>::new(&[1, 2, 3, 4]);
    ///
    /// let mut value = 0;
    /// while coin.check_leading_zeros(value) < 2 {
    ///     value += 1;
    /// }
    ///
    /// coin.reseed_with_int(value);
    /// assert!(coin.leading_zeros() >= 2);
    /// ```
    pub fn leading_zeros(&self) -> u32 {
        let bytes = self.seed.as_bytes();
        let seed_head = u64::from_le_bytes(bytes[..8].try_into().unwrap());
        seed_head.trailing_zeros()
    }

    /// Computes hash(`seed` || `value`) and returns the number of leading zeros in the resulting
    /// value if it is interpreted as an integer in big-endian byte order.
    pub fn check_leading_zeros(&self, value: u64) -> u32 {
        let new_seed = H::merge_with_int(self.seed, value);
        let bytes = new_seed.as_bytes();
        let seed_head = u64::from_le_bytes(bytes[..8].try_into().unwrap());
        seed_head.trailing_zeros()
    }

    // DRAW METHODS
    // --------------------------------------------------------------------------------------------

    /// Returns the next pseudo-random field element.
    ///
    /// # Errors
    /// Returns an error if a valid field element could not be generated after 1000 calls to the
    /// PRNG.
    pub fn draw<E>(&mut self) -> Result<E, RandomCoinError>
    where
        E: FieldElement<BaseField = B>,
    {
        for _ in 0..1000 {
            // get the next pseudo-random value and take the first ELEMENT_BYTES from it
            let value = self.next();
            let bytes = &value.as_bytes()[..E::ELEMENT_BYTES];

            // check if the bytes can be converted into a valid field element; if they can,
            // return; otherwise try again
            if let Some(element) = E::from_random_bytes(bytes) {
                return Ok(element);
            }
        }

        Err(RandomCoinError::FailedToDrawFieldElement(1000))
    }

    /// Returns the next pair of pseudo-random field elements.
    ///
    /// # Errors
    /// Returns an error if any of the field elements could not be generated after 100 calls to
    /// the PRNG;
    pub fn draw_pair<E>(&mut self) -> Result<(E, E), RandomCoinError>
    where
        E: FieldElement<BaseField = B>,
    {
        Ok((self.draw()?, self.draw()?))
    }

    /// Returns the next triplet of pseudo-random field elements.
    ///
    /// # Errors
    /// Returns an error if any of the field elements could not be generated after 100 calls to
    /// the PRNG;
    pub fn draw_triple<E>(&mut self) -> Result<(E, E, E), RandomCoinError>
    where
        E: FieldElement<BaseField = B>,
    {
        Ok((self.draw()?, self.draw()?, self.draw()?))
    }

    /// Returns a vector of unique integers selected from the range [0, domain_size).
    ///
    /// # Errors
    /// Returns an error if the specified number of unique integers could not be generated
    /// after 1000 calls to the PRNG.
    ///
    /// # Panics
    /// Panics if:
    /// - `domain_size` is not a power of two.
    /// - `num_values` is greater than or equal to `domain_size`.
    ///
    /// # Examples
    /// ```
    /// # use std::collections::HashSet;
    /// # use winter_crypto::{RandomCoin, hashers::Blake3_256};
    /// # use math::fields::f128::BaseElement;
    /// let mut coin = RandomCoin::<BaseElement, Blake3_256<BaseElement>>::new(&[1, 2, 3, 4]);
    ///
    /// let num_values = 20;
    /// let domain_size = 64;
    /// let values = coin.draw_integers(num_values, domain_size).unwrap();
    ///
    /// assert_eq!(num_values, values.len());
    ///
    /// let mut value_set = HashSet::new();
    /// for value in values {
    ///     assert!(value < domain_size);
    ///     assert!(value_set.insert(value));
    /// }
    /// ```
    pub fn draw_integers(
        &mut self,
        num_values: usize,
        domain_size: usize,
    ) -> Result<Vec<usize>, RandomCoinError> {
        assert!(
            domain_size.is_power_of_two(),
            "domain size must be a power of two"
        );
        assert!(
            num_values < domain_size,
            "number of values must be smaller than domain size"
        );

        // determine how many bits are needed to represent valid values in the domain
        let v_mask = (domain_size - 1) as u64;

        // draw values from PRNG until we get as many unique values as specified by num_queries
        let mut values = Vec::new();
        for i in 0..1000 {
            println!("number of iteration {:?}", i);
            // get the next pseudo-random value and read the first 8 bytes from it
            let bytes: [u8; 8] = self.next().as_bytes()[..8].try_into().unwrap();

            // convert to integer and limit the integer to the number of bits which can fit
            // into the specified domain
            let value = (u64::from_le_bytes(bytes) & v_mask) as usize;

            if values.contains(&value) {
                println!("duplicate!");
                continue;
            }
            values.push(value);
            if values.len() == num_values {
                break;
            }
        }

        if values.len() < num_values {
            return Err(RandomCoinError::FailedToDrawIntegers(
                num_values,
                values.len(),
                1000,
            ));
        }

        Ok(values)
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Updates the state by incrementing the counter and returns hash(seed || counter)
    fn next(&mut self) -> H::Digest {
        self.counter += 1;
        H::merge_with_int(self.seed, self.counter)
    }
}

// RANDOM COIN ERROR
// ================================================================================================

/// Defines errors which can occur when drawing values from a random coin.
#[derive(Debug, PartialEq, Eq)]
pub enum RandomCoinError {
    /// A valid element could not be drawn from the field after the specified number of tries.
    FailedToDrawFieldElement(usize),
    /// The required number of integer values could not be drawn from the specified domain after
    /// the specified number of tries.
    FailedToDrawIntegers(usize, usize, usize),
}

impl fmt::Display for RandomCoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToDrawFieldElement(num_tries) => {
                write!(
                    f,
                    "failed to generate a valid field element after {num_tries} tries"
                )
            }
            Self::FailedToDrawIntegers(num_expected, num_actual, num_tries) => {
                write!(
                    f,
                    "needed to draw {num_expected} integers from a domain, but drew only {num_actual} after {num_tries} tries"
                )
            }
        }
    }
}
