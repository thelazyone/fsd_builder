use crate::models::character::Character;
use crate::models::unit::Unit;
use crate::models::support::Support;

#[derive(Clone, PartialEq)]
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


#[derive(Clone, PartialEq)]
pub struct Roster {
    pub elements : Vec<RosterElement>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster {elements: Vec::<RosterElement>::new()}
    }

    pub fn load(&mut self) {
        self.clear();
        self.add_element(Character{name: "char1".to_string(), points:2}.into());
        self.add_element(Unit{name: "unit1".to_string(), points:3}.into());
        self.add_element(Unit{name: "unit2".to_string(), points:4}.into());
        self.add_element(Support{name: "support1".to_string(), points:5}.into());
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    fn add_element(&mut self, element: RosterElement) {
        self.elements.push(element);
    }

    // TODO implement
    // fn check_validity (&self) -> Result(None, ) {

    //     // TODO implement 

    //     Ok();
    // }
}