use super::InNeighborhood;

#[derive(Debug, Default, Clone)]
pub struct InVec {
    values: Vec<(usize, usize)>,
}

impl<'a> InNeighborhood<'a, 'a> for InVec {
    type Iter = std::slice::Iter<'a, (usize, usize)>;
    fn push(&mut self, value: usize) {
        self.values.push((value, 1));
    }
    fn iter(&'a self) -> Self::Iter {
        self.values.iter()
    }
}
