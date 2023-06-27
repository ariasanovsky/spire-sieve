use super::{AddFirst, OneCmp, TwoCmp, ThreeCmp};

impl AddFirst for OneCmp {
    fn add_first(&mut self) {
        *self = match self {
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => unreachable!(),
        }
    }
}

impl AddFirst for TwoCmp {
    fn add_first(&mut self) {
        *self = match self {
            Self::OneOne => Self::TwoOne,
            Self::OneTwo => Self::TwoTwo,
            Self::OneThree => Self::TwoThree,
            Self::OneFour => Self::TwoFour,
            Self::OneFive => unreachable!(),
            Self::TwoOne => Self::ThreeOne,
            Self::TwoTwo => Self::ThreeTwo,
            Self::TwoThree => Self::ThreeThree,
            Self::TwoFour => unreachable!(),
            Self::ThreeOne => Self::FourOne,
            Self::ThreeTwo => Self::FourTwo,
            Self::ThreeThree => unreachable!(),
            Self::FourOne => Self::FiveOne,
            Self::FourTwo => unreachable!(),
            Self::FiveOne => unreachable!(),
        }
    }
}

impl AddFirst for ThreeCmp {
    fn add_first(&mut self) {
        *self = match self {
            Self::OneOneOne => Self::TwoOneOne,
            Self::OneOneTwo => Self::TwoOneTwo,
            Self::OneOneThree => Self::TwoOneThree,
            Self::OneOneFour => unreachable!(),
            Self::OneTwoOne => Self::TwoTwoOne,
            Self::OneTwoTwo => Self::TwoTwoTwo,
            Self::OneTwoThree => unreachable!(),
            Self::OneThreeOne => Self::TwoThreeOne,
            Self::OneThreeTwo => unreachable!(),
            Self::OneFourOne => unreachable!(),
            Self::TwoOneOne => Self::ThreeOneOne,
            Self::TwoOneTwo => Self::ThreeOneTwo,
            Self::TwoOneThree => unreachable!(),
            Self::TwoTwoOne => Self::ThreeTwoOne,
            Self::TwoTwoTwo => unreachable!(),
            Self::TwoThreeOne => unreachable!(),
            Self::ThreeOneOne => Self::FourOneOne,
            Self::ThreeOneTwo => unreachable!(),
            Self::ThreeTwoOne => unreachable!(),
            Self::FourOneOne => unreachable!(),
        }
    }
}
