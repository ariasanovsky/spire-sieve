use self::backend::{NeighborhoodArray, NeighborhoodOfAtMostThreeConsecutiveElements};

use super::InNeighborhood;

mod backend;

#[derive(Debug, Clone, Copy, Default)]
struct InByte(u8);

impl<'a> InNeighborhood<'a, 'a> for InByte {
    type Iter = std::slice::Iter<'a, (usize, usize)>;

    fn min(&'a self) -> Option<&'a usize> {
        const MIN: [Option<usize>; 233] = InByte::min_table();
        MIN[self.0 as usize].as_ref()
    }

    fn max(&'a self) -> Option<&'a usize> {
        const MAX: [Option<usize>; 233] = InByte::max_table();
        MAX[self.0 as usize].as_ref()
    }

    fn push(&mut self, value: usize) {
        const SUM: [[u8; 7]; 233] = InByte::sum_table();
        self.0 = SUM[self.0 as usize][value];
    }

    fn iter(&'a self) -> Self::Iter {
        const ARRAYS: [NeighborhoodArray; 233] = NeighborhoodArray::at_most_six();
        ARRAYS[self.0 as usize].slice().iter()
    }
}

impl InByte {
    const fn min_table() -> [Option<usize>; 233] {
        let arrays = NeighborhoodArray::at_most_six();
        let mut table = [None; 233];
        let mut i = 0;
        while i < table.len() {
            table[i] = if let Some(min) = arrays[i].min() {
                Some(*min)
            } else {
                None
            };
            i += 1;
        }
        table
    }

    const fn max_table() -> [Option<usize>; 233] {
        let arrays = NeighborhoodArray::at_most_six();
        let mut table = [None; 233];
        let mut i = 0;
        while i < table.len() {
            table[i] = if let Some(max) = arrays[i].max() {
                Some(*max)
            } else {
                None
            };
            i += 1;
        }
        table
    }

    const fn sum_table() -> [[u8; 7]; 233] {
        let arrays = NeighborhoodArray::at_most_six();
        let mut table = [[0; 7]; 233];
        let mut i = 0;
        while i < arrays.len() {
            let array = arrays[i];
            let mut j = 0;
            while j < 7 {
                let sum = array.plus(j);
                if let Some(sum) = sum {
                    let mut k = 0;
                    while k < arrays.len() {
                        if arrays[k].const_eq(&sum) {
                            table[i][j] = k as u8;
                            break;
                        }
                        k += 1;
                    }
                } else {
                    table[i][j] = 0;
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
        const ARRAYS: [NeighborhoodArray; 233] = NeighborhoodArray::at_most_six();
        const MIN: [Option<usize>; 233] = InByte::min_table();
        for (byte, (array, min)) in ARRAYS.iter().zip(MIN.iter()).enumerate() {
            assert_eq!(
                array.min().copied(),
                *min,
                "byte: {byte}, array: {array}, min: {min:?}"
            );
        }
    }

    #[test]
    fn test_max_table() {
        const ARRAYS: [NeighborhoodArray; 233] = NeighborhoodArray::at_most_six();
        const MAX: [Option<usize>; 233] = InByte::max_table();
        for (byte, (array, max)) in ARRAYS.iter().zip(MAX.iter()).enumerate() {
            assert_eq!(
                array.max().copied(),
                *max,
                "byte: {byte}, array: {array}, max: {max:?}"
            );
        }
    }

    #[test]
    fn test_sum_table() {
        const ARRAYS: [NeighborhoodArray; 233] = NeighborhoodArray::at_most_six();
        const SUM: [[u8; 7]; 233] = InByte::sum_table();
        for (i, (array, sums)) in ARRAYS.iter().zip(SUM.iter()).enumerate() {
            for (j, sum) in sums.iter().enumerate() {
                if let Some((array_sum, sum_position)) = array.plus(j).and_then(|array_sum| {
                    ARRAYS
                        .iter()
                        .position(|array| array.const_eq(&array_sum))
                        .map(|position| (array_sum, position))
                }) {
                    assert_eq!(
                        *sum as usize, sum_position,
                        "
{i}:\t{array}
\t{sums:?}
\t{array} + {j} = {array_sum}
\tfoo"
                    );
                } else {
                    assert_eq!(
                        *sum, 0,
                        "
{i}:\t{array}
\t{sums:?}
\t{array} + {j} = None
\tbar"
                    );
                }
            }
        }
    }
}
