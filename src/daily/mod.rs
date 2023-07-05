use crate::{
    character::Character,
    neow::NeowBonuses,
    seed::{Seed, SeedString},
};

use self::modifier::{DifficultyMod, GenericMod, StarterMod};

pub mod modifier;
pub mod parse;

#[derive(Debug, PartialEq, Eq)]
pub struct Daily {
    pub date: String,
    pub character: Character,
    pub starter_mod: StarterMod,
    pub generic_mod: GenericMod,
    pub difficulty_mod: DifficultyMod,
    pub seed_string: SeedString,
    pub seed: Seed,
    pub no_combat_paths: Option<String>,
    pub no_combat_paths_ascension: Option<String>,
    pub neow_bonuses: NeowBonuses,
}

#[cfg(test)]
mod test_daily_parser {
    //use crate::neow::{Drawback, FirstBonus, SecondBonus, ThirdBonus};

    use super::*;

    static FIRST_DAILIES: [&str; 10] = [
        "1601/01/02,IRONCLAD,Insanity,Purple Cards,DeadlyEvents,E06ALSGPMJ59",
        "1601/01/03,IRONCLAD,Heirloom,Flight,Midas,4MKRHI0VBSRTJ",
        "1601/01/04,THE_SILENT,Draft,Blue Cards,Terminal,1BKMD2MXNW8ZP",
        "1601/01/05,DEFECT,Specialized,Colorless Cards,Lethality,3VPY3QN5WFLVN",
        "1601/01/06,DEFECT,Heirloom,Purple Cards,Night Terrors,QKLWQHKXEBD6",
        "1601/01/07,DEFECT,Specialized,Green Cards,Lethality,2AS174BGWFTWM",
        "1601/01/08,IRONCLAD,Chimera,Vintage,DeadlyEvents,GM7H4007EIF5",
        "1601/01/09,WATCHER,SealedDeck,Hoarder,Midas,3B2CDYJHBERBN",
        "1601/01/10,DEFECT,Allstar,ControlledChaos,Terminal,34A6Q7K5GUY7G",
        "1601/01/11,WATCHER,Chimera,Hoarder,Lethality,1L1SHNEBRYMRT",
    ];

    static FIRST_DATES: [&str; 10] = [
        "1601/01/02",
        "1601/01/03",
        "1601/01/04",
        "1601/01/05",
        "1601/01/06",
        "1601/01/07",
        "1601/01/08",
        "1601/01/09",
        "1601/01/10",
        "1601/01/11",
    ];

    static FIRST_CHARACTERS: [Character; 10] = [
        Character::Ironclad,
        Character::Ironclad,
        Character::Silent,
        Character::Defect,
        Character::Defect,
        Character::Defect,
        Character::Ironclad,
        Character::Watcher,
        Character::Defect,
        Character::Watcher,
    ];

    static FIRST_STARTER_MODS: [StarterMod; 10] = [
        StarterMod::Insanity,
        StarterMod::Heirloom,
        StarterMod::Draft,
        StarterMod::Specialized,
        StarterMod::Heirloom,
        StarterMod::Specialized,
        StarterMod::Chimera,
        StarterMod::SealedDeck,
        StarterMod::AllStar,
        StarterMod::Chimera,
    ];

    static FIRST_GENERIC_MODS: [GenericMod; 10] = [
        GenericMod::PurpleCards,
        GenericMod::Flight,
        GenericMod::BlueCards,
        GenericMod::ColorlessCards,
        GenericMod::PurpleCards,
        GenericMod::GreenCards,
        GenericMod::Vintage,
        GenericMod::Hoarder,
        GenericMod::ControlledChaos,
        GenericMod::Hoarder,
    ];

    static FIRST_DIFFICULTY_MODS: [DifficultyMod; 10] = [
        DifficultyMod::DeadlyEvents,
        DifficultyMod::Midas,
        DifficultyMod::Terminal,
        DifficultyMod::Lethality,
        DifficultyMod::NightTerrors,
        DifficultyMod::Lethality,
        DifficultyMod::DeadlyEvents,
        DifficultyMod::Midas,
        DifficultyMod::Terminal,
        DifficultyMod::Lethality,
    ];

    static FIRST_STRING_SEEDS: [&str; 10] = [
        "E06ALSGPMJ59",
        "4MKRHI0VBSRTJ",
        "1BKMD2MXNW8ZP",
        "3VPY3QN5WFLVN",
        "QKLWQHKXEBD6",
        "2AS174BGWFTWM",
        "GM7H4007EIF5",
        "3B2CDYJHBERBN",
        "34A6Q7K5GUY7G",
        "1L1SHNEBRYMRT",
    ];

    static FIRST_SEEDS: [i64; 10] = [
        1352185017444331709,
        -2748520984017838642,
        4498195575498320139,
        -5343793369089174818,
        2470626458777712311,
        7798508225620735407,
        1606064808466752205,
        -7240547502651087233,
        -7894770768367195055,
        5411679170527668388,
    ];

    #[test]
    fn test_parse_date() {
        for (&true_date, &daily_string) in FIRST_DATES.iter().zip(FIRST_DAILIES.iter()) {
            let daily: Daily = daily_string.parse().unwrap();
            assert_eq!(true_date, &daily.date);
        }
    }

    #[test]
    fn test_parse_character() {
        for (true_character, &daily_string) in FIRST_CHARACTERS.iter().zip(FIRST_DAILIES.iter()) {
            let daily: Daily = daily_string.parse().unwrap();
            assert_eq!(true_character, &daily.character);
        }
    }

    #[test]
    fn test_parse_starter_mod() {
        for (true_starter_mod, &daily_string) in FIRST_STARTER_MODS.iter().zip(FIRST_DAILIES.iter())
        {
            let daily: Daily = daily_string.parse().unwrap();
            assert_eq!(true_starter_mod, &daily.starter_mod);
        }
    }

    #[test]
    fn test_parse_generic_mod() {
        for (true_generic_mod, &daily_string) in FIRST_GENERIC_MODS.iter().zip(FIRST_DAILIES.iter())
        {
            let daily: Daily = daily_string.parse().unwrap();
            assert_eq!(true_generic_mod, &daily.generic_mod);
        }
    }

    #[test]
    fn test_parse_difficulty_mod() {
        for (true_difficulty_mod, &daily_string) in
            FIRST_DIFFICULTY_MODS.iter().zip(FIRST_DAILIES.iter())
        {
            let daily: Daily = daily_string.parse().unwrap();
            assert_eq!(true_difficulty_mod, &daily.difficulty_mod);
        }
    }

    #[test]
    fn test_parse_seed_string() {
        for (&true_string_seed, &daily_string) in
            FIRST_STRING_SEEDS.iter().zip(FIRST_DAILIES.iter())
        {
            let daily: Daily = daily_string.parse().unwrap();
            let true_seed: SeedString = true_string_seed.parse().unwrap();
            assert_eq!(true_seed, daily.seed_string);
        }
    }

    #[test]
    fn test_parse_seed() {
        for (&true_seed, &daily_string) in FIRST_SEEDS.iter().zip(FIRST_DAILIES.iter()) {
            let daily: Daily = daily_string.parse().unwrap();
            assert_eq!(true_seed, daily.seed.seed);
        }
    }
}
