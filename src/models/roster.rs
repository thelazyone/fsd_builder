use crate::models::character::Character;
use crate::models::unit::Unit;
use crate::models::support::Support;
use crate::models::element::Element;

enum RosterElement {
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

pub struct Roster {
    elements : Vec<RosterElement>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster {elements: Vec::<RosterElement>::new()}
    }

    pub fn load() -> Roster {
        let mut roster = Roster::new();

        // temporary loading, a for testing only 
        roster.add_element(Character{name: "char1".to_string(), points:2}.into());
        roster.add_element(Unit{name: "unit1".to_string(), points:3}.into());
        roster.add_element(Unit{name: "unit2".to_string(), points:4}.into());
        roster.add_element(Support{name: "support1".to_string(), points:5}.into());
        
        // TODO
        roster
    }

    fn add_element(&mut self, element: RosterElement) {
        self.elements.push(element);
    }
}