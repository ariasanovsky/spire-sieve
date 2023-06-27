use libgdx_xs128::{rng::Random, RandomXS128};

mod display;

#[derive(Debug)]
pub enum NodeKind {
    Monster,
    Elite,
    Event,
    Rest,
    Shop,
    Treasure,
}

#[derive(Debug, Default)]
pub struct InNeighborhood(Vec<usize>);

#[derive(Debug, Default)]
pub struct OutNeighborhood(Vec<usize>);

const WIDTH: u64 = 7;
const LAST_POSITION: usize = WIDTH as usize - 1;
const HEIGHT: usize = 15;
const PATHS: u64 = 6;

#[derive(Debug, Default)]
pub struct Map([[(InNeighborhood, OutNeighborhood, Option<NodeKind>); WIDTH as usize]; HEIGHT]);

impl Map {
    pub fn generate(rng: &mut Random) -> Map {
        let mut map: Map = Default::default();
        map.create_paths(rng);
        map.filter_redundant_edges_from_first_row();
        map
    }

    fn create_paths(&mut self, rng: &mut Random) {
        let first_position = self.create_first_path(rng);
        self.create_second_path(rng, first_position);
        (2..PATHS).for_each(|_| {
            // dbg!(path);
            self.create_path(rng);
        })
    }

    fn create_first_path(&mut self, rng: &mut Random) -> usize {
        let first_position = rng.next_capped_u64(WIDTH) as usize;
        let mut position = first_position;
        // let path = 0;
        // dbg!(path);

        for row in 0..HEIGHT - 1 {
            // println!("row {row}:\t{position}");
            let next_position = self.next_position(rng, row, position);
            self.add_edge(row, position, next_position);
            position = next_position;
        }
        first_position
    }

    fn add_edge(&mut self, row: usize, position: usize, next_position: usize) {
        let (_, out_neigh, _) = &mut self.0[row][position];
        out_neigh.0.push(next_position);
        let (in_neigh, _, _) = &mut self.0[row + 1][next_position];
        in_neigh.0.push(position);
    }

    fn next_position(&self, rng: &mut Random, row: usize, position: usize) -> usize {
        let min_position = if position == 0 { 0 } else { position - 1 };
        let n_possible_positions = if position == 0 || position == WIDTH as usize - 1 {
            2
        } else {
            3
        };
        let adjustment = rng.next_capped_u64(n_possible_positions) as usize;
        let mut next_position = adjustment + min_position;
        // dbg!(next_position, &rng);
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
        let (next_in_neighborhood, _, _) = &self.0[row + 1][next_position];
        let old_next_position = next_position;
        for &neighbor in &next_in_neighborhood.0 {
            // let foo = 3;
            if neighbor == position {
                continue;
            }
            if !self.gca_skip(row, neighbor, position) {
                continue;
            }
            next_position = match next_position.cmp(&position) {
                std::cmp::Ordering::Greater => {
                    next_position = position + rng.next_capped_u64(2) as usize;
                    if next_position == 0 {
                        old_next_position
                    } else {
                        next_position - 1
                    }
                }
                std::cmp::Ordering::Equal => {
                    next_position = position + rng.next_capped_u64(3) as usize;
                    if next_position == 0 {
                        old_next_position + 1
                    } else if next_position >= LAST_POSITION {
                        old_next_position - 1
                    } else {
                        next_position - 1
                    }
                }
                std::cmp::Ordering::Less => {
                    next_position = position + rng.next_capped_u64(2) as usize;
                    if next_position >= WIDTH as usize {
                        old_next_position
                    } else {
                        next_position
                    }
                }
            };
            // dbg!(next_position, &rng);
            // let foo = 3;
        }
        next_position
    }

    fn gca_skip(&self, row: usize, position: usize, neighbor: usize) -> bool {
        let (left_position, right_position) = if position < row {
            (position, neighbor)
        } else {
            (neighbor, position)
        };

        let (in_neighborhood, _, _) = &self.0[row][left_position];
        let left_max = in_neighborhood.0.iter().max();
        let (in_neighborhood, _, _) = &self.0[row][right_position];
        let right_min = in_neighborhood.0.iter().min();
        match (left_max, right_min) {
            (Some(&left_max), Some(&right_min)) => left_max == right_min,
            _ => false,
        }
    }

    fn cpanx(&self, row: usize, position: usize, mut next_position: usize) -> usize {
        assert!(position <= LAST_POSITION);
        assert!(next_position <= LAST_POSITION);

        if position != 0 {
            let sibling_position = position - 1;
            let (_, out_neighborhood, _) = &self.0[row][sibling_position];
            let out_neighbor = out_neighborhood.0.iter().max();
            if let Some(&out_neighbor) = out_neighbor {
                if out_neighbor > next_position {
                    next_position = out_neighbor;
                    // dbg!(next_position);
                    // let foo = 3;
                }
            }
        }

        if position != WIDTH as usize - 1 {
            let sibling_position = position + 1;
            let (_, out_neighborhood, _) = &self.0[row][sibling_position];
            let out_neighbor = out_neighborhood.0.iter().min();
            if let Some(&out_neighbor) = out_neighbor {
                if out_neighbor < next_position {
                    next_position = out_neighbor;
                    // dbg!(next_position);
                    // let foo = 3;
                }
            }
        }
        next_position
    }

    fn create_second_path(&mut self, rng: &mut Random, first_position: usize) {
        let mut position = rng.next_capped_u64(WIDTH) as usize;
        while position == first_position {
            position = rng.next_capped_u64(WIDTH) as usize;
        }

        // let path = 1;
        // dbg!(path);

        for row in 0..HEIGHT - 1 {
            // println!("row {row}:\t{position}");
            let next_position = self.next_position(rng, row, position);
            self.add_edge(row, position, next_position);
            position = next_position;
        }
    }

    fn create_path(&mut self, rng: &mut Random) {
        let mut position = rng.next_capped_u64(WIDTH) as usize;
        for row in 0..HEIGHT - 1 {
            // println!("row {row}:\t{position}");
            let next_position = self.next_position(rng, row, position);
            self.add_edge(row, position, next_position);
            position = next_position;
        }
    }
}

impl Map {
    fn filter_redundant_edges_from_first_row(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut rng = Random::from(1);
        let map = Map::generate(&mut rng);
        dbg!(&map);
        println!("{map}");
    }

    #[test]
    fn test_special_map() {
        let mut rng = Random::from(533907583096 + 1);
        let map = Map::generate(&mut rng);
        //dbg!(&map);
        println!("{map}");
    }
}
