use crate::models::character::Character;
use crate::models::unit::Unit;
use crate::models::support::Support;

// For serialization
use serde::{Serialize, Deserialize};
use serde_json;

// For custom serde errors:
use serde::de;


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum RosterElement {
    ElemCharacter(Character),
    ElemUnit(Unit),
    ElemSupport(Support),
    ElemOther((String, u32)),
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
        self.elements.push(element);
    }

    // JSON serialization (static methods):
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        
        let roster: Roster = serde_json::from_str(json_str)?;

        // let mut roster: Roster = serde_json::from_str(json_str)?;
        // roster.add_element(Character{name: "char1".to_string(), points:2}.into());
        // roster.add_element(Unit{name: "unit1".to_string(), points:3}.into());
        // roster.add_element(Unit{name: "unit2".to_string(), points:4}.into());
        // roster.add_element(Support{name: "support1".to_string(), points:5}.into());

        if roster.version < 1 { // Assuming 1 is the current version
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