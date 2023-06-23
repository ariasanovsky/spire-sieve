use libgdx_xs128::rng::Random;
use libgdx_xs128::RandomXS128;

#[derive(Debug, PartialEq, Eq)]
pub struct NeowBonuses {
    pub first: FirstBonus,
    pub second: SecondBonus,
    pub third: (Drawback, ThirdBonus),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FirstBonus {
    ThreeCards,
    OneRandomRareCard,
    RemoveCard,
    UpgradeCard,
    TransformCard,
    RandomColorless,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SecondBonus {
    ThreeSmallPotions,
    RandomCommonRelic,
    TenPercentHpBonus,
    ThreeEnemyKill,
    HundredGold,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ThirdBonus {
    RandomColorlessChoice,
    RemoveTwo,
    OneRareRelic,
    ThreeRareCards,
    TwoFiftyGold,
    TransformTwoCards,
    TwentyPercentHpBonus,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Drawback {
    TenPercentHpLoss,
    NoGold,
    Curse,
    PercentDamage,
}

impl NeowBonuses {
    pub fn generate(mut rng: Random) -> Self {
        let first = match rng.next_capped_u64(6) {
            0 => FirstBonus::ThreeCards,
            1 => FirstBonus::OneRandomRareCard,
            2 => FirstBonus::RemoveCard,
            3 => FirstBonus::UpgradeCard,
            4 => FirstBonus::TransformCard,
            5 => FirstBonus::RandomColorless,
            _ => unreachable!(),
        };

        let second = match rng.next_capped_u64(5) {
            0 => SecondBonus::ThreeSmallPotions,
            1 => SecondBonus::RandomCommonRelic,
            2 => SecondBonus::TenPercentHpBonus,
            3 => SecondBonus::ThreeEnemyKill,
            4 => SecondBonus::HundredGold,
            _ => unreachable!(),
        };

        let drawback = match rng.next_capped_u64(4) {
            0 => Drawback::TenPercentHpLoss,
            1 => Drawback::NoGold,
            2 => Drawback::Curse,
            3 => Drawback::PercentDamage,
            _ => unreachable!(),
        };

        let third_bonus = match &drawback {
            Drawback::TenPercentHpLoss => {
                match rng.next_capped_u64(6) {
                    0 => ThirdBonus::RandomColorlessChoice,
                    1 => ThirdBonus::RemoveTwo,
                    2 => ThirdBonus::OneRareRelic,
                    3 => ThirdBonus::ThreeRareCards,
                    4 => ThirdBonus::TwoFiftyGold,
                    5 => ThirdBonus::TransformTwoCards,
                    _ => unreachable!(),
                }
            }
            Drawback::NoGold => {
                match rng.next_capped_u64(6) {
                    0 => ThirdBonus::RandomColorlessChoice,
                    1 => ThirdBonus::RemoveTwo,
                    2 => ThirdBonus::OneRareRelic,
                    3 => ThirdBonus::ThreeRareCards,
                    4 => ThirdBonus::TransformTwoCards,
                    5 => ThirdBonus::TwentyPercentHpBonus,
                    _ => unreachable!(),
                }
            }
            Drawback::Curse => {
                match rng.next_capped_u64(6) {
                    0 => ThirdBonus::RandomColorlessChoice,
                    1 => ThirdBonus::OneRareRelic,
                    2 => ThirdBonus::ThreeRareCards,
                    3 => ThirdBonus::TwoFiftyGold,
                    4 => ThirdBonus::TransformTwoCards,
                    5 => ThirdBonus::TwentyPercentHpBonus,
                    _ => unreachable!(),
                }
            }
            Drawback::PercentDamage => {
                match rng.next_capped_u64(7) {
                    0 => ThirdBonus::RandomColorlessChoice,
                    1 => ThirdBonus::RemoveTwo,
                    2 => ThirdBonus::OneRareRelic,
                    3 => ThirdBonus::ThreeRareCards,
                    4 => ThirdBonus::TwoFiftyGold,
                    5 => ThirdBonus::TransformTwoCards,
                    6 => ThirdBonus::TwentyPercentHpBonus,
                    _ => unreachable!(),
                }
            }
        };

        let third = (drawback, third_bonus);

        Self { first, second, third }
    }
}