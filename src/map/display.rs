use std::fmt::Display;

use super::{OutNeighborhood, Map};

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row, nodes) in self.0.iter().enumerate().rev() {
            write!(f, "\n{: <6}", "")?;
            for (position, (_, out_neighborhood, _)) in nodes.iter().enumerate() {
                let (mut right, mut middle, mut left) = (" ", " ", " ");
                for neighbor in &out_neighborhood.0 {
                    match neighbor.cmp(&position) {
                        std::cmp::Ordering::Less => left = r"\",
                        std::cmp::Ordering::Equal => middle = "|",
                        std::cmp::Ordering::Greater => right = "/",
                    }
                }
                write!(f, "{left}{middle}{right}")?;
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
        let position = self.1;
        let mut neighbors = self.0 .0.clone();
        neighbors.sort();
        neighbors.dedup();

        match &neighbors[..] {
            [] => write!(f, "   ")?,
            [one] => write!(
                f,
                "{} ",
                match position.cmp(one) {
                    std::cmp::Ordering::Less => r"  \",
                    std::cmp::Ordering::Equal => " | ",
                    std::cmp::Ordering::Greater => "/  ",
                }
            )?,
            [_, two] => write!(
                f,
                "{} ",
                match position.cmp(two) {
                    std::cmp::Ordering::Less => r" |\",
                    std::cmp::Ordering::Equal => r"\| ",
                    std::cmp::Ordering::Greater => " |/",
                }
            )?,
            _ => write!(f, "\\|/ ")?,
        }
        Ok(())
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
