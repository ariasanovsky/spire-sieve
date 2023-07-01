use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use libgdx_xs128::{rng::Random, RandomXS128};

use crate::seed::{Seed, SeedString};

use super::DefaultMap;

const BAD_PATH_SEEDS: &[&str] = &[
    "8AFF4ZZ6",
    "XXKBUJNS",
    "1J432TK4I",
    "3QJ3DI01K",
    "3XTMF0PHJ",
    "3YT8RJBX1",
    "4DM63LTVA",
];

const OTHER_SEEDS: &[u64] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 1000, 10000, 100000, 1000000, 10000001,
];

#[test]
pub fn write_maps_to_file() {
    let received = PathBuf::from(".map_tests/received.txt");
    let mut received = File::create(&received).unwrap();

    for seed in BAD_PATH_SEEDS {
        let seed_string: SeedString = seed.parse().unwrap();
        let seed: Seed = seed_string.clone().into();
        let mut rng = Random::new(seed.seed as u64);
        let map = DefaultMap::generate(&mut rng, true);
        let map = map.to_string();
        let map_string = format!("{seed_string},{seed:?}\n{map}\n\n");
        received.write_all(map_string.as_bytes()).unwrap();
    }

    for (i, seed) in OTHER_SEEDS.iter().enumerate() {
        let seed = Seed::from(*seed);
        let seed_string = SeedString::from(seed.clone());
        let mut rng = Random::new(seed.seed as u64);
        let ascension = i % 2 == 0;
        let map = DefaultMap::generate(&mut rng, ascension);
        let map_string = format!("{seed_string},{seed:?}\n{map}\n\n");
        received.write_all(map_string.as_bytes()).unwrap();
    }

    let approved = PathBuf::from(".map_tests/approved.txt");
    let approved = fs::read_to_string(&approved).unwrap();

    let received = PathBuf::from(".map_tests/received.txt");
    let received = fs::read_to_string(&received).unwrap();

    assert_eq!(approved, received);
}
