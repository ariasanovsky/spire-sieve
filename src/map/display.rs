use std::fmt::Display;

use super::{in_neighborhood::InNeighborhood, out_neighborhood::OutNeighborhood, Map, NodeKind, Act};

impl Display for Act {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", match self {
            Act::One => "Act I",
            Act::Two => "Act II",
            Act::Three => "Act III",
})
    }
}

impl<const PATHS: usize, In, Out> Display for Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = self.skeleton.rows.iter().zip(self.kinds).enumerate().rev();
        if let Some((row, (nodes, _))) = rows.next() {
            write!(f, "\n{: <6}", row)?;
            for in_neighborhood in nodes.in_neighborhoods() {
                if in_neighborhood.is_empty() {
                    write!(f, "   ")?;
                } else {
                    write!(f, " {} ", NodeKind::Rest)?;
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
                    write!(f, " {kind} ",)?;
                }
            }
        }
        Ok(())
    }
}

pub(crate) struct EnumeratedOutNeighborhood<'a, Out: OutNeighborhood<'a>>(pub &'a Out, pub usize);

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

#[cfg(feature = "std")]
#[cfg(test)]
mod display_tests {
    use std::println;

    use libgdx_xs128::RandomXS128;

    use crate::map::{*, in_neighborhood::in_vec::InVec, out_neighborhood::out_vec::OutVec};

    #[test]
    fn test_display() {
        let mut rng = Random::new(2);
        let map = Map::<6, InVec, OutVec>::generate(&mut rng, true);
        println!("{map}");
    }

    #[test]
    fn test_display_2() {
        let mut rng = Random::new(533907583096 + 1);
        let map = Map::<6, InVec, OutVec>::generate(&mut rng, true);
        println!("{map}");
    }
}
