use yew::prelude::*;
use crate::components::{top_menu::TopMenu, left_bar::LeftBar, right_bar::RightBar, main_canvas::MainCanvas};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <div class="top-menu">
                    <TopMenu />
                </div>
                <div class="body">
                    <div class="left-bar">
                        <LeftBar />
                    </div>
                    <div class="main-canvas">
                        <MainCanvas />
                    </div>
                    <div class="right-bar">
                        <RightBar />
                    </div>
                </div>
            </div>
        }
    }
}
