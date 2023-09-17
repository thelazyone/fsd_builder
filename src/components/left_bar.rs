use yew::prelude::*;

pub struct LeftBar;

impl Component for LeftBar {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        LeftBar
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div class="left-bar">
                <button>{"Add Unit"}</button><br/>
                <button>{"Add Character"}</button><br/>
                <button>{"Add Support"}</button><br/>
                <div class="details-section">
                    // This section will be used to display details in the future
                </div>
            </div>
        }
    }
}
