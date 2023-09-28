// TODO this should be moved in a different folder than 
// "models", more like "data" or indeed "catalogue"
use crate::models::character::Character;
use crate::models::unit::Unit;
use crate::models::support::Support;
use crate::models::element::Element;

// For serialization
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Tech,
    Enlisted,
    Conglomerate,
    Union,
}

pub struct ArmyList {
    characters : Vec<Character>,
    units: Vec<Unit>,
    supports: Vec<Support>,
}


// Various utility functions for the armylist: 
impl ArmyList {

    pub fn get_characters (&self) -> Vec<(String, u32)> {
        let mut out_vec = Vec::<(String, u32)>::new();
        for elem in &self.characters {
            out_vec.push((elem.get_name(), elem.get_points()));
        }
        out_vec
    }

    pub fn get_units (&self) -> Vec<(String, u32)> {
        let mut out_vec = Vec::<(String, u32)>::new();
        for elem in &self.units {
            out_vec.push((elem.get_name(), elem.get_points()));
        }
        out_vec
    }

    pub fn get_supports (&self) -> Vec<(String, u32)> {
        let mut out_vec = Vec::<(String, u32)>::new();
        for elem in &self.supports {
            out_vec.push((elem.get_name(), elem.get_points()));
        }
        out_vec
    }
}


// this function currently contains ALL the information regarding the various factions.
// This would be probably better implemented in separate files. TODO.
//
// Another important TODO:
// The Elements should have a get_rules that returns a closure as part of the Trait.
// This allows special rules for different units to be implemented code wise.
impl ArmyList {
    pub fn new( faction : Faction) -> ArmyList {
        let mut new_list = ArmyList {
            characters: Vec::<Character>::new(),
            units: Vec::<Unit>::new(),
            supports: Vec::<Support>::new(),
        };

        match faction {
            Faction::Tech => {

                // Characters
                new_list.characters.push(Character {    name: "Sentient AI".to_string(),                            points: 3});
                new_list.characters.push(Character {    name: "Fire Control AI".to_string(),                        points: 3});
                new_list.characters.push(Character {    name: "Pilot AI".to_string(),                               points: 2});
            
                // Units
                new_list.units.push(Unit {              name: "Battle Robots".to_string(),                          points: 3});
                new_list.units.push(Unit {              name: "Heavy Robots".to_string(),                           points: 4});
                new_list.units.push(Unit {              name: "Light Spider Drones".to_string(),                    points: 2});
                new_list.units.push(Unit {              name: "Heavy Spider Drones".to_string(),                    points: 4});
                new_list.units.push(Unit {              name: "Socrates Battle Rig (Brawler)".to_string(),          points: 8});
                new_list.units.push(Unit {              name: "Socrates Battle Rig (Sharpshooter)".to_string(),     points: 8});
                new_list.units.push(Unit {              name: "Socrates Battle Rig (Demolition)".to_string(),       points: 8});
                new_list.units.push(Unit {              name: "Zeno Battle Rig".to_string(),                        points: 5});
                new_list.units.push(Unit {              name: "Solon Battle Tank (Minigun)".to_string(),            points: 6});
                new_list.units.push(Unit {              name: "Solon Battle Tank (Cannon)".to_string(),             points: 6});
                new_list.units.push(Unit {              name: "Syro Runner Rig".to_string(),                        points: 3});
                new_list.units.push(Unit {              name: "Thales Fighter".to_string(),                         points: 7});
                new_list.units.push(Unit {              name: "Gun Platform".to_string(),                           points: 7});
        
                // Supports
                new_list.supports.push(Support {        name: "Satellite Uplink".to_string(),                       points: 3});
                new_list.supports.push(Support {        name: "Cluster Strike".to_string(),                         points: 5});
                new_list.supports.push(Support {        name: "Eye in the Sky".to_string(),                         points: 4});
                new_list.supports.push(Support {        name: "Jamming".to_string(),                                points: 3});
                new_list.supports.push(Support {        name: "Orbital Bombing".to_string(),                        points: 4});
                new_list.supports.push(Support {        name: "Orbital Deployment".to_string(),                     points: 2});
                new_list.supports.push(Support {        name: "Software Upgrade".to_string(),                       points: 1});
                new_list.supports.push(Support {        name: "Twin Missile Strike".to_string(),                    points: 7});
            }
            Faction::Union => {

            }
            Faction::Conglomerate => {

            }
            Faction::Enlisted => {

            }
            _ => panic!("Faction not ready yet!")
        }

        new_list
    }
}