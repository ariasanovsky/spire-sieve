use super::{in_neighborhood::InNeighborhood, out_neighborhood::OutNeighborhood, NodeKind, WIDTH};

#[derive(Debug, Default)]
pub struct Row<In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    values: [(In, Out, Option<NodeKind>); WIDTH as usize],
}

impl<In, Out> Row<In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    pub fn values(&self) -> &[(In, Out, Option<NodeKind>)] {
        &self.values
    }

    pub fn out_neighborhood(&self, position: usize) -> &Out {
        &self.values[position].1
    }

    pub fn out_neighborhood_mut(&mut self, position: usize) -> &mut Out {
        &mut self.values[position].1
    }

    pub fn in_neighborhood(&self, position: usize) -> &In {
        &self.values[position].0
    }

    pub fn in_neighborhood_mut(&mut self, position: usize) -> &mut In {
        &mut self.values[position].0
    }

    pub fn kind(&self, position: usize) -> Option<&NodeKind> {
        self.values[position].2.as_ref()
    }

    pub fn kind_mut(&mut self, position: usize) -> &mut Option<NodeKind> {
        &mut self.values[position].2
    }

    pub fn in_neighborhoods(&self) -> impl Iterator<Item = &In> {
        self.values
            .iter()
            .map(|(in_neighborhood, _, _)| in_neighborhood)
    }

    pub fn out_neighborhoods(&self) -> impl Iterator<Item = &Out> {
        self.values.iter().map(|(_, out, _)| out)
    }

    pub fn kinds(&self) -> impl Iterator<Item = &Option<NodeKind>> {
        self.values.iter().map(|(_, _, kind)| kind)
    }

    pub fn count_in_neighborhoods(&self) -> usize {
        self.in_neighborhoods()
            .filter(|in_neighborhood| !in_neighborhood.is_empty())
            .count()
    }

    pub fn count_out_neighborhoods(&self) -> usize {
        self.out_neighborhoods()
            .filter(|out| !out.is_empty())
            .count()
    }
}

impl<In, Out> Row<In, Out>
where
    In: for<'a> InNeighborhood<'a>,
    Out: for<'a> OutNeighborhood<'a>,
{
    pub fn set_kind(&mut self, position: usize, kind: NodeKind) {
        *self.kind_mut(position) = Some(kind);
    }

    pub fn set_kinds(&mut self, kind: NodeKind) {
        for position in 0..WIDTH as usize {
            self.set_kind(position, kind);
        }
    }
}
