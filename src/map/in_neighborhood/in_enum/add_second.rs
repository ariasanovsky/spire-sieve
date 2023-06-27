use super::{AddSecond, ThreeCmp, TwoCmp};

impl AddSecond for TwoCmp {
    fn add_second(&mut self) {
        *self = match self {
            Self::OneOne => Self::OneTwo,
            Self::OneTwo => Self::OneThree,
            Self::OneThree => Self::OneFour,
            Self::OneFour => Self::OneFive,
            Self::OneFive => unreachable!(),
            Self::TwoOne => Self::TwoTwo,
            Self::TwoTwo => Self::TwoThree,
            Self::TwoThree => Self::TwoFour,
            Self::TwoFour => unreachable!(),
            Self::ThreeOne => Self::ThreeTwo,
            Self::ThreeTwo => Self::ThreeThree,
            Self::ThreeThree => unreachable!(),
            Self::FourOne => Self::FourTwo,
            Self::FourTwo => unreachable!(),
            Self::FiveOne => unreachable!(),
        }
    }
}

impl AddSecond for ThreeCmp {
    fn add_second(&mut self) {
        *self = match self {
            Self::OneOneOne => Self::OneTwoOne,
            Self::OneOneTwo => Self::OneTwoTwo,
            Self::OneOneThree => Self::OneTwoThree,
            Self::OneOneFour => unreachable!(),
            Self::OneTwoOne => Self::OneThreeOne,
            Self::OneTwoTwo => Self::OneThreeTwo,
            Self::OneTwoThree => unreachable!(),
            Self::OneThreeOne => Self::OneFourOne,
            Self::OneThreeTwo => unreachable!(),
            Self::OneFourOne => unreachable!(),
            Self::TwoOneOne => Self::TwoTwoOne,
            Self::TwoOneTwo => Self::TwoTwoTwo,
            Self::TwoOneThree => unreachable!(),
            Self::TwoTwoOne => Self::TwoThreeOne,
            Self::TwoTwoTwo => unreachable!(),
            Self::TwoThreeOne => unreachable!(),
            Self::ThreeOneOne => Self::ThreeTwoOne,
            Self::ThreeOneTwo => unreachable!(),
            Self::ThreeTwoOne => unreachable!(),
            Self::FourOneOne => unreachable!(),
        }
    }
}
