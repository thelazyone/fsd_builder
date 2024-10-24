use crate::models::armylist::Faction;
use crate::models::roster::RosterElement;

// pub type GenericElementType = (String, u32, Vec<String>, String);

pub enum SharedMessage {
    NoOp, // Dummy message for no-operation
    
    ClearRoster,
    LoadRoster,
    SaveRoster,

    ToggleMenu(Faction),
    ShowUnits(Faction),
    ShowCharacters(Faction),
    ShowSupports(Faction),

    AddToRoster(RosterElement),
    AddToElement(usize /* Index of the element */, RosterElement),
    RemoveCharacterFromElement(usize),
    NotifyRosterUpdated,
    DeleteElement(usize),
    ReorderElements,

    FileSelected,
    FileContentReceived(String),

    ShowTooltip(usize),
    MoveTooltip(i32, i32),
    HideTooltip,
    SelectElement(usize),
    DeselectElements,

    ToggleTheme,

    // Add more if needed
}