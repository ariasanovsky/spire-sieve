use crate::character::Character;

// pub type CardPool = &'static [Card];

use strum::EnumCount;
use strum_macros::EnumCount;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, EnumCount)]
pub enum UnlockLevel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Unlocks {
    levels: [UnlockLevel; Character::COUNT],
}

impl Unlocks {
    pub const fn new(levels: [UnlockLevel; Character::COUNT]) -> Self {
        Self { levels }
    }

    pub const fn encode(&self) -> usize {
        let mut n = 0;
        let mut i = 0;
        const UNLOCK_LEVELS: usize = UnlockLevel::COUNT as usize;
        while i < Character::COUNT {
            n *= UNLOCK_LEVELS;
            n += self.levels[i] as usize;
            i += 1;
        }
        n
    }

    pub const fn decode(mut n: usize) -> Option<Self> {
        let mut levels = [UnlockLevel::Zero; Character::COUNT];
        let mut i = 0;
        const UNLOCK_LEVELS: usize = UnlockLevel::COUNT as usize;
        while i < Character::COUNT {
            levels[i] = if let Some(level) = UnlockLevel::from_repr(n % UNLOCK_LEVELS) {
                level
            } else {
                return None;
            };
            i += 1;
            n /= UNLOCK_LEVELS;
        }
        Some(Self { levels })
    }
}

// #[derive(Debug)]
// pub struct CardPool {
//     first: usize,
//     last: usize,
//     reversed: bool,
// }

// impl CardPool {
//     const fn new(first: usize, last: usize, reversed: bool) -> Self {
//         Self { first, last, reversed }
//     }

//     const fn first<const CHARACTER: usize>() -> usize {
//         [0, 70, 140, 210][CHARACTER]
//     }
// }

// const IRONCLAD_FIRST: Card = Card::Anger;
// const IRONCLAD_LAST: Card = Card::Juggernaut;

// const IRONCLAD_ALL: CardPool = CardPool::new(
//     IRONCLAD_FIRST as usize,
//     IRONCLAD_LAST as usize,
//     false,
// );

// pub struct CharacterCardPool<const CHARACTER: usize> {
//     all: CardPool,
//     common: CardPool,
//     uncommon: CardPool,
//     rare: CardPool,
// }

// impl<const CHARACTER: usize> CharacterCardPool<CHARACTER> {

// }

// #[derive(Debug)]
// pub struct CharacterCardSubPool<const CHARACTER: usize, const RARITY: usize> {
//     all: CardPool<CHARACTER, {Rarity::Any as usize}>,
//     common: CardPool<CHARACTER, {Rarity::Common as usize}>,
//     uncommon: CardPool<CHARACTER, {Rarity::Uncommon as usize}>,
//     rare: CardPool<CHARACTER, {Rarity::Rare as usize}>,
// }

// #[derive(Debug)]
// pub struct CharacterCardPool<const CHARACTER: usize> {
//     all: CardPool<CHARACTER, {Rarity::Any as usize}>,
//     common: CardPool<CHARACTER, {Rarity::Common as usize}>,
//     uncommon: CardPool<CHARACTER, {Rarity::Uncommon as usize}>,
//     rare: CardPool<CHARACTER, {Rarity::Rare as usize}>,
// }

// impl<const CHARACTER: usize> CharacterCardPool<CHARACTER> {
//     pub fn all(&self) -> &CardPool<CHARACTER, {Rarity::Any as usize}> {
//         &self.all
//     }

//     pub fn common(&self) -> &CardPool<CHARACTER, {Rarity::Common as usize}> {
//         &self.common
//     }

//     pub fn uncommon(&self) -> &CardPool<CHARACTER, {Rarity::Uncommon as usize}> {
//         &self.uncommon
//     }

//     pub fn rare(&self) -> &CardPool<CHARACTER, {Rarity::Rare as usize}> {
//         &self.rare
//     }
// }

// const RAW_POOL_PARAMETERS: [(usize, usize, usize, usize); 4] = [
//     (1, 20, 36, 16),
//     (73, 19, 33, 19),
//     (144, 18, 36, 17),
//     (215, 19, 35, 17),
// ];

// const fn pool_parameters<
//     const START: usize,
//     const COMMON: usize,
//     const UNCOMMON: usize,
//     const RARE: usize,
// >() -> [(usize, usize); 4] {
//     [
//         (START, COMMON + UNCOMMON + RARE),
//         (START, COMMON),
//         (START + COMMON, UNCOMMON),
//         (START + COMMON + UNCOMMON, RARE),
//     ]
// }

