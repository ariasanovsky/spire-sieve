use libgdx_xs128::{rng::Random, RandomXS128};

mod display;
mod in_neighborhood;
mod out_neighborhood;

use in_neighborhood::{InNeighborhood, InVec};
use out_neighborhood::{OutNeighborhood, OutVec};

#[derive(Debug)]
pub enum NodeKind {
    Monster,
    Elite,
    Event,
    Rest,
    Shop,
    Treasure,
}

// #[derive(Debug, Default)]
// pub struct InNeighborhood(Vec<usize>);

// #[derive(Debug, Default)]
// pub struct OutNeighborhood(Vec<usize>);

const WIDTH: u64 = 7;
const LAST_POSITION: usize = WIDTH as usize - 1;
const HEIGHT: usize = 15;
const PATHS: u64 = 6;

//pub type Row = [(InVec, OutVec, Option<NodeKind>); WIDTH as usize];

#[derive(Debug, Default)]
pub struct Row {
    values: [(InVec, OutVec, Option<NodeKind>); WIDTH as usize],
}

impl Row {
    fn out_neighborhood(&self, position: usize) -> &OutVec {
        &self.values[position].1
    }

    fn out_neighborhood_mut(&mut self, position: usize) -> &mut OutVec {
        &mut self.values[position].1
    }

    fn in_neighborhood(&self, position: usize) -> &InVec {
        &self.values[position].0
    }

    fn in_neighborhood_mut(&mut self, position: usize) -> &mut InVec {
        &mut self.values[position].0
    }

    fn out_neighborhoods(&self) -> impl Iterator<Item = &OutVec> {
        self.values.iter().map(|(_, out, _)| out)
    }
}

#[derive(Debug, Default)]
pub struct Map {
    rows: [Row; HEIGHT],
}

impl Map {
    fn row(&self, row: usize) -> &Row {
        &self.rows[row]
    }

    fn row_mut(&mut self, row: usize) -> &mut Row {
        &mut self.rows[row]
    }
}

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

    fn add_edge(&mut self, row: usize, position: usize, next_position: usize) {
        let out_neighborhood = self.row_mut(row).out_neighborhood_mut(position);
        out_neighborhood.push(next_position);

        let in_neighborhood = &mut self.row_mut(row + 1).in_neighborhood_mut(next_position);
        in_neighborhood.push(position);
    }

    fn remove_first_row_edge(&mut self, position: usize, next_position: usize) {
        let out_neighborhood = &mut self.row_mut(0).out_neighborhood_mut(position);
        out_neighborhood.remove(next_position);
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
        let next_in_neighborhood = &self.row(row + 1).in_neighborhood(next_position);
        let old_next_position = next_position;
        let rerolls = next_in_neighborhood
            .iter()
            .filter(|neighbor| !position.eq(&neighbor.0))
            .filter(|&&neighbor| !self.gca_skip(row, neighbor.0, position))
            .map(|neighbor| neighbor.1)
            .sum();
        for _ in 0..rerolls {
            // let foo = 3;
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

        let row = self.row(row);
        let left_in_neighborhood = row.in_neighborhood(left_position);
        let right_in_neighborhood = row.in_neighborhood(right_position);
        InNeighborhood::gca_skip(left_in_neighborhood, right_in_neighborhood)
    }

    fn cpanx(&self, row: usize, position: usize, mut next_position: usize) -> usize {
        if position != 0 {
            let sibling_position = position - 1;
            let out_neighborhood = self.row(row).out_neighborhood(sibling_position);
            if let Some(&out_neighbor) = out_neighborhood
                .max()
                .filter(|&out_neighbor| next_position.lt(out_neighbor))
            {
                next_position = out_neighbor;
            }
        }

        if position != LAST_POSITION {
            let sibling_position = position + 1;
            let out_neighborhood = self.row(row).out_neighborhood(sibling_position);
            if let Some(&out_neighbor) = out_neighborhood
                .min()
                .filter(|&out_neighbor| next_position.gt(out_neighbor))
            {
                next_position = out_neighbor;
            }
        }
        next_position
    }
}

impl Map {
    fn filter_redundant_edges_from_first_row(&mut self) {
        let mut visited = [false; WIDTH as usize];
        let removals: Vec<_> = self
            .row(0)
            .out_neighborhoods()
            .enumerate()
            .flat_map(|(position, out_neighborhood)| {
                out_neighborhood.iter()
                .filter_map(move |&next_position| {
                    if visited[next_position] {
                        Some((position, next_position))
                    } else {
                        visited[next_position] = true;
                        None
                    }
                })
            })
            .collect();
        for (position, next_position) in removals {
            let out_neighborhood = &mut self.row_mut(0).out_neighborhood_mut(position);
            out_neighborhood.remove(next_position);
        }
    }
}

impl Row {

}

impl Map {
    
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
