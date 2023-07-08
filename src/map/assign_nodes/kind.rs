#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    #[default]
    Unassigned,
    Monster,
    Elite,
    Event,
    Rest,
    Shop,
    Treasure,
    Empty,
}

impl NodeKind {
    pub(crate) fn incompatible_with(&self, row: usize) -> bool {
        match row {
            0..=4 => [Self::Elite, Self::Rest].contains(self),
            13.. => self.eq(&Self::Rest),
            _ => false,
        }
    }

    pub fn char(&self) -> char {
        match self {
            Self::Unassigned => '*',
            Self::Monster => 'M',
            Self::Elite => 'E',
            Self::Event => '?',
            Self::Rest => 'R',
            Self::Shop => '$',
            Self::Treasure => 'T',
            Self::Empty => ' ',
        }
    }

    pub fn is_assigned(&self) -> bool {
        !matches!(self, Self::Unassigned)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}
