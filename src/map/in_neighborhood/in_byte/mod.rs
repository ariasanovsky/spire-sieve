use crate::map::in_neighborhood::{NEIGHBORHOODS, WIDTH};

use super::{in_array::InArray, InNeighborhood};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]

pub struct InByte(u16);

impl From<InArray> for InByte {
    fn from(array: InArray) -> Self {
        const ARRAYS: [InArray; NEIGHBORHOODS] = InArray::at_most_six();
        for (i, other_array) in ARRAYS.iter().enumerate() {
            if array.const_eq(other_array) {
                return Self(i as u16);
            }
        }
        // unreachable!("{array} could not be found")
        Self(0)
    }
}

impl From<InByte> for InArray {
    fn from(in_byte: InByte) -> Self {
        const ARRAYS: [InArray; NEIGHBORHOODS] = InArray::at_most_six();
        ARRAYS[in_byte.0 as usize]
    }
}

impl<'a> InNeighborhood<'a> for InByte {
    type Iter = core::slice::Iter<'a, (usize, usize)>;

    fn min(&self) -> Option<&usize> {
        const MIN: [Option<usize>; NEIGHBORHOODS] = InByte::min_table();
        MIN[self.0 as usize].as_ref()
    }

    fn max(&self) -> Option<&usize> {
        const MAX: [Option<usize>; NEIGHBORHOODS] = InByte::max_table();
        MAX[self.0 as usize].as_ref()
    }

    fn push(&mut self, value: usize) {
        const SUM: [[InByte; WIDTH]; NEIGHBORHOODS] = InByte::sum_table();
        *self = SUM[self.0 as usize][value];
    }

    fn iter(&self) -> Self::Iter {
        const ARRAYS: [InArray; NEIGHBORHOODS] = InArray::at_most_six();
        ARRAYS[self.0 as usize].slice().iter()
    }
}

impl InByte {
    const fn min_table() -> [Option<usize>; NEIGHBORHOODS] {
        let arrays = InArray::at_most_six();
        let mut table = [None; NEIGHBORHOODS];
        let mut i = 0;
        while i < table.len() {
            table[i] = if let Some(min) = arrays[i].min() {
                Some(*min)
            } else {
                assert!(i == 0);
                None
            };
            i += 1;
        }
        table
    }

    const fn max_table() -> [Option<usize>; NEIGHBORHOODS] {
        let arrays = InArray::at_most_six();
        let mut table = [None; NEIGHBORHOODS];
        let mut i = 0;
        while i < table.len() {
            table[i] = if let Some(max) = arrays[i].max() {
                Some(*max)
            } else {
                assert!(i == 0);
                None
            };
            i += 1;
        }
        table
    }

    const fn sum_table() -> [[Self; WIDTH]; NEIGHBORHOODS] {
        let arrays = InArray::at_most_six();
        let mut table = [[Self(0); WIDTH]; NEIGHBORHOODS];
        let mut i = 0;
        while i < arrays.len() {
            let array = arrays[i];
            let mut j = 0;
            while j < WIDTH {
                let sum = array.plus(j);
                if let Some(sum) = sum {
                    let mut k = 0;
                    while k < arrays.len() {
                        if arrays[k].const_eq(&sum) {
                            table[i][j] = Self(k as u16);
                            break;
                        }
                        k += 1;
                    }
                } else {
                    table[i][j] = Self(0);
                }
                j += 1;
            }
            i += 1;
        }
        table
    }
}

#[cfg(test)]
mod test_in_byte_tables {
    use super::*;

    #[test]
    fn test_min_table() {
        const ARRAYS: [InArray; NEIGHBORHOODS] = InArray::at_most_six();
        const MIN: [Option<usize>; NEIGHBORHOODS] = InByte::min_table();
        for (byte, (array, min)) in ARRAYS.iter().zip(MIN.iter()).enumerate() {
            #[cfg(feature = "std")]
            assert_eq!(
                array.min().copied(),
                *min,
                "byte: {byte}, array: {array}, min: {min:?}"
            );

            #[cfg(not(feature = "std"))]
            assert_eq!(
                array.min().copied(),
                *min
            );
        }
    }

