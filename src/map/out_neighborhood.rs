pub trait OutNeighborhood {
    fn min(&self) -> Option<usize>;
    fn max(&self) -> Option<usize>;
    fn push(&mut self, value: usize);
}

pub struct OutVec(Vec<usize>);
impl OutNeighborhood for OutVec {
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
