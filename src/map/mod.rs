use libgdx_xs128::{rng::Random, RandomXS128};

mod display;
mod filter;
mod in_neighborhood;
pub mod node_kind;
mod out_neighborhood;
mod row;
pub mod skeleton;

#[allow(unused)]
mod tests;

use in_neighborhood::InNeighborhood;
use out_neighborhood::{out_vec::OutVec, OutNeighborhood};
use row::Row;

use self::in_neighborhood::in_vec::InVec;
use self::node_kind::NodeKind;

pub const WIDTH: u64 = 7;
pub const LAST_POSITION: usize = WIDTH as usize - 1;
pub const HEIGHT: usize = 15;
// pub const PATHS: u64 = 6;

pub const REST_ROW: usize = HEIGHT - 1;
pub const BEFORE_REST_ROW: usize = REST_ROW - 1;
pub const TREASURE_ROW: usize = 8;

#[derive(Debug, Default)]
pub struct Map<const PATHS: usize, In, Out>
where
    In: for<'a> InNeighborhood<'a, 'a>,
    Out: for<'a> OutNeighborhood<'a, 'a>,
{
    rows: [Row<In, Out>; HEIGHT],
}

type DefaultMap = Map<6, InVec, OutVec>;

impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a, 'a>,
    Out: for<'a> OutNeighborhood<'a, 'a>,
{
    fn row(&self, row: usize) -> &Row<In, Out> {
        &self.rows[row]
    }

    fn row_mut(&mut self, row: usize) -> &mut Row<In, Out> {
        &mut self.rows[row]
    }

    fn rows(&self) -> &[Row<In, Out>; HEIGHT] {
        &self.rows
    }
}

impl<const HEIGHT: usize, In, Out> Map<HEIGHT, In, Out>
where
    In: for<'a> InNeighborhood<'a, 'a>,
    Out: for<'a> OutNeighborhood<'a, 'a>,
{
    pub fn generate(rng: &mut Random, ascension: bool) -> DefaultMap {
        let mut map: DefaultMap = DefaultMap::default();
        map.create_paths(rng);
        map.filter_redundant_edges_from_first_row();
        map.assign_rooms(rng, ascension);
        map
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
}

impl DefaultMap {
    fn filter_redundant_edges_from_first_row(&mut self) {
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

#[derive(Debug, Clone, Copy)]
pub enum EliteBuff {
    Strength,
    MaxHP,
    Metallicize,
    Regenerate,
}

impl DefaultMap {
    pub fn burning_elite_position(&self, rng: &mut Random) -> Option<(usize, usize)> {
        let mut positions = Vec::new();
        for (y, row) in self.rows().iter().enumerate() {
            for (x, kind) in row.kinds().enumerate() {
                if let Some(NodeKind::Elite) = kind {
                    positions.push((x, y));
                }
            }
        }
        let pos = rng.next_capped_u64(positions.len() as u64) as usize;
        positions.get(pos).copied()
    }

    pub fn burning_elite_buff(rng: &mut Random) -> EliteBuff {
        match rng.next_capped_u64(4) {
            0 => EliteBuff::Strength,
            1 => EliteBuff::MaxHP,
            2 => EliteBuff::Metallicize,
            3 => EliteBuff::Regenerate,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut rng = Random::from(1);
        let map = DefaultMap::generate(&mut rng, true);
        dbg!(&map);
        println!("{map}");
    }

    #[test]
    fn test_special_map() {
        let mut rng = Random::from(533907583096 + 1);
        let map = DefaultMap::generate(&mut rng, true);
        println!("{map}");
        let burning_elite_position = map.burning_elite_position(&mut rng);
        dbg!(burning_elite_position);
        let burning_elite_buff = DefaultMap::burning_elite_buff(&mut rng);
        dbg!(burning_elite_buff);
    }

    fn test_map_with_seed(seed: i64) {
        dbg!(seed);
        let mut rng = Random::from(seed + 1);
        let map = DefaultMap::generate(&mut rng, true);
        println!("{map}");
        let burning_elite_position = map.burning_elite_position(&mut rng);
        dbg!(burning_elite_position);
        let burning_elite_buff = DefaultMap::burning_elite_buff(&mut rng);
        dbg!(burning_elite_buff);
    }

    #[test]
    fn test_special_maps() {
        const SEEDS: &[i64] = &[
            533907583096,
            2118750211857,
            3481836885783,
            8399213486180,
            8867133130014,
            8930754426721,
            9884674834485,
        ];
        for &seed in SEEDS {
            test_map_with_seed(seed);
            println!();
        }
    }
}
