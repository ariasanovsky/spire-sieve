use super::SeedString;

impl std::fmt::Display for SeedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.seed)
    }
}
