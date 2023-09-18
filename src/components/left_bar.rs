use yew::prelude::*;

// For browser debugging
use web_sys::console;

pub struct LeftBar{
    pub on_show_units: Callback<()>,
    pub on_show_characters: Callback<()>,
    pub on_show_supports: Callback<()>,
}

pub enum Msg {
    ShowUnitsClicked,
    ShowCharactersClicked,
    ShowSupportsClicked,
}

// TODO absolutely not sure why this is needed.
// TODO find a way to explain this (gpt?)
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_show_units: Callback<()>,
    pub on_show_characters: Callback<()>,
    pub on_show_supports: Callback<()>,
}

impl Component for LeftBar {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        LeftBar{
            on_show_units: ctx.props().on_show_units.clone(),
            on_show_characters: ctx.props().on_show_characters.clone(),
            on_show_supports: ctx.props().on_show_supports.clone(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowCharactersClicked => {
                console::log_1(&"Called Show Characters".into());
                self.on_show_characters.emit(()); // Emit the callback when the button is clicked
                true
            }

            Msg::ShowUnitsClicked => {
                console::log_1(&"Called Show Units".into());
                self.on_show_units.emit(()); // Emit the callback when the button is clicked
                true
            }

            Msg::ShowSupportsClicked => {
                console::log_1(&"Called Show Supports".into());
                self.on_show_supports.emit(()); // Emit the callback when the button is clicked
                true
            }
            _ => panic!("Unhandled message!")
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="left-bar">
                
                <button onclick={ctx.link().callback(|_| Msg::ShowUnitsClicked)}>{"Add Unit"}</button>
                <button onclick={ctx.link().callback(|_| Msg::ShowCharactersClicked)}>{"Add Character"}</button>
                <button onclick={ctx.link().callback(|_| Msg::ShowSupportsClicked)}>{"Add Support"}</button>

                <div class="details-section">
                    // This section will be used to display details in the future
                    // TODO
                </div>

            </div>
        }
    }
}