    #[test]
    fn test_max_table() {
        const ARRAYS: [InArray; NEIGHBORHOODS] = InArray::at_most_six();
        const MAX: [Option<usize>; NEIGHBORHOODS] = InByte::max_table();
        for (byte, (array, max)) in ARRAYS.iter().zip(MAX.iter()).enumerate() {
            #[cfg(feature = "std")]
            assert_eq!(
                array.max().copied(),
                *max,
                "byte: {byte}, array: {array}, max: {max:?}"
            );
            
            #[cfg(not(feature = "std"))]
            assert_eq!(
                array.max().copied(),
                *max
            );
        }
    }

    #[test]
    fn test_sum_table() {
        const ARRAYS: [InArray; NEIGHBORHOODS] = InArray::at_most_six();
        const SUM: [[InByte; WIDTH]; NEIGHBORHOODS] = InByte::sum_table();
        for (i, (array, sums)) in ARRAYS.iter().zip(SUM.iter()).enumerate() {
            for (j, sum) in sums.iter().enumerate() {
                if let Some((array_sum, sum_position)) = array.plus(j).and_then(|array_sum| {
                    ARRAYS
                        .iter()
                        .position(|array| array.const_eq(&array_sum))
                        .map(|position| (array_sum, position))
                }) {
                    #[cfg(not(feature = "std"))]
                    assert_eq!(
                        sum.0 as usize, sum_position
                    );

                    #[cfg(feature = "std")]
                    assert_eq!(
                        sum.0 as usize, sum_position,
                        "
{i}:\t{array}
\t{sums:?}
\t{array} + {j} = {array_sum}
\tfoo"
                    );
                } else {
                    #[cfg(feature = "std")]
                    assert_eq!(
                        sum.0, 0,
                        "
{i}:\t{array}
\t{sums:?}
\t{array} + {j} = None
\tbar"
                    );

                    #[cfg(not(feature = "std"))]
                    assert_eq!(
                        sum.0, 0
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod test_invec_against_neighborhood_array {
    
    use super::*;
    const ARRAYS: [InArray; NEIGHBORHOODS] = InArray::at_most_six();

    #[test]
    fn test_bijection() {
        for (i, &array) in ARRAYS.iter().enumerate() {
            let in_neighborhood = InByte::from(array);
            let new_array: InArray = in_neighborhood.try_into().unwrap();
            #[cfg(not(feature = "std"))]
            assert_eq!(
                new_array,
                array
            );

            #[cfg(feature = "std")]
            assert_eq!(
                new_array,
                array,
                "{i}:\tarray: {array}, in_neighborhood: {in_neighborhood:?}, new_array: {new_array}"
            );
        }
    }

    #[test]
    fn test_invec_min() {
        for array in ARRAYS {
            let invec = InByte::from(array);
            assert_eq!(invec.min(), array.min());
        }
    }

    #[test]
    fn test_invec_max() {
        for array in ARRAYS {
            let invec = InByte::from(array);
            assert_eq!(invec.max(), array.max());
        }
    }

    #[test]
    fn test_push() {
        for array in ARRAYS {
            for position in 0..WIDTH {
                let mut in_neighborhood = InByte::from(array);
                in_neighborhood.push(position);
                let array_sum: Option<InArray> = array.plus(position);
                #[cfg(feature = "std")]
                assert_eq!(
                    in_neighborhood,
                    array_sum.map(Into::into).unwrap_or_default(),
                    "
{array} + {position} = {array_sum:?}
invec: {in_neighborhood:?}"
                );

                #[cfg(not(feature = "std"))]
                assert_eq!(
                    in_neighborhood,
                    array_sum.map(Into::into).unwrap_or_default()
                );
            }
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_iter() {
        use alloc::vec::Vec;
        for array in ARRAYS.iter() {
            let invec = InByte::from(*array);
            let vec: Vec<_> = invec.iter().collect();
            assert_eq!(vec, array.slice().iter().collect::<Vec<_>>())
        }
    }
}
