mod backend;

use backend::NeighborhoodOfAtMostThreeConsecutiveElements;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeighborhoodArray {
    Zero([(usize, usize); 0]),
    One([(usize, usize); 1]),
    Two([(usize, usize); 2]),
    Three([(usize, usize); 3]),
}

impl NeighborhoodArray {
    pub(crate) const fn at_most_six() -> [Self; 233] {
        let mut neighborhoods = [Self::Zero([]); 233];
        let intervals = NeighborhoodOfAtMostThreeConsecutiveElements::at_most_six();
        let mut i = 0;
        while i < neighborhoods.len() {
            neighborhoods[i] = Self::new(intervals[i]);
            i += 1;
        }
        neighborhoods
    }

    const fn new(neighborhood: NeighborhoodOfAtMostThreeConsecutiveElements) -> Self {
        use NeighborhoodOfAtMostThreeConsecutiveElements::*;
        match neighborhood {
            Empty => Self::Zero([]),
            One(neighbor, multiplicity) => {
                Self::One([(neighbor as usize, multiplicity.0 as usize)])
            }
            Two(neighbor1, multiplicity1, multiplicity2) => Self::Two([
                (neighbor1 as usize, multiplicity1.0 as usize),
                (neighbor1 as usize + 1, multiplicity2.0 as usize),
            ]),
            Three(neighbor1, multiplicity1, multiplicity2, multiplicity3) => Self::Three([
                (neighbor1 as usize, multiplicity1.0 as usize),
                (neighbor1 as usize + 1, multiplicity2.0 as usize),
                (neighbor1 as usize + 2, multiplicity3.0 as usize),
            ]),
        }
    }

    pub(crate) const fn slice(&self) -> &[(usize, usize)] {
        use NeighborhoodArray::*;
        match self {
            Zero(slice) => slice,
            One(slice) => slice,
            Two(slice) => slice,
            Three(slice) => slice,
        }
    }

    pub const fn min(&self) -> Option<&usize> {
        use NeighborhoodArray::*;
        Some(match self {
            Zero(_) => return None,
            One(slice) => &slice[0].0,
            Two(slice) => &slice[0].0,
            Three(slice) => &slice[0].0,
        })
    }

    pub const fn max(&self) -> Option<&usize> {
        use NeighborhoodArray::*;
        Some(match self {
            Zero(_) => return None,
            One(slice) => &slice[0].0,
            Two(slice) => &slice[1].0,
            Three(slice) => &slice[2].0,
        })
    }

    pub const fn plus(&self, position: usize) -> Option<Self> {
        use NeighborhoodArray::*;
        Some(match *self {
            Zero([]) => Self::One([(position, 1)]),
            One([(a, mult)]) => {
                if position == a {
                    Self::One([(a, mult + 1)])
                } else if position == a + 1 {
                    Self::Two([(a, mult), (position, 1)])
                } else if a == position + 1 {
                    Self::Two([(position, 1), (a, mult)])
                } else {
                    return None;
                }
            }
            Two([(a, m_a), (b, m_b)]) if b == a + 1 => {
                if position == a {
                    Self::Two([(a, m_a + 1), (b, m_b)])
                } else if position == b {
                    Self::Two([(a, m_a), (b, m_b + 1)])
                } else if position == b + 1 {
                    Self::Three([(a, m_a), (b, m_b), (b + 1, 1)])
                } else if a == position + 1 {
                    Self::Three([(position, 1), (a, m_a), (b, m_b)])
                } else {
                    return None;
                }
            }
            Three([(a, m_a), (b, m_b), (c, m_c)]) if b == a + 1 && c == b + 1 => {
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
        use NeighborhoodArray::*;
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

impl Display for NeighborhoodArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeighborhoodArray::Zero([]) => write!(f, "[]"),
            NeighborhoodArray::One([a]) => write!(f, "{}", a.0.to_string().repeat(a.1)),
            NeighborhoodArray::Two([a, b]) => write!(
                f,
                "{}{}",
                a.0.to_string().repeat(a.1),
                b.0.to_string().repeat(b.1)
            ),
            NeighborhoodArray::Three([a, b, c]) => write!(
                f,
                "{}{}{}",
                a.0.to_string().repeat(a.1),
                b.0.to_string().repeat(b.1),
                c.0.to_string().repeat(c.1)
            ),
        }
    }
}
