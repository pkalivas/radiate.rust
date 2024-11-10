
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Id(usize);

impl Id {
    fn new(id: usize) -> Self {
        Id(id)
    }
}

impl Default for Id {
    fn default() -> Self {
        Id(0)
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}