use strum_macros::EnumCount;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

#[cfg(feature = "std")]
pub mod try_from;

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, EnumCount)]
pub enum Character {
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

pub const fn basics(character: Character) -> usize {
    [9, 10, 8, 8][character as usize]
}
