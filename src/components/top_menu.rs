use yew::prelude::*;

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

pub enum Msg {
    LoadRosterClicked,
    ClearRosterClicked,
    SaveRosterClicked,
}

pub struct TopMenu{
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_load_roster: Callback<SharedMessage>,
    pub on_clear_roster: Callback<SharedMessage>,
    pub on_save_roster: Callback<SharedMessage>,
}

impl Component for TopMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        TopMenu {
        }    
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadRosterClicked => {
                ctx.props().on_load_roster.emit(SharedMessage::LoadRoster); 
                true
            }
            Msg::ClearRosterClicked => {
                ctx.props().on_clear_roster.emit(SharedMessage::ClearRoster); 
                true
            }
            Msg::LoadRosterClicked => {
                ctx.props().on_clear_roster.emit(SharedMessage::LoadRoster); 
                true
            }
            _ => panic!("Unhandled message!")
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="top-menu">
                <div class="title">
                    <span class="title">{"FULL SPECTRUM DOMINANCE - ARMY BUILDER"}</span>
                </div>
                <div class="menu">
                    <button onclick={ctx.link().callback(|_| Msg::ClearRosterClicked)}>{"Clear Roster"}</button>
                    <button onclick={ctx.link().callback(|_| Msg::LoadRosterClicked)}>{"Load Roster"}</button>
                    <button>{"Export Roster"}</button>
                </div>
            </div>
        }
    }
}
