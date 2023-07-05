use super::OutNeighborhood;

pub struct OutArray;

impl<'a> OutNeighborhood<'a> for OutArray {
    type Iter = std::slice::Iter<'a, usize>;

    fn update_position_from_left(&self, _value: &mut usize) {
        todo!()
    }

    fn update_position_from_right(&self, _value: &mut usize) {
        todo!()
    }

    fn push(&mut self, _value: usize) {
        todo!()
    }

    fn remove(&mut self, _value: usize) {
        todo!()
    }

    fn iter(&self) -> Self::Iter {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}
