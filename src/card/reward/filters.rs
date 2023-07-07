use libgdx_xs128::rng::Random;

use crate::{filter::SeedFilter, unlock::Unlocks, card::Card, character::Character};

use super::CardRewarder;

pub struct CardRewardFilter<'a, const REWARDS: usize> {
    reward: CardRewarder<'a, REWARDS>,
    rejected_cards: &'a [Card],
}

impl<'a, const N: usize> CardRewardFilter<'a, N> {
    pub const fn new(
        character: Character,
        unlocks: Option<Unlocks>,
        rejected_cards: &'a [Card],
    ) -> Self {
        Self {
            reward: CardRewarder::new(character, unlocks),
            rejected_cards,
        }
    }
}

impl<'a, const REWARDS: usize> SeedFilter for CardRewardFilter<'a, REWARDS> {
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let rewards = self.reward.generate_rewards(rng);
        rewards
            .iter()
            .any(|reward| reward.iter().any(|card| self.rejected_cards.contains(card)))
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod card_reward_tests {

    use std::dbg;

    use libgdx_xs128::{rng::Random, RandomXS128};

    use crate::{
        card::{
            reward::{CardRewarder, filters::CardRewardFilter},
            Card,
        },
        character::Character,
        filter::SeedFilter,
        seed::{Seed, SeedString},
    };

    #[test]
    fn test_unwinnable_seed() {
        let seed: SeedString = "18ISL35FYK4".parse().unwrap();
        let seed: Seed = seed.into();
        let filter: CardRewardFilter<'_, 3> = CardRewardFilter::new(Character::Silent, None, &[]);
        assert!(!filter.reject(seed.clone()));

        let rewarder = CardRewarder::new(Character::Silent, None);
        let mut rng = Random::new(seed.seed as u64);
        let rewards = rewarder.generate_rewards(&mut rng);

        dbg!(&rewards);
        assert_eq!(
            rewards,
            [
                [Card::Prepared, Card::DodgeAndRoll, Card::EscapePlan],
                [Card::EscapePlan, Card::Outmaneuver, Card::Prepared],
                [Card::Prepared, Card::DodgeAndRoll, Card::Footwork],
            ]
        )
    }

    #[test]
    fn const_test_unwinnable_seed() {
        let seed: SeedString = "18ISL35FYK4".parse().unwrap();
        let seed: Seed = seed.into();
        const FILTER: CardRewardFilter<'_, 3> = CardRewardFilter::new(Character::Silent, None, &[]);
        assert!(!FILTER.reject(seed.clone()));

        const REWARDER: CardRewarder<'_, 3> = CardRewarder::new(Character::Silent, None);
        let mut rng = Random::new(seed.seed as u64);
        let rewards = REWARDER.generate_rewards(&mut rng);

        dbg!(&rewards);
        assert_eq!(
            rewards,
            [
                [Card::Prepared, Card::DodgeAndRoll, Card::EscapePlan],
                [Card::EscapePlan, Card::Outmaneuver, Card::Prepared],
                [Card::Prepared, Card::DodgeAndRoll, Card::Footwork],
            ]
        )
    }
}
