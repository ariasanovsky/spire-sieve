use crate::map::{
    in_neighborhood::{in_array::InArray, in_byte::InByte, in_vec::InVec, InNeighborhood},
    out_neighborhood::{out_array::OutArray, out_byte::OutByte, out_vec::OutVec, OutNeighborhood},
};

use super::{row::Row, Map, Skeleton};

pub trait NotCanonical {}

impl NotCanonical for InByte {}
impl NotCanonical for InVec {}
impl NotCanonical for OutByte {}
impl NotCanonical for OutVec {}

pub type CanonicalRow = Row<InArray, OutArray>;
pub type CanonicalSkeleton<const PATHS: usize> = Skeleton<PATHS, InArray, OutArray>;
pub type CanonicalMap<const PATHS: usize> = Map<PATHS, InArray, OutArray>;

impl<const PATHS: usize, In, Out> From<Skeleton<PATHS, In, Out>> for CanonicalSkeleton<PATHS>
where
    In: for<'a> InNeighborhood<'a> + NotCanonical,
    Out: for<'a> OutNeighborhood<'a> + NotCanonical,
    Row<In, Out>: Into<CanonicalRow>,
{
    fn from(value: Skeleton<PATHS, In, Out>) -> Self {
        Self {
            rows: value.rows.map(Into::into),
        }
    }
}

impl<In, Out> From<Row<In, Out>> for CanonicalRow
where
    In: for<'a> InNeighborhood<'a> + NotCanonical + Into<InArray>,
    Out: for<'a> OutNeighborhood<'a> + NotCanonical + Into<OutArray>,
{
    fn from(value: Row<In, Out>) -> Self {
        Self {
            values: value.values.map(|(in_neighborhood, out_neighborhood)| {
                (in_neighborhood.into(), out_neighborhood.into())
            }),
        }
    }
}

impl<const PATHS: usize, In, Out> From<Map<PATHS, In, Out>> for CanonicalMap<PATHS>
where
    In: for<'a> InNeighborhood<'a> + NotCanonical + Into<InArray>,
    Out: for<'a> OutNeighborhood<'a> + NotCanonical + Into<OutArray>,
    Skeleton<PATHS, In, Out>: Into<CanonicalSkeleton<PATHS>>,
{
    fn from(value: Map<PATHS, In, Out>) -> Self {
        Self {
            skeleton: value.skeleton.into(),
            kinds: value.kinds,
        }
    }
}

#[cfg(test)]
mod test_canonized_maps {
    fn foo() {
        for i in 0..10 {}
    }
}
