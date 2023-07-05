use std::str::FromStr;

use super::{Seed, SeedString, ALPHABET, BASE};

fn letter_index(letter: u8) -> u8 {
    match letter {
        _ if letter < b'A' => letter.wrapping_sub(b'0'),
        _ if letter < b'O' => letter.wrapping_sub(b'A').wrapping_add(10),
        _ => letter.wrapping_sub(b'A').wrapping_add(9),
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidCharacter(u8),
    InvalidLength(usize),
}

impl FromStr for SeedString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seed: [u8; 13] = [b' '; 13];
        let mut i = 13;
        for c in s.bytes().rev() {
            if i == 0 {
                return Err(Error::InvalidLength(s.bytes().len()));
            }
            i -= 1;
            seed[i] = match c {
                b' ' => continue,
                b'O' => b'0',
                b'0'..=b'9' | b'A'..=b'N' | b'P'..=b'Z' => c,
                _ => return Err(Error::InvalidCharacter(c)),
            };
        }
        Ok(Self { seed })
    }
}

impl From<SeedString> for Seed {
    fn from(value: SeedString) -> Self {
        value.seed.into()
    }
}

impl From<Seed> for SeedString {
    fn from(value: Seed) -> Self {
        let mut seed = value.seed;
        let mut s: [u8; 13] = [b' '; 13];
        for i in (0..13).rev() {
            if seed == 0 {
                break;
            }
            let c = seed % BASE;
            seed /= BASE;
            let c = ALPHABET[c as usize];
            s[i] = c;
        }
        Self { seed: s }
    }
}

impl From<[u8; 13]> for Seed {
    fn from(value: [u8; 13]) -> Self {
        let value: &[u8; 13] = &value;
        value.into()
    }
}

impl From<&[u8; 13]> for Seed {
    fn from(value: &[u8; 13]) -> Self {
        let mut seed: i64 = 0;
        value
            .iter()
            .skip_while(|c| **c == b' ')
            //.map(|c| letter_index(*c))
            .for_each(|c| {
                let index = letter_index(*c);
                seed = seed.wrapping_mul(BASE);
                seed = seed.wrapping_add(index as i64);
            });
        Self { seed }
    }
}

impl From<i64> for Seed {
    fn from(seed: i64) -> Self {
        Self { seed }
    }
}

impl From<u64> for Seed {
    fn from(seed: u64) -> Self {
        Self { seed: seed as i64 }
    }
}

#[cfg(test)]
mod test_seed_conversions {

    use super::super::{Seed, SeedString};

    #[test]
    fn seed_zero() {
        let seed = Seed::from(0i64);
        assert_eq!(seed.seed, 0);

        let seed = Seed::from(0u64);
        assert_eq!(seed.seed, 0);

        let seed = Seed::from(b"            0");
        assert_eq!(seed.seed, 0);

        let seed_string: SeedString = String::from("0").parse().unwrap();
        let seed = Seed::from(seed_string.clone());
        dbg!(&seed);
        assert_eq!(seed.seed, 0, "{seed_string}");
    }

    #[test]
    fn random_positive_seed() {
        let seed = Seed::from(3218453378341624389i64);
        assert_eq!(seed.seed, 3218453378341624389);

        let seed = Seed::from(3218453378341624389u64);
        assert_eq!(seed.seed, 3218453378341624389);

        let seed = Seed::from(b" YBQ7FPFZSX1U");
        assert_eq!(seed.seed, 3218453378341624389);

        let seed: Seed = String::from("YBQ7FPFZSX1U")
            .parse::<SeedString>()
            .unwrap()
            .into();
        assert_eq!(seed.seed, 3218453378341624389);
    }

    #[test]
    fn random_negative_seed() {
        let seed = Seed::from(-706882697283956955i64);
        assert_eq!(seed.seed, -706882697283956955);

        let seed = Seed::from(-706882697283956955i64 as u64);
        assert_eq!(seed.seed, -706882697283956955);

        let seed = Seed::from(b"58QVGLNE8PU3W");
        assert_eq!(seed.seed, -706882697283956955);

        let seed: Seed = String::from("58QVGLNE8PU3W")
            .parse::<SeedString>()
            .unwrap()
            .into();
        assert_eq!(seed.seed, -706882697283956955);
    }
}
