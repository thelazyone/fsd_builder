use yew::prelude::*;
use crate::components::{top_menu::TopMenu, left_bar::LeftBar, right_bar::RightBar, main_canvas::MainCanvas};
use crate::models::roster::Roster;

// Handling the roster as a pointer
use std::rc::Rc;
use std::cell::RefCell;

// For browser debugging
use web_sys::console;

pub enum Msg {
    LoadRoster,
    ClearRoster,
    SaveRoster, 
}

pub struct App{
    roster: Rc<RefCell<Roster>>,

    // State variable to track roster updates. 
    // This is because I'm lazy and I didn't implement the PartialEq.
    roster_updated: bool, 
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            roster: Rc::new(RefCell::new(Roster::new().into())),
            roster_updated: true,  // Initialize the state variable
        }
    }

    fn update(&mut self, _: &Context<Self>, msg : Self::Message) -> bool {
        match msg {
            Msg::LoadRoster => {
                console::log_1(&"Called LOAD for the roster".into());
                self.roster.borrow_mut().load(); // Load the roster when the message is received
                self.roster_updated = !self.roster_updated;  // Toggle the state variable
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
                    <LeftBar />
                </div>
                <div class="main-canvas">
                <MainCanvas roster={self.roster.clone()} roster_updated={self.roster_updated} />
                </div>
                <div class="right-bar">
                    <RightBar />
                </div>
            </div>
        }
    }
}
