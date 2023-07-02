pub mod in_byte;
pub mod in_vec;

pub trait InNeighborhood<'a, 'b>
where
    'b: 'a,
{
    type Iter: Iterator<Item = &'b (usize, usize)> + 'a;
    fn min(&'a self) -> Option<&'a usize> {
        self.iter().map(|(a, _)| a).min()
    }
    fn max(&'a self) -> Option<&'a usize> {
        self.iter().map(|(a, _)| a).max()
    }
    fn push(&mut self, value: usize);
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
