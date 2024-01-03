pub struct DesktopSelector {
    pub reference_selector: Option<String>,
    pub descriptor: Option<DesktopDescriptor>,
    pub modifiers: Vec<DesktopModifier>
}


pub enum DesktopDescriptor {
    // TODO path
    Any,
    Last,
    Newest,
    Older,
    Newer,
    Focused,
    Nth(u32),
    Id(String),
    Name(String)
}

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

impl DesktopSelector {
    pub(crate) fn assemble(&self) -> String {
        let mut result: String = match &self.reference_selector {
            Some(reference_selector) => format!("{}#", reference_selector),
            None => String::new()
        };
        match &self.descriptor {
            Some(descriptor) => {
                result.push_str(&descriptor.get_string());
            },
            None => {}
        }
        for modifier in &self.modifiers {
            result.push_str(&modifier.get_string());
        }
        result
    }
}

impl DesktopDescriptor {
    pub fn get_string(&self) -> String {
        match self {
            DesktopDescriptor::Any => String::from("any"),
            DesktopDescriptor::Last => String::from("last"),
            DesktopDescriptor::Newest => String::from("newest"),
            DesktopDescriptor::Older => String::from("older"),
            DesktopDescriptor::Newer => String::from("newer"),
            DesktopDescriptor::Focused => String::from("focused"),
            DesktopDescriptor::Nth(n) => format!("^{}", n),
            DesktopDescriptor::Id(id) => format!("{}", id),
            DesktopDescriptor::Name(name) => format!("{}", name)
        }
    }
}

impl DesktopModifier {
    pub fn get_string(&self) -> String {
        match self {
            DesktopModifier::Id(id) => id.clone(),
            DesktopModifier::Name(name) => name.clone(),
            DesktopModifier::Focused => String::from(".focused"),
            DesktopModifier::NotFocused => String::from(".!focused"),
            DesktopModifier::Active => String::from(".active"),
            DesktopModifier::NotActive => String::from(".!active"),
            DesktopModifier::Occupied => String::from(".occupied"),
            DesktopModifier::NotOccupied => String::from(".!occupied"),
            DesktopModifier::Urgent => String::from(".urgent"),
            DesktopModifier::NotUrgent => String::from(".!urgent"),
            DesktopModifier::Local => String::from(".local"),
            DesktopModifier::NotLocal => String::from(".!local")
        }
    }
}
