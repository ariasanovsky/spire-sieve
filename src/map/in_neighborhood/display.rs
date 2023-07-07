use std::fmt::Display;

use super::{in_array::InArray, in_byte::InByte, in_vec::InVec, InNeighborhood};

impl Display for InByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "[]")
        } else {
            self.iter()
                .try_for_each(|(value, count)| (0..*count).try_for_each(|_| write!(f, "{}", value)))
        }
    }
}

impl Display for InArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "[]")
        } else {
            self.iter()
                .try_for_each(|(value, count)| (0..*count).try_for_each(|_| write!(f, "{}", value)))
        }
    }
}

impl Display for InVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "[]")
        } else {
            self.iter()
                .try_for_each(|(value, count)| (0..*count).try_for_each(|_| write!(f, "{}", value)))
        }
    }
}

// impl Display for InVec {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.iter().try_for_each(|(value, count)| {
//             write!(f, "{}", value)?;
//             if *count > 1 {
//                 write!(f, "({})", count)?;
//             }
//             Ok(())
//         })
//     }
// }

// impl Display for InArray {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             InArray::Zero([]) => write!(f, "[]"),
//             InArray::One([a]) => write!(f, "{}", a.0.to_string().repeat(a.1)),
//             InArray::Two([a, b]) => write!(
//                 f,
//                 "{}{}",
//                 a.0.to_string().repeat(a.1),
//                 b.0.to_string().repeat(b.1)
//             ),
//             InArray::Three([a, b, c]) => write!(
//                 f,
//                 "{}{}{}",
//                 a.0.to_string().repeat(a.1),
//                 b.0.to_string().repeat(b.1),
//                 c.0.to_string().repeat(c.1)
//             ),
//         }
//     }
// }
