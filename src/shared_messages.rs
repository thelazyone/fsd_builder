use crate::models::armylist::Faction;

pub type GenericElementType = (String, u32, String);

pub enum SharedMessage {
    NoOp, // Dummy message for no-operation
    
    ClearRoster,
    LoadRoster,
    SaveRoster,

    ToggleMenu(Faction),
    ShowUnits(Faction),
    ShowCharacters(Faction),
    ShowSupports(Faction),

    AddToRoster(GenericElementType),
    AddToElement(usize /* Index of the element */, GenericElementType),
    NotifyRosterUpdated,
    DeleteElement(usize),
    ReorderElements,

    FileSelected,
    FileContentReceived(String),

    ShowTooltip(usize),
    MoveTooltip(i32, i32),
    HideTooltip,

    ToggleTheme,

    // Add more if needed
}