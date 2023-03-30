use super::{
    crypto::hash::{Rpo256, RpoDigest},
    utils::collections::Vec,
    Felt, FieldElement,
};

// RE-EXPORTS
// ================================================================================================

pub use winter_crypto::{DefaultRandomCoin as WinterRandomCoin, RandomCoin, RandomCoinError};

// RPO RANDOM COIN
// ================================================================================================

/// PRNG based on RPO hash function.
///
/// Right now, this is just a wrapper around [winter_crypto::DefaultRandomCoin], but in the future
/// this can be implemented more efficiently using sponge properties of RPO.
pub struct RpoRandomCoin(WinterRandomCoin<Rpo256>);

impl RpoRandomCoin {
    pub fn new(seed: &[Felt]) -> Self {
        Self(WinterRandomCoin::new(seed))
    }

    pub fn draw<E: FieldElement<BaseField = Felt>>(&mut self) -> Result<E, RandomCoinError> {
        RandomCoin::draw(self)
    }
}

impl RandomCoin for RpoRandomCoin {
    type BaseField = Felt;
    type Hasher = Rpo256;

    fn new(seed: &[Felt]) -> Self {
        Self(WinterRandomCoin::new(seed))
    }

    fn reseed(&mut self, data: RpoDigest) {
        self.0.reseed(data)
    }

    fn reseed_with_int(&mut self, value: u64) {
        self.0.reseed_with_int(value)
    }

    fn leading_zeros(&self) -> u32 {
        self.0.leading_zeros()
    }

    fn check_leading_zeros(&self, value: u64) -> u32 {
        self.0.check_leading_zeros(value)
    }

    fn draw<E: FieldElement<BaseField = Felt>>(&mut self) -> Result<E, RandomCoinError> {
        self.0.draw()
    }

    fn draw_integers(
        &mut self,
        num_values: usize,
        domain_size: usize,
    ) -> Result<Vec<usize>, RandomCoinError> {
        self.0.draw_integers(num_values, domain_size)
    }
}
