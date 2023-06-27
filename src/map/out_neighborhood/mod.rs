#[derive(Debug, Default)]
pub struct OutVec {
    pub(crate) values: Vec<usize>,
}

pub trait OutNeighborhood<'a, 'b>
where
    'b: 'a,
{
    fn min(&'a self) -> Option<&'a usize>;
    fn max(&'a self) -> Option<&'a usize>;
    fn push(&mut self, value: usize);
}

impl<'a> OutNeighborhood<'a, 'a> for OutVec {
    fn min(&'a self) -> Option<&'a usize> {
        self.values.iter().min()
    }

    fn max(&'a self) -> Option<&'a usize> {
        self.values.iter().max()
    }

    fn push(&mut self, value: usize) {
        self.values.push(value);
    }
}