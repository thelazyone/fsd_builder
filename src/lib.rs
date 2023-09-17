mod components {
    pub mod top_menu;
    pub mod left_bar;
    pub mod right_bar;
    pub mod main_canvas;
}
mod app;

use wasm_bindgen::prelude::*;
use web_sys::Element;
use crate::app::App;

#[wasm_bindgen]
pub fn init_app(root: Element) {
    yew::Renderer::<App>::with_root(root).render();
}