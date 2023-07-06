mod backend;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InArray {
    Zero([(usize, usize); 0]),
    One([(usize, usize); 1]),
    Two([(usize, usize); 2]),
    Three([(usize, usize); 3]),
}

impl Default for InArray {
    fn default() -> Self {
        Self::Zero([])
    }
}

impl<'a> InNeighborhood<'a> for InArray {
    type Iter = std::slice::Iter<'a, (usize, usize)>;

    fn push(&mut self, value: usize) {
        *self = self.plus(value).expect(&format!("{self} + {value}"))
    }

    fn iter(&'a self) -> Self::Iter {
        self.slice().iter()
    }
}

use crate::map::out_neighborhood::out_array::OutArray;

impl InArray {
    pub(crate) const fn at_most_six() -> [Self; NEIGHBORHOODS] {
        let mut neighborhoods = [Self::Zero([]); NEIGHBORHOODS];
        let compositions = StrongCompositionOfLengthAtMostThree::compositions_of_six();
        let mut n = 0;
        let mut i = 0;
        while i < compositions.len() {
            let composition = &compositions[i];
            let mut j = 0;
            while j < ARRAYS.len() {
                let array = &ARRAYS[j];
                use StrongCompositionOfLengthAtMostThree::*;
                let pair = match (composition, array) {
                    (Empty, OutArray::Zero([])) => Some(Self::Zero([])),
                    (One(x), OutArray::One([a])) => Some(Self::One([(*a, *x)])),
                    (Two(x, y), OutArray::Two([a, b])) => Some(Self::Two([(*a, *x), (*b, *y)])),
                    (Three(x, y, z), OutArray::Three([a, b, c])) => {
                        Some(Self::Three([(*a, *x), (*b, *y), (*c, *z)]))
                    }
                    _ => None,
                };
                if let Some(pair) = pair {
                    neighborhoods[n] = pair;
                    n += 1;
                }
                j += 1;
            }
            i += 1;
        }
        assert!(n == NEIGHBORHOODS);
        neighborhoods
    }

    pub(crate) const fn slice(&self) -> &[(usize, usize)] {
        use InArray::*;
        match self {
            Zero(slice) => slice,
            One(slice) => slice,
            Two(slice) => slice,
            Three(slice) => slice,
        }
    }

    pub const fn min(&self) -> Option<&usize> {
        use InArray::*;
        Some(match self {
            Zero(_) => return None,
            One(slice) => &slice[0].0,
            Two(slice) => &slice[0].0,
            Three(slice) => &slice[0].0,
        })
    }

    pub const fn max(&self) -> Option<&usize> {
        use InArray::*;
        Some(match self {
            Zero(_) => return None,
            One(slice) => &slice[0].0,
            Two(slice) => &slice[1].0,
            Three(slice) => &slice[2].0,
        })
    }

    pub const fn plus(&self, position: usize) -> Option<Self> {
        use InArray::*;
        Some(match *self {
            Zero([]) => Self::One([(position, 1)]),
            One([(a, mult)]) => {
                if position == a {
                    Self::One([(a, mult + 1)])
                } else if position == a + 1 || position == a + 2 {
                    Self::Two([(a, mult), (position, 1)])
                } else if a == position + 1 || a == position + 2 {
                    Self::Two([(position, 1), (a, mult)])
                } else {
                    return None;
                }
            }
            Two([(a, m_a), (b, m_b)]) => {
                if position == a {
                    Self::Two([(a, m_a + 1), (b, m_b)])
                } else if position == b {
                    Self::Two([(a, m_a), (b, m_b + 1)])
                } else if b == a + 1 && position == a + 2 {
                    Self::Three([(a, m_a), (b, m_b), (b + 1, 1)])
                } else if a == position + 1 && b == position + 2 {
                    Self::Three([(position, 1), (a, m_a), (b, m_b)])
                } else if position == a + 1 && b == a + 2 {
                    Self::Three([(a, m_a), (position, 1), (b, m_b)])
                } else {
                    return None;
                }
            }
            Three([(a, m_a), (b, m_b), (c, m_c)]) => {
                if position == a {
                    Self::Three([(a, m_a + 1), (b, m_b), (c, m_c)])
                } else if position == b {
                    Self::Three([(a, m_a), (b, m_b + 1), (c, m_c)])
                } else if position == c {
                    Self::Three([(a, m_a), (b, m_b), (c, m_c + 1)])
                } else {
                    return None;
                }
            }
            _ => return None,
        })
    }

    pub(super) const fn const_eq(&self, other: &Self) -> bool {
        use InArray::*;
        match (self, other) {
            (Zero([]), Zero([])) => true,
            (One(a), One(b)) => {
                let mut i = 0;
                while i < a.len() {
                    let a = a[i];
                    let b = b[i];
                    if a.0 != b.0 || a.1 != b.1 {
                        return false;
                    }
                    i += 1;
                }
                true
            }
            (Two(a), Two(b)) => {
                let mut i = 0;
                while i < a.len() {
                    let a = a[i];
                    let b = b[i];
                    if a.0 != b.0 || a.1 != b.1 {
                        return false;
                    }
                    i += 1;
                }
                true
            }
            (Three(a), Three(b)) => {
                let mut i = 0;
                while i < a.len() {
                    let a = a[i];
                    let b = b[i];
                    if a.0 != b.0 || a.1 != b.1 {
                        return false;
                    }
                    i += 1;
                }
                true
            }
            _ => false,
        }
    }
}

use std::fmt::Display;

use crate::map::out_neighborhood::out_array::ARRAYS;

use self::backend::StrongCompositionOfLengthAtMostThree;

use super::{InNeighborhood, NEIGHBORHOODS};

impl Display for InArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InArray::Zero([]) => write!(f, "[]"),
            InArray::One([a]) => write!(f, "{}", a.0.to_string().repeat(a.1)),
            InArray::Two([a, b]) => write!(
                f,
                "{}{}",
                a.0.to_string().repeat(a.1),
                b.0.to_string().repeat(b.1)
            ),
            InArray::Three([a, b, c]) => write!(
                f,
                "{}{}{}",
                a.0.to_string().repeat(a.1),
                b.0.to_string().repeat(b.1),
                c.0.to_string().repeat(c.1)
            ),
        }
    }
}
