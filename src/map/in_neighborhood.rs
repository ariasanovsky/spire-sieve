pub trait InNeighborhood: IntoIterator<Item = usize> {
    fn min(&self) -> Option<usize>;
    fn max(&self) -> Option<usize>;
    fn push(&mut self, value: usize);
}

impl Iterator for InVec {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct InVec(Vec<usize>);
impl InNeighborhood for InVec {
    fn min(&self) -> Option<usize> {
        self.0.iter().min().copied()
    }
    fn max(&self) -> Option<usize> {
        self.0.iter().max().copied()
    }
    fn push(&mut self, value: usize) {
        self.0.push(value)
    }
}
