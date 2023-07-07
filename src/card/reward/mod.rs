use core::array;

use libgdx_xs128::{rng::Random, RandomXS128};

use crate::{character::Character, unlock::Unlocks};

use super::{pool::CharacterCards, Card, Rarity};

// #[cfg(feature = "std")]
pub mod filters;

pub struct CardRewarder<'a, const REWARDS: usize> {
    common: &'a [Card],
    uncommon: &'a [Card],
    rare: &'a [Card],
}

impl<'a, const REWARDS: usize> CardRewarder<'a, REWARDS> {}

pub type CardReward = [Card; 3];

#[derive(Debug)]
pub struct Offset {
    offset: i64,
}

const DEFAULT_OFFSET: i64 = -5;
const UNCOMMON_CUTOFF: i64 = 37;
const RARE_CUTOFF: i64 = 3;

impl Offset {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn decrement(&mut self) {
        self.offset -= 1;
    }

    fn adjusted_percentage(&self, rng: &mut Random) -> i64 {
        rng.next_capped_u64(100) as i64 + self.offset
    }

    fn generate_rarity(&mut self, rng: &mut Random) -> Rarity {
        let adjusted_percentage = self.adjusted_percentage(rng);
        if adjusted_percentage < RARE_CUTOFF {
            self.reset();
            Rarity::Rare
        } else if adjusted_percentage <= UNCOMMON_CUTOFF {
            Rarity::Uncommon
        } else {
            self.decrement();
            Rarity::Common
        }
    }
}

impl Default for Offset {
    fn default() -> Self {
        Self {
            offset: DEFAULT_OFFSET,
        }
    }
}

impl<'a, const REWARDS: usize> CardRewarder<'a, REWARDS> {
    pub const fn new(character: Character, unlocks: Option<Unlocks>) -> Self {
        if let Some(_unlocks) = unlocks {
            unimplemented!()
        } else {
            let CharacterCards {
                all: _,
                common,
                uncommon,
                rare,
            } = CharacterCards::new(character);
            Self {
                common: common.slice,
                uncommon: uncommon.slice,
                rare: rare.slice,
            }
        }
    }

    pub fn generate_rewards(&self, rng: &mut Random) -> [CardReward; REWARDS] {
        let mut offset = Offset::default();
        array::from_fn(|_| self.generate_reward(rng, &mut offset))
    }

    pub fn generate_reward(&self, rng: &mut Random, offset: &mut Offset) -> CardReward {
        let mut reward: [Card; 3] = Default::default();
        for i in 0..3 {
            let rarity = offset.generate_rarity(rng);
            let mut card = self.generate_card(rng, rarity);
            while reward.contains(&card) {
                card = self.generate_card(rng, rarity);
            }
            reward[i] = card;
        }
        rng.advance(3);
        reward
    }

    pub fn generate_card(&self, rng: &mut Random, rarity: Rarity) -> Card {
        let cards = self.card_pool(rarity);
        cards[rng.next_capped_u64(cards.len() as u64) as usize]
    }

    pub fn card_pool(&self, rarity: Rarity) -> &[Card] {
        match rarity {
            Rarity::Common => self.common,
            Rarity::Uncommon => self.uncommon,
            Rarity::Rare => self.rare,
        }
    }
}
