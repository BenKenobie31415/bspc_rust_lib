pub enum MonitorModifier {
    Focused,
    NotFocused,
    Occupied,
    NotOccupied
}

impl MonitorModifier {
    pub fn get_string(&self) -> String {
        String::from(match self {
            MonitorModifier::Focused => ".focused",
            MonitorModifier::NotFocused => ".!focused",
            MonitorModifier::Occupied => ".occupied",
            MonitorModifier::NotOccupied => ".!occupied",
        })
    }
}
