use std::str::FromStr;

use alloc::string::String;

#[derive(Debug, PartialEq, Eq)]
pub enum StarterMod {
    Insanity,
    Heirloom,
    Draft,
    Specialized,
    Chimera,
    SealedDeck,
    AllStar,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GenericMod {
    PurpleCards,
    Flight,
    BlueCards,
    ColorlessCards,
    RedCards,
    GreenCards,
    Vintage,
    Hoarder,
    ControlledChaos,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DifficultyMod {
    DeadlyEvents,
    Midas,
    Terminal,
    Lethality,
    NightTerrors,
}

#[derive(Debug)]
pub enum Error {
    ParseStarterMod(String),
    ParseGenericMod(String),
    ParseDifficultyMod(String),
}

impl FromStr for StarterMod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Insanity" => Self::Insanity,
            "Heirloom" => Self::Heirloom,
            "Draft" => Self::Draft,
            "Specialized" => Self::Specialized,
            "Chimera" => Self::Chimera,
            "SealedDeck" => Self::SealedDeck,
            "Allstar" => Self::AllStar,
            _ => return Err(Error::ParseStarterMod(s.into())),
        })
    }
}

impl FromStr for GenericMod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Purple Cards" => Self::PurpleCards,
            "Flight" => Self::Flight,
            "Blue Cards" => Self::BlueCards,
            "Colorless Cards" => Self::ColorlessCards,
            "Red Cards" => Self::RedCards,
            "Green Cards" => Self::GreenCards,
            "Vintage" => Self::Vintage,
            "Hoarder" => Self::Hoarder,
            "ControlledChaos" => Self::ControlledChaos,
            _ => return Err(Error::ParseGenericMod(s.into())),
        })
    }
}

impl FromStr for DifficultyMod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "DeadlyEvents" => Self::DeadlyEvents,
            "Midas" => Self::Midas,
            "Terminal" => Self::Terminal,
            "Lethality" => Self::Lethality,
            "Night Terrors" => Self::NightTerrors,
            _ => return Err(Error::ParseDifficultyMod(s.into())),
        })
    }
}
