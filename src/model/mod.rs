use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote {
    pub name: String,
    pub order: Vec<Vote_Element>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote_Element {
    pub position: i8,
    pub name: String,
}
impl std::fmt::Display for Vote_Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "Position: {} Name: {:?}",
               self.position,
               self.name)
    }
}


impl std::fmt::Display for Vote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "name: {} order: {:?}",
               self.name,
               self.order)
    }
}

