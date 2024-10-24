use crate::models::character::Character;
use crate::models::unit::Unit;
use crate::models::support::Support;

// For serialization
use serde::{Serialize, Deserialize};
use serde_json;

// For custom serde errors:
use serde::de;

// For browser debugging
use web_sys::console;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RosterElement {
    ElemCharacter(Character),
    ElemUnit(Unit),
    ElemSupport(Support),
    ElemOther((String, u32, Vec<String>, String)),
}

impl From<Character> for RosterElement {
    fn from(character: Character) -> Self {
        RosterElement::ElemCharacter(character)
    }
}

impl From<Unit> for RosterElement {
    fn from(unit: Unit) -> Self {
        RosterElement::ElemUnit(unit)
    }
}

impl From<Support> for RosterElement {
    fn from(support: Support) -> Self {
        RosterElement::ElemSupport(support)
    }
}

impl RosterElement {
    pub fn get_name_and_points(&self) -> (String, u32) {
        match self {
            RosterElement::ElemCharacter(elem) => {(elem.name.clone(), elem.points)}
            RosterElement::ElemUnit(elem) => {(elem.name.clone(), elem.points)}
            RosterElement::ElemSupport(elem) => {(elem.name.clone(), elem.points)}
            RosterElement::ElemOther(elem) => {(elem.0.clone(), elem.1)}
        }
    }
}


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Roster {
    pub version : u32,
    pub elements : Vec<RosterElement>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster {elements: Vec::<RosterElement>::new(), version: 1}
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn add_element(&mut self, element: RosterElement) {
        console::log_1(&format!("Adding element").into());
        self.elements.push(element);
        console::log_1(&format!("Now it has {:?} elements", self.elements.len()).into());
    }

    // JSON serialization (static methods):
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        
        let roster: Roster = serde_json::from_str(json_str)?;

        if roster.version < 1 { // Assuming 1 is the current version // TODO handle versioning better
            // Handle older versions differently
            // For now, just return an error
            return Err(de::Error::custom("Roster version is too old"));
        }
        Ok(roster)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    // TODO implement
    // fn check_validity (&self) -> Result(None, ) {

    //     // TODO implement 

    //     Ok();
    // }
}