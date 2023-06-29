pub mod display;
pub mod from;

const LETTERS: usize = 35;
const BASE: i64 = LETTERS as i64;
static ALPHABET: &[u8; LETTERS] = b"0123456789ABCDEFGHIJKLMNPQRSTUVWXYZ";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Seed {
    pub(crate) seed: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SeedString {
    pub(crate) seed: String,
}
