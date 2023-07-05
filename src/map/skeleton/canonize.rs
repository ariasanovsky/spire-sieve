use crate::map::{
    in_neighborhood::{in_array::InArray, in_byte::InByte, in_vec::InVec, InNeighborhood},
    out_neighborhood::{out_array::OutArray, out_byte::OutByte, out_vec::OutVec, OutNeighborhood},
};

use super::Skeleton;

// impl<const PATHS: usize, In, Out> Skeleton<PATHS, In, Out>
// where
//     In: for<'a> InNeighborhood<'a> + Into<InArray>,
//     Out: for<'a> OutNeighborhood<'a> + Into<OutArray>,
// {
//     pub fn _canonize(&self) -> Skeleton<PATHS, InArray, OutArray> {
//         let new_rows = array::from_fn(|row| {
//             let row = &self.rows[row];
//             let new_row = array::from_fn(|position| {
//                 let (_in_neighborhood, _out_neighborhood) = &row.values[position];
//                 // (in_neighborhood.into(), out_neighborhood.into())
//                 todo!()
//             });
//             Row { values: new_row }
//         });
//         Skeleton {
//             rows: new_rows,
//         }
//     }
// }

pub trait NotCanonical {}

impl NotCanonical for InByte {}
impl NotCanonical for InVec {}
impl NotCanonical for OutByte {}
impl NotCanonical for OutVec {}

type CanonicalSkeleton<const PATHS: usize> = Skeleton<PATHS, InArray, OutArray>;

impl<const PATHS: usize, In, Out> From<Skeleton<PATHS, In, Out>> for CanonicalSkeleton<PATHS>
where
    In: for<'a> InNeighborhood<'a> + Into<InArray> + NotCanonical,
    Out: for<'a> OutNeighborhood<'a> + Into<OutArray> + NotCanonical,
{
    fn from(_value: Skeleton<PATHS, In, Out>) -> Self {
        todo!()
    }
}
