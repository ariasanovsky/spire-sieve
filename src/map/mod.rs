use libgdx_xs128::rng::Random;

pub mod assign_nodes;
mod display;
mod filters;
mod in_neighborhood;
mod out_neighborhood;
mod row;
pub mod skeleton;

#[allow(unused)]
mod tests;

use in_neighborhood::InNeighborhood;
use out_neighborhood::{out_vec::OutVec, OutNeighborhood};
use row::Row;

use self::assign_nodes::NodeKind;
use self::in_neighborhood::in_vec::InVec;
use self::in_neighborhood::in_byte::InByte;

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
type _NewDefaultMap = Map<6, InByte, OutVec>;

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

    fn remove_first_row_edge(&mut self, position: usize, next_position: usize) {
        let out_neighborhood = &mut self.row_mut(0).out_neighborhood_mut(position);
        out_neighborhood.remove(next_position);
    }
}

impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a, 'a> + Default,
    Out: for<'a> OutNeighborhood<'a, 'a>,
{
    pub fn generate(rng: &mut Random, ascension: bool) -> Map<PATHS, In, Out> {
        let mut map = Map::default();
        map.create_paths(rng);
        map.filter_redundant_edges_from_first_row();
        map.assign_rooms(rng, ascension);
        map
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn print_first_map() {
        print_map_with_seed::<InVec>(1);
    }

    #[test]
    fn print_map_for_special_seed() {
        let seed = 533907583096i64;
        print_map_with_seed::<InVec>(seed);
    }

    fn print_map_with_seed<In>(seed: i64)
    where
        In: for<'a> InNeighborhood<'a, 'a> + Default,
    {
        dbg!(seed);
        let mut rng = Random::from(seed + 1);
        let map: Map<6, In, OutVec> = Map::generate(&mut rng, true);
        println!("{map}");
        let burning_elite_position = map.burning_elite_position(&mut rng);
        dbg!(burning_elite_position);
        let burning_elite_buff = Map::<6, In, OutVec>::burning_elite_buff(&mut rng);
        dbg!(burning_elite_buff);
    }

    #[test]
    fn print_special_maps() {
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
            print_map_with_seed::<InVec>(seed);
            println!();
        }
    }
}
