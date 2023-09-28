use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub model: Vec<(String, u32)>,
    pub on_add_to_roster: Callback<(String, u32)>,
}

pub struct RightBar {}

impl Component for RightBar {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        RightBar {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="right-bar">
                { 
                    for ctx.props().model.iter().map(|(name, points)| {
                        let name = name.clone();
                        let points = *points;
                        let callback = ctx.props().on_add_to_roster.clone();
                        let button_string = name.clone().to_uppercase();
                        html!{
                            <button onclick={Callback::from(move |_| callback.emit((name.clone(), points)))}>
                                { button_string}<br />{ format!("{:?} Points", points) }
                            </button>
                        }
                    })
                }
            </div>
        }
    }
}
