use yew::prelude::*;

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

pub struct TopMenu{
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_load_roster: Callback<SharedMessage>,
    pub on_save_roster: Callback<SharedMessage>,
    pub on_clear_roster: Callback<SharedMessage>,
}

impl Component for TopMenu {
    type Message = SharedMessage;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        TopMenu {
        }    
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg {

        //     _ => panic!("Unhandled message!")
        // }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="top-menu">
                <div class="title">
                    <span class="title">{"FULL SPECTRUM DOMINANCE - ARMY BUILDER"}</span>
                </div>
                <div class="menu">
                    <button onclick={ctx.props().on_clear_roster.reform(|_| SharedMessage::ClearRoster)}>{"Clear Roster"}</button>
                    <button onclick={ctx.props().on_load_roster.reform(|_| SharedMessage::LoadRoster)}>{"Load Roster"}</button>
                    <button onclick={ctx.props().on_action.reform(|_| SharedMessage::SaveRoster)}>{"Save Roster"}</button>
                </div>
            </div>
        }
    }
}
