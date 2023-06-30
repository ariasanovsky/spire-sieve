use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::character::Character;

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
    pub const fn full() -> Self {
        Self {
            levels: [UnlockLevel::Five; Character::COUNT],
        }
    }

    pub const fn new(levels: [UnlockLevel; Character::COUNT]) -> Self {
        Self { levels }
    }
}
