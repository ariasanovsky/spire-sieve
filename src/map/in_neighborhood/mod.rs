pub mod in_enum;

#[derive(Debug, Default)]
pub struct InVec {
    values: Vec<(usize, usize)>,
}

pub trait InNeighborhood<'a, 'b>
where
    'b: 'a,
{
    type Iter: Iterator<Item = &'b (usize, usize)> + 'a;
    fn min(&'a self) -> Option<&'a usize> {
        self.iter().map(|(a, _)| a).min()
    }
    fn max(&'a self) -> Option<&'a usize> {
        self.iter().map(|(a, _)| a).max()
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
    type Iter = std::slice::Iter<'a, (usize, usize)>;
    fn push(&mut self, value: usize) {
        self.values.push((value, 1));
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
    type Iter = std::slice::Iter<'static, (usize, usize)>;

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
            Self::One => [(1, 1)].iter(),
            Self::OneTwo => [(1, 1), (2, 1)].iter(),
        }
    }
}
