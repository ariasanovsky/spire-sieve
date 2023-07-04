use std::fmt::Display;

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

#[derive(Debug, Clone, Copy)]
pub struct Multiplicity(pub u8);

impl Multiplicity {
    const fn const_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NeighborhoodOfAtMostThreeConsecutiveElements {
    Empty,
    One(u8, Multiplicity),
    Two(u8, Multiplicity, Multiplicity),
    Three(u8, Multiplicity, Multiplicity, Multiplicity),
}

impl NeighborhoodOfAtMostThreeConsecutiveElements {
    pub const fn at_most_six() -> [Self; 233] {
        let mut neighborhoods = [Self::Empty; 233];
        let mut i = 1;
        let mut index = 0;
        let comps = StrongCompositionOfLengthAtMostThree::compositions_of_six();
        while index < comps.len() {
            let comp = &comps[index];
            let mut first_neighbor = 0;
            while first_neighbor < 7 {
                if let Some(neighborhood) = Self::new(first_neighbor, comp) {
                    if neighborhood.is_empty() {
                        break;
                    }
                    neighborhoods[i] = neighborhood;
                    assert!(!neighborhoods[i].is_empty() || i == 0);
                    i += 1;
                }
                first_neighbor += 1;
            }
            index += 1;
        }
        assert!(i == 233);

        let mut i = 0;
        while i < 233 {
            let neighborhood = neighborhoods[i];
            assert!(neighborhood.const_eq(&neighborhood));
            let mut j = 0;
            while j < i {
                assert!(!neighborhood.const_eq(&neighborhoods[j]));
                j += 1;
            }
            i += 1;
        }
        neighborhoods
    }

    const fn is_empty(&self) -> bool {
        use NeighborhoodOfAtMostThreeConsecutiveElements::*;
        matches!(self, Empty)
    }

    const fn const_eq(&self, other: &Self) -> bool {
        use NeighborhoodOfAtMostThreeConsecutiveElements::*;
        match (self, other) {
            (Empty, Empty) => true,
            (One(neighbor1, multiplicity1), One(neighbor2, multiplicity2)) => {
                *neighbor1 == *neighbor2 && multiplicity1.const_eq(multiplicity2)
            }
            (
                Two(neighbor1, multiplicity1, multiplicity2),
                Two(neighbor2, multiplicity3, multiplicity4),
            ) => {
                *neighbor1 == *neighbor2
                    && multiplicity1.const_eq(multiplicity3)
                    && multiplicity2.const_eq(multiplicity4)
            }
            (
                Three(neighbor1, multiplicity1, multiplicity2, multiplicity3),
                Three(neighbor2, multiplicity4, multiplicity5, multiplicity6),
            ) => {
                *neighbor1 == *neighbor2
                    && multiplicity1.const_eq(multiplicity4)
                    && multiplicity2.const_eq(multiplicity5)
                    && multiplicity3.const_eq(multiplicity6)
            }
            _ => false,
        }
    }

    const fn new(
        first_neighbor: u8,
        multiplicities: &StrongCompositionOfLengthAtMostThree,
    ) -> Option<Self> {
        use StrongCompositionOfLengthAtMostThree::*;
        Some(match multiplicities {
            Empty => Self::Empty,
            One(multiplicity) => Self::One(first_neighbor, Multiplicity(*multiplicity)),
            Two(multiplicity1, multiplicity2) => {
                if first_neighbor >= 6 {
                    return None;
                } else {
                    Self::Two(
                        first_neighbor,
                        Multiplicity(*multiplicity1),
                        Multiplicity(*multiplicity2),
                    )
                }
            }
            Three(multiplicity1, multiplicity2, multiplicity3) => {
                if first_neighbor >= 5 {
                    return None;
                } else {
                    Self::Three(
                        first_neighbor,
                        Multiplicity(*multiplicity1),
                        Multiplicity(*multiplicity2),
                        Multiplicity(*multiplicity3),
                    )
                }
            }
        })
    }

