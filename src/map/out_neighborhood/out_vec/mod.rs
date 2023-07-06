use std::fmt::Display;

use super::{OutNeighborhood, out_array::OutArray};

#[derive(Debug, Default, Clone)]
pub struct OutVec {
    pub values: Vec<usize>,
}

impl From<OutVec> for OutArray {
    fn from(value: OutVec) -> Self {
        let mut values = value.values;
        values.sort();
        match values[..] {
            [] => Self::Zero([]),
            [a] => Self::One([a]),
            [a, b] => Self::Two([a, b]),
            [a, b, c] => Self::Three([a, b, c]),
            _ => unreachable!(),
        }
    }
}

impl Display for OutVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            let mut values = self.values.clone();
            values.sort();
            values
        }.iter().try_for_each(|v| write!(f, "{v}"))
    }
}

impl<'a> OutNeighborhood<'a> for OutVec {
    type Iter = std::slice::Iter<'a, usize>;

    fn update_position_from_left(&self, value: &mut usize) {
        if let Some(max) = self.values.iter().max().copied() {
            *value = max.max(*value)
        }
    }

    fn update_position_from_right(&self, value: &mut usize) {
        if let Some(min) = self.values.iter().min().copied() {
            *value = min.min(*value)
        }
    }

    fn push(&mut self, value: usize) {
        // self.values.push(value);
        if !self.values.contains(&value) {
            self.values.push(value);
        }
    }

    fn remove(&mut self, value: usize) {
        self.values.retain(|&v| v != value);
    }

    fn iter(&'a self) -> Self::Iter {
        self.values.iter()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
