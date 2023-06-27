mod add_first;
mod add_second;
mod add_third;
mod plus;

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

pub trait AddFirst {
    fn add_first(&mut self);
}

pub trait AddSecond {
    fn add_second(&mut self);
}

pub trait AddThird {
    fn add_third(&mut self);
}

pub trait Plus {
    type Next;
    fn plus_prev(self) -> Self::Next;
    fn plus_next(self) -> Self::Next;
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
