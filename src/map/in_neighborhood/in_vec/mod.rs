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

impl TryInto<InArray> for InVec {
    type Error = ();
    fn try_into(self) -> Result<InArray, ()> {
        let mut counts: [usize; 7] = [0; 7];
        for (value, count) in self.values {
            counts[value] += count;
        }
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
        use InArray::*;
        Ok(match &shortened_vec[..] {
            [] => Zero([]),
            [(value, count)] => One([(*value, *count)]),
            [(value1, count1), (value2, count2)] if *value1 + 1 == *value2 => {
                Two([(*value1, *count1), (*value2, *count2)])
            }
            [(value1, count1), (value2, count2), (value3, count3)]
                if *value1 + 1 == *value2 && *value2 + 1 == *value3 =>
            {
                Three([(*value1, *count1), (*value2, *count2), (*value3, *count3)])
            }
            _ => return Err(()),
        })
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
    use super::*;
    const ARRAYS: [InArray; 233] = InArray::at_most_six();

    #[test]
    fn test_bijection() {
        for array in ARRAYS {
            let in_neighborhood = InVec::from(array);
            let new_array: InArray = in_neighborhood.try_into().unwrap();
            assert_eq!(new_array, array);
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
