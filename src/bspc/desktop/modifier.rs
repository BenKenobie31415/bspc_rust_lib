pub enum DesktopModifier {
    Id(String),
    Name(String),
    Focused,
    NotFocused,
    Active,
    NotActive,
    Occupied,
    NotOccupied,
    Urgent,
    NotUrgent,
    Local,
    NotLocal
}

impl DesktopModifier {
    pub fn get_string(&self) -> String {
        String::from(match self {
            DesktopModifier::Id(id) => id,
            DesktopModifier::Name(name) => name,
            DesktopModifier::Focused => ".focused",
            DesktopModifier::NotFocused => ".!focused",
            DesktopModifier::Active => ".active",
            DesktopModifier::NotActive => ".!active",
            DesktopModifier::Occupied => ".occupied",
            DesktopModifier::NotOccupied => ".!occupied",
            DesktopModifier::Urgent => ".urgent",
            DesktopModifier::NotUrgent => ".!urgent",
            DesktopModifier::Local => ".local",
            DesktopModifier::NotLocal => ".!local"
        })
    }
}
