use super::OutNeighborhood;

#[derive(Debug, Clone)]
pub enum OutArray {
    Zero([usize; 0]),
    One([usize; 1]),
    Two([usize; 2]),
    Three([usize; 3]),
}

pub const OUT_NEIGHBORHOODS: usize = 24;

pub const ARRAYS: [OutArray; OUT_NEIGHBORHOODS] = [
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
    OutArray::Two([0, 2]),
    OutArray::Two([1, 3]),
    OutArray::Two([2, 4]),
    OutArray::Two([3, 5]),
    OutArray::Two([4, 6]),
    OutArray::Three([0, 1, 2]),
    OutArray::Three([1, 2, 3]),
    OutArray::Three([2, 3, 4]),
    OutArray::Three([3, 4, 5]),
    OutArray::Three([4, 5, 6]),
];

impl Default for OutArray {
    fn default() -> Self {
        Self::Zero([])
    }
}

impl OutArray {
    pub const fn values(&self) -> &[usize] {
        match self {
            OutArray::Zero(values) => values,
            OutArray::One(values) => values,
            OutArray::Two(values) => values,
            OutArray::Three(values) => values,
        }
    }

    pub const fn min(&self) -> Option<&usize> {
        if let [min, ..] = self.values() {
            Some(min)
        } else {
            None
        }
    }

    pub const fn max(&self) -> Option<&usize> {
        if let [.., max] = self.values() {
            Some(max)
        } else {
            None
        }
    }

    pub const fn _contains(&self, position: usize) -> bool {
        let mut contents = self.values();
        while let Some((value, next_contents)) = contents.split_first() {
            if *value == position {
                return true;
            }
            contents = next_contents;
        }
        false
    }

    pub const fn const_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Zero([]), Self::Zero([])) => true,
            (Self::One([a]), Self::One([b])) => *a == *b,
            (Self::Two([a, b]), Self::Two([c, d])) => *a == *c && *b == *d,
            (Self::Three([a, b, c]), Self::Three([d, e, f])) => *a == *d && *b == *e && *c == *f,
            _ => false,
        }
    }

    pub const fn plus(&self, position: usize) -> Option<Self> {
        Some(match self {
            Self::Zero([]) => Self::One([position]),
            Self::One([a]) => {
                if *a == position {
                    Self::One([position])
                } else if *a == position + 1 || *a == position + 2 {
                    Self::Two([position, *a])
                } else if position == *a + 1 || position == *a + 2 {
                    Self::Two([*a, position])
                } else {
                    return None;
                }
            }
            Self::Two([a, b]) => {
                if *a == position || *b == position {
                    Self::Two([*a, *b])
                } else if *a == position + 1 && *b == *a + 1 {
                    Self::Three([position, *a, *b])
                } else if *b == *a + 1 && position == *b + 1 {
                    Self::Three([*a, *b, position])
                } else if position == *a + 1 && *b == position + 1 {
                    Self::Three([*a, position, *b])
                } else {
                    return None;
                }
            }
            Self::Three([a, b, c]) => {
                if *a == position || *b == position || *c == position {
                    Self::Three([*a, *b, *c])
                } else {
                    return None;
                }
            }
        })
    }

    pub const fn minus(&self, position: usize) -> Option<Self> {
        Some(match self {
            Self::Zero([]) => return None,
            Self::One([a]) => {
                if *a == position {
                    Self::Zero([])
                } else {
                    return None;
                }
            }
            Self::Two([a, b]) => {
                if *a == position {
                    Self::One([*b])
                } else if *b == position {
                    Self::One([*a])
                } else {
                    return None;
                }
            }
            Self::Three([a, b, c]) => {
                if *a == position {
                    Self::Two([*b, *c])
                } else if *c == position {
                    Self::Two([*a, *b])
                } else {
                    return None;
                }
            }
        })
    }
}

impl<'a> OutNeighborhood<'a> for OutArray {
    type Iter = core::slice::Iter<'a, usize>;

    fn update_position_from_left(&self, value: &mut usize) {
        if let Some(max) = self.max().copied() {
            *value = max.max(*value);
        }
    }

    fn update_position_from_right(&self, value: &mut usize) {
        if let Some(min) = self.min().copied() {
            *value = min.min(*value);
        }
    }

    fn push(&mut self, value: usize) {
        *self = match self.plus(value) {
            Some(array) => array,
            None => unreachable!("{self:?} + {value}"),
        }
    }

    fn remove(&mut self, value: usize) {
        *self = match self.minus(value) {
            Some(array) => array,
            None => unreachable!(),
        }
    }

    fn iter(&'a self) -> Self::Iter {
        self.values().iter()
    }

    fn is_empty(&self) -> bool {
        self.values().is_empty()
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test_out_array {
    use std::println;

    use crate::map::WIDTH;

    use super::ARRAYS;

    #[test]
    fn test_plus_table() {
        for array in ARRAYS {
            println!("[{array}]");
            for i in 0..WIDTH as usize {
                println!("\t+ {i} = {:?}", array.plus(i));
            }
        }
    }
}
