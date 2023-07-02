#![allow(unused)]
mod out_byte;
pub mod out_vec;

pub trait OutNeighborhood<'a, 'b>: Default
where
    'b: 'a,
{
    type Iter: Iterator<Item = &'b usize> + 'a;
    fn update_position_from_left(&'a self, value: &'a mut usize);
    fn update_position_from_right(&'a self, value: &'a mut usize);
    fn push(&mut self, value: usize);
    fn remove(&mut self, value: usize);
    fn iter(&'a self) -> Self::Iter;
    fn is_empty(&self) -> bool;
}
