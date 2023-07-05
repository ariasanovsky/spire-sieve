use super::{out_array::OutArray, OutNeighborhood};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutByte(u8);

const ARRAYS: [OutArray; 19] = [
    OutArray::Zero([]),
    OutArray::One([0]),
    OutArray::One([1]),
    OutArray::One([2]),
    OutArray::One([3]),
    OutArray::One([4]),
    OutArray::One([5]),
    OutArray::One([6]),
    OutArray::Two([0, 1]),
    OutArray::Two([1, 2]),
    OutArray::Two([2, 3]),
    OutArray::Two([3, 4]),
    OutArray::Two([4, 5]),
    OutArray::Two([5, 6]),
    OutArray::Three([0, 1, 2]),
    OutArray::Three([1, 2, 3]),
    OutArray::Three([2, 3, 4]),
    OutArray::Three([3, 4, 5]),
    OutArray::Three([4, 5, 6]),
];

impl OutByte {
    const fn position(array: &OutArray) -> usize {
        let mut i = 0;
        while i < ARRAYS.len() {
            if ARRAYS[i].const_eq(array) {
                return i;
            }
            i += 1;
        }
        unreachable!()
    }

    const fn left_update_table() -> [[usize; 7]; 19] {
        let mut table = [[0; 7]; 19];
        const MAX_TABLE: [Option<usize>; 19] = OutByte::max_table();
        let mut i = 0;
        while i < table.len() {
            let mut value = 0;
            while value < table[i].len() {
                if let Some(max) = MAX_TABLE[i] {
                    table[i][value] = if max < value { value } else { max };
                } else {
                    table[i][value] = value;
                }
                value += 1;
            }
            i += 1;
        }
        table
    }

    const fn right_update_table() -> [[usize; 7]; 19] {
        let mut table = [[0; 7]; 19];
        const MIN_TABLE: [Option<usize>; 19] = OutByte::min_table();
        let mut i = 0;
        while i < table.len() {
            let mut value = 0;
            while value < table[i].len() {
                if let Some(min) = MIN_TABLE[i] {
                    table[i][value] = if value < min { value } else { min };
                } else {
                    table[i][value] = value;
                }
                value += 1;
            }
            i += 1;
        }
        table
    }

    const fn max_table() -> [Option<usize>; 19] {
        let mut table = [None; 19];
        let mut i = 0;
        while i < table.len() {
            if let Some(max) = ARRAYS[i].max() {
                table[i] = Some(*max);
            }
            i += 1;
        }
        table
    }

    const fn min_table() -> [Option<usize>; 19] {
        let mut table = [None; 19];
        let mut i = 0;
        while i < table.len() {
            if let Some(min) = ARRAYS[i].min() {
                table[i] = Some(*min);
            }
            i += 1;
        }
        table
    }

    const fn push_table() -> [[Self; 7]; 19] {
        let mut table = [[Self(0); 7]; 19];
        let mut i = 0;
        while i < table.len() {
            let mut j = 0;
            let array = &ARRAYS[i];
            while j < table[i].len() {
                if let Some(array) = array.plus(j) {
                    table[i][j] = Self(Self::position(&array) as u8);
                }
                j += 1;
            }
            i += 1;
        }
        table
    }

    const fn remove_table() -> [[Self; 7]; 19] {
        let mut table = [[Self(0); 7]; 19];
        let mut i = 0;
        while i < table.len() {
            let mut j = 0;
            let array = &ARRAYS[i];
            while j < table[i].len() {
                if let Some(array) = array.minus(j) {
                    table[i][j] = Self(Self::position(&array) as u8);
                }
                j += 1;
            }
            i += 1;
        }
        table
    }
}

impl<'a> OutNeighborhood<'a> for OutByte {
    type Iter = std::slice::Iter<'a, usize>;

    fn update_position_from_left(&self, value: &mut usize) {
        const LEFT_UPDATE_TABLE: [[usize; 7]; 19] = OutByte::left_update_table();
        *value = LEFT_UPDATE_TABLE[self.0 as usize][*value];
    }

    fn update_position_from_right(&self, value: &mut usize) {
        const RIGHT_UPDATE_TABLE: [[usize; 7]; 19] = OutByte::right_update_table();
        *value = RIGHT_UPDATE_TABLE[self.0 as usize][*value];
    }

    fn push(&mut self, value: usize) {
        const PUSH_TABLE: [[OutByte; 7]; 19] = OutByte::push_table();
        *self = PUSH_TABLE[self.0 as usize][value];
    }

    fn remove(&mut self, value: usize) {
        const REMOVE_TABLE: [[OutByte; 7]; 19] = OutByte::remove_table();
        *self = REMOVE_TABLE[self.0 as usize][value];
    }

    fn iter(&'a self) -> Self::Iter {
        ARRAYS[self.0 as usize].iter()
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
