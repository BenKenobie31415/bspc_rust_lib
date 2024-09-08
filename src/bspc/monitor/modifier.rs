use crate::bspc::modifier::Modifier;

pub enum MonitorModifier {
    Focused,
    NotFocused,
    Occupied,
    NotOccupied
}

impl Modifier for MonitorModifier {
    fn get_string(&self) -> String {
        String::from(match self {
            MonitorModifier::Focused => ".focused",
            MonitorModifier::NotFocused => ".!focused",
            MonitorModifier::Occupied => ".occupied",
            MonitorModifier::NotOccupied => ".!occupied",
        })
    }
}
