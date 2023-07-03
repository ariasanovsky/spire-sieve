use self::backend::NeighborhoodOfAtMostThreeConsecutiveElements;

use super::InNeighborhood;

mod backend;

#[derive(Debug, Clone, Copy)]
struct InByte(u8);

impl Default for InByte {
    fn default() -> Self {
        Self(0)
    }
}

impl<'a> InNeighborhood<'a, 'a> for InByte {
    type Iter = std::slice::Iter<'a, (usize, usize)>;

    fn min(&'a self) -> Option<&'a usize> {
        todo!()
    }

    fn max(&'a self) -> Option<&'a usize> {
        todo!()
    }

    fn push(&mut self, value: usize) {
        todo!()
    }

    fn iter(&'a self) -> Self::Iter {
        todo!()
    }
}
