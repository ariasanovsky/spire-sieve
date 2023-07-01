use libgdx_xs128::{rng::Random, RandomXS128};

use crate::map::LAST_POSITION;

use super::{
    in_neighborhood::InNeighborhood, out_neighborhood::OutNeighborhood, Map, HEIGHT, PATHS, WIDTH,
};

impl Map {
    pub(super) fn create_paths(&mut self, rng: &mut Random) {
        let first_position = self.create_first_path(rng);
        self.create_second_path(rng, first_position);
        (2..PATHS).for_each(|_| {
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
        assert!([-1, 0, 1].contains(&(next_position as isize - position as isize)));

        let next_in_neighborhood = &self.row(row + 1).in_neighborhood(next_position);
        let rerolls = next_in_neighborhood
            .iter()
            .filter(|neighbor| !position.eq(&neighbor.0))
            .filter(|&&neighbor| !self.gca_skip(row, neighbor.0, position))
            .map(|neighbor| neighbor.1)
            .sum();
        for _ in 0..rerolls {
            next_position = match next_position.cmp(&position) {
                std::cmp::Ordering::Greater => {
                    next_position = position + rng.next_capped_u64(2) as usize;
                    next_position.max(1) - 1
                }
                std::cmp::Ordering::Equal => {
                    next_position = position + rng.next_capped_u64(3) as usize;
                    if next_position == 0 {
                        1
                    } else if next_position >= LAST_POSITION {
                        position - 1
                    } else {
                        next_position - 1
                    }
                }
                std::cmp::Ordering::Less => {
                    next_position = position + rng.next_capped_u64(2) as usize;
                    next_position.min(LAST_POSITION)
                }
            };

            assert!([-1, 0, 1].contains(&(next_position as isize - position as isize)));
        }
        next_position
    }

    fn gca_skip(&self, row: usize, position: usize, neighbor: usize) -> bool {
        let (left_position, right_position) = if position < row {
            (position, neighbor)
        } else {
            (neighbor, position)
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
