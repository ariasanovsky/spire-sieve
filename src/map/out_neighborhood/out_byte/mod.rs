use std::fmt::Display;

use crate::map::in_neighborhood::WIDTH;

use super::{out_array::{OutArray, ARRAYS, OUT_NEIGHBORHOODS}, OutNeighborhood};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutByte(u8);

impl Default for OutByte {
    fn default() -> Self {
        Self(0)
    }
}

impl Display for OutByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<OutByte> for OutArray {
    fn from(value: OutByte) -> Self {
        ARRAYS[value.0 as usize].clone()
    }
}

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

    const fn left_update_table() -> [[usize; WIDTH]; OUT_NEIGHBORHOODS] {
        let mut table = [[0; WIDTH]; OUT_NEIGHBORHOODS];
        const MAX_TABLE: [Option<usize>; OUT_NEIGHBORHOODS] = OutByte::max_table();
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

    const fn right_update_table() -> [[usize; WIDTH]; OUT_NEIGHBORHOODS] {
        let mut table = [[0; WIDTH]; OUT_NEIGHBORHOODS];
        const MIN_TABLE: [Option<usize>; OUT_NEIGHBORHOODS] = OutByte::min_table();
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

    const fn max_table() -> [Option<usize>; OUT_NEIGHBORHOODS] {
        let mut table = [None; OUT_NEIGHBORHOODS];
        let mut i = 0;
        while i < table.len() {
            if let Some(max) = ARRAYS[i].max() {
                table[i] = Some(*max);
            }
            i += 1;
        }
        table
    }

    const fn min_table() -> [Option<usize>; OUT_NEIGHBORHOODS] {
        let mut table = [None; OUT_NEIGHBORHOODS];
        let mut i = 0;
        while i < table.len() {
            if let Some(min) = ARRAYS[i].min() {
                table[i] = Some(*min);
            }
            i += 1;
        }
        table
    }

    const fn push_table() -> [[Self; WIDTH]; OUT_NEIGHBORHOODS] {
        let mut table = [[Self(0); WIDTH]; OUT_NEIGHBORHOODS];
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

    const fn remove_table() -> [[Self; WIDTH]; OUT_NEIGHBORHOODS] {
        let mut table = [[Self(0); WIDTH]; OUT_NEIGHBORHOODS];
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
        const LEFT_UPDATE_TABLE: [[usize; WIDTH]; OUT_NEIGHBORHOODS] = OutByte::left_update_table();
        *value = LEFT_UPDATE_TABLE[self.0 as usize][*value];
    }

    fn update_position_from_right(&self, value: &mut usize) {
        const RIGHT_UPDATE_TABLE: [[usize; WIDTH]; OUT_NEIGHBORHOODS] = OutByte::right_update_table();
        *value = RIGHT_UPDATE_TABLE[self.0 as usize][*value];
    }

    fn push(&mut self, value: usize) {
        const PUSH_TABLE: [[OutByte; WIDTH]; OUT_NEIGHBORHOODS] = OutByte::push_table();
        *self = PUSH_TABLE[self.0 as usize][value];
    }

    fn remove(&mut self, value: usize) {
        const REMOVE_TABLE: [[OutByte; WIDTH]; OUT_NEIGHBORHOODS] = OutByte::remove_table();
        *self = REMOVE_TABLE[self.0 as usize][value];
    }

    fn iter(&'a self) -> Self::Iter {
        ARRAYS[self.0 as usize].iter()
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
