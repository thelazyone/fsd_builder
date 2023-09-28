use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::{top_menu::TopMenu, left_bar::LeftBar, right_bar::RightBar, main_canvas::MainCanvas};
use crate::models::roster::Roster;

// Importing the quasi-static Armmylist
use crate::models::armylist::{ArmyList, self};

// Handling the roster as a pointer
use std::rc::Rc;
use std::cell::RefCell;

// For the file selection
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// For browser debugging
use web_sys::console;

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

use crate::models::roster::RosterElement;

#[wasm_bindgen]
extern "C" {
    fn downloadFile(content: &str, filename: &str);
}

pub struct App{

    // Roster Logic
    roster: Rc<RefCell<Roster>>,

    // Right Bar Model:
    right_bar_model: Vec<(String, u32)>,

    // input file
    file_input_ref: NodeRef,
}


impl Component for App {
    type Message = SharedMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let file_input_ref = NodeRef::default();

        if let Some(input) = file_input_ref.cast::<web_sys::HtmlInputElement>() {
            let link = ctx.link().clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {

                // Handle the selected file here
                
                // ... rest of the code to process the file
                link.send_message(SharedMessage::FileSelected);
            }) as Box<dyn FnMut(_)>);
    
            input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget(); // Don't forget to forget the closure to prevent it from being dropped
        }
        
        App {
            roster: Rc::new(RefCell::new(Roster::new().into())),
            right_bar_model: Vec::<(String, u32)>::new(),
            file_input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg : Self::Message) -> bool {
        match msg {

            SharedMessage::LoadRoster => {
                if let Some(input) = self.file_input_ref.cast::<web_sys::HtmlInputElement>() {
                    
                    // This will only trigger the file dialog. Everything else has been set up as a closure
                    // In the create() method.
                    input.click(); 
                }
                true
            }

            SharedMessage::SaveRoster => {
                match self.roster.borrow().to_json() {
                    Ok(json_string) => {

                        // Trigger a file download with the JSON string
                        downloadFile(&json_string, "roster.json");
                    },
                    Err(e) => {
                        console::log_1(&format!("Error serializing roster: {:?}", e).into());
                    }
                }
                false
            }

            SharedMessage::ClearRoster => {
                console::log_1(&"Called CLEAR for the roster".into());
                self.roster.borrow_mut().clear();
                ctx.link().callback(|_| SharedMessage::NotifyRosterUpdated).emit(());
                true            
            }

            SharedMessage::FileContentReceived(text) => {
                match Roster::from_json(&text) {
                    Ok(roster) => {
                        *self.roster.borrow_mut() = roster; // todo can't 
                    }

                    Err(e) => {
                        // Do something TODO
                    }
                }

                true
            }

            SharedMessage::ShowUnits => {
                self.right_bar_model = armylist::ArmyList::new_tech().get_units().
                    into_iter().map(|elem| {elem}).collect();
                true
            }
            SharedMessage::ShowCharacters => {
                self.right_bar_model = armylist::ArmyList::new_tech().get_characters().
                    into_iter().map(|elem| {elem}).collect();
                true
            }
            SharedMessage::ShowSupports => {
                self.right_bar_model = armylist::ArmyList::new_tech().get_supports().
                    into_iter().map(|elem| {elem}).collect();
                true
            }
    
            SharedMessage::AddToRoster(name, points) => {
                self.roster.borrow_mut().add_element(RosterElement::ElemOther((name, points)).into()); // Implement the add_element method
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
                        on_add_to_roster={ctx.link().callback(|(name, points)| SharedMessage::AddToRoster(name, points))}
                    />                    
                </div>

            // File Selection Popup
            <input type="file" ref={self.file_input_ref.clone()} style="display: none" onchange={
                let link_clone_outer = ctx.link().clone(); // Clone the link outside of the callback
                let link_clone_inner = link_clone_outer.clone(); // Clone the link for the inner closure
                link_clone_outer.callback(move |event: web_sys::Event| {
                    if let Some(target) = event.target() {
                        if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                            if let Some(files) = input.files() {
                                if let Some(file) = files.get(0) {
                                    
                                    // Get the name of the file
                                    let file_name = file.name();
                                    console::log_1(&format!("Selected file name: {}", file_name).into());
                        
                                    // Read the content of the file
                                    let file_reader = web_sys::FileReader::new().unwrap();
                                    let file_reader_rc = Rc::new(file_reader); // Wrap the FileReader in an Rc
                                    let file_reader_clone = file_reader_rc.clone(); // Clone the Rc for the closure

                                     // Clone the link for the onload closure. Yep, another cloning.
                                    let link_clone_for_onload = link_clone_inner.clone();
                                    let onload_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                                        if let Ok(result) = file_reader_clone.result() {
                                            if let Some(text) = result.as_string() {
                                                
                                                link_clone_for_onload.send_message(SharedMessage::FileContentReceived(text));
                                            }
                                        }
                                    }) as Box<dyn FnMut(_)>);
                        
                                    file_reader_rc.add_event_listener_with_callback("load", onload_closure.as_ref().unchecked_ref()).unwrap();
                                    onload_closure.forget();
                        
                                    file_reader_rc.read_as_text(&file).unwrap();
                                }
                            }
                        }
                    }
                    SharedMessage::NoOp // Return a dummy message
                })
            }/>
        </div>
        }
    }
}
