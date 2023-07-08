use libgdx_xs128::rng::Random;
use libgdx_xs128::RandomXS128;

#[cfg(feature = "std")]
use crate::map::assign_nodes::buffed_elite::EliteBuff;
use crate::map::in_neighborhood::InNeighborhood;
use crate::map::out_neighborhood::OutNeighborhood;
use crate::map::skeleton::Skeleton;
// use crate::map::in_neighborhood::in_vec::InVec;
// use crate::map::out_neighborhood::out_vec::OutVec;
use crate::seed::Seed;
use crate::filter::SeedFilter;

pub struct Bottleneck<In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    row: usize,
    _phantom: core::marker::PhantomData<(In, Out)>,
}

impl<In, Out> Bottleneck<In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,

{
    pub const fn new(floor: usize) -> Self {
        Self { row: floor - 1, _phantom: core::marker::PhantomData }
    }

    pub const fn const_default() -> Self {
        Self::new(6)
    }
}

impl<In, Out> SeedFilter for Bottleneck<In, Out>
where
    In: for<'a> InNeighborhood<'a> + Default,
    Out: for<'a> OutNeighborhood<'a> + Default,
{
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let map = Skeleton::<6, In, Out>::generate(rng);
        map.row(self.row).count_out_neighborhoods() != 1
    }

    fn reject<T: Into<Seed>>(&self, seed: T) -> bool {
        let seed: Seed = seed.into();
        let mut rng = Random::new(seed.seed as u64 + 1);
        self.reject_rng(&mut rng)
    }

    // fn reject(&self, seed: &Seed) -> bool {
    //     let mut rng = Random::new(seed.seed as u64 + 1);
    //     self.reject_rng(&mut rng)
    // }
}

impl<In, Out> Default for Bottleneck<In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    fn default() -> Self {
        Self::const_default()
    }
}

#[cfg(feature = "std")]
pub struct BurningEliteBottleneck<'a> {
    row: usize,
    required_buffs: Option<&'a [EliteBuff]>,
}

#[cfg(feature = "std")]
impl<'a> BurningEliteBottleneck<'a> {
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

#[cfg(feature = "std")]
impl<'a> Default for BurningEliteBottleneck<'a> {
    fn default() -> Self {
        Self::const_default()
    }
}

#[cfg(feature = "std")]
use crate::map::{
    in_neighborhood::in_vec::InVec,
    out_neighborhood::out_vec::OutVec,
    Map,
};

#[cfg(feature = "std")]
impl<'a> SeedFilter for BurningEliteBottleneck<'a> {
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

    fn reject<T: Into<Seed>>(&self, seed: T) -> bool {
        let seed: Seed = seed.into();
        let mut rng = Random::new(seed.seed as u64 + 1);
        self.reject_rng(&mut rng)
    }

    // fn reject(&self, seed: &Seed) -> bool {
    //     let mut rng = Random::new(seed.seed as u64 + 1);
    //     self.reject_rng(&mut rng)
    // }
}

struct OnePath<In, Out>
where
    In: for<'a> InNeighborhood<'a> + Default,
    Out: for<'a> OutNeighborhood<'a> + Default,
{
    length: usize,
    _phantom: core::marker::PhantomData<(In, Out)>,
}

impl<In, Out> OnePath<In, Out>
where
    In: for<'a> InNeighborhood<'a> + Default,
    Out: for<'a> OutNeighborhood<'a> + Default,
{
    pub const fn new(length: usize) -> Self {
        Self { length, _phantom: core::marker::PhantomData }
    }

    pub const fn const_default() -> Self {
        Self::new(6)
    }
}

impl<In, Out> Default for OnePath<In, Out>
where
    In: for<'a> InNeighborhood<'a> + Default,
    Out: for<'a> OutNeighborhood<'a> + Default,
{
    fn default() -> Self {
        Self::const_default()
    }
}

impl<In, Out> SeedFilter for OnePath<In, Out>
where
    In: for<'a> InNeighborhood<'a> + Default,
    Out: for<'a> OutNeighborhood<'a> + Default,
{
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let map = Skeleton::<6, In, Out>::generate(rng);
        for row in 1..self.length {
            if map.row(row).count_out_neighborhoods() != 1 {
                return true;
            }
        }
        false
    }

    fn reject<T: Into<Seed>>(&self, seed: T) -> bool {
        let seed: Seed = seed.into();
        let mut rng = Random::new(seed.seed as u64 + 1);
        self.reject_rng(&mut rng)
    }

    // fn reject(&self, seed: &Seed) -> bool {
    //     let mut rng = Random::new(seed.seed as u64 + 1);
    //     self.reject_rng(&mut rng)
    // }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod bottleneck_filter_tests {
    use crate::map::_ONE_PATH_BURNING_ELITE_BOTTLENECKS;

    use super::*;

    #[test]
    fn test_bottleneck_filter() {
        for &seed in _ONE_PATH_BURNING_ELITE_BOTTLENECKS {
            // let seed = SeedString::from_str(seed).unwrap();
            // let seed: Seed = seed.into();
            const FILTER: Bottleneck<InVec, OutVec> = Bottleneck::const_default();
            assert!(!FILTER.reject::<Seed>(seed.into()));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            // let seed = Seed::from(seed);
            const FILTER: Bottleneck<InVec, OutVec> = Bottleneck::const_default();
            assert!(FILTER.reject(seed));
        }
    }

    #[test]
    fn test_burning_elite_bottleneck_filter() {
        const FILTER: BurningEliteBottleneck = BurningEliteBottleneck::const_default();
        for &seed in _ONE_PATH_BURNING_ELITE_BOTTLENECKS {
            // let seed = SeedString::from_str(seed).unwrap();
            // let seed: Seed = seed.into();
            assert!(!FILTER.reject::<Seed>(seed.into()));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            let seed = Seed::from(seed);
            assert!(FILTER.reject(seed));
        }
    }

    #[test]
    fn test_one_path() {
        const FILTER: OnePath<InVec, OutVec> = OnePath::const_default();
        for &seed in _ONE_PATH_BURNING_ELITE_BOTTLENECKS {
            // let seed = SeedString::from_str(seed).unwrap();
            // let seed: Seed = seed.into();
            assert!(!FILTER.reject(seed));
        }

        for seed in [1u64, 2, 3, 4, 5] {
            let seed = Seed::from(seed);
            assert!(FILTER.reject(seed));
        }
    }
}
