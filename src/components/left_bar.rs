use yew::prelude::*;
use crate::shared_messages::SharedMessage;

pub struct LeftBar {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_show_units: Callback<SharedMessage>,
    pub on_show_characters: Callback<SharedMessage>,
    pub on_show_supports: Callback<SharedMessage>,
}

impl Component for LeftBar {
    type Message = SharedMessage;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        LeftBar {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="left-bar">
                <button onclick={ctx.props().on_show_units.reform(|_| SharedMessage::ShowUnits)}>{"Add Unit"}</button>
                <button onclick={ctx.props().on_show_characters.reform(|_| SharedMessage::ShowCharacters)}>{"Add Character"}</button>
                <button onclick={ctx.props().on_show_supports.reform(|_| SharedMessage::ShowSupports)}>{"Add Support"}</button>
                <div class="details-section">
                    // This section will be used to display details in the future
                    // TODO
                </div>
            </div>
        }
    }
}