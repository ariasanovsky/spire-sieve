use super::OutNeighborhood;

static PLUS: [[OutByte; 7]; 29] = OutByte::push_table();
static MAX: [Option<u8>; 29] = OutByte::max_table();
static MIN: [Option<u8>; 29] = OutByte::min_table();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct OutByte(u8);

impl OutByte {
    pub const fn push_table() -> [[OutByte; 7]; 29] {
        let mut i = 0;
        let mut table = [[OutByte(0); 7]; 29];
        let mut interval = IntervalHelper::Zero;
        while i < 29 {
            let mut j = 0;
            while j < 7 {
                let sum = interval.plus(j);
                table[i][j] = OutByte(sum.index());
                j += 1;
            }
            interval = if let Some(interval) = interval.next() {
                interval
            } else {
                IntervalHelper::Zero
            };
            i += 1;
        }
        table
    }

    pub const fn max_table() -> [Option<u8>; 29] {
        let mut i = 0;
        let mut table = [None; 29];
        let mut interval = IntervalHelper::Zero;
        while i < 29 {
            table[i] = interval.max();
            interval = if let Some(interval) = interval.next() {
                interval
            } else {
                IntervalHelper::Zero
            };
            i += 1;
        }
        table
    }

    pub const fn min_table() -> [Option<u8>; 29] {
        let mut i = 0;
        let mut table = [None; 29];
        let mut interval = IntervalHelper::Zero;
        while i < 29 {
            table[i] = interval.min();
            interval = if let Some(interval) = interval.next() {
                interval
            } else {
                IntervalHelper::Zero
            };
            i += 1;
        }
        table
    }
}

impl<'a> OutNeighborhood<'a, 'a> for OutByte {
    type Iter = std::slice::Iter<'a, usize>;

    fn update_position_from_left(&'a self, value: &'a mut usize) {
        if let Some(max) = MAX[self.0 as usize] {
            *value = (max as usize).min(*value)
        }
    }

    fn update_position_from_right(&'a self, value: &'a mut usize) {
        if let Some(min) = MIN[self.0 as usize] {
            *value = (min as usize).max(*value)
        }
    }

    fn push(&mut self, value: usize) {
        *self = PLUS[self.0 as usize][value];
    }

    fn remove(&mut self, value: usize) {
        todo!()
    }

    fn iter(&'a self) -> Self::Iter {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Eq, PartialEq)]
enum IntervalHelper {
    Zero,
    One(u8),
    Two(u8, u8),
}

impl IntervalHelper {
    const fn const_cmp(a: u8, b: u8) -> std::cmp::Ordering {
        if a < b {
            std::cmp::Ordering::Less
        } else if a == b {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    }
    
    const fn plus(&self, next: usize) -> IntervalHelper {
        use IntervalHelper::*;
        match self {
            Zero => One(next as u8),
            One(a) => match Self::const_cmp(next as u8, *a) {
                std::cmp::Ordering::Less => Two(next as u8, *a),
                std::cmp::Ordering::Equal => One(*a),
                std::cmp::Ordering::Greater => Two(*a, next as u8),
            },
            Two(a, b) => match Self::const_cmp(next as u8, *a) {
                std::cmp::Ordering::Less => Two(next as u8, *b),
                std::cmp::Ordering::Equal => Two(*a, *b),
                std::cmp::Ordering::Greater => match Self::const_cmp(next as u8, *b) {
                    std::cmp::Ordering::Less => Two(*a, *b),
                    std::cmp::Ordering::Equal => Two(*a, *b),
                    std::cmp::Ordering::Greater => Two(*a, next as u8),
                },
            },
        }
    }

    const fn next(&self) -> Option<IntervalHelper> {
        use IntervalHelper::*;
        Some(match self {
            Zero => One(0),
            One(a) => {
                if *a == 6 {
                    Two(0, 1)
                } else {
                    One(*a + 1)
                }
            },
            Two(a, b) => {
                if *b == *a + 1 {
                    if *b == 6 {
                        return None;
                    } else {
                        Two(0, *b + 1)
                    }
                } else {
                    Two(*a + 1, *b)
                }
            }
        })
    }

    const fn index(&self) -> u8 {
        use IntervalHelper::*;
        match self {
            Zero => 0,
            One(a) => *a + 1,
            Two(a, b) => {
                let b_plus_1 = *b + 1;
                let b_plus_1_choose_2 = b_plus_1.wrapping_mul(*b).wrapping_div(2);
                let b_minus_a = b.wrapping_sub(*a);
                8 + b_plus_1_choose_2.wrapping_sub(b_minus_a)
            }
        }
    }

    const fn min(&self) -> Option<u8> {
        use IntervalHelper::*;
        match self {
            Zero => None,
            One(a) => Some(*a),
            Two(a, _) => Some(*a),
        }
    }

    const fn max(&self) -> Option<u8> {
        use IntervalHelper::*;
        match self {
            Zero => None,
            One(a) => Some(*a),
            Two(_, b) => Some(*b),
        }
    }
}

#[cfg(test)]
mod interval_tests {
    use super::{IntervalHelper::*, OutByte};

    #[test]
    fn test_tables() {
        const PLUS: [[OutByte; 7]; 29] = OutByte::push_table();
        const MAX: [Option<u8>; 29] = OutByte::max_table();
        const MIN: [Option<u8>; 29] = OutByte::min_table();

        let mut i = 0;
        let mut interval = Zero;
        while i < 29 {
            println!("{i}\t{interval:?}");
            println!("\t{:?}..{:?}", interval.min(), interval.max());
            println!("\t{:?}..{:?}", MIN[i as usize], MAX[i as usize]);
            let out_byte = OutByte(i);
            println!("\t{out_byte:?}");
            assert_eq!(interval.index(), i);
            assert_eq!(interval.min(), MIN[i as usize]);
            assert_eq!(interval.max(), MAX[i as usize]);
            for j in 0..7 {
                let sum = interval.plus(j);
                let sum_byte = OutByte(sum.index());
                println!("\t\t+ {j} = {sum:?} -> {:?}", sum_byte);
                assert_eq!(sum_byte, PLUS[i as usize][j]);
            }
            interval = interval.next().unwrap_or(Zero);
            i += 1;
        }
    }
}
