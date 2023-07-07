use core::array;

use libgdx_xs128::{rng::Random, RandomXS128};

use crate::{character::Character, filter::SeedFilter, seed};

use super::Card;

pub struct PandorasBox<'a> {
    cards: &'a [Card],
}

impl<'a> PandorasBox<'a> {
    pub const fn new(cards: &'a [Card]) -> Self {
        Self { cards }
    }

    pub fn pandoras_box(&self, seed: &seed::Seed) -> [Card; 9] {
        let n = self.cards.len();
        let mut rng = Random::new(seed.seed as u64);
        array::from_fn(|_| self.cards[rng.next_capped_u64(n as u64) as usize])
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

pub struct AnonymousPandoraBox {
    cards: u64,
    basics: usize,
}

impl AnonymousPandoraBox {
    pub const fn new(_character: Character) -> Self {
        // let basics: usize = basics(character);
        // let range = card_pool_range(character, None);
        todo!()
        // let cards = range.1 as u64 - range.0 as u64 + 1;
        // Self { cards, basics }
    }
}

impl SeedFilter for AnonymousPandoraBox {
    fn reject_rng(&self, rng: &mut Random) -> bool {
        let first = rng.next_capped_u64(self.cards);
        for _ in 1..self.basics {
            if first != rng.next_capped_u64(self.cards) {
                return true;
            }
        }
        false
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use std::dbg;

    use crate::{card::pandoras_box::PandorasBox, seed};

    #[test]
    fn test_pandoras_box() {
        let seed = seed::Seed { seed: 1 };
        const FILTER: PandorasBox = PandorasBox::new(&crate::card::CARDS);
        let cards = FILTER.pandoras_box(&seed);
        dbg!(cards);
        dbg!(FILTER.constant_pandoras_box(&seed));
    }
}
