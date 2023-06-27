use super::{OneCmp, Plus, ThreeCmp, TwoCmp};

impl Plus for OneCmp {
    type Next = TwoCmp;
    fn plus_prev(self) -> Self::Next {
        match self {
            Self::One => Self::Next::OneOne,
            Self::Two => Self::Next::OneTwo,
            Self::Three => Self::Next::OneThree,
            Self::Four => Self::Next::OneFour,
            Self::Five => Self::Next::OneFive,
            Self::Six => unreachable!(),
        }
    }
    fn plus_next(self) -> Self::Next {
        match self {
            Self::One => Self::Next::OneOne,
            Self::Two => Self::Next::TwoOne,
            Self::Three => Self::Next::ThreeOne,
            Self::Four => Self::Next::FourOne,
            Self::Five => Self::Next::FiveOne,
            Self::Six => unreachable!(),
        }
    }
}

impl Plus for TwoCmp {
    type Next = ThreeCmp;

    fn plus_prev(self) -> Self::Next {
        match self {
            Self::OneOne => Self::Next::OneOneOne,
            Self::OneTwo => Self::Next::OneOneTwo,
            Self::OneThree => Self::Next::OneOneThree,
            Self::OneFour => Self::Next::OneOneFour,
            Self::OneFive => unreachable!(),
            Self::TwoOne => Self::Next::OneTwoOne,
            Self::TwoTwo => Self::Next::OneTwoTwo,
            Self::TwoThree => Self::Next::OneTwoThree,
            Self::TwoFour => unreachable!(),
            Self::ThreeOne => Self::Next::OneThreeOne,
            Self::ThreeTwo => Self::Next::OneThreeTwo,
            Self::ThreeThree => unreachable!(),
            Self::FourOne => Self::Next::OneFourOne,
            Self::FourTwo => unreachable!(),
            Self::FiveOne => unreachable!(),
        }
    }

    fn plus_next(self) -> Self::Next {
        match self {
            Self::OneOne => Self::Next::OneOneOne,
            Self::OneTwo => Self::Next::OneTwoOne,
            Self::OneThree => Self::Next::OneThreeOne,
            Self::OneFour => Self::Next::OneFourOne,
            Self::OneFive => unreachable!(),
            Self::TwoOne => Self::Next::TwoOneOne,
            Self::TwoTwo => Self::Next::TwoTwoOne,
            Self::TwoThree => Self::Next::TwoThreeOne,
            Self::TwoFour => unreachable!(),
            Self::ThreeOne => Self::Next::ThreeOneOne,
            Self::ThreeTwo => Self::Next::ThreeTwoOne,
            Self::ThreeThree => unreachable!(),
            Self::FourOne => Self::Next::FourOneOne,
            Self::FourTwo => unreachable!(),
            Self::FiveOne => unreachable!(),
        }
    }
}
