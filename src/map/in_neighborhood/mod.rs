pub mod one_cmp;
pub mod two_cmp;
pub mod three_cmp;

#[derive(Debug, Default)]
pub struct InVec {
    values: Vec<usize>,
}

pub trait InNeighborhood<'a, 'b>
where
    'b: 'a,
{
    type Iter: Iterator<Item = &'b usize> + 'a;
    fn min(&'a self) -> Option<&'a usize> {
        self.iter().min()
    }
    fn max(&'a self) -> Option<&'a usize> {
        self.iter().max()
    }
    fn push(&mut self, value: usize);
    fn iter(&'a self) -> Self::Iter;
    fn gca_skip(left: &'a Self, right: &'a Self) -> bool {
        match (left.max(), right.min()) {
            (Some(left_max), Some(right_min)) => left_max != right_min,
            _ => true,
        }
    }
}

impl<'a> InNeighborhood<'a, 'a> for InVec {
    type Iter = std::slice::Iter<'a, usize>;
    fn push(&mut self, value: usize) {
        self.values.push(value);
    }
    fn iter(&'a self) -> Self::Iter {
        self.values.iter()
    }
}

#[derive(Debug, Default)]
enum _OtherNeighborhood {
    #[default]
    Empty,
    One,
    OneTwo,
}

impl<'a> InNeighborhood<'a, 'static> for _OtherNeighborhood {
    type Iter = std::slice::Iter<'static, usize>;

    fn min(&'a self) -> Option<&'a usize> {
        match self {
            Self::Empty => None,
            Self::One => Some(&1),
            Self::OneTwo => Some(&1),
        }
    }

    fn max(&'a self) -> Option<&'a usize> {
        match self {
            Self::Empty => None,
            Self::One => Some(&1),
            Self::OneTwo => Some(&2),
        }
    }

    fn push(&mut self, value: usize) {
        match (&self, value) {
            (Self::Empty, 1) => *self = Self::One,
            (Self::One, 2) => *self = Self::OneTwo,
            _ => unimplemented!("Invalid value"),
        }
    }

    fn iter(&'a self) -> Self::Iter {
        match self {
            Self::Empty => [].iter(),
            Self::One => [1].iter(),
            Self::OneTwo => [1, 2].iter(),
        }
    }
}

#[derive(Debug, Default)]
pub enum InEnum {
    #[default]
    Empty,
    Less(OneCmp),
    Equal(OneCmp),
    Greater(OneCmp),
    LessEqual(TwoCmp),
    EqualGreater(TwoCmp),
    LessEqualGreater(ThreeCmp),
}

#[derive(Debug, Default)]
pub enum OneCmp {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Default)]
pub enum TwoCmp {
    #[default]
    OneOne,
    OneTwo,
    OneThree,
    OneFour,
    OneFive,
    TwoOne,
    TwoTwo,
    TwoThree,
    TwoFour,
    ThreeOne,
    ThreeTwo,
    ThreeThree,
    FourOne,
    FourTwo,
    FiveOne,   
}

#[derive(Debug, Default)]
pub enum ThreeCmp {
    #[default]
    OneOneOne,
    OneOneTwo,
    OneOneThree,
    OneOneFour,
    OneTwoOne,
    OneTwoTwo,
    OneTwoThree,
    OneThreeOne,
    OneThreeTwo,
    OneFourOne,
    TwoOneOne,
    TwoOneTwo,
    TwoOneThree,
    TwoTwoOne,
    TwoTwoTwo,
    TwoThreeOne,
    ThreeOneOne,
    ThreeOneTwo,
    ThreeTwoOne,
    FourOneOne,
}

pub trait AddFirst {
    fn add_first(&mut self);
}

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

impl OneCmp {
    pub fn plus_next(self) -> TwoCmp {
        match self {
            Self::One => TwoCmp::OneOne,
            Self::Two => TwoCmp::TwoOne,
            Self::Three => TwoCmp::ThreeOne,
            Self::Four => TwoCmp::FourOne,
            Self::Five => TwoCmp::FiveOne,
            Self::Six => unreachable!(),
        }
    }
}

impl TwoCmp {
    pub fn add_second(&mut self) {
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

    pub fn plus_next(self) -> ThreeCmp {
        match self {
            Self::OneOne => ThreeCmp::OneOneOne,
            Self::OneTwo => ThreeCmp::OneTwoOne,
            Self::OneThree => ThreeCmp::OneOneThree,
            Self::OneFour => ThreeCmp::OneOneFour,
            Self::OneFive => unreachable!(),
            Self::TwoOne => ThreeCmp::OneTwoOne,
            Self::TwoTwo => ThreeCmp::OneTwoTwo,
            Self::TwoThree => ThreeCmp::OneTwoThree,
            Self::TwoFour => unreachable!(),
            Self::ThreeOne => ThreeCmp::OneThreeOne,
            Self::ThreeTwo => ThreeCmp::OneThreeTwo,
            Self::ThreeThree => unreachable!(),
            Self::FourOne => ThreeCmp::OneFourOne,
            Self::FourTwo => unreachable!(),
            Self::FiveOne => unreachable!(),
        }
    }
}

impl ThreeCmp {
    pub fn add_second(&mut self) {
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

    pub fn add_third(&mut self) {
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

// pub struct NumberedInEnum {
//     pub in_enum: InEnum,
//     pub position: usize,
// }

// impl<'a> InNeighborhood<'a> for NumberedInEnum {
//     fn min(&self) -> Option<usize> {
//         let Self { in_enum, position } = self;
//         Some(match in_enum {
//             InEnum::Empty => return None,
//             InEnum::Less(_) | InEnum::LessEqual(_) | InEnum::LessEqualGreater(_)
//                 => position.wrapping_sub(1),
//             InEnum::Equal(_) | InEnum::EqualGreater(_) => *position,
//             InEnum::Greater(_) => position.wrapping_add(1),
//         })
//     }

//     fn max(&self) -> Option<usize> {
//         let Self { in_enum, position } = self;
//         Some(match in_enum {
//             InEnum::Empty => return None,
//             InEnum::Less(_) => position.wrapping_sub(1),
//             InEnum::Equal(_) | InEnum::LessEqual(_) => *position,
//             InEnum::Greater(_) | InEnum::EqualGreater(_) | InEnum::LessEqualGreater(_)
//                 => position.wrapping_sub(1),
//         })
//     }

//     fn push(&mut self, value: usize) {
//         let cmp = value.cmp(&self.position);
//         self.in_enum.push(cmp);
//     }
// }

// trait OrderingContainer {
//     fn push(&mut self, cmp: std::cmp::Ordering);
// }

// impl OrderingContainer for InEnum {
//     fn push(&mut self, cmp: std::cmp::Ordering) {

//         // match (&self, cmp) {
//         //     (InEnum::Empty, std::cmp::Ordering::Less) => *self = InEnum::Less(Default::default()),
//         //     (InEnum::Empty, std::cmp::Ordering::Equal) => *self = InEnum::Equal(Default::default()),
//         //     (InEnum::Empty, std::cmp::Ordering::Greater) => *self = InEnum::Greater(Default::default()),
//         //     (InEnum::Less(mut in_one), std::cmp::Ordering::Less) => in_one.add_first(),
//         //     (InEnum::Less(in_one), std::cmp::Ordering::Equal) => todo!(),
//         //     (InEnum::Less(_), std::cmp::Ordering::Greater) => unreachable!(),
//         //     (InEnum::Equal(_), std::cmp::Ordering::Less) => todo!(),
//         //     (InEnum::Equal(_), std::cmp::Ordering::Equal) => todo!(),
//         //     (InEnum::Equal(_), std::cmp::Ordering::Greater) => todo!(),
//         //     (InEnum::Greater(_), std::cmp::Ordering::Less) => unreachable!(),
//         //     (InEnum::Greater(_), std::cmp::Ordering::Equal) => todo!(),
//         //     (InEnum::Greater(_), std::cmp::Ordering::Greater) => todo!(),
//         //     (InEnum::LessEqual(_), std::cmp::Ordering::Less) => todo!(),
//         //     (InEnum::LessEqual(_), std::cmp::Ordering::Equal) => todo!(),
//         //     (InEnum::LessEqual(_), std::cmp::Ordering::Greater) => todo!(),
//         //     (InEnum::EqualGreater(_), std::cmp::Ordering::Less) => todo!(),
//         //     (InEnum::EqualGreater(_), std::cmp::Ordering::Equal) => todo!(),
//         //     (InEnum::EqualGreater(_), std::cmp::Ordering::Greater) => todo!(),
//         //     (InEnum::LessEqualGreater(_), std::cmp::Ordering::Less) => todo!(),
//         //     (InEnum::LessEqualGreater(_), std::cmp::Ordering::Equal) => todo!(),
//         //     (InEnum::LessEqualGreater(_), std::cmp::Ordering::Greater) => todo!(),
//         // }
//         todo!()
//     }
// }

// impl Iterator for NumberedInEnum {
//     type Item = usize;
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }