pub mod in_array;
pub mod in_byte;
pub mod in_vec;

pub trait InNeighborhood<'a>:
{
    type Iter: Iterator<Item = &'a (usize, usize)>;
    fn min(&'a self) -> Option<&'a usize> {
        self.iter().map(|(value, _)| value).min()
    }
    fn max(&'a self) -> Option<&'a usize> {
        self.iter().map(|(value, _)| value).max()
    }
    fn push(&'a mut self, value: usize);
    fn iter(&'a self) -> Self::Iter;
    fn gca_skip(left: &'a Self, right: &'a Self) -> bool {
        match (left.max(), right.min()) {
            (Some(left_max), Some(right_min)) => left_max != right_min,
            _ => true,
        }
    }
    fn is_empty(&'a self) -> bool {
        self.iter().next().is_none()
    }
}

// pub trait TestInNeighborhood<'a, 'b>:
//     InNeighborhood<'a, 'b>
//     + From<NeighborhoodArray> + TryInto<NeighborhoodArray>
//     + Debug + Default
// where
//     'b: 'a,
//     <Self as TryInto<NeighborhoodArray>>::Error: Debug,
//     <Self as InNeighborhood<'a, 'b>>::Iter: Iterator<Item = &'b (usize, usize)> + 'a,
    
// {
//     fn test_bijection() {
//         for array in NeighborhoodArray::at_most_six() {
//             let in_neighborhood: Self = Self::from(array);
//             let new_array: NeighborhoodArray = in_neighborhood.try_into().unwrap();
//             assert_eq!(array, new_array);
//         }    fn test_bijection() {
    //     for array in NeighborhoodArray::at_most_six() {
    //         let in_neighborhood: Self = Self::from(array);
    //         let new_array: NeighborhoodArray = in_neighborhood.try_into().unwrap();
    //         assert_eq!(array, new_array);
    //     }
    // }

    // fn test_min() {
    //     for array in NeighborhoodArray::at_most_six() {
    //         let in_neighborhood = Self::from(array);
    //         let array_min = array.min();
    //         let in_neighborhood_min = in_neighborhood.min();
    //         assert_eq!(in_neighborhood.min().copied(), array.min().copied());
    //     }
    // }
//     }

//     fn test_min() {
//         for array in NeighborhoodArray::at_most_six() {
//             let in_neighborhood = Self::from(array);
//             match (in_neighborhood.min().copied(), array.min()) {
//                 _ => todo!()
//             }
//             // let array_min = array.min();
//             // let in_neighborhood_min = in_neighborhood.min();
//             // assert_eq!(in_neighborhood.min().copied(), array.min().copied());
//         }
//     }
    
//     // #[test]
//     // fn test_invec_min() {
//     //     for array in ARRAYS.iter() {
//     //         let invec = InVec::from(*array);
//     //         assert_eq!(invec.min(), array.min());
//     //     }
//     // }

//     // #[test]
//     // fn test_invec_max() {
//     //     for array in ARRAYS.iter() {
//     //         let invec = InVec::from(*array);
//     //         assert_eq!(invec.max(), array.max());
//     //     }
//     // }

//     // #[test]
//     // fn test_push() {
//     //     for array in ARRAYS.iter() {
//     //         for position in 0..7 {
//     //             let array_sum: Option<NeighborhoodArray> = array.plus(position);
//     //             let mut invec = InVec::from(*array);
//     //             invec.push(position);
//     //             let vec_sum: Option<NeighborhoodArray> = invec.try_into().ok();
//     //             assert_eq!(vec_sum, array_sum);
//     //         }
//     //     }
//     // }

//     // #[test]
//     // fn test_iter() {
//     //     for array in ARRAYS.iter() {
//     //         let invec = InVec::from(*array);
//     //         let vec: Vec<_> = invec.iter().collect();
//     //         assert_eq!(
//     //             vec,
//     //             array.slice().iter().collect::<Vec<_>>()
//     //         )
//     //     }
//     // }





// }