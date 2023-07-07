use crate::map::in_neighborhood::WIDTH;

use super::{in_array::InArray, InNeighborhood};

#[derive(Debug, Default, Clone)]
pub struct InVec {
    values: Vec<(usize, usize)>,
}

impl From<InArray> for InVec {
    fn from(array: InArray) -> Self {
        match array {
            InArray::Zero([]) => Self { values: vec![] },
            InArray::One(a) => Self { values: a.into() },
            InArray::Two(a) => Self { values: a.into() },
            InArray::Three(a) => Self { values: a.into() },
        }
    }
}

impl From<InVec> for InArray {
    fn from(value: InVec) -> InArray {
        // dbg!(&value);
        let mut counts: [usize; WIDTH] = [0; WIDTH];
        for (value, count) in value.values {
            counts[value] += count;
        }
        // dbg!(&counts);
        let shortened_vec = counts
            .iter()
            .enumerate()
            .filter_map(|(value, count)| {
                if *count == 0 {
                    None
                } else {
                    Some((value, *count))
                }
            })
            .collect::<Vec<_>>();
        // dbg!(&shortened_vec);
        match &shortened_vec[..] {
            [] => InArray::Zero([]),
            [a] => InArray::One([*a]),
            [a, b] => InArray::Two([*a, *b]),
            [a, b, c] => InArray::Three([*a, *b, *c]),
            _ => unreachable!(),
        }
    }
}

impl<'a> InNeighborhood<'a> for InVec {
    type Iter = std::slice::Iter<'a, (usize, usize)>;
    fn min(&'a self) -> Option<&'a usize> {
        self.iter().map(|(value, _)| value).min()
    }

    fn max(&'a self) -> Option<&'a usize> {
        self.iter().map(|(value, _)| value).max()
    }
    fn push(&mut self, value: usize) {
        self.values.push((value, 1));
    }
    fn iter(&'a self) -> Self::Iter {
        self.values.iter()
    }
}

// impl<'a> TestInNeighborhood<'a, 'a> for InVec {}

#[cfg(test)]
mod test_invec_against_neighborhood_array {
    use crate::map::in_neighborhood::NEIGHBORHOODS;

    use super::*;
    const ARRAYS: [InArray; NEIGHBORHOODS] = InArray::at_most_six();

    #[test]
    fn test_bijection() {
        for array in ARRAYS {
            dbg!(&array);
            let in_neighborhood = InVec::from(array);
            let new_array: InArray = in_neighborhood.clone().into();
            assert_eq!(
                new_array, array,
                "{array} -> {in_neighborhood} -> {new_array}"
            );
        }
    }

    #[test]
    fn test_invec_min() {
        for array in ARRAYS {
            let invec = InVec::from(array);
            assert_eq!(invec.min(), array.min());
        }
    }

    #[test]
    fn test_invec_max() {
        for array in ARRAYS {
            let invec = InVec::from(array);
            assert_eq!(invec.max(), array.max());
        }
    }

    #[test]
    fn test_push() {
        for array in ARRAYS {
            for position in 0..7 {
                let array_sum: Option<InArray> = array.plus(position);
                let mut invec = InVec::from(array);
                invec.push(position);
                let vec_sum: Option<InArray> = invec.try_into().ok();
                assert_eq!(vec_sum, array_sum);
            }
        }
    }

    #[test]
    fn test_iter() {
        for array in ARRAYS.iter() {
            let invec = InVec::from(*array);
            let vec: Vec<_> = invec.iter().collect();
            assert_eq!(vec, array.slice().iter().collect::<Vec<_>>())
        }
    }
}
