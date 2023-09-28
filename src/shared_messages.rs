use crate::models::armylist::Faction;

pub enum SharedMessage {
    NoOp, // Dummy message for no-operation
    
    ClearRoster,
    LoadRoster,
    SaveRoster,

    ToggleMenu(Faction),
    ShowUnits(Faction),
    ShowCharacters(Faction),
    ShowSupports(Faction),

    AddToRoster(String, u32),
    NotifyRosterUpdated,
    DeleteElement(usize),

    FileSelected,
    FileContentReceived(String),

    ShowTooltip(usize),
    MoveTooltip(i32, i32),
    HideTooltip,

    // Add more if needed
}