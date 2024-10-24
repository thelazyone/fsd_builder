use super::element::Element;
use serde::{Serialize, Deserialize};
use crate::models::roster::RosterElement;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub name : String,
    pub points : u32,
    pub attached_elements : Vec<RosterElement>,
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
        self.attached_elements.iter().map(|elem| {self.get_name()}).collect()
    }
}