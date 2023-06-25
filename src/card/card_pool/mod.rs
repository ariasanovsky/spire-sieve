use crate::card::card::{cards, Card};
use crate::character::Character;

#[derive(Debug, Clone, Copy)]
pub struct CardPool(&'static [Card]);

#[derive(Debug)]
pub struct CharacterCardPool {
    all: CardPool,
    common: CardPool,
    uncommon: CardPool,
    rare: CardPool,
}

impl CharacterCardPool {
    pub fn all(&self) -> CardPool {
        self.all
    }

    pub fn common(&self) -> CardPool {
        self.common
    }

    pub fn uncommon(&self) -> CardPool {
        self.uncommon
    }

    pub fn rare(&self) -> CardPool {
        self.rare
    }

    pub fn pools(&self) -> [CardPool; 4] {
        [self.all, self.common, self.uncommon, self.rare]
    }
}

const RAW_POOL_PARAMETERS: [(usize, usize, usize, usize); 4] = [
    (1, 20, 36, 16),
    (73, 19, 33, 19),
    (144, 18, 36, 17),
    (215, 19, 35, 17),
];

const fn pool_parameters<
    const START: usize,
    const COMMON: usize,
    const UNCOMMON: usize,
    const RARE: usize,
>() -> [(usize, usize); 4] {
    [
        (START, COMMON + UNCOMMON + RARE),
        (START, COMMON),
        (START + COMMON, UNCOMMON),
        (START + COMMON + UNCOMMON, RARE),
    ]
}

const POOL_PARAMETERS: [[(usize, usize); 4]; 4] = [
    pool_parameters::<
        { RAW_POOL_PARAMETERS[0].0 },
        { RAW_POOL_PARAMETERS[0].1 },
        { RAW_POOL_PARAMETERS[0].2 },
        { RAW_POOL_PARAMETERS[0].3 },
    >(),
    pool_parameters::<
        { RAW_POOL_PARAMETERS[1].0 },
        { RAW_POOL_PARAMETERS[1].1 },
        { RAW_POOL_PARAMETERS[1].2 },
        { RAW_POOL_PARAMETERS[1].3 },
    >(),
    pool_parameters::<
        { RAW_POOL_PARAMETERS[2].0 },
        { RAW_POOL_PARAMETERS[2].1 },
        { RAW_POOL_PARAMETERS[2].2 },
        { RAW_POOL_PARAMETERS[2].3 },
    >(),
    pool_parameters::<
        { RAW_POOL_PARAMETERS[3].0 },
        { RAW_POOL_PARAMETERS[3].1 },
        { RAW_POOL_PARAMETERS[3].2 },
        { RAW_POOL_PARAMETERS[3].3 },
    >(),
];

const IRONCLAD_PARAMETERS: [(usize, usize); 4] = POOL_PARAMETERS[Character::Ironclad as usize];
const SILENT_PARAMETERS: [(usize, usize); 4] = POOL_PARAMETERS[Character::Silent as usize];
const DEFECT_PARAMETERS: [(usize, usize); 4] = POOL_PARAMETERS[Character::Defect as usize];
const WATCHER_PARAMETERS: [(usize, usize); 4] = POOL_PARAMETERS[Character::Watcher as usize];

pub const IRONCLAD_CARD_POOL: CharacterCardPool = CharacterCardPool {
    all: CardPool(&cards::<
        { IRONCLAD_PARAMETERS[0].0 },
        { IRONCLAD_PARAMETERS[0].1 },
        false,
    >()),
    common: CardPool(&cards::<
        { IRONCLAD_PARAMETERS[1].0 },
        { IRONCLAD_PARAMETERS[1].1 },
        false,
    >()),
    uncommon: CardPool(&cards::<
        { IRONCLAD_PARAMETERS[2].0 },
        { IRONCLAD_PARAMETERS[2].1 },
        false,
    >()),
    rare: CardPool(&cards::<
        { IRONCLAD_PARAMETERS[3].0 },
        { IRONCLAD_PARAMETERS[3].1 },
        false,
    >()),
};

pub const SILENT_CARD_POOL: CharacterCardPool = CharacterCardPool {
    all: CardPool(&cards::<
        { SILENT_PARAMETERS[0].0 },
        { SILENT_PARAMETERS[0].1 },
        false,
    >()),
    common: CardPool(&cards::<
        { SILENT_PARAMETERS[1].0 },
        { SILENT_PARAMETERS[1].1 },
        false,
    >()),
    uncommon: CardPool(&cards::<
        { SILENT_PARAMETERS[2].0 },
        { SILENT_PARAMETERS[2].1 },
        false,
    >()),
    rare: CardPool(&cards::<
        { SILENT_PARAMETERS[3].0 },
        { SILENT_PARAMETERS[3].1 },
        false,
    >()),
};

pub const DEFECT_CARD_POOL: CharacterCardPool = CharacterCardPool {
    all: CardPool(&cards::<
        { DEFECT_PARAMETERS[0].0 },
        { DEFECT_PARAMETERS[0].1 },
        false,
    >()),
    common: CardPool(&cards::<
        { DEFECT_PARAMETERS[1].0 },
        { DEFECT_PARAMETERS[1].1 },
        false,
    >()),
    uncommon: CardPool(&cards::<
        { DEFECT_PARAMETERS[2].0 },
        { DEFECT_PARAMETERS[2].1 },
        false,
    >()),
    rare: CardPool(&cards::<
        { DEFECT_PARAMETERS[3].0 },
        { DEFECT_PARAMETERS[3].1 },
        false,
    >()),
};

pub const WATCHER_CARD_POOL: CharacterCardPool = CharacterCardPool {
    all: CardPool(&cards::<
        { WATCHER_PARAMETERS[0].0 },
        { WATCHER_PARAMETERS[0].1 },
        false,
    >()),
    common: CardPool(&cards::<
        { WATCHER_PARAMETERS[1].0 },
        { WATCHER_PARAMETERS[1].1 },
        false,
    >()),
    uncommon: CardPool(&cards::<
        { WATCHER_PARAMETERS[2].0 },
        { WATCHER_PARAMETERS[2].1 },
        false,
    >()),
    rare: CardPool(&cards::<
        { WATCHER_PARAMETERS[3].0 },
        { WATCHER_PARAMETERS[3].1 },
        false,
    >()),
};

#[cfg(test)]
mod test_character_card_pool_initializations {
    use super::*;

    fn card_pool_info(card_pool: CardPool) -> (Card, Card, usize) {
        (
            *card_pool.0.first().unwrap(),
            *card_pool.0.last().unwrap(),
            card_pool.0.len(),
        )
    }

    fn character_card_pool_info(
        character_card_pool: &CharacterCardPool,
    ) -> [(Card, Card, usize); 4] {
        character_card_pool.pools().map(card_pool_info)
    }

    #[test]
    fn all_card_pool_info() {
        [
            &IRONCLAD_CARD_POOL,
            &SILENT_CARD_POOL,
            &DEFECT_CARD_POOL,
            &WATCHER_CARD_POOL,
        ]
        .into_iter()
        .map(character_card_pool_info)
        .flatten()
        .for_each(|info| {
            dbg!(info);
        });
    }
}
