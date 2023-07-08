use std::println;

use crate::{filter::SeedFilter, seed::SeedString};

pub struct Sieve<F: SeedFilter> {
    start: u64,
    end: u64,
    filter: F,
}

impl<F: SeedFilter> Sieve<F> {
    pub const fn new(start: u64, end: u64, filter: F) -> Self {
        Self { start, end, filter }
    }

    pub fn run(&self) {
        for seed in self.start..=self.end {
            if !self.filter.reject(seed) {
                println!("{}", SeedString::from(seed));
            }
        }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test_sieve {
    use super::*;
    use crate::map::{filters::bottleneck::Bottleneck, _ONE_PATH_BURNING_ELITE_BOTTLENECKS, in_neighborhood::in_vec::InVec, out_neighborhood::out_vec::OutVec};

    #[test]
    fn test_sieve_finds_bottleneck_seed() {
        // const START: i64 = 1i64;
        // const END: SeedString = unsafe { SeedString::const_new(
        //     b"            Z"
        // )};
        // todo! const trait bounds ;(

        const START: u64 = unsafe {
            SeedString::const_new(_ONE_PATH_BURNING_ELITE_BOTTLENECKS[0])
                .const_seed()
                .seed as u64
                - 10
        };
        const END: u64 = START + 20;

        // const BOTTLNECK: u64

        const FILTER: Bottleneck<InVec, OutVec> = Bottleneck::const_default();
        const SIEVE: Sieve<Bottleneck<InVec, OutVec>> = Sieve::new(START, END, FILTER);
        SIEVE.run();
    }
}
