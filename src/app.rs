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

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

pub struct App{

    // Roster Logic
    roster: Rc<RefCell<Roster>>,

    // Right Bar Model:
    right_bar_model: Vec<(String, u32)>,
}


impl Component for App {
    type Message = SharedMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            roster: Rc::new(RefCell::new(Roster::new().into())),
            right_bar_model: Vec::<(String, u32)>::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg : Self::Message) -> bool {
        match msg {

            // Roster Handling
            // SharedMessage::LoadRoster => {
            //     console::log_1(&"Called LOAD for the roster".into());
            //     self.roster.borrow_mut().load(); // Load the roster when the message is received
            //     false
            // }
            // Roster Handling
            SharedMessage::LoadRoster => {
                console::log_1(&"Called LOAD for the roster".into());
                self.roster.borrow_mut().load();
                ctx.link().callback(|_| SharedMessage::NotifyRosterUpdated).emit(());
                true
            }
            SharedMessage::SaveRoster => {

                // TODO

                false 
            }
            SharedMessage::ClearRoster => {
                console::log_1(&"Called CLEAR for the roster".into());
                self.roster.borrow_mut().clear();
                ctx.link().callback(|_| SharedMessage::NotifyRosterUpdated).emit(());
                true            
            }

            _ => false // Passing to the child objects to be handled.
        }    
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <div class="top-menu">
                    <TopMenu 
                        on_load_roster= {ctx.link().callback(|_| SharedMessage::LoadRoster)} 
                        on_clear_roster= {ctx.link().callback(|_| SharedMessage::ClearRoster)} 
                        on_save_roster= {ctx.link().callback(|_| SharedMessage::SaveRoster)} 
                    />
                </div>
                <div class="left-bar">
                    <LeftBar
                        on_show_units= {ctx.link().callback(|_| SharedMessage::ShowUnits)} 
                        on_show_characters= {ctx.link().callback(|_| SharedMessage::ShowCharacters)} 
                        on_show_supports= {ctx.link().callback(|_| SharedMessage::ShowSupports)} 
                    />
                </div>
                <div class="main-canvas">
                    <MainCanvas 
                        roster={self.roster.clone()} 
                        on_roster_updated={ctx.link().callback(|_| SharedMessage::NotifyRosterUpdated)}
                    />
                </div>
                <div class="right-bar">
                    <RightBar 
                        model={self.right_bar_model.clone()}
                    />
                </div>
            </div>
        }
    }
}
