use super::InNeighborhood;

#[derive(Debug, Default, Clone)]
pub struct InVec {
    values: Vec<(usize, usize)>,
}

impl<'a> InNeighborhood<'a, 'a> for InVec {
    type Iter = std::slice::Iter<'a, (usize, usize)>;
    fn min(&'a self) -> Option<&'a usize> {
        self.values.iter().map(|(value, _)| value).min()
    }

    fn max(&'a self) -> Option<&'a usize> {
        self.values.iter().map(|(value, _)| value).min()
    }
    fn push(&mut self, value: usize) {
        self.values.push((value, 1));
    }
    fn iter(&'a self) -> Self::Iter {
        self.values.iter()
    }
}
