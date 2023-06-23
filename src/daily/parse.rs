use std::str::FromStr;

use libgdx_xs128::rng::Random;
use libgdx_xs128::SeedInitializer;

use crate::neow::NeowBonuses;
use crate::seed::Seed;
use crate::seed::SeedString;

use super::Daily;

#[derive(Debug)]
pub enum Error {
    NoDate,
    NoCharacter,
    NoStarterMod,
    NoGenericMod,
    NoDifficultyMod,
    NoSeedString,
    ParseSeed(std::num::ParseIntError),
    ParseCharacter(crate::character::Error),
    ParseDailyModifier(crate::daily::modifier::Error),
    ParseSeedString(crate::seed::from::Error),
    InconsistentSeeds(Seed, SeedString),
}

impl FromStr for Daily {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(',');
        let date = splits.next().ok_or(Error::NoDate)?;
        let character = splits
            .next()
            .ok_or(Error::NoCharacter)?
            .parse()
            .map_err(Error::ParseCharacter)?;

        let starter_mod = splits
            .next()
            .ok_or(Error::NoStarterMod)?
            .parse()
            .map_err(Error::ParseDailyModifier)?;
        let generic_mod = splits
            .next()
            .ok_or(Error::NoGenericMod)?
            .parse()
            .map_err(Error::ParseDailyModifier)?;
        let difficulty_mod = splits
            .next()
            .ok_or(Error::NoDifficultyMod)?
            .parse()
            .map_err(Error::ParseDailyModifier)?;

        let seed_string: SeedString = splits
            .next()
            .ok_or(Error::NoSeedString)?
            .parse()
            .map_err(Error::ParseSeedString)?;
        let seed: Seed = match splits.next() {
            Some(seed) => {
                let seed: i64 = seed.parse().map_err(Error::ParseSeed)?;
                let seed = Seed { seed };
                let other_seed_string: SeedString = seed.clone().into();
                if seed_string != other_seed_string {
                    return Err(Error::InconsistentSeeds(seed, seed_string));
                }
                seed
            }
            None => seed_string.clone().into(),
        };

        let no_combat_paths = splits.next();
        let no_combat_paths_ascension = splits.next();

        let seed_initializer: SeedInitializer = seed.seed.into();
        let rng: Random = seed_initializer.into();
        let neow_bonuses = NeowBonuses::generate(rng);

        Ok(Self {
            date: date.into(),
            character,
            starter_mod,
            generic_mod,
            difficulty_mod,
            seed_string,
            seed,
            no_combat_paths: no_combat_paths.map(Into::into),
            no_combat_paths_ascension: no_combat_paths_ascension.map(Into::into),
            neow_bonuses,
        })
    }
}
