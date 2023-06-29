use std::str::FromStr;

use strum_macros::EnumCount;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

#[derive(Debug, PartialEq, Eq, Clone, Copy,FromRepr, EnumIter, EnumCount)]
pub enum Character {
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

#[derive(Debug)]
pub enum Error {
    ParseCharacter(String),
}

impl FromStr for Character {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "IRONCLAD" | "THE_IRONCLAD" => Self::Ironclad,
            "SILENT" | "THE_SILENT" => Self::Silent,
            "DEFECT" | "THE_DEFECT" => Self::Defect,
            "WATCHER" | "THE_WATCHER" => Self::Watcher,
            _ => return Err(Error::ParseCharacter(s.into())),
        })
    }
}
