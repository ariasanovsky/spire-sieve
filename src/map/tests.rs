use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use libgdx_xs128::{rng::Random, RandomXS128};

use crate::{
    map::{in_neighborhood::in_vec::InVec, out_neighborhood::out_vec::OutVec, Map},
    seed::{Seed, SeedString},
};

use super::{
    in_neighborhood::{in_byte::InByte, InNeighborhood},
    out_neighborhood::OutNeighborhood,
};

const BAD_PATH_SEEDS: &[&str] = &[
    "8AFF4ZZ6",
    "XXKBUJNS",
    "1J432TK4I",
    "3QJ3DI01K",
    "3XTMF0PHJ",
    "3YT8RJBX1",
    "4DM63LTVA",
];

#[test]
fn write_maps_to_file() {
    let received = PathBuf::from(".map_tests/received.txt");
    let mut received = File::create(&received).unwrap();

    for seed in BAD_PATH_SEEDS {
        let seed_string: SeedString = seed.parse().unwrap();
        let seed: Seed = seed_string.clone().into();
        let mut rng = Random::new(seed.seed as u64);
        let map: Map<6, InVec, OutVec> = Map::generate(&mut rng, true);
        let map = map.to_string();
        let map_string = format!("{seed_string},{seed:?}\n{map}\n\n");
        received.write_all(map_string.as_bytes()).unwrap();
    }

    for (i, seed) in (1u64..10_000).enumerate() {
        let seed = Seed::from(seed);
        let seed_string = SeedString::from(seed.clone());
        let mut rng = Random::new(seed.seed as u64);
        let ascension = i % 2 == 0;
        let map: Map<6, InVec, OutVec> = Map::generate(&mut rng, ascension);
        let map_string = format!("{seed_string},{seed:?}\n{map}\n\n");
        received.write_all(map_string.as_bytes()).unwrap();
    }

    let approved = PathBuf::from(".map_tests/approved.txt");
    let approved = fs::read_to_string(&approved).unwrap();

    let received = PathBuf::from(".map_tests/received.txt");
    let received = fs::read_to_string(&received).unwrap();

    assert_eq!(approved, received, "");
}

#[test]
fn compare_invec_to_inbytes() {
    for seed in (533907583096i64 + 3..533907583096 + 10) {
        let seed = Seed::from(seed);
        let first_map_string = map_string::<InVec, OutVec>(seed.clone(), true);
        // println!("VEC:\n{first_map_string}");
        let second_map_string = map_string::<InByte, OutVec>(seed.clone(), true);
        // println!("BYTE:\n{second_map_string}");
        assert_eq!(
            first_map_string,
            second_map_string,
            "seed: {seed:?}, {}",
            SeedString::from(seed.clone())
        );
    }
}

fn map_string<In, Out>(seed: Seed, ascension: bool) -> String
where
    In: for<'a> InNeighborhood<'a> + Default,
    Out: for<'a> OutNeighborhood<'a> + Default,
{
    let seed_string = SeedString::from(seed.clone());
    let mut rng = Random::new(seed.seed as u64);
    let map: Map<6, In, Out> = Map::generate(&mut rng, ascension);
    let map_string = format!("{seed_string},{seed:?}\n{map}\n\n");
    map_string
}
