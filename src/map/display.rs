use std::fmt::Display;

use super::{InNeighborhood, Map};

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|(in_neighborhood, _, _)| in_neighborhood)
                    .enumerate()
                    .map(|(i, in_neighborhood)| EnumeratedInNeighborhood(in_neighborhood, i))
            })
            .enumerate()
            .rev()
            .map(|(i, row)| {
                write!(f, "{i}\t")?;
                row.map(|in_neighborhood| write!(f, "{in_neighborhood}"))
                    .collect::<Result<Vec<_>, _>>()?;
                write!(f, "\n\n")
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }
}

struct EnumeratedInNeighborhood<'a>(&'a InNeighborhood, usize);

impl Display for EnumeratedInNeighborhood<'_> {
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
