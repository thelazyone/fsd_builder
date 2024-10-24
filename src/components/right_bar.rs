use yew::prelude::*;

use crate::models::roster::RosterElement;

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub model: Vec<RosterElement>,
    pub on_element_action: Callback<SharedMessage>,
    pub selected_element_index: Option<usize>, 
    pub selected_element_is_unit: bool,
    pub selected_unit_has_character: bool,
    pub on_deselect_elements: Callback<SharedMessage>,
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

                        // Duplicating the elem for some ownership reason, not entirely clear.
                        let elem = elem.clone();

                        let callback = ctx.props().on_element_action.clone();
                        let selected_index = ctx.props().selected_element_index;

                        let (name, points) = &elem.clone().get_name_and_points();
                        let image = match &elem {
                            RosterElement::ElemCharacter(_) => {"character.png".to_string()}
                            RosterElement::ElemUnit(unit) => unit.image.clone(),
                            RosterElement::ElemSupport(_) => {"support.png".to_string()}
                            _ => {"".to_string()}
                        };

                        // Must check what to grey out
                        let is_character = matches!(elem, RosterElement::ElemCharacter(_));
                        let mut disable_button = false;
                        if is_character {
                            // Disable if no unit is selected or unit already has a character
                            if !ctx.props().selected_element_is_unit || ctx.props().selected_unit_has_character {
                                disable_button = true;
                            }
                        }
                        
                        html! {
                            <button
                                disabled={disable_button}
                                onclick={Callback::from(move |_| {
                                    let mut should_be_attached: bool = false;
                                    if let Some(index) = selected_index {
                                        // Checking if attachable.
                                        if let RosterElement::ElemCharacter(character) = elem.clone() {
                                            should_be_attached = true;
                                            callback.emit(SharedMessage::AddToElement(index, elem.clone()));
                                        }
                                    }
                                    if !should_be_attached {
                                        // No element selected, add to roster
                                        callback.emit(SharedMessage::DeselectElements);
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
                // Add "Remove Character" button if applicable
                {
                    if ctx.props().selected_element_is_unit && ctx.props().selected_unit_has_character {
                        let callback = ctx.props().on_element_action.clone();
                        let selected_index = ctx.props().selected_element_index.unwrap();
                        html! {
                            <button
                                onclick={Callback::from(move |_| {
                                    callback.emit(SharedMessage::RemoveCharacterFromElement(selected_index));
                                })}
                                >
                                { "REMOVE CHARACTER" }
                            </button>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}
