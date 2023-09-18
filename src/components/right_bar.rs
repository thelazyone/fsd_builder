use yew::prelude::*;

// For browser debugging
use web_sys::console;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub model: Vec<(String, u32)>,
}

pub struct RightBar{
    props: Props,
}

pub enum Msg {
    ShowUnits,
    ShowCharacters,
    ShowSupports,
}

impl Component for RightBar {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        RightBar {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _: &Context<Self>, new_props: &Self::Properties) -> bool {
        // Manually handle the roster_updated bool to determine if the component should re-render
        if new_props.model!= self.props.model {
            self.props.model = new_props.model.clone();
            console::log_1(&"Right Bar Props changed!".into());
            true
        } else {
            console::log_1(&"Right Bar Props not changed!".into());
            false
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        console::log_1(&"Right Bar Test".into());
        for _ in &self.props.model {
            console::log_1(&"Adding Button".into());
        }
        html! {
            <div class="right-bar">
                { for self.props.model.iter().map(|elem|  html!{<li>{ format!("{:?}<br>{:?}", elem.0, elem.1) }</li>} ) }

                <div class="details-section">
                    // This section will be used to display details in the future
                </div>
            </div>
        }
    }
}
