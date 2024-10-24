use yew::prelude::*;

use crate::models::roster::RosterElement;

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub model: Vec<RosterElement>,
    pub on_element_action: Callback<SharedMessage>,
    pub selected_element_index: Option<usize>, 
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
                    for ctx.props().model.iter().map(|elem| {
                        let callback = ctx.props().on_element_action.clone();
                        let selected_index = ctx.props().selected_element_index;

                        let (name, points) = &elem.clone().get_name_and_points();
                        let image = match &elem {
                            RosterElement::ElemCharacter(_) => {"character.png".to_string()}
                            RosterElement::ElemUnit(unit) => unit.image.clone(),
                            RosterElement::ElemSupport(_) => {"support.png".to_string()}
                            _ => {"".to_string()}
                        };
                        let button_string = name.clone().to_uppercase();
                        
                        let elem = elem.clone();

                        html! {
                            <button
                                onclick={Callback::from(move |_| {
                                    if let Some(index) = selected_index {
                                        // If an element is selected, send AddToElement
                                        callback.emit(SharedMessage::AddToElement(index, elem.clone()));
                                    } else {
                                        // No element selected, add to roster
                                        callback.emit(SharedMessage::AddToRoster(elem.clone()));
                                    }
                                })}
                                >
                                { name.to_uppercase() }
                                <br />
                                { format!("{} Points", &points) }
                            </button>
                        }
                    })
                }
            </div>
        }
    }
}
