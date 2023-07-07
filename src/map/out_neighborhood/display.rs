use std::fmt::Display;

use super::{out_array::OutArray, out_byte::OutByte, out_vec::OutVec, OutNeighborhood};

impl Display for OutVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // {
        //     let mut values = self.values.clone();
        //     values.sort();
        //     values
        // }.iter().try_for_each(|v| write!(f, "{v}"))
        self.iter().try_for_each(|value| write!(f, "{value}"))
    }
}
impl Display for OutArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "{:?}",
        //     self.values()
        // )
        self.iter().try_for_each(|value| write!(f, "{value}"))
    }
}

impl Display for OutByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().try_for_each(|value| write!(f, "{value}"))
    }
}
