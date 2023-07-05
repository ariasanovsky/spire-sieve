use libgdx_xs128::{rng::Random, RandomXS128};

use crate::seed::Seed;

pub trait SeedFilter {
    fn reject_rng(&self, rng: &mut Random) -> bool;
    fn reject<T: Into<Seed>>(&self, seed: T) -> bool {
        let seed: Seed = seed.into();
        let mut rng = Random::new(seed.seed as u64);
        self.reject_rng(&mut rng)
    }
}
