use super::SeedString;

impl std::fmt::Display for SeedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.seed
            .iter()
            .skip_while(|c| **c == b' ')
            .try_for_each(|c| write!(f, "{}", *c as char))
    }
}
