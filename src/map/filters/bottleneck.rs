use libgdx_xs128::rng::Random;
use libgdx_xs128::RandomXS128;

use crate::map::assign_nodes::buffed_elite::EliteBuff;
use crate::map::in_neighborhood::in_vec::InVec;
use crate::map::out_neighborhood::out_vec::OutVec;
use crate::seed::Seed;
use crate::{filter::SeedFilter, map::Map};

pub struct Bottleneck {
    row: usize,
}

impl Bottleneck {
    pub const fn new(floor: usize) -> Self {
        Self { row: floor - 1 }
    }

    pub const fn const_default() -> Self {
        Self::new(6)
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
        Self::const_default()
    }
}

pub struct BuffedEliteBottleneck<'a> {
    row: usize,
    required_buffs: Option<&'a [EliteBuff]>,
}

impl<'a> BuffedEliteBottleneck<'a> {
    pub const fn new(floor: usize, required_buffs: Option<&'a [EliteBuff]>) -> Self {
        Self {
            row: floor - 1,
            required_buffs,
        }
    }

    pub const fn const_default() -> Self {
        Self::new(6, None)
    }
}

impl<'a> Default for BuffedEliteBottleneck<'a> {
    fn default() -> Self {
        Self::const_default()
    }
}

impl<'a> SeedFilter for BuffedEliteBottleneck<'a> {
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let map = Map::<6, InVec, OutVec>::generate(rng, true);
        if map.row(self.row).count_out_neighborhoods() != 1 {
            return true;
        }
        !map.burning_elite(rng).is_some_and(|info| {
            !self
                .required_buffs
                .is_some_and(|required_buffs| !required_buffs.contains(&info.buff))
        })
    }

    fn reject_seed(&self, seed: &Seed) -> bool {
        let mut rng = Random::new(seed.seed as u64 + 1);
        self.reject_rng(&mut rng)
    }
}

struct OnePath {
    length: usize,
}

impl OnePath {
    pub const fn new(length: usize) -> Self {
        Self { length }
    }
}

impl Default for OnePath {
    fn default() -> Self {
        Self::new(6)
    }
}

impl SeedFilter for OnePath {
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let map = Map::<6, InVec, OutVec>::generate(rng, true);
        for row in 1..self.length {
            if map.row(row).count_out_neighborhoods() != 1 {
                return true;
            }
        }
        false
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
            const FILTER: Bottleneck = Bottleneck::const_default();
            assert!(!FILTER.reject_seed(&seed));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            let seed = Seed::from(seed);
            const FILTER: Bottleneck = Bottleneck::const_default();
            assert!(FILTER.reject_seed(&seed));
        }
    }

    #[test]
    fn test_buffed_elite_bottleneck_filter() {
        const FILTER: BuffedEliteBottleneck = BuffedEliteBottleneck::const_default();
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
            assert!(!FILTER.reject_seed(&seed));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            let seed = Seed::from(seed);
            assert!(FILTER.reject_seed(&seed));
        }
    }

    #[test]
    fn test_one_path() {
        const FILTER: OnePath = OnePath::default();
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
            assert!(!FILTER.reject_seed(&seed));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            let seed = Seed::from(seed);
            assert!(FILTER.reject_seed(&seed));
        }
    }
}
