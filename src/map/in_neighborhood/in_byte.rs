

#[derive(Debug, Clone, Copy)]
struct Multiplicity(u8);

#[derive(Debug, Clone, Copy)]
enum NeighborhoodOfAtMostThreeConsecutiveElements {
    Empty,
    One(u8, Multiplicity),
    Two(u8, Multiplicity, Multiplicity),
    Three(u8, Multiplicity, Multiplicity, Multiplicity),
}

impl NeighborhoodOfAtMostThreeConsecutiveElements {
    fn neighborhoods_at_most_six() -> [Self; 233] {
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
                    i += 1;
                }
                first_neighbor += 1;
            }
            index += 1;
        }
        neighborhoods
    }

    const fn is_empty(&self) -> bool {
        use NeighborhoodOfAtMostThreeConsecutiveElements::*;
        match self {
            Empty => true,
            _ => false,
        }
    }
    
    const fn new(first_neighbor: u8, multiplicities: &StrongCompositionOfLengthAtMostThree) -> Option<Self> {
        use StrongCompositionOfLengthAtMostThree::*;
        Some(match multiplicities {
            Empty => Self::Empty,
            One(multiplicity) => Self::One(first_neighbor, Multiplicity(*multiplicity)),
            Two(multiplicity1, multiplicity2) => {
                if first_neighbor >= 6 {
                    return None;
                } else {
                    Self::Two(first_neighbor, Multiplicity(*multiplicity1), Multiplicity(*multiplicity2))
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
                    return None
                }
            },
            Two(neighbor, Multiplicity(multiplicity1), Multiplicity(multiplicity2)) => {
                if new_neighbor == *neighbor {
                    Two(*neighbor, Multiplicity(*multiplicity1 + 1), Multiplicity(*multiplicity2))
                } else if new_neighbor == *neighbor + 1 {
                    Two(*neighbor, Multiplicity(*multiplicity1), Multiplicity(*multiplicity2 + 1))
                } else if new_neighbor == *neighbor + 2 {
                    Three(*neighbor, Multiplicity(*multiplicity1), Multiplicity(*multiplicity2), Multiplicity(1))
                } else if *neighbor == new_neighbor + 1 {
                    Three(new_neighbor, Multiplicity(1), Multiplicity(*multiplicity1), Multiplicity(*multiplicity2))
                } else {
                    return None
                }
            },
            Three(neighbor, Multiplicity(multiplicity1), Multiplicity(multiplicity2), Multiplicity(multiplicity3)) => {
                if new_neighbor == *neighbor {
                    Three(*neighbor, Multiplicity(*multiplicity1 + 1), Multiplicity(*multiplicity2), Multiplicity(*multiplicity3))
                } else if new_neighbor == *neighbor + 1 {
                    Three(*neighbor, Multiplicity(*multiplicity1), Multiplicity(*multiplicity2 + 1), Multiplicity(*multiplicity3))
                } else if new_neighbor == *neighbor + 2 {
                    Three(*neighbor, Multiplicity(*multiplicity1), Multiplicity(*multiplicity2), Multiplicity(*multiplicity3 + 1))
                } else {
                    return None
                }
            },
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

#[cfg(test)]
mod test_in_bytes {
    use super::NeighborhoodOfAtMostThreeConsecutiveElements;

    #[test]
    fn test_neighborhoods() {
        let neighborhoods = NeighborhoodOfAtMostThreeConsecutiveElements::neighborhoods_at_most_six();
        for (index, neighborhood) in neighborhoods.iter().enumerate() {
            println!("{index}: {neighborhood:?}");
        }
    }
}