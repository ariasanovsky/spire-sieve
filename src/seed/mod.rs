use libgdx_xs128::{rng::Random, RandomXS128};

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
