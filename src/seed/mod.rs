mod display;
mod from;

const N_LETTERS: usize = 35;
const BASE: i64 = N_LETTERS as i64;
static ALPHABET: &[u8; N_LETTERS] = b"0123456789ABCDEFGHIJKLMNPQRSTUVWXYZ";

#[derive(Debug)]
struct Seed {
    seed: i64,
}

#[derive(Debug)]
struct SeedString {
    seed: String,
}
