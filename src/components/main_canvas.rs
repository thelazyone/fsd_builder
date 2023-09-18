use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

// Pointer to roster, which is only one for the app.
use std::rc::Rc;
use std::cell::RefCell;

// For browser debugging
use web_sys::console;

// probably to remove (those messages are for right_bar)
pub enum Msg {
    AddUnit,
    AddCharacter,
    AddSupport,
}

// Using the Roster as a model for the canvas
use crate::models::roster::{Roster, RosterElement};

#[derive(Properties, Clone)]
pub struct Props {
    pub roster: Rc<RefCell<Roster>>,
    pub roster_updated: bool,  // Add a prop to receive the roster_updated state variable
}
// Note that this is a temp hack because comparing the content of roster 
// currently isn't a priority. This might change in the future.
// In particular it would be useful to check when to refresh.
impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.roster_updated == other.roster_updated
    }
}

pub struct MainCanvas {
    props: Props,
}

impl Component for MainCanvas {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        MainCanvas {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddUnit => {
                // Logic to add unit to canvas
            }
            Msg::AddCharacter => {
                // Logic to add character to canvas
            }
            Msg::AddSupport => {
                // Logic to add support to canvas
            }
        }
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div class="central-area">
                <canvas id="game-canvas" width="800" height="600"></canvas>
            </div>
        }
    }

    fn changed(&mut self, _: &Context<Self>, new_props: &Self::Properties) -> bool {
        // Manually handle the roster_updated bool to determine if the component should re-render
        if new_props.roster_updated != self.props.roster_updated {
            self.props.roster_updated = new_props.roster_updated;
            console::log_1(&"changed returned true".into());
            true
        } else {
            console::log_1(&"changed returned false".into());
            false
        }
    }
    
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
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
    
            // Here you can set up your canvas, like setting a background color, etc.
        } else {
            // When it's not the first render, we want to redraw the canvas with the new elements
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

}

impl MainCanvas{
    // Simple rendering of the various elements of the roster.
    // 
    // fn render_roster_element(&self, elem: &RosterElement) -> Html {
    //     match elem {
    //         RosterElement::ElemCharacter(character) => html! { <li>{ format!("Character: {:?}", character.name) }</li> },
    //         RosterElement::ElemUnit(unit) => html! { <li>{ format!("Unit: {:?}", unit.name) }</li> },
    //         RosterElement::ElemSupport(support) => html! { <li>{ format!("Support: {:?}", support.name) }</li> },
    //         RosterElement::ElemOther((name, value)) => html! { <li>{ format!("Other: {} - {}", name, value) }</li> },
    //     }
    //}
    
    fn get_element_name(&self, elem: &RosterElement) -> String {
        match elem {
            RosterElement::ElemCharacter(character) => format!("Character: {:?}", character.name),
            RosterElement::ElemUnit(unit) => format!("Unit: {:?}", unit.name),
            RosterElement::ElemSupport(support) => format!("Support: {:?}", support.name),
            RosterElement::ElemOther((name, value))=> format!("Other: {} - {}", name, value),
        }
    }
}