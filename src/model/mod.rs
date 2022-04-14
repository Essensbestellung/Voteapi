use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote {
    pub name: String,
    pub order: Vec<VoteElement>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoteElement {
    pub position: i8,
    pub name: String,
}
impl std::fmt::Display for VoteElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Position: {} Name: {:?}", self.position, self.name)
    }
}

impl std::fmt::Display for Vote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: {} order: {:?}", self.name, self.order)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Result {
    pub timestamp: u64,
    pub elements: Vec<ResultElement>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultElement {
    pub votes: u32,
    pub name: String,
}

impl PartialEq<String> for ResultElement {
    fn eq(&self, other: &String) -> bool {
        self.name == other.clone()
    }
}
