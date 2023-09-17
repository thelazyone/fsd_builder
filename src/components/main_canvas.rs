use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub enum Msg {
    AddUnit,
    AddCharacter,
    AddSupport,
}

pub struct MainCanvas;

impl Component for MainCanvas {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        MainCanvas
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

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
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
        }
    }
}
