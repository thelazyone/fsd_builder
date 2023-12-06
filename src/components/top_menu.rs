use yew::{prelude::*, callback};

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

pub struct TopMenu{
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_load_roster: Callback<SharedMessage>,
    pub on_save_roster: Callback<SharedMessage>,
    pub on_clear_roster: Callback<SharedMessage>,
    pub on_toggle_theme: Callback<SharedMessage>,

    pub is_dark_mode: bool,
}

impl Component for TopMenu {
    type Message = SharedMessage;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        TopMenu {
        }    
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // match msg {

        //     _ => panic!("Unhandled message!")
        // }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dark_mode_label = if ctx.props().is_dark_mode { "Light Mode" } else { "Dark Mode" };


        html! {
            <div class="top-menu">
                <div class="title">
                    <span class="title">{"FULL SPECTRUM DOMINANCE - ARMY BUILDER"}</span>
                </div>
                <div class="menu">
                    <button onclick={ctx.props().on_clear_roster.reform(|_| SharedMessage::ClearRoster)}>{"Clear Roster"}</button>
                    <button onclick={ctx.props().on_load_roster.reform(|_| SharedMessage::LoadRoster)}>{"Load Roster"}</button>
                    <button onclick={ctx.props().on_save_roster.reform(|_| SharedMessage::SaveRoster)}>{"Save Roster"}</button>
                    <button onclick={ctx.props().on_toggle_theme.reform(|_| SharedMessage::ToggleTheme)}>{dark_mode_label}</button> // TODO implement Light mode, depending on which one is on!
                    </div>
            </div>
        }
    }
}
