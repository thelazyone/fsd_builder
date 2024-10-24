use super::element::Element;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub name : String,
    pub points : u32,
    pub attached_elements : Vec<String>,
    pub image : String,
}

impl Element for Unit {
    fn get_name (&self) -> String {
        self.name.clone()
    }

    fn get_points (&self) -> u32 {
        self.points
    }

    fn get_attached (&self) -> Vec<String> {
        self.attached_elements.clone()
    }
}