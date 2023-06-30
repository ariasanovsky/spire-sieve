use crate::character::Character;

use super::{Card, CARDS, REV_CARDS, CardSlice, Rarity};

pub struct CharacterCards<'a> {
    pub all: CardSlice<'a>,
    pub common: CardSlice<'a>,  
    pub uncommon: CardSlice<'a>,
    pub rare: CardSlice<'a>,
}

impl<'a> CharacterCards<'a> {
    pub const fn new(character: Character) -> Self {
        Self {
            all: CardSliceParameters::new(character, None).slice(&CARDS, false),
            common: CardSliceParameters::new(character, Some(Rarity::Common)).slice(&REV_CARDS, true),
            uncommon: CardSliceParameters::new(character, Some(Rarity::Uncommon)).slice(&REV_CARDS, true),
            rare: CardSliceParameters::new(character, Some(Rarity::Rare)).slice(&REV_CARDS, true),
         }
    }
}

struct CardSliceParameters {
    first: Card,
    len: usize,
}

impl CardSliceParameters {
    pub const fn new(character: Character, rarity: Option<Rarity>) -> Self {
        use Character::*;
        use Rarity::*;
        use Card::*;
        let (first, len) = if let Some(rarity) = rarity {
            match (character, rarity) {
                (Ironclad, Common) => (SwordBoomerang, 20),
                (Ironclad, Uncommon) => (Evolve, 36),
                (Ironclad, Rare) => (DoubleTap, 16),
                (Silent, Common) => (FlyingKnee, 19),
                (Silent, Uncommon) => (Predator, 33),
                (Silent, Rare) => (Alchemize, 19),
                (Defect, Common) => (SteamBarrier, 18),
                (Defect, Uncommon) => (DoomAndGloom, 36),
                (Defect, Rare) => (MultiCast, 17),
                (Watcher, Common) => (EmptyFist, 19),
                (Watcher, Uncommon) => (Pray, 35),
                (Watcher, Rare) => (Judgment, 17),
            }
        } else {
            match character {
                Ironclad => (SwordBoomerang, 72),
                Silent => (FlyingKnee, 71),
                Defect => (SteamBarrier, 71),
                Watcher => (EmptyFist, 71),
            }
        };
        Self { first, len }
    }

    pub const fn last(&self) -> Card {
        CARDS[self.first as usize + self.len - 1]
    }

    pub const fn slice<'b>(&self, cards: &'b [Card], reversed: bool) -> CardSlice<'b> {
        let last = self.last();
        if reversed {
            CardSlice::new(cards).trim_inclusive(last, self.first)
        } else {
            CardSlice::new(cards).trim_inclusive(self.first, last)
        }
    }
}
