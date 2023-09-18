use yew::prelude::*;

pub enum Msg {
    LoadRosterClicked,
    ClearRosterClicked,
    SaveRosterClicked,
}

pub struct TopMenu{
    on_load_roster: Callback<()>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_load_roster: Callback<()>,
}

impl Component for TopMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        TopMenu {
            on_load_roster: ctx.props().on_load_roster.clone(),
        }    
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadRosterClicked => {
                self.on_load_roster.emit(()); // Emit the callback when the button is clicked
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
                    <button>{"Clear Roster"}</button>
                    <button onclick={ctx.link().callback(|_| Msg::LoadRosterClicked)}>{"Load Roster"}</button>
                    <button>{"Export Roster"}</button>
                </div>
            </div>
        }
    }
}
