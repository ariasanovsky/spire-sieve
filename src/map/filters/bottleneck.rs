use libgdx_xs128::RandomXS128;
use libgdx_xs128::rng::Random;

use crate::{filter::SeedFilter, map::Map};
use crate::map::in_neighborhood::in_vec::InVec;
use crate::map::out_neighborhood::out_vec::OutVec;
use crate::seed::Seed;

struct Bottleneck {
    row: usize,
}

impl Bottleneck {
    pub fn new(floor: usize) -> Self {
        Self { row: floor - 1 }
    }
}

impl SeedFilter for Bottleneck {
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let map = Map::<6, InVec, OutVec>::generate(rng, false);
        map.row(self.row).count_out_neighborhoods() != 1
    }

    fn reject_seed(&self, seed: &Seed) -> bool {
        let mut rng = Random::new(seed.seed as u64 + 1);
        self.reject_rng(&mut rng)
    }
}

#[cfg(test)]
mod bottleneck_filter_tests {
    use std::str::FromStr;

    use crate::seed::SeedString;

    use super::*;

    #[test]
    fn test_bottleneck_filter() {
        for seed in [
            "8AFF4ZZ6",
            "XXKBUJNS",
            "1J432TK4I",
            "3QJ3DI01K",
            "3XTMF0PHJ",
            "3YT8RJBX1",
            "4DM63LTVA",
        ] {
            let seed = SeedString::from_str(seed).unwrap();
            let seed: Seed = seed.into();
            let filter = super::Bottleneck::new(6);
            assert!(!filter.reject_seed(&seed));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            let seed = Seed::from(seed);
            let filter = super::Bottleneck::new(6);
            assert!(filter.reject_seed(&seed));
        }
        
    }
}