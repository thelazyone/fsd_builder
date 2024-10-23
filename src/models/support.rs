use super::element::Element;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Support {
    pub name : String,
    pub points : u32,
}

impl Element for Support {
    fn get_name (&self) -> String {
        self.name.clone()
    }

    fn get_points (&self) -> u32 {
        self.points
    }

    fn get_attached(&self) -> Vec<String> {
        Vec::<String>::new()
    }
}