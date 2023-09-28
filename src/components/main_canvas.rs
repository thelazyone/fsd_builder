use yew::prelude::*;

// Pointer to roster, which is only one for the app.
use std::rc::Rc;
use std::cell::RefCell;

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

// For browser debugging
use web_sys::console;

// Using the Roster as a model for the canvas
use crate::models::roster::{Roster, RosterElement};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub roster: Rc<RefCell<Roster>>,
    pub on_roster_updated: Callback<()>
}

pub struct MainCanvas {
    props: Props,
}

impl Component for MainCanvas {
    type Message = SharedMessage;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        MainCanvas {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SharedMessage::NotifyRosterUpdated => {
                console::log_1(&"Roster updated notification received in MAIN CANVAS".into());
                true
            }
            _ => panic!("Wrong message received!")
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let roster = self.props.roster.borrow();
        html! {
            <div class="central-area">
                {
                    for roster.elements.iter().enumerate().map(|(i, elem)| {
                        html!{
                            <div class="hoverable-area" data-tooltip={self.get_tooltip_content(elem)}>
                                { self.get_element_name(elem) }
                            </div>
                        }
                    })
                }
            </div>
        }
    }

    fn changed(&mut self, _: &Context<Self>, new_props: &Self::Properties) -> bool {
        let old_elements = &self.props.roster.borrow().elements.clone();
        let new_elements = &new_props.roster.borrow().elements.clone();
        
        console::log_1(&"CALLED CHANGED!".into());
        console::log_1(&format!("Old Roster has {:?} elements", old_elements.len()).into());
        console::log_1(&format!("New Roster has {:?} elements", new_elements.len()).into());
        
        self.props = new_props.clone();
        true
    }
}

impl MainCanvas {
    // Simple rendering of the various elements of the roster.
    fn get_element_name(&self, elem: &RosterElement) -> String {
        match elem {
            RosterElement::ElemCharacter(character) => format!("Character: {:?}", character.name),
            RosterElement::ElemUnit(unit) => format!("Unit: {:?}", unit.name),
            RosterElement::ElemSupport(support) => format!("Support: {:?}", support.name),
            RosterElement::ElemOther((name, value)) => format!("{} - {}", name, value),
        }
    }

    fn get_tooltip_content(&self, elem: &RosterElement) -> String {
        match elem {
            RosterElement::ElemCharacter(character) => format!("Character Details: {:?}", character.name),
            RosterElement::ElemUnit(unit) => format!("Unit Details: {:?}", unit.name),
            RosterElement::ElemSupport(support) => format!("Support Details: {:?}", support.name),
            RosterElement::ElemOther((name, value)) => format!("Other Details: {} - {}", name, value),
        }
    }
}
