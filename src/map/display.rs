use std::fmt::Display;

use super::{Map, OutVec, in_neighborhood::InNeighborhood, NodeKind};

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = self.rows.iter().enumerate().rev();
        if let Some((row, nodes)) = rows.next() {
            write!(f, "\n{: <6}", row)?;
            for in_neighborhood in nodes.in_neighborhoods() {
                if in_neighborhood.is_empty() {
                    write!(f, "   ")?;
                } else {
                    write!(f, " {} ", NodeKind::Rest.to_char())?;
                }
            }
        }
        for (row, nodes) in rows {
            write!(f, "\n{: <6}", "")?;
            for (position, out_neighborhood) in nodes.out_neighborhoods().enumerate() {
                write!(
                    f,
                    "{}",
                    EnumeratedOutNeighborhood(out_neighborhood, position)
                )?;
            }
            write!(f, "\n{: <6}", row)?;
            for (_, out_neighborhood, kind) in &nodes.values {
                if out_neighborhood.values.is_empty() {
                    write!(f, "   ")?;
                } else {
                    write!(f, " {} ", match kind {
                        None => '*',
                        Some(kind) => kind.to_char(),
                    })?;
                }
            }
        }
        Ok(())
    }
}

struct EnumeratedOutNeighborhood<'a>(&'a OutVec, usize);

impl Display for EnumeratedOutNeighborhood<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut right, mut middle, mut left) = (" ", " ", " ");
        for neighbor in &self.0.values {
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
        let map = Map::generate(&mut rng, true);
        println!("{map}");
    }

    #[test]
    fn test_display_2() {
        let mut rng = Random::new(533907583096 + 1);
        let map = Map::generate(&mut rng, true);
        println!("{map}");
    }
}
