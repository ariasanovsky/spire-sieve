use super::OutNeighborhood;

#[derive(Debug, Clone)]
pub enum OutArray {
    Zero([usize; 0]),
    One([usize; 1]),
    Two([usize; 2]),
    Three([usize; 3]),
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
                } else if *a == position + 1 {
                    Self::Two([position, *a])
                } else if position == *a + 1 {
                    Self::Two([*a, position])
                } else {
                    return None;
                }
            }
            Self::Two([a, b]) => {
                if *a == position || *b == position {
                    Self::Two([*a, *b])
                } else if *a == position + 1 {
                    Self::Three([position, *a, *b])
                } else if position == *b + 1 {
                    Self::Three([*a, *b, position])
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
    type Iter = std::slice::Iter<'a, usize>;

    fn update_position_from_left(&self, value: &mut usize) {
        if let Some(max) = self.max() {
            *value = *max;
        }
    }

    fn update_position_from_right(&self, value: &mut usize) {
        if let Some(min) = self.min() {
            *value = *min;
        }
    }

    fn push(&mut self, value: usize) {
        *self = match self.plus(value) {
            Some(array) => array,
            None => unreachable!(),
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
