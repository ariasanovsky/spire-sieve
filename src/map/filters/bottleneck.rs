use libgdx_xs128::RandomXS128;
use libgdx_xs128::rng::Random;

use crate::map::EliteBuff;
use crate::map::node_kind::NodeKind;
use crate::{filter::SeedFilter, map::Map};
use crate::map::in_neighborhood::in_vec::InVec;
use crate::map::out_neighborhood::out_vec::OutVec;
use crate::seed::Seed;

struct Bottleneck {
    row: usize,
}

impl Bottleneck {
    pub const fn new(floor: usize) -> Self {
        Self { row: floor - 1 }
    }
}

impl SeedFilter for Bottleneck {
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let map = Map::<6, InVec, OutVec>::generate(rng, true);
        map.row(self.row).count_out_neighborhoods() != 1
    }

    fn reject_seed(&self, seed: &Seed) -> bool {
        let mut rng = Random::new(seed.seed as u64 + 1);
        self.reject_rng(&mut rng)
    }
}

impl Default for Bottleneck {
    fn default() -> Self {
        Self::new(6)
    }
}

struct BuffedEliteBottleneck<'a> {
    row: usize,
    required_buffs: Option<&'a [EliteBuff]>,
}

impl<'a> BuffedEliteBottleneck<'a> {
    pub const fn new(floor: usize, required_buffs: Option<&'a [EliteBuff]>) -> Self {
        Self { row: floor - 1, required_buffs }
    }
}

impl<'a> Default for BuffedEliteBottleneck<'a> {
    fn default() -> Self {
        Self::new(6, None)
    }
}

impl<'a> SeedFilter for BuffedEliteBottleneck<'a> {
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let map = Map::<6, InVec, OutVec>::generate(rng, true);
        if map.row(self.row).count_out_neighborhoods() != 1 {
            return true;
        }
        !map.burning_elite(rng).is_some_and(|info| {
            !self.required_buffs.is_some_and(|required_buffs| {
                !required_buffs.contains(&info.buff)
            })
        })
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
            let filter = Bottleneck::default();
            assert!(!filter.reject_seed(&seed));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            let seed = Seed::from(seed);
            let filter = Bottleneck::default();
            assert!(filter.reject_seed(&seed));
        }
    }

    #[test]
    fn test_buffed_elite_bottleneck_filter() {
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
            let filter = BuffedEliteBottleneck::default();
            assert!(!filter.reject_seed(&seed));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            let seed = Seed::from(seed);
            let filter = BuffedEliteBottleneck::default();
            assert!(filter.reject_seed(&seed));
        }
    }
}