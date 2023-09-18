use yew::prelude::*;
use crate::components::{top_menu::TopMenu, left_bar::LeftBar, right_bar::RightBar, main_canvas::MainCanvas};
use crate::models::roster::Roster;

// Importing the quasi-static Armmylist
use crate::models::armylist::{ArmyList, self};

// Handling the roster as a pointer
use std::rc::Rc;
use std::cell::RefCell;

// For browser debugging
use web_sys::console;


pub enum Msg {

    // Roster Manipulation
    LoadRoster,
    ClearRoster,
    SaveRoster, 

    // Elements showing on right bar
    ShowUnits,
    ShowCharacters,
    ShowSupports,
}

pub struct App{

    // Roster Logic
    roster: Rc<RefCell<Roster>>,

    // State variable to track roster updates. 
    // This is because I'm lazy and I didn't implement the PartialEq.
    roster_updated: bool, 

    // Right Bar Model:
    right_bar_model: Vec<(String, u32)>,
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            roster: Rc::new(RefCell::new(Roster::new().into())),
            roster_updated: true,  // Initialize the state variable
            right_bar_model: Vec::<(String, u32)>::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg : Self::Message) -> bool {
        match msg {

            // Roster Handling
            Msg::LoadRoster => {
                console::log_1(&"Called LOAD for the roster".into());
                self.roster.borrow_mut().load(); // Load the roster when the message is received
                self.roster_updated = !self.roster_updated;  // Toggle the state variable
                true
            }
            Msg::SaveRoster => {

                // TODO

                true 
            }
            Msg::ClearRoster => {

                // TODO

                true
            }

            // Right Bar Visualization
            Msg::ShowUnits => {
                self.right_bar_model = armylist::ArmyList::new_tech().get_units().
                    into_iter().map(|elem| {elem}).collect();
                true
            }
            Msg::ShowCharacters => {
                self.right_bar_model = armylist::ArmyList::new_tech().get_characters().
                    into_iter().map(|elem| {elem}).collect();
                true
            }
            Msg::ShowSupports => {
                self.right_bar_model = armylist::ArmyList::new_tech().get_supports().
                    into_iter().map(|elem| {elem}).collect();
                true
            }

            _ => panic!("Unhandled message!")
        }    
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <div class="top-menu">
                <TopMenu on_load_roster= {ctx.link().callback(|_| Msg::LoadRoster)} />
                </div>
                <div class="left-bar">
                <LeftBar
                    on_show_units= {ctx.link().callback(|_| Msg::ShowUnits)} 
                    on_show_characters= {ctx.link().callback(|_| Msg::ShowCharacters)} 
                    on_show_supports= {ctx.link().callback(|_| Msg::ShowSupports)} 
                />
                </div>
                <div class="main-canvas">
                <MainCanvas roster={self.roster.clone()} roster_updated={self.roster_updated} />
                </div>
                <div class="right-bar">
                    <RightBar model={self.right_bar_model.clone()}/>
                </div>
            </div>
        }
    }
}