    const fn plus(&self, new_neighbor: u8) -> Option<Self> {
        use NeighborhoodOfAtMostThreeConsecutiveElements::*;
        Some(match self {
            Empty => One(new_neighbor, Multiplicity(1)),
            One(neighbor, Multiplicity(multiplicity)) => {
                if new_neighbor == *neighbor {
                    One(*neighbor, Multiplicity(*multiplicity + 1))
                } else if new_neighbor == *neighbor + 1 {
                    Two(*neighbor, Multiplicity(*multiplicity), Multiplicity(1))
                } else if *neighbor == new_neighbor + 1 {
                    Two(new_neighbor, Multiplicity(1), Multiplicity(*multiplicity))
                } else {
                    return None;
                }
            }
            Two(neighbor, Multiplicity(multiplicity1), Multiplicity(multiplicity2)) => {
                if new_neighbor == *neighbor {
                    Two(
                        *neighbor,
                        Multiplicity(*multiplicity1 + 1),
                        Multiplicity(*multiplicity2),
                    )
                } else if new_neighbor == *neighbor + 1 {
                    Two(
                        *neighbor,
                        Multiplicity(*multiplicity1),
                        Multiplicity(*multiplicity2 + 1),
                    )
                } else if new_neighbor == *neighbor + 2 {
                    Three(
                        *neighbor,
                        Multiplicity(*multiplicity1),
                        Multiplicity(*multiplicity2),
                        Multiplicity(1),
                    )
                } else if *neighbor == new_neighbor + 1 {
                    Three(
                        new_neighbor,
                        Multiplicity(1),
                        Multiplicity(*multiplicity1),
                        Multiplicity(*multiplicity2),
                    )
                } else {
                    return None;
                }
            }
            Three(
                neighbor,
                Multiplicity(multiplicity1),
                Multiplicity(multiplicity2),
                Multiplicity(multiplicity3),
            ) => {
                if new_neighbor == *neighbor {
                    Three(
                        *neighbor,
                        Multiplicity(*multiplicity1 + 1),
                        Multiplicity(*multiplicity2),
                        Multiplicity(*multiplicity3),
                    )
                } else if new_neighbor == *neighbor + 1 {
                    Three(
                        *neighbor,
                        Multiplicity(*multiplicity1),
                        Multiplicity(*multiplicity2 + 1),
                        Multiplicity(*multiplicity3),
                    )
                } else if new_neighbor == *neighbor + 2 {
                    Three(
                        *neighbor,
                        Multiplicity(*multiplicity1),
                        Multiplicity(*multiplicity2),
                        Multiplicity(*multiplicity3 + 1),
                    )
                } else {
                    return None;
                }
            }
        })
    }
}

#[derive(Debug)]
enum StrongCompositionOfLengthAtMostThree {
    Empty,
    One(u8),
    Two(u8, u8),
    Three(u8, u8, u8),
}

impl StrongCompositionOfLengthAtMostThree {
    const fn compositions_of_six() -> [Self; 42] {
        use StrongCompositionOfLengthAtMostThree::*;
        [
            // 0
            Empty,
            // 1
            One(1),
            // 2
            Two(1, 1),
            One(2),
            // 3
            Three(1, 1, 1),
            Two(1, 2),
            Two(2, 1),
            One(3),
            // 4
            Three(1, 1, 2),
            Three(1, 2, 1),
            Three(2, 1, 1),
            Two(2, 2),
            Two(1, 3),
            Two(3, 1),
            One(4),
            // 5
            Three(1, 2, 2),
            Three(2, 1, 2),
            Three(2, 2, 1),
            Three(1, 1, 3),
            Three(1, 3, 1),
            Three(3, 1, 1),
            Two(2, 3),
            Two(3, 2),
            Two(1, 4),
            Two(4, 1),
            One(5),
            // 6
            Three(2, 2, 2),
            Three(1, 2, 3),
            Three(1, 3, 2),
            Three(2, 1, 3),
            Three(2, 3, 1),
            Three(3, 1, 2),
            Three(3, 2, 1),
            Three(1, 1, 4),
            Three(1, 4, 1),
            Three(4, 1, 1),
            Two(3, 3),
            Two(2, 4),
            Two(4, 2),
            Two(1, 5),
            Two(5, 1),
            One(6),
        ]
    }
}

impl Display for NeighborhoodOfAtMostThreeConsecutiveElements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeighborhoodOfAtMostThreeConsecutiveElements::Empty => Ok(()),
            NeighborhoodOfAtMostThreeConsecutiveElements::One(first, mult) => {
                write!(f, "{}", first.to_string().repeat(mult.0 as usize))
            }
            NeighborhoodOfAtMostThreeConsecutiveElements::Two(first, mult1, mult2) => {
                write!(
                    f,
                    "{}{}",
                    first.to_string().repeat(mult1.0 as usize),
                    (first + 1).to_string().repeat(mult2.0 as usize)
                )
            }
            NeighborhoodOfAtMostThreeConsecutiveElements::Three(first, mult1, mult2, mult3) => {
                write!(
                    f,
                    "{}{}{}",
                    first.to_string().repeat(mult1.0 as usize),
                    (first + 1).to_string().repeat(mult2.0 as usize),
                    (first + 2).to_string().repeat(mult3.0 as usize)
                )
            }
        }
    }
}

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

#[cfg(test)]
mod test_in_bytes_backend {
    use crate::map::in_neighborhood::in_byte::backend::NeighborhoodArray;

    use super::NeighborhoodOfAtMostThreeConsecutiveElements;

    #[test]
    fn test_neighborhoods() {
        let neighborhoods = NeighborhoodOfAtMostThreeConsecutiveElements::at_most_six();
        for (index, neighborhood) in neighborhoods.iter().enumerate() {
            println!("{index}: {neighborhood}");
        }
    }

    #[test]
    fn test_array_plus() {
        const ARRAYS: [NeighborhoodArray; 233] = NeighborhoodArray::at_most_six();
        for (i, array) in ARRAYS.iter().enumerate() {
            for j in 0..7 {
                if let Some(sum) = array.plus(j) {
                    println!("{i}:\t{array} + {j} = {sum}");
                } else {
                    println!("{i}:\t{array} + {j} = None");
                }
            }
            if i == 10 {
                break;
            }
        }
    }
}
