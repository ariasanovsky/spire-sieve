use libgdx_xs128::{rng::Random, RandomXS128};

use crate::seed::from::letter_index;

pub mod display;
pub mod from;

const LETTERS: usize = 35;
const BASE: i64 = LETTERS as i64;
static ALPHABET: &[u8; LETTERS] = b"0123456789ABCDEFGHIJKLMNPQRSTUVWXYZ";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Seed {
    pub(crate) seed: i64,
}

impl Seed {
    pub fn rng(&self) -> Random {
        self.offset_rng(0)
    }

    pub fn offset_rng(&self, offset: i64) -> Random {
        Random::new((self.seed.wrapping_add(offset)) as u64)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SeedString {
    pub(crate) seed: [u8; 13],
}

impl SeedString {
    pub const unsafe fn const_new(seed: &[u8; 13]) -> Self {
        Self { seed: *seed }
    }

    pub const fn const_seed(&self) -> Seed {
        let mut seed: i64 = 0;
        let mut i = 0;
        let string = &self.seed;
        while i < self.seed.len() {
            let letter = string[i];
            if letter != b' ' {
                let index = letter_index(letter);
                seed = seed.wrapping_mul(BASE);
                seed = seed.wrapping_add(index as i64);
            }
            i += 1;
        }
        Seed { seed }
    }
}
