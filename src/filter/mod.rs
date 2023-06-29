use libgdx_xs128::{rng::Random, RandomXS128};

use crate::seed;

pub trait SeedFilter {
    fn reject_rng(&self, rng: &mut Random) -> bool;
    fn reject_seed(&self, seed: &seed::Seed) -> bool {
        let mut rng = Random::new(seed.seed as u64);
        self.reject_rng(&mut rng)
    }
}
