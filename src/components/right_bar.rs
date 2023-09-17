use yew::prelude::*;

pub struct RightBar;

impl Component for RightBar {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        RightBar
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div class="left-bar">
                <button>{"Unit 1"}</button><br/>
                <button>{"Unit 2"}</button><br/>
                <button>{"Unit 3"}</button><br/>
                <div class="details-section">
                    // This section will be used to display details in the future
                </div>
            </div>
        }
    }
}
