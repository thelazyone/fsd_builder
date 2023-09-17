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
            <div class="right-bar">
                // Add the content for the right bar here
            </div>
        }
    }
}
