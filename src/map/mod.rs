use libgdx_xs128::rng::Random;

#[cfg(feature = "std")]
pub mod assign_nodes;
pub mod canonize;
pub mod display;
pub mod filters;
mod in_neighborhood;
mod out_neighborhood;
pub mod row;
pub mod skeleton;

#[allow(unused)]
mod tests;

use in_neighborhood::InNeighborhood;
use out_neighborhood::{out_vec::OutVec, OutNeighborhood};
use row::Row;

use self::assign_nodes::NodeKind;
use self::in_neighborhood::in_byte::InByte;
use self::in_neighborhood::in_vec::InVec;
use self::skeleton::Skeleton;

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
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    skeleton: Skeleton<PATHS, In, Out>,
    kinds: [[NodeKind; WIDTH as usize]; HEIGHT],
}

#[derive(Debug, Default)]
pub enum Act {
    #[default]
    One,
    Two,
    Three,
    // Four,
}

impl Act {
    const fn seed_offset(&self) -> i64 {
        match self {
            Act::One => 1,
            Act::Two => todo!(),
            Act::Three => todo!(),
        }
    }
}

impl crate::seed::Seed {
    pub fn map_rng(&self, act: Act) -> Random {
        self.offset_rng(act.seed_offset())
    }
}

type _DefaultMap = Map<6, InVec, OutVec>;
type _NewDefaultMap = Map<6, InByte, OutVec>;

impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    fn row(&self, row: usize) -> &Row<In, Out> {
        &self.skeleton.rows[row]
    }

    fn row_mut(&mut self, row: usize) -> &mut Row<In, Out> {
        &mut self.skeleton.rows[row]
    }

    fn _rows(&self) -> &[Row<In, Out>; HEIGHT] {
        &self.skeleton.rows
    }

    fn remove_first_row_edge(&mut self, position: usize, next_position: usize) {
        let out_neighborhood = &mut self.row_mut(0).out_neighborhood_mut(position);
        out_neighborhood.remove(next_position);
    }

    fn kind(&self, row: usize, position: usize) -> &NodeKind {
        &self.kinds[row][position]
    }

    fn _kind_mut(&mut self, row: usize, position: usize) -> &mut NodeKind {
        &mut self.kinds[row][position]
    }

    pub fn kinds(&self, row: usize) -> impl Iterator<Item = &NodeKind> {
        self.kinds[row].iter()
    }
}

use std::fmt::Debug;
use std::fmt::Display;

impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a> + Default + Debug + Display,
    Out: for<'a> OutNeighborhood<'a> + Default + Debug + Display,
{
    pub fn generate(rng: &mut Random, ascension: bool) -> Map<PATHS, In, Out> {
        let skeleton = Skeleton::generate(rng);
        let mut map = Self {
            skeleton,
            kinds: [[NodeKind::default(); WIDTH as usize]; HEIGHT],
        };

        map.filter_redundant_edges_from_first_row();
        map.assign_rooms(rng, ascension);

        // let mut map = Map::default();
        // map.create_paths(rng);
        // map.filter_redundant_edges_from_first_row();
        // map.assign_rooms(rng, ascension);
        // map

        map
    }
}

#[cfg(test)]
mod map_tests {
    use std::{println, dbg};

    use crate::seed::Seed;

    use super::*;

    #[test]
    fn print_first_map() {
        print_map_with_seed::<InVec>(1i64.into());
    }

    #[test]
    fn print_map_for_special_seed() {
        let seed = 533907583096i64;
        print_map_with_seed::<InVec>(seed.into());
    }

    fn print_map_with_seed<In>(seed: Seed)
    where
        In: for<'a> InNeighborhood<'a> + Default + Debug + Display,
    {
        dbg!(&seed);
        let mut rng = seed.map_rng(Act::One);
        let map: Map<6, In, OutVec> = Map::generate(&mut rng, true);
        println!("{map}");
        let burning_elite_position = map.burning_elite_position(&mut rng);
        dbg!(burning_elite_position);
        let burning_elite_buff = Map::<6, In, OutVec>::burning_elite_buff(&mut rng);
        dbg!(burning_elite_buff);
    }

    #[test]
    fn print_one_path_burning_elite_bottleneck_maps() {
        // const SEEDS: &[i64] = &[
        //     533907583096,
        //     2118750211857,
        //     3481836885783,
        //     8399213486180,
        //     8867133130014,
        //     8930754426721,
        //     9884674834485,
        // ];
        for &seed in _ONE_PATH_BURNING_ELITE_BOTTLENECKS {
            print_map_with_seed::<InVec>(seed.into());
            println!();
        }
    }
}

pub const _ONE_PATH_BURNING_ELITE_BOTTLENECKS: &[&[u8; 13]] = &[
    b"     8AFF4ZZ6",
    b"     XXKBUJNS",
    b"    1J432TK4I",
    b"    3QJ3DI01K",
    b"    3XTMF0PHJ",
    b"    3YT8RJBX1",
    b"    4DM63LTVA",
];
