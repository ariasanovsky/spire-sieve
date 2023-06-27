use std::fmt::Display;

use super::{Map, OutNeighborhood};

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row, nodes) in self.0.iter().enumerate().rev() {
            write!(f, "\n{: <6}", "")?;
            for (position, (_, out_neighborhood, _)) in nodes.iter().enumerate() {
                write!(f, "{}", EnumeratedOutNeighborhood(out_neighborhood, position))?;
            }
            write!(f, "\n{: <6}", row)?;
            for (_position, (_, out_neighborhood, _)) in nodes.iter().enumerate() {
                if out_neighborhood.0.is_empty() {
                    write!(f, "   ")?;
                } else {
                    write!(f, " * ")?;
                }
            }
        }
        Ok(())
    }
}

struct EnumeratedOutNeighborhood<'a>(&'a OutNeighborhood, usize);

impl Display for EnumeratedOutNeighborhood<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut right, mut middle, mut left) = (" ", " ", " ");
        for neighbor in &self.0.0 {
            match neighbor.cmp(&self.1) {
                std::cmp::Ordering::Less => left = r"\",
                std::cmp::Ordering::Equal => middle = "|",
                std::cmp::Ordering::Greater => right = "/",
            }
        }
        write!(f, "{left}{middle}{right}")
    }
}

#[cfg(test)]
mod display_tests {
    use crate::map::*;

    #[test]
    fn test_display() {
        let mut rng = Random::new(2);
        let map = Map::generate(&mut rng);
        println!("{map}");
    }

    #[test]
    fn test_display_2() {
        let mut rng = Random::new(533907583096 + 1);
        let map = Map::generate(&mut rng);
        println!("{map}");
    }
}
