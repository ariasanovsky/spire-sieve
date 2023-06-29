use std::array;

use libgdx_xs128::{rng::Random, RandomXS128};

use crate::filter::SeedFilter;

use super::{Card, Rarity};

pub struct CardRewarder<'a, const REWARDS: usize> {
    common: &'a [Card],
    uncommon: &'a [Card],
    rare: &'a [Card],
}
pub type CardReward = [Card; 3];

#[derive(Debug)]
pub struct Offset {
    offset: u64,    
}

const DEFAULT_OFFSET: u64 = 5;
const UNCOMMON_CUTOFF: u64 = 44;
const RARE_CUTOFF: u64 = 3;

impl Offset {
    fn reset(&mut self) {
        *self = Self::default();
    }
    
    fn decrement(&mut self) {
        self.offset -= 1;
    }

    fn adjusted_percentage(&self, rng: &mut Random) -> u64 {
        rng.next_capped_u64(100) + self.offset
    }

    fn generate_rarity(&mut self, rng: &mut Random) -> Rarity {
        let adjusted_percentage = self.adjusted_percentage(rng);
        if adjusted_percentage < RARE_CUTOFF {
            self.reset();
            Rarity::Rare
        } else if adjusted_percentage < UNCOMMON_CUTOFF {
            Rarity::Uncommon
        } else {
            self.decrement();
            Rarity::Common
        }
    }
}

impl Default for Offset {
    fn default() -> Self {
        Self { offset: DEFAULT_OFFSET, }
    }
}

impl<'a, const REWARDS: usize> CardRewarder<'a, REWARDS> {
    pub const fn new(common: &'a [Card], uncommon: &'a [Card], rare: &'a [Card]) -> Self {
        Self { common, uncommon, rare, }
    }

    pub fn generate_rewards(&self, rng: &mut Random) -> [CardReward; REWARDS] {
        let mut offset = Offset::default();
        array::from_fn(|_| {
            self.generate_reward(rng, &mut offset)
        })
    }

    pub fn generate_reward(&self, rng: &mut Random, offset: &mut Offset) -> CardReward {
        let mut reward: [Card; 3] = Default::default();
        for i in 0..3 {
            let rarity = offset.generate_rarity(rng);
            let mut card = self.generate_card(rng, rarity);
            while reward[..i].contains(&card) {
                card = self.generate_card(rng, rarity);
            }
            reward[i] = card;
        }
        reward
    }

    pub fn generate_card(&self, rng: &mut Random, rarity: Rarity) -> Card {
        let cards = self.card_pool(rarity);
        let card = cards[rng.next_capped_u64(cards.len() as u64) as usize];
        card
    }

    pub fn card_pool(&self, rarity: Rarity) -> &[Card] {
        match rarity {
            Rarity::Common => self.common,
            Rarity::Uncommon => self.uncommon,
            Rarity::Rare => self.rare,
        }
    }
}

pub struct CardRewardFilter<'a, const REWARDS: usize> {
    reward: CardRewarder<'a, REWARDS>,
    rejected_cards: &'a [Card],
}

impl<'a, const REWARDS: usize> SeedFilter for CardRewardFilter<'a, REWARDS> {
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let rewards = self.reward.generate_rewards(rng);
        rewards.iter().any(|reward| {
            reward.iter().any(|card| {
                self.rejected_cards.contains(card)
            })
        })
    }
}
