#[cfg(feature = "std")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use libgdx_xs128::{rng::Random, RandomXS128};

#[cfg(feature = "std")]
use self::kind::NodeKind;

#[cfg(feature = "std")]
use super::{
    in_neighborhood::InNeighborhood, out_neighborhood::OutNeighborhood, Map, BEFORE_REST_ROW,
    HEIGHT, REST_ROW, TREASURE_ROW, WIDTH,
};

#[cfg(feature = "std")]
pub mod buffed_elite;
#[cfg(feature = "std")]
pub mod display;
pub mod kind;

#[cfg(feature = "std")]
impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
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
        self.skeleton
            .rows
            .iter()
            .map(|row| row.count_in_neighborhoods())
            .sum()
    }

    fn count_out_neighborhoods(&self) -> usize {
        self.skeleton
            .rows
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

#[cfg(feature = "std")]
impl<const HEIGHT: usize, In, Out> Map<HEIGHT, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
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

#[cfg(feature = "std")]
impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
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
                    self.set_kind(row, position, kind);
                }
            }
        }
        self.set_constant_rows();
        self.populate_unassigned_nodes();
    }

    pub fn set_kind(&mut self, row: usize, position: usize, kind: NodeKind) {
        self.kinds[row][position] = kind;
    }

    pub fn set_kinds(&mut self, row: usize, kind: NodeKind) {
        for position in 0..WIDTH as usize {
            self.set_kind(row, position, kind);
        }
    }

    fn set_constant_rows(&mut self) {
        self.set_kinds(0, NodeKind::Monster);
        self.set_kinds(TREASURE_ROW, NodeKind::Treasure);
        self.set_kinds(REST_ROW, NodeKind::Rest);
        // self.row_mut(0).set_kinds(NodeKind::Monster);
        // self.row_mut(TREASURE_ROW).set_kinds(NodeKind::Treasure);
        // self.row_mut(REST_ROW).set_kinds(NodeKind::Rest);
    }

    fn populate_unassigned_nodes(&mut self) {
        for row in 0..HEIGHT {
            for position in 0..WIDTH as usize {
                if self.kind(row, position).is_assigned() {
                    continue;
                }
                if self.row(row).in_neighborhood(position).is_empty() {
                    continue;
                }
                // self.row_mut(row).set_kind(position, NodeKind::Monster);
                self.set_kind(row, position, NodeKind::Monster);
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
                let sibling_kinds: Vec<NodeKind> = siblings
                    .iter()
                    .map(|&sibling| *self.kind(row, sibling))
                    .filter(NodeKind::is_assigned)
                    .collect();
                if [
                    NodeKind::Rest,
                    NodeKind::Shop,
                    NodeKind::Elite,
                    NodeKind::Monster,
                    NodeKind::Event,
                ]
                .contains(kind)
                    && sibling_kinds.contains(kind)
                {
                    return false;
                }
                true
            })
            .map(|position| rooms.remove(position))
    }

    fn in_neighbor_kinds(&self, row: usize, position: usize) -> Vec<NodeKind> {
        self.row(row)
            .in_neighborhood(position)
            .iter()
            .map(|&(in_neighbor, _)| *self.kind(row - 1, in_neighbor))
            .filter(|kind| kind.is_assigned() && !kind.is_empty())
            .collect()
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
