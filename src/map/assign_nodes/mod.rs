use std::fmt::Debug;

use libgdx_xs128::{rng::Random, RandomXS128};

use super::{
    in_neighborhood::{InNeighborhood}, out_neighborhood::OutNeighborhood, Map, BEFORE_REST_ROW,
    HEIGHT, REST_ROW, TREASURE_ROW, WIDTH,
};

pub mod buffed_elite;

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
    pub(super) fn incompatible_with(&self, row: usize) -> bool {
        match row {
            0..=4 => [Self::Elite, Self::Rest].contains(self),
            13.. => self.eq(&Self::Rest),
            _ => false,
        }
    }

    pub fn char(&self) -> char {
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

impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a, 'a>,
    Out: for<'a> OutNeighborhood<'a, 'a>,
{
    fn first_count(&self) -> usize {
        self.count_in_neighborhoods() + self.count_final_rest_sites()
            - self.count_penultimate_out_neighborhoods()
    }

    fn count_final_rest_sites(&self) -> usize {
        self.row(REST_ROW).count_in_neighborhoods()
    }

    fn count_penultimate_out_neighborhoods(&self) -> usize {
        self.row(BEFORE_REST_ROW).count_out_neighborhoods()
    }

    fn count_in_neighborhoods(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.count_in_neighborhoods())
            .sum()
    }

    fn count_out_neighborhoods(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.count_out_neighborhoods())
            .sum()
    }

    fn count_treasure_rooms(&self) -> usize {
        self.row(TREASURE_ROW).count_out_neighborhoods()
    }

    fn count_first_floor(&self) -> usize {
        self.row(0).count_out_neighborhoods()
    }

    fn count_pre_assigned_nodes(&self) -> usize {
        self.count_final_rest_sites() + self.count_treasure_rooms() + self.count_first_floor()
    }

    fn fill_room_array(count: usize, ascension: bool) -> Vec<NodeKind> {
        let chances: [(NodeKind, f32); 4] = [
            (NodeKind::Shop, 0.05),
            (NodeKind::Rest, 0.12),
            (NodeKind::Elite, if ascension { 0.08 * 1.6 } else { 0.08 }),
            (NodeKind::Event, 0.22),
        ];

        let mut rooms = Vec::with_capacity(count);
        for (kind, chance) in chances {
            let kind_count = (chance * count as f32).round() as usize;
            for _ in 0..kind_count {
                rooms.push(kind);
            }
        }
        rooms
    }

    fn adjusted_recount(&self) -> usize {
        self.count_out_neighborhoods() - self.count_pre_assigned_nodes()
            + self.count_final_rest_sites()
    }
}
impl<const HEIGHT: usize, In, Out> Map<HEIGHT, In, Out>
where
    In: for<'a> InNeighborhood<'a, 'a>,
    Out: for<'a> OutNeighborhood<'a, 'a>,
{
    pub(super) fn filter_redundant_edges_from_first_row(&mut self) {
        let mut visited = [false; WIDTH as usize];
        let removals: Vec<_> = self
            .row(0)
            .out_neighborhoods()
            .enumerate()
            .flat_map(|(position, out_neighborhood)| {
                out_neighborhood.iter().filter_map(move |&next_position| {
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
            self.remove_first_row_edge(position, next_position);
        }
    }
}

impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a, 'a>,
    Out: for<'a> OutNeighborhood<'a, 'a>,
{
    pub fn assign_rooms(&mut self, rng: &mut Random, ascension: bool) {
        let first_count = self.first_count();
        let mut rooms = Self::fill_room_array(first_count, ascension);
        let recount = self.adjusted_recount();
        let new_size = rooms.len().max(recount);
        rooms.resize(new_size, NodeKind::Monster);
        Self::shuffle(&mut rooms, rng);
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

    fn next_kind(
        &self,
        rooms: &mut Vec<NodeKind>,
        row: usize,
        position: usize,
    ) -> Option<NodeKind> {
        rooms
            .iter()
            .position(|kind| {
                if kind.incompatible_with(row) {
                    return false;
                }
                if [NodeKind::Rest, NodeKind::Shop, NodeKind::Elite].contains(kind)
                    && self.in_neighbor_kinds(row, position).contains(kind)
                {
                    return false;
                }
                let siblings = self.siblings(row, position);
                let sibling_kinds: Vec<&NodeKind> = siblings
                    .iter()
                    .filter_map(|&sibling| self.row(row).kind(sibling))
                    .collect();
                if [
                    NodeKind::Rest,
                    NodeKind::Shop,
                    NodeKind::Elite,
                    NodeKind::Monster,
                    NodeKind::Event,
                ]
                .contains(kind)
                    && sibling_kinds.contains(&kind)
                {
                    return false;
                }
                true
            })
            .map(|position| rooms.remove(position))
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
            .flat_map(|&(parent, _)| self.row(row - 1).out_neighborhood(parent).iter())
            .filter_map(|&sibling| Some(sibling).filter(|&sibling| sibling != position))
            .collect()
    }
}
