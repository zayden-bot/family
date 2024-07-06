#[derive(Debug, PartialEq)]
pub enum Relationship {
    Partner,
    Parent,
    Child,
    None,
}

impl std::fmt::Display for Relationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Relationship::Partner => write!(f, "Partner"),
            Relationship::Parent => write!(f, "Parent"),
            Relationship::Child => write!(f, "Child"),
            Relationship::None => write!(f, "None"),
        }
    }
}
