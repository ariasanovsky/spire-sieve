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
    use crate::{
        map::{
            in_neighborhood::{in_array::InArray, in_byte::InByte, in_vec::InVec},
            out_neighborhood::{out_array::OutArray, out_byte::OutByte, out_vec::OutVec},
            skeleton::Skeleton,
            Act,
        },
        seed::Seed,
    };

    use super::CanonicalSkeleton;

    #[test]
    fn test_canonized_skeletons() {
        let seed: Seed = 1i64.into();
        let vec_vec: Skeleton<6, InVec, OutVec> = Skeleton::generate(&mut seed.map_rng(Act::One));
        // let vec_array: Skeleton<6, InVec, OutArray> = Skeleton::generate(&mut seed.map_rng(Act::One));
        let vec_byte: Skeleton<6, InVec, OutByte> = Skeleton::generate(&mut seed.map_rng(Act::One));
        // let array_vec: Skeleton<6, InArray, OutVec> = Skeleton::generate(&mut seed.map_rng(Act::One));
        let array_array: Skeleton<6, InArray, OutArray> =
            Skeleton::generate(&mut seed.map_rng(Act::One));
        // let array_byte: Skeleton<6, InArray, OutByte> = Skeleton::generate(&mut seed.map_rng(Act::One));
        let byte_vec: Skeleton<6, InByte, OutVec> = Skeleton::generate(&mut seed.map_rng(Act::One));
        // let byte_array: Skeleton<6, InByte, OutArray> = Skeleton::generate(&mut seed.map_rng(Act::One));
        let byte_byte: Skeleton<6, InByte, OutByte> =
            Skeleton::generate(&mut seed.map_rng(Act::One));

        let canonical_maps: &[CanonicalSkeleton<6>] = &[
            vec_vec.into(),
            // vec_array.into(),
            vec_byte.into(),
            // array_vec.into(),
            // array_array.into(),
            // array_byte.into(),
            byte_vec.into(),
            // byte_array.into(),
            byte_byte.into(),
        ];

        let canonical_map: String = array_array.to_string();
        for map in canonical_maps {
            let map: String = map.to_string();
            assert_eq!(map.to_string(), canonical_map, "{map}",);
        }
    }
}
