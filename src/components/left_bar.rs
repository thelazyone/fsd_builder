use yew::prelude::*;
use crate::shared_messages::SharedMessage;
use crate::models::armylist::Faction;
use web_sys::console;

pub struct LeftBar {
    expanded_menu: Option<Faction>,
}

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
        LeftBar {expanded_menu: None,}
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SharedMessage::ToggleMenu(faction) => {
                if self.expanded_menu.as_ref() == Some(&faction) {
                    self.expanded_menu = None;
                } else {
                    self.expanded_menu = Some(faction);
                }
                true
            }

            _ => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="left-bar">
                { self.render_menu(ctx, Faction::Tech) }
                { self.render_menu(ctx, Faction::Enlisted) }
                { self.render_menu(ctx, Faction::Conglomerate) }
                { self.render_menu(ctx, Faction::Union) }
            </div>
        }
    }
}

impl LeftBar {
    fn render_menu(&self, ctx: &Context<Self>, faction: Faction) -> Html {
        let is_expanded = self.expanded_menu.as_ref() == Some(&faction);
        let button_text = format!("{:?}", faction);
        let out_faction = faction.clone();


        html! {
            <div class={if is_expanded { "left-menu expanded" } else { "left-menu" }}>
                <button onclick={ctx.link().callback(move |_| SharedMessage::ToggleMenu(faction.clone()))}>
                    { button_text }
                </button>
                <div class="left-menu-content">
                    <button onclick={ctx.props().on_show_units.reform(move|_| SharedMessage::ShowUnits(out_faction))}>{"Add Units"}</button>
                    <button onclick={ctx.props().on_show_characters.reform(move|_| SharedMessage::ShowCharacters(out_faction))}>{"Add Characters"}</button>
                    <button onclick={ctx.props().on_show_supports.reform(move|_| SharedMessage::ShowSupports(out_faction))}>{"Add Supports"}</button>
                </div>
            </div>
        }
    }
}