#![allow(unused)]
mod byte_tables;
pub mod out_vec;

pub trait OutNeighborhood<'a, 'b>
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
