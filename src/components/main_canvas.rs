use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

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
        html! {
            <div class="central-area">
                // TODO the canvas should just fill the space!
                // Possibly even not being a canvas at all, just absolute positioning 
                // Of HTML buttons.
                <canvas id="game-canvas" width="800" height="600"></canvas>
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
    
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        console::log_1(&"CALLED RENDER!".into());
        
        let canvas: HtmlCanvasElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("game-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let roster = self.props.roster.borrow();
        console::log_1(&format!("Rendering canvas with {:?} elements", roster.elements.len()).into());

        // Clear the canvas before redrawing
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        for (i, elem) in roster.elements.iter().enumerate() {
            // Set the fill style for each element
            context.set_fill_style(&"blue".into());

            // Draw a rectangle for each element
            context.fill_rect(10.0, 10.0 + (i as f64 * 30.0), 100.0, 20.0);

            // Set the fill style for the text
            context.set_fill_style(&"white".into());

            // Draw the text for each element
            context.fill_text(&format!("{:?}", self.get_element_name(elem)), 20.0, 30.0 + (i as f64 * 30.0)).unwrap();
        }
    }

}

impl MainCanvas{
    // Simple rendering of the various elements of the roster.
    fn get_element_name(&self, elem: &RosterElement) -> String {
        match elem {
            RosterElement::ElemCharacter(character) => format!("Character: {:?}", character.name),
            RosterElement::ElemUnit(unit) => format!("Unit: {:?}", unit.name),
            RosterElement::ElemSupport(support) => format!("Support: {:?}", support.name),
            RosterElement::ElemOther((name, value))=> format!("Other: {} - {}", name, value),
        }
    }
}