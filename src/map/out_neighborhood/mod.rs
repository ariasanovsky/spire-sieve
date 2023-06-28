#[derive(Debug, Default)]
pub struct OutVec {
    pub(crate) values: Vec<usize>,
}

pub trait OutNeighborhood<'a, 'b>
where
    'b: 'a,
{
    type Iter: Iterator<Item = &'b usize> + 'a;
    fn min(&'a self) -> Option<&'a usize>;
    fn max(&'a self) -> Option<&'a usize>;
    fn push(&mut self, value: usize);
    fn remove(&mut self, value: usize);
    fn iter(&'a self) -> Self::Iter;
}

impl<'a> OutNeighborhood<'a, 'a> for OutVec {
    type Iter = std::slice::Iter<'a, usize>;
    fn min(&'a self) -> Option<&'a usize> {
        self.values.iter().min()
    }

    fn max(&'a self) -> Option<&'a usize> {
        self.values.iter().max()
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
}
