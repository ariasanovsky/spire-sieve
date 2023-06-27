use super::{AddThird, ThreeCmp};

impl AddThird for ThreeCmp {
    fn add_third(&mut self) {
        *self = match self {
            Self::OneOneOne => Self::OneOneTwo,
            Self::OneOneTwo => Self::OneOneThree,
            Self::OneOneThree => Self::OneOneFour,
            Self::OneOneFour => unreachable!(),
            Self::OneTwoOne => Self::OneTwoTwo,
            Self::OneTwoTwo => Self::OneTwoThree,
            Self::OneTwoThree => unreachable!(),
            Self::OneThreeOne => Self::OneThreeTwo,
            Self::OneThreeTwo => unreachable!(),
            Self::OneFourOne => unreachable!(),
            Self::TwoOneOne => Self::TwoOneTwo,
            Self::TwoOneTwo => Self::TwoOneThree,
            Self::TwoOneThree => unreachable!(),
            Self::TwoTwoOne => Self::TwoTwoTwo,
            Self::TwoTwoTwo => unreachable!(),
            Self::TwoThreeOne => unreachable!(),
            Self::ThreeOneOne => Self::ThreeOneTwo,
            Self::ThreeOneTwo => unreachable!(),
            Self::ThreeTwoOne => unreachable!(),
            Self::FourOneOne => unreachable!(),
        }
    }
}
