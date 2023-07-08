use libgdx_xs128::{rng::Random, RandomXS128};

use crate::map::LAST_POSITION;

use super::{in_neighborhood::InNeighborhood, out_neighborhood::OutNeighborhood, HEIGHT, WIDTH};

use super::row::Row;

#[cfg(feature = "std")]
pub mod display;

#[derive(Default)]
pub struct Skeleton<const PATHS: usize, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    pub(super) rows: [Row<In, Out>; HEIGHT],
}

impl<const PATHS: usize, In, Out> Skeleton<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a> + Default,
    Out: for<'a> OutNeighborhood<'a> + Default,
{
    pub fn generate(rng: &mut Random) -> Skeleton<PATHS, In, Out> {
        let mut skeleton = Skeleton::default();
        skeleton.create_paths(rng);
        skeleton
    }
}

impl<const PATHS: usize, In, Out> Skeleton<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    pub(crate) fn row(&self, row: usize) -> &Row<In, Out> {
        &self.rows[row]
    }

    fn row_mut(&mut self, row: usize) -> &mut Row<In, Out> {
        &mut self.rows[row]
    }

    fn _rows(&self) -> &[Row<In, Out>; HEIGHT] {
        &self.rows
    }
}

impl<const PATHS: usize, In, Out> Skeleton<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    fn create_paths(&mut self, rng: &mut Random) {
        let first_position = self.create_first_path(rng);
        self.create_second_path(rng, first_position);
        (2..6).for_each(|_| {
            self.create_path(rng);
        })
    }

    fn create_first_path(&mut self, rng: &mut Random) -> usize {
        let first_position = rng.next_capped_u64(WIDTH) as usize;
        let mut position = first_position;
        for row in 0..HEIGHT - 1 {
            let next_position = self.next_position(rng, row, position);
            self.add_edge(row, position, next_position);
            position = next_position;
        }
        first_position
    }

    fn add_edge(&mut self, row: usize, position: usize, next_position: usize) {
        // dbg!(row, position, next_position);
        let out_neighborhood = self.row_mut(row).out_neighborhood_mut(position);
        // println!("out: [{out_neighborhood}] + {next_position} =");
        out_neighborhood.push(next_position);
        // println!("\t[{out_neighborhood}]");

        let in_neighborhood = &mut self.row_mut(row + 1).in_neighborhood_mut(next_position);
        // println!("in: [{in_neighborhood}] + {position} =");
        in_neighborhood.push(position);
        // println!("\t[{in_neighborhood}]");
    }

    fn create_second_path(&mut self, rng: &mut Random, first_position: usize) {
        let mut position = rng.next_capped_u64(WIDTH) as usize;
        while position == first_position {
            position = rng.next_capped_u64(WIDTH) as usize;
        }

        for row in 0..HEIGHT - 1 {
            let next_position = self.next_position(rng, row, position);
            self.add_edge(row, position, next_position);
            position = next_position;
        }
    }

    fn create_path(&mut self, rng: &mut Random) {
        let mut position = rng.next_capped_u64(WIDTH) as usize;
        for row in 0..HEIGHT - 1 {
            let next_position = self.next_position(rng, row, position);
            self.add_edge(row, position, next_position);
            position = next_position;
        }
    }

    fn next_position(&self, rng: &mut Random, row: usize, position: usize) -> usize {
        let min_position = if position == 0 { 0 } else { position - 1 };
        let n_possible_positions = if position == 0 || position == LAST_POSITION {
            2
        } else {
            3
        };
        let adjustment = rng.next_capped_u64(n_possible_positions) as usize;
        let mut next_position = adjustment + min_position;
        next_position = self.cpplr(rng, row, position, next_position);
        next_position = self.cpanx(row, position, next_position);
        next_position
    }

    fn cpplr(
        &self,
        rng: &mut Random,
        row: usize,
        position: usize,
        mut next_position: usize,
    ) -> usize {
        debug_assert!([-1, 0, 1].contains(&(next_position as isize - position as isize)));

        let next_in_neighborhood = &self.row(row + 1).in_neighborhood(next_position);
        let rerolls = next_in_neighborhood
            .iter()
            .filter(|neighbor| !position.eq(&neighbor.0))
            .filter(|neighbor| !self.gca_skip(row, neighbor.0, position))
            .map(|neighbor| neighbor.1)
            .sum();
        for _ in 0..rerolls {
            next_position = match next_position.cmp(&position) {
                core::cmp::Ordering::Greater => {
                    next_position = position + rng.next_capped_u64(2) as usize;
                    next_position.max(1) - 1
                }
                core::cmp::Ordering::Equal => {
                    next_position = position + rng.next_capped_u64(3) as usize;
                    if next_position == 0 {
                        1
                    } else if next_position >= LAST_POSITION {
                        position - 1
                    } else {
                        next_position - 1
                    }
                }
                core::cmp::Ordering::Less => {
                    next_position = position + rng.next_capped_u64(2) as usize;
                    next_position.min(LAST_POSITION)
                }
            };

            debug_assert!([-1, 0, 1].contains(&(next_position as isize - position as isize)));
        }
        next_position
    }

    fn gca_skip(&self, row: usize, neighbor: usize, position: usize) -> bool {
        let (left_position, right_position) = if neighbor < row {
            (neighbor, position)
        } else {
            (position, neighbor)
        };

        let row = self.row(row);
        let left_in_neighborhood = row.in_neighborhood(left_position);
        let right_in_neighborhood = row.in_neighborhood(right_position);
        InNeighborhood::gca_skip(left_in_neighborhood, right_in_neighborhood)
    }

    fn cpanx(&self, row: usize, position: usize, mut next_position: usize) -> usize {
        if position != 0 {
            let sibling_position = position - 1;
            let out_neighborhood = self.row(row).out_neighborhood(sibling_position);
            out_neighborhood.update_position_from_left(&mut next_position);
        }

        if position != LAST_POSITION {
            let sibling_position = position + 1;
            let out_neighborhood = self.row(row).out_neighborhood(sibling_position);
            out_neighborhood.update_position_from_right(&mut next_position);
        }
        next_position
    }
}
