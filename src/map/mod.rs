use libgdx_xs128::{rng::Random, RandomXS128};

mod display;
mod in_neighborhood;
mod out_neighborhood;

use in_neighborhood::{InNeighborhood, InVec};
use out_neighborhood::{OutNeighborhood, OutVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Monster,
    Elite,
    Event,
    Rest,
    Shop,
    Treasure,
}

impl NodeKind {
    fn incompatible_with(&self, row: usize) -> bool {
        match row {
            0..=4 => [Self::Elite, Self::Rest].contains(self),
            13.. => self.eq(&Self::Rest),
            _ => false,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Monster => 'M',
            Self::Elite => 'E',
            Self::Event => '?',
            Self::Rest => 'R',
            Self::Shop => '$',
            Self::Treasure => 'T',
        }
    }
}

// #[derive(Debug, Default)]
// pub struct InNeighborhood(Vec<usize>);

// #[derive(Debug, Default)]
// pub struct OutNeighborhood(Vec<usize>);

const WIDTH: u64 = 7;
const LAST_POSITION: usize = WIDTH as usize - 1;
const HEIGHT: usize = 15;
const PATHS: u64 = 6;

const REST_ROW: usize = HEIGHT - 1;
const BEFORE_REST_ROW: usize = REST_ROW - 1;
const TREASURE_ROW: usize = 8;

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

    fn kind(&self, position: usize) -> Option<&NodeKind> {
        self.values[position].2.as_ref()
    }

    fn kind_mut(&mut self, position: usize) -> &mut Option<NodeKind> {
        &mut self.values[position].2
    }

    fn in_neighborhoods(&self) -> impl Iterator<Item = &InVec> {
        self.values.iter().map(|(in_neighborhood, _, _)| in_neighborhood)
    }
    
    fn out_neighborhoods(&self) -> impl Iterator<Item = &OutVec> {
        self.values.iter().map(|(_, out, _)| out)
    }

    fn kinds(&self) -> impl Iterator<Item = &Option<NodeKind>> {
        self.values.iter().map(|(_, _, kind)| kind)
    }

    fn count_in_neighborhoods(&self) -> usize {
        self.in_neighborhoods()
            .filter(|in_neighborhood| !in_neighborhood.is_empty())
            .count()
    }

    fn count_out_neighborhoods(&self) -> usize {
        self.out_neighborhoods().filter(|out| !out.is_empty()).count()
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
    pub fn generate(rng: &mut Random, ascension: bool) -> Map {
        let mut map: Map = Default::default();
        map.create_paths(rng);
        map.filter_redundant_edges_from_first_row();
        map.assign_rooms(rng, ascension);
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
        if row == 11 && [5, 6].contains(&position) {
            // dbg!(rng);
        }
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
        let rerolls = next_in_neighborhood
            .iter()
            .filter(|neighbor| !position.eq(&neighbor.0))
            .filter(|&&neighbor| !self.gca_skip(row, neighbor.0, position))
            .map(|neighbor| neighbor.1)
            .sum();
        for _ in 0..rerolls {
            // let foo = 3;
            
            
            if row == 11 && [5, 6].contains(&position) {
                let in_neighborhood_12_6: Vec<_> = self.row(12).in_neighborhood(6).iter().map(|&x| x.0).collect();
                let out_neighborhood_11_5: Vec<_> = self.row(11).out_neighborhood(5).iter().collect();
                let out_neighborhood_11_6: Vec<_> = self.row(11).out_neighborhood(6).iter().collect();
                dbg!(
                    &rng,
                    in_neighborhood_12_6,
                    out_neighborhood_11_5,
                    out_neighborhood_11_6,
                    next_position,
                );
                let foo = 3;
            }
            
            
            
            
            
            next_position = match next_position.cmp(&position) {
                std::cmp::Ordering::Greater => {
                    next_position = position + rng.next_capped_u64(2) as usize;
                    if next_position == 0 {
                        position
                    } else {
                        next_position - 1
                    }
                }
                std::cmp::Ordering::Equal => {
                    next_position = position + rng.next_capped_u64(3) as usize;
                    if next_position == 0 {
                        position + 1
                    } else if next_position >= LAST_POSITION {
                        position - 1
                    } else {
                        next_position - 1
                    }
                }
                std::cmp::Ordering::Less => {
                    next_position = position + rng.next_capped_u64(2) as usize;
                    if next_position >= WIDTH as usize {
                        position
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
        if row == 11 && [5, 6].contains(&position) {
            let in_neighborhood_12_6: Vec<_> = self.row(12).in_neighborhood(6).iter().map(|&x| x.0).collect();
            let out_neighborhood_11_5: Vec<_> = self.row(11).out_neighborhood(5).iter().collect();
            let out_neighborhood_11_6: Vec<_> = self.row(11).out_neighborhood(6).iter().collect();
            // dbg!(
            //     in_neighborhood_12_6,
            //     out_neighborhood_11_5,
            //     out_neighborhood_11_6,
            //     next_position,
            // );
            let foo = 3;
        }
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

        if row == 11 && [5, 6].contains(&position) {
            let in_neighborhood_12_6: Vec<_> = self.row(12).in_neighborhood(6).iter().map(|&x| x.0).collect();
            let out_neighborhood_11_5: Vec<_> = self.row(11).out_neighborhood(5).iter().collect();
            let out_neighborhood_11_6: Vec<_> = self.row(11).out_neighborhood(6).iter().collect();
            // dbg!(
            //     in_neighborhood_12_6,
            //     out_neighborhood_11_5,
            //     out_neighborhood_11_6,
            //     next_position,
            // );
            println!();
            let foo = 3;
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

impl Map {
    fn first_count(&self) -> usize {
        self.count_in_neighborhoods()
            + self.count_final_rest_sites()
            - self.count_penultimate_out_neighborhoods()
    }
    
    fn count_final_rest_sites(&self) -> usize {
        self.row(REST_ROW).count_in_neighborhoods()
    }

    fn count_penultimate_out_neighborhoods(&self) -> usize {
        self.row(BEFORE_REST_ROW).count_out_neighborhoods()
    }
    
    fn count_in_neighborhoods(&self) -> usize {
        self.rows.iter().map(|row| row.count_in_neighborhoods()).sum()
    }

    fn count_out_neighborhoods(&self) -> usize {
        self.rows.iter().map(|row| row.count_out_neighborhoods()).sum()
    }

    fn count_treasure_rooms(&self) -> usize {
        self.row(TREASURE_ROW).count_out_neighborhoods()
    }

    fn count_first_floor(&self) -> usize {
        self.row(0).count_out_neighborhoods()
    }

    fn count_pre_assigned_nodes(&self) -> usize {
        self.count_final_rest_sites()
            + self.count_treasure_rooms()
            + self.count_first_floor()
    }
    
    fn fill_room_array(count: usize, ascension: bool) -> Vec<NodeKind> {
        let chances: [(NodeKind, f32); 4] = [
            (NodeKind::Shop, 0.05),
            (NodeKind::Rest, 0.12),
            (NodeKind::Elite, if ascension {
                0.08 * 1.6
            } else {
                0.08
            }),
            (NodeKind::Event, 0.22),
        ];

        dbg!(count);

        let mut rooms = Vec::with_capacity(count);
        for (kind, chance) in chances {
            let kind_count = (chance * count as f32).round() as usize;
            dbg!(kind, kind_count);
            for _ in 0..kind_count {
                rooms.push(kind);
            }
        }
        rooms
    }

    fn adjusted_recount(&self) -> usize {
        self.count_out_neighborhoods() - self.count_pre_assigned_nodes() + self.count_final_rest_sites()
    }
}

impl Map {
    fn assign_rooms(&mut self, rng: &mut Random, ascension: bool) {
        let first_count = self.first_count();
        dbg!(first_count);
        let mut rooms = Self::fill_room_array(first_count, ascension);
        let recount = self.adjusted_recount();
        dbg!(recount);
        let new_size = rooms.len().max(recount);
        rooms.resize(new_size, NodeKind::Monster);
        Self::shuffle(&mut rooms, rng);
        dbg!(&rooms);
        for row in 0..HEIGHT {
            if [0, REST_ROW, TREASURE_ROW].contains(&row) {
                continue;
            }
            for position in 0..WIDTH as usize {
                if self.row(row).out_neighborhood(position).is_empty() {
                    continue;
                }
                if let Some(kind) = self.next_kind(&mut rooms, row, position) {
                    self.row_mut(row).set_kind(position, kind);
                }
            }
        }
        dbg!("leftovers:", rooms);
        self.set_constant_rows();
        self.populate_unassigned_nodes();
    }

    fn set_constant_rows(&mut self) {
        self.row_mut(0).set_kinds(NodeKind::Monster);
        self.row_mut(TREASURE_ROW).set_kinds(NodeKind::Treasure);
        self.row_mut(REST_ROW).set_kinds(NodeKind::Rest);
    }

    fn populate_unassigned_nodes(&mut self) {
        for row in 0..HEIGHT {
            for position in 0..WIDTH as usize {
                if self.row(row).kind(position).is_some() {
                    continue;
                }
                if self.row(row).in_neighborhood(position).is_empty() {
                    continue;
                }
                self.row_mut(row).set_kind(position, NodeKind::Monster);
            }
        }
    }
    
    fn shuffle(rooms: &mut [NodeKind], rng: &mut Random) {
        for i in (2..=rooms.len()).rev() {
            let j = rng.next_capped_u64(i as u64) as usize;
            rooms.swap(i - 1, j);
        }
    }

    fn next_kind(&self, rooms: &mut Vec<NodeKind>, row: usize, position: usize) -> Option<NodeKind> {
        rooms.iter().position(|kind| {
            if kind.incompatible_with(row) {
                return false;
            }
            if [NodeKind::Rest, NodeKind::Shop, NodeKind::Elite].contains(kind)
            && self.in_neighbor_kinds(row, position).contains(&kind) {
                return false;
            }
            let siblings = self.siblings(row, position);
            let node = &self.row(row).values[position];
            let printable_in_neighborhood = &node.0.iter().map(|&(neighbor, _)| neighbor).collect::<Vec<_>>();
            dbg!(&siblings, printable_in_neighborhood, position, row);
            let foo = 3;
            let sibling_kinds: Vec<&NodeKind> = siblings.iter().filter_map(|&sibling| {
                self.row(row).kind(sibling)
            }).collect();
            if [NodeKind::Rest, NodeKind::Shop, NodeKind::Elite, NodeKind::Monster, NodeKind::Event].contains(kind)
            && sibling_kinds.contains(&kind) {
                return false;
            }
            true
        })
        .map(|position| {
            let kind = rooms.remove(position);
            //dbg!(position, &kind, rooms.len());
            kind
            
        })
    }

    fn in_neighbor_kinds(&self, row: usize, position: usize) -> Vec<NodeKind> {
        let mut kinds = Vec::new();
        for &(in_neighbor, _) in self.row(row).in_neighborhood(position).iter() {
            if let Some(kind) = self.row(row - 1).kind(in_neighbor) {
                kinds.push(*kind);
            }
        }
        kinds
    }

    fn siblings(&self, row: usize, position: usize) -> Vec<usize> {
        self.row(row)
            .in_neighborhood(position)
            .iter()
            .flat_map(|&(parent, _)| {
                self.row(row - 1).out_neighborhood(parent).iter()
            })
            .filter_map(|&sibling| {
                Some(sibling)
                .filter(|&sibling| sibling != position)
            })
            .collect()
    }
}

impl Row {
    fn set_kind(&mut self, position: usize, kind: NodeKind) {
        self.values[position].2 = Some(kind);
    }

    fn set_kinds(&mut self, kind: NodeKind) {
        for position in 0..WIDTH as usize {
            self.set_kind(position, kind);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut rng = Random::from(1);
        let map = Map::generate(&mut rng, true);
        dbg!(&map);
        println!("{map}");
    }

    #[test]
    fn test_special_map() {
        let mut rng = Random::from(533907583096 + 1);
        let map = Map::generate(&mut rng, true);
        //dbg!(&map);
        println!("{map}");
    }
}
