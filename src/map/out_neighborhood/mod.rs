#[cfg(feature = "std")]
pub mod display;
pub mod out_array;
pub mod out_byte;
#[cfg(feature = "std")]
pub mod out_vec;

pub trait OutNeighborhood<'a> {
    type Iter: Iterator<Item = &'a usize>;
    fn update_position_from_left(&self, value: &mut usize);
    fn update_position_from_right(&self, value: &mut usize);
    fn push(&mut self, value: usize);
    fn remove(&mut self, value: usize);
    fn iter(&'a self) -> Self::Iter;
    fn is_empty(&self) -> bool;
}
