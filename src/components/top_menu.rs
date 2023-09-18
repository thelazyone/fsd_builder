use yew::prelude::*;

pub struct TopMenu;

impl Component for TopMenu {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        TopMenu
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div class="top-menu">
                <div class="title">
                    <span class="title">{"FULL SPECTRUM DOMINANCE - ARMY BUILDER"}</span>
                </div>
                <div class="menu">
                    <button>{"Clear Roster"}</button>
                    <button>{"Load Roster"}</button>
                    <button>{"Export Roster"}</button>
                </div>
            </div>
        }
    }
}
