pub enum SharedMessage {
    NoOp, // Dummy message for no-operation
    
    ClearRoster,
    LoadRoster,
    SaveRoster,

    ShowUnits,
    ShowCharacters,
    ShowSupports,

    NotifyRosterUpdated,

    FileSelected,
    FileContentReceived(String),

    // Add more if needed
}