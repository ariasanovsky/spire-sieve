use std::array;

use libgdx_xs128::{rng::Random, RandomXS128};

use crate::{
    card::{card_pool_range, Card, Rarity},
    character::Character,
    seed,
};

pub trait Filter {
    fn reject(&self, seed: &seed::Seed) -> bool;
}

pub struct PandorasBoxFilter<const CHARACTER: usize, const UNLOCKS: usize>;

impl<const CHARACTER: usize, const UNLOCKS: usize> PandorasBoxFilter<CHARACTER, UNLOCKS> {
    pub fn pandoras_box(&self, seed: &seed::Seed) -> [Card; 9] {
        const RANGE: (Card, Card, bool) = card_pool_range(Character::Ironclad, Rarity::Any);
        const FIRST: usize = RANGE.0 as usize;
        const LAST: usize = RANGE.1 as usize;
        const N: usize = LAST - FIRST + 1;
        const REVERSE: bool = RANGE.2;
        const CARDS: [Card; N] = crate::card::cards::<FIRST, N, REVERSE>();
        dbg!(CARDS);
        let mut rng = Random::new(seed.seed as u64);
        array::from_fn(|_| CARDS[rng.next_capped_u64(N as u64) as usize])
    }

    pub fn constant_pandoras_box(&self, seed: &seed::Seed) -> Option<Card> {
        let cards = self.pandoras_box(seed);
        let first = cards[0];
        let leftover = &cards[1..];
        for other in leftover {
            if first != *other {
                return None;
            }
        }
        Some(first)
    }
}

#[test]
fn test_pandoras_box() {
    let seed = seed::Seed { seed: 1 };
    use crate::card::card_pool::Unlocks;
    const FILTER: PandorasBoxFilter<
        { Character::Ironclad as usize },
        { Unlocks::full().encode() },
    > = PandorasBoxFilter;
    let cards = FILTER.pandoras_box(&seed);
    dbg!(cards);
    dbg!(FILTER.constant_pandoras_box(&seed));
}
