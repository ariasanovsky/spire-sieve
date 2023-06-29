use libgdx_xs128::{rng::Random, RandomXS128};

use crate::seed;

pub trait Filter {
    fn accept_rng(&self, rng: &mut Random) -> bool;
    fn accept_seed(&self, seed: &seed::Seed) -> bool {
        let mut rng = Random::new(seed.seed as u64);
        self.accept_rng(&mut rng)
    }
}
