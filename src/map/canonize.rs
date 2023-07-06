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
    use crate::{seed::Seed, map::{skeleton::Skeleton, Act, in_neighborhood::{in_vec::InVec, in_array::InArray, in_byte::InByte}, out_neighborhood::{out_vec::OutVec, out_array::OutArray, out_byte::OutByte}}};

    use super::CanonicalSkeleton;

    #[test]
    fn test_canonized_skeletons() {
        let seed: Seed = 1i64.into();
        let vec_vec_skeleton: Skeleton<6, InVec, OutVec> = Skeleton::generate(&mut seed.map_rng(Act::One));
        let vec_array_skeleton: Skeleton<6, InVec, OutArray> = Skeleton::generate(&mut seed.map_rng(Act::One));
        assert_eq!(
            vec_vec_skeleton.to_string(),
            vec_array_skeleton.to_string()
        );
        
        let array_vec_skeleton: Skeleton<6, InArray, OutVec> = Skeleton::generate(&mut seed.map_rng(Act::One));
        assert_eq!(
            vec_vec_skeleton.to_string(),
            array_vec_skeleton.to_string()
        );

        let array_array_skeleton: Skeleton<6, InArray, OutArray> = Skeleton::generate(&mut seed.map_rng(Act::One));
        assert_eq!(
            vec_vec_skeleton.to_string(),
            array_array_skeleton.to_string()
        );

        let vec_byte_skeleton: Skeleton<6, InVec, OutByte> = Skeleton::generate(&mut seed.map_rng(Act::One));
        assert_eq!(
            vec_vec_skeleton.to_string(),
            vec_byte_skeleton.to_string()
        );

        let byte_vec_skeleton: Skeleton<6, InByte, OutVec> = Skeleton::generate(&mut seed.map_rng(Act::One));
        assert_eq!(
            vec_vec_skeleton.to_string(),
            byte_vec_skeleton.to_string()
        );

        let byte_array_skeleton: Skeleton<6, InByte, OutArray> = Skeleton::generate(&mut seed.map_rng(Act::One));
        assert_eq!(
            vec_vec_skeleton.to_string(),
            byte_array_skeleton.to_string()
        );

        let byte_byte_skeleton: Skeleton<6, InByte, OutByte> = Skeleton::generate(&mut seed.map_rng(Act::One));
        assert_eq!(
            vec_vec_skeleton.to_string(),
            byte_byte_skeleton.to_string()
        );


        // println!("{vec_vec_skeleton}");
        // let byte_byte_skeleton: Skeleton<6, InByte, OutByte> = Skeleton::generate(&mut seed.map_rng(Act::One));

        // let canonical_maps: [CanonicalSkeleton<6>; 3] = [
        //     vec_vec_skeleton.into(),
        //     array_array_skeleton,
        //     byte_byte_skeleton.into(),
        // ];
    }
}
