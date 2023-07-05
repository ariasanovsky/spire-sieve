use std::fmt::Display;

use super::{in_neighborhood::InNeighborhood, out_neighborhood::OutNeighborhood, Map, NodeKind};

impl<const PATHS: usize, In, Out> Display for Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = self.rows.iter().zip(self.kinds).enumerate().rev();
        if let Some((row, (nodes, kinds))) = rows.next() {
            write!(f, "\n{: <6}", row)?;
            for in_neighborhood in nodes.in_neighborhoods() {
                if in_neighborhood.is_empty() {
                    write!(f, "   ")?;
                } else {
                    write!(f, " {} ", NodeKind::Rest.char())?;
                }
            }
        }
        for (row, (nodes, kinds)) in rows {
            write!(f, "\n{: <6}", "")?;
            for (position, out_neighborhood) in nodes.out_neighborhoods().enumerate() {
                write!(
                    f,
                    "{}",
                    EnumeratedOutNeighborhood(out_neighborhood, position)
                )?;
            }
            write!(f, "\n{: <6}", row)?;
            for (out_neighborhood, kind) in nodes.out_neighborhoods().zip(kinds) {
                if out_neighborhood.is_empty() {
                    write!(f, "   ")?;
                } else {
                    write!(
                        f,
                        " {} ",
                        kind.char()
                    )?;
                }
            }
        }
        Ok(())
    }
}

struct EnumeratedOutNeighborhood<'a, Out: OutNeighborhood<'a>>(&'a Out, usize);

impl<Out> Display for EnumeratedOutNeighborhood<'_, Out>
where
    Out: for<'a> OutNeighborhood<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut right, mut middle, mut left) = (" ", " ", " ");
        for neighbor in self.0.iter() {
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
    use libgdx_xs128::RandomXS128;

    use crate::map::*;

    #[test]
    fn test_display() {
        let mut rng = Random::new(2);
        let map = _DefaultMap::generate(&mut rng, true);
        println!("{map}");
    }

    #[test]
    fn test_display_2() {
        let mut rng = Random::new(533907583096 + 1);
        let map = _DefaultMap::generate(&mut rng, true);
        println!("{map}");
    }
}