// const POOL_PARAMETERS: [[(usize, usize); 4]; 4] = [
//     pool_parameters::<
//         { RAW_POOL_PARAMETERS[0].0 },
//         { RAW_POOL_PARAMETERS[0].1 },
//         { RAW_POOL_PARAMETERS[0].2 },
//         { RAW_POOL_PARAMETERS[0].3 },
//     >(),
//     pool_parameters::<
//         { RAW_POOL_PARAMETERS[1].0 },
//         { RAW_POOL_PARAMETERS[1].1 },
//         { RAW_POOL_PARAMETERS[1].2 },
//         { RAW_POOL_PARAMETERS[1].3 },
//     >(),
//     pool_parameters::<
//         { RAW_POOL_PARAMETERS[2].0 },
//         { RAW_POOL_PARAMETERS[2].1 },
//         { RAW_POOL_PARAMETERS[2].2 },
//         { RAW_POOL_PARAMETERS[2].3 },
//     >(),
//     pool_parameters::<
//         { RAW_POOL_PARAMETERS[3].0 },
//         { RAW_POOL_PARAMETERS[3].1 },
//         { RAW_POOL_PARAMETERS[3].2 },
//         { RAW_POOL_PARAMETERS[3].3 },
//     >(),
// ];

// const IRONCLAD_PARAMETERS: [(usize, usize); 4] = POOL_PARAMETERS[Character::Ironclad as usize];
// const SILENT_PARAMETERS: [(usize, usize); 4] = POOL_PARAMETERS[Character::Silent as usize];
// const DEFECT_PARAMETERS: [(usize, usize); 4] = POOL_PARAMETERS[Character::Defect as usize];
// const WATCHER_PARAMETERS: [(usize, usize); 4] = POOL_PARAMETERS[Character::Watcher as usize];

// pub const IRONCLAD_CARD_POOL: CharacterCardPool = CharacterCardPool {
//     all: &cards::<{ IRONCLAD_PARAMETERS[0].0 }, { IRONCLAD_PARAMETERS[0].1 }, false>(),
//     common: &cards::<{ IRONCLAD_PARAMETERS[1].0 }, { IRONCLAD_PARAMETERS[1].1 }, false>(),
//     uncommon: &cards::<{ IRONCLAD_PARAMETERS[2].0 }, { IRONCLAD_PARAMETERS[2].1 }, false>(),
//     rare: &cards::<{ IRONCLAD_PARAMETERS[3].0 }, { IRONCLAD_PARAMETERS[3].1 }, false>(),
// };

// pub const SILENT_CARD_POOL: CharacterCardPool = CharacterCardPool {
//     all: &cards::<{ SILENT_PARAMETERS[0].0 }, { SILENT_PARAMETERS[0].1 }, false>(),
//     common: &cards::<{ SILENT_PARAMETERS[1].0 }, { SILENT_PARAMETERS[1].1 }, false>(),
//     uncommon: &cards::<{ SILENT_PARAMETERS[2].0 }, { SILENT_PARAMETERS[2].1 }, false>(),
//     rare: &cards::<{ SILENT_PARAMETERS[3].0 }, { SILENT_PARAMETERS[3].1 }, false>(),
// };

// pub const DEFECT_CARD_POOL: CharacterCardPool = CharacterCardPool {
//     all: &cards::<{ DEFECT_PARAMETERS[0].0 }, { DEFECT_PARAMETERS[0].1 }, false>(),
//     common: &cards::<{ DEFECT_PARAMETERS[1].0 }, { DEFECT_PARAMETERS[1].1 }, false>(),
//     uncommon: &cards::<{ DEFECT_PARAMETERS[2].0 }, { DEFECT_PARAMETERS[2].1 }, false>(),
//     rare: &cards::<{ DEFECT_PARAMETERS[3].0 }, { DEFECT_PARAMETERS[3].1 }, false>(),
// };

// pub const WATCHER_CARD_POOL: CharacterCardPool = CharacterCardPool {
//     all: &cards::<{ WATCHER_PARAMETERS[0].0 }, { WATCHER_PARAMETERS[0].1 }, false>(),
//     common: &cards::<{ WATCHER_PARAMETERS[1].0 }, { WATCHER_PARAMETERS[1].1 }, false>(),
//     uncommon: &cards::<{ WATCHER_PARAMETERS[2].0 }, { WATCHER_PARAMETERS[2].1 }, false>(),
//     rare: &cards::<{ WATCHER_PARAMETERS[3].0 }, { WATCHER_PARAMETERS[3].1 }, false>(),
// };

// #[cfg(test)]
// mod test_character_card_pool_initializations {
//     use super::*;

//     fn card_pool_info(card_pool: CardPool) -> (Card, Card, usize) {
//         (
//             *card_pool.first().unwrap(),
//             *card_pool.last().unwrap(),
//             card_pool.len(),
//         )
//     }

//     fn character_card_pool_info(
//         character_card_pool: &CharacterCardPool,
//     ) -> [(Card, Card, usize); 4] {
//         character_card_pool.pools().map(card_pool_info)
//     }

//     #[test]
//     fn all_card_pool_info() {
//         [
//             &IRONCLAD_CARD_POOL,
//             &SILENT_CARD_POOL,
//             &DEFECT_CARD_POOL,
//             &WATCHER_CARD_POOL,
//         ]
//         .into_iter()
//         .map(character_card_pool_info)
//         .flatten()
//         .for_each(|info| {
//             dbg!(info);
//         });
//     }
// }
