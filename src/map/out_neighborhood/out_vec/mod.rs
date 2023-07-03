use super::OutNeighborhood;

#[derive(Debug, Default, Clone)]
pub struct OutVec {
    pub values: Vec<usize>,
}

impl<'a> OutNeighborhood<'a, 'a> for OutVec {
    type Iter = std::slice::Iter<'a, usize>;

    fn update_position_from_left(&'a self, value: &'a mut usize) {
        if let Some(max) = self.values.iter().max().copied() {
            *value = max.max(*value)
        }
    }

    fn update_position_from_right(&'a self, value: &'a mut usize) {
        if let Some(min) = self.values.iter().min().copied() {
            *value = min.min(*value)
        }
    }

    fn push(&mut self, value: usize) {
        self.values.push(value);
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
