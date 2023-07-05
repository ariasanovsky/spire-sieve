use libgdx_xs128::{rng::Random, RandomXS128};

use crate::map::{in_neighborhood::InNeighborhood, out_neighborhood::OutNeighborhood, Map};

use super::NodeKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EliteBuff {
    Strength,
    MaxHP,
    Metallicize,
    Regenerate,
}

#[derive(Debug, Clone)]
pub struct EliteInfo {
    pub buff_x: usize,
    pub buff_y: usize,
    pub buff: EliteBuff,
    pub buff_index: usize,
    pub count: usize,
}

impl<const PATHS: usize, In, Out> Map<PATHS, In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    pub fn burning_elite(&self, rng: &mut Random) -> Option<EliteInfo> {
        let ((x, y), buff_index, count) = self.burning_elite_position(rng)?;
        let buff = Self::burning_elite_buff(rng);
        Some(EliteInfo {
            buff_x: x,
            buff_y: y,
            buff,
            buff_index,
            count,
        })
    }

    pub(crate) fn burning_elite_position(
        &self,
        rng: &mut Random,
    ) -> Option<((usize, usize), usize, usize)> {
        let mut positions = Vec::new();
        for (y, row) in self.kinds.iter().enumerate() {
            for (x, kind) in row.iter().enumerate() {
                // if let Some(NodeKind::Elite) = kind {
                //     positions.push((x, y));
                // }
                if matches!(kind, NodeKind::Elite) {
                    positions.push((x, y));
                }
            }
        }
        // for (y, row) in self.kinds.iter().enumerate() {
        //     for (x, kind) in row.kinds().enumerate() {
                // if let Some(NodeKind::Elite) = kind {
                //     positions.push((x, y));
                // }
                
        //     }
        // }
        let count = positions.len();
        let pos = rng.next_capped_u64(count as u64) as usize;
        positions
            .get(pos)
            .copied()
            .map(|(x, y)| ((x, y), pos, count))
    }

    pub(crate) fn burning_elite_buff(rng: &mut Random) -> EliteBuff {
        match rng.next_capped_u64(4) {
            0 => EliteBuff::Strength,
            1 => EliteBuff::MaxHP,
            2 => EliteBuff::Metallicize,
            3 => EliteBuff::Regenerate,
            _ => unreachable!(),
        }
    }
}
