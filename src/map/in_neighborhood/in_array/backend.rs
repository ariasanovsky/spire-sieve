#[derive(Debug)]
pub enum StrongCompositionOfLengthAtMostThree {
    Empty,
    One(usize),
    Two(usize, usize),
    Three(usize, usize, usize),
}

impl StrongCompositionOfLengthAtMostThree {
    pub const fn compositions_of_six() -> [Self; 42] {
        use StrongCompositionOfLengthAtMostThree::*;
        [
            // 0
            Empty,
            // 1
            One(1),
            // 2
            Two(1, 1),
            One(2),
            // 3
            Three(1, 1, 1),
            Two(1, 2),
            Two(2, 1),
            One(3),
            // 4
            Three(1, 1, 2),
            Three(1, 2, 1),
            Three(2, 1, 1),
            Two(2, 2),
            Two(1, 3),
            Two(3, 1),
            One(4),
            // 5
            Three(1, 2, 2),
            Three(2, 1, 2),
            Three(2, 2, 1),
            Three(1, 1, 3),
            Three(1, 3, 1),
            Three(3, 1, 1),
            Two(2, 3),
            Two(3, 2),
            Two(1, 4),
            Two(4, 1),
            One(5),
            // 6
            Three(2, 2, 2),
            Three(1, 2, 3),
            Three(1, 3, 2),
            Three(2, 1, 3),
            Three(2, 3, 1),
            Three(3, 1, 2),
            Three(3, 2, 1),
            Three(1, 1, 4),
            Three(1, 4, 1),
            Three(4, 1, 1),
            Two(3, 3),
            Two(2, 4),
            Two(4, 2),
            Two(1, 5),
            Two(5, 1),
            One(6),
        ]
    }
}
