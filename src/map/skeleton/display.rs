use std::fmt::Display;

use crate::map::{
    assign_nodes::NodeKind, display::EnumeratedOutNeighborhood, in_neighborhood::InNeighborhood,
    out_neighborhood::OutNeighborhood,
};

use super::Skeleton;

impl<const PATHS: usize, In, Out> Display for Skeleton<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = self.rows.iter().enumerate().rev();
        if let Some((row, nodes)) = rows.next() {
            write!(f, "\n{: <6}", row)?;
            for in_neighborhood in nodes.in_neighborhoods() {
                if in_neighborhood.is_empty() {
                    write!(f, "   ")?;
                } else {
                    write!(f, " {} ", NodeKind::Rest)?;
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
            for out_neighborhood in nodes.out_neighborhoods() {
                write!(
                    f,
                    " {} ",
                    if out_neighborhood.is_empty() {
                        NodeKind::Empty
                    } else {
                        NodeKind::Unassigned
                    }
                )?
            }
        }
        Ok(())
    }
}
