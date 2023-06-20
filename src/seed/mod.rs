mod display;
mod from;

const N_LETTERS: usize = 35;
const BASE: i64 = N_LETTERS as i64;
static ALPHABET: &[u8; N_LETTERS] = b"0123456789ABCDEFGHIJKLMNPQRSTUVWXYZ";

#[derive(Debug)]
pub struct Seed {
    pub(crate) seed: i64,
}

#[derive(Debug)]
pub struct SeedString {
    seed: String,
}
